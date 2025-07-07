use common::solana::{get_fee_payer, get_signers};
use proto::pb::pumpfun::amm::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{base58, block_view::InstructionView, pb::sf::solana::r#type::v1::Block};
use substreams_solana_idls::pumpfun::amm as pumpfun;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, Error> {
    let mut events = pb::Events::default();

    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&params);

    // transactions
    for tx in block.transactions() {
        let mut transaction = pb::Transaction::default();
        let tx_meta = tx.meta.as_ref().expect("Transaction meta should be present");
        transaction.fee = tx_meta.fee;
        transaction.compute_units_consumed = tx_meta.compute_units_consumed();
        transaction.signature = tx.hash().to_vec();

        if let Some(fee_payer) = get_fee_payer(tx) {
            transaction.fee_payer = fee_payer;
        }
        if let Some(signers) = get_signers(tx) {
            transaction.signers = signers;
        }

        // Include instructions and events
        for instruction in tx.walk_instructions() {
            let program_id = instruction.program_id().0;

            // Skip instructions
            if program_id != &pumpfun::PROGRAM_ID.to_vec() {
                continue;
            }

            let mut base = pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: instruction.stack_height(),
                instruction: None,
            };

            // -- Instructions --
            match pumpfun::instructions::unpack(instruction.data()) {
                // -- Buy --
                Ok(pumpfun::instructions::PumpFunAmmInstruction::Buy(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::BuyInstruction(pb::BuyInstruction {
                        accounts: Some(get_trade_accounts(&instruction)),
                        base_amount_out: event.base_amount_out,
                        max_quote_amount_in: event.max_quote_amount_in,
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- Sell --
                Ok(pumpfun::instructions::PumpFunAmmInstruction::Sell(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::SellInstruction(pb::SellInstruction {
                        accounts: Some(get_trade_accounts(&instruction)),
                        base_amount_in: event.base_amount_in,
                        min_quote_amount_out: event.min_quote_amount_out,
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- CreatePool V2 --
                Ok(pumpfun::instructions::PumpFunAmmInstruction::CreatePoolV2(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::CreatePoolInstruction(pb::CreatePoolInstruction {
                        index: event.index as u32,
                        base_amount_in: event.base_amount_in,
                        quote_amount_in: event.quote_amount_in,
                        coin_creator: Some(event.coin_creator.to_bytes().to_vec()),
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- CreatePool V1 --
                Ok(pumpfun::instructions::PumpFunAmmInstruction::CreatePoolV1(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::CreatePoolInstruction(pb::CreatePoolInstruction {
                        index: event.index as u32,
                        base_amount_in: event.base_amount_in,
                        quote_amount_in: event.quote_amount_in,
                        coin_creator: None,
                    }));
                    transaction.instructions.push(base.clone());
                }
                _ => {}
            }
            // -- Events --
            match pumpfun::anchor_self_cpi::unpack(instruction.data()) {
                // -- Buy V2 --
                Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::BuyEventV2(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::BuyEvent(pb::BuyEvent {
                        timestamp: event.timestamp,
                        base_amount_out: event.base_amount_out,
                        max_quote_amount_in: event.max_quote_amount_in,
                        user_base_token_reserves: event.user_base_token_reserves,
                        user_quote_token_reserves: event.user_quote_token_reserves,
                        pool_base_token_reserves: event.pool_base_token_reserves,
                        pool_quote_token_reserves: event.pool_quote_token_reserves,
                        quote_amount_in: event.quote_amount_in,
                        lp_fee_basis_points: event.lp_fee_basis_points,
                        lp_fee: event.lp_fee,
                        protocol_fee_basis_points: event.protocol_fee_basis_points,
                        protocol_fee: event.protocol_fee,
                        quote_amount_in_with_lp_fee: event.quote_amount_in_with_lp_fee,
                        user_quote_amount_in: event.user_quote_amount_in,
                        pool: event.pool.to_bytes().to_vec(),
                        user: event.user.to_bytes().to_vec(),
                        user_base_token_account: event.user_base_token_account.to_bytes().to_vec(),
                        user_quote_token_account: event.user_quote_token_account.to_bytes().to_vec(),
                        protocol_fee_recipient: event.protocol_fee_recipient.to_bytes().to_vec(),
                        protocol_fee_recipient_token_account: event.protocol_fee_recipient_token_account.to_bytes().to_vec(),
                        coin_creator: Some(event.coin_creator.to_bytes().to_vec()),
                        coin_creator_fee_basis_points: Some(event.coin_creator_fee_basis_points),
                        coin_creator_fee: Some(event.coin_creator_fee),
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- Sell V2 --
                Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::SellEventV2(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::SellEvent(pb::SellEvent {
                        timestamp: event.timestamp,
                        base_amount_in: event.base_amount_in,
                        min_quote_amount_out: event.min_quote_amount_out,
                        user_base_token_reserves: event.user_base_token_reserves,
                        user_quote_token_reserves: event.user_quote_token_reserves,
                        pool_base_token_reserves: event.pool_base_token_reserves,
                        pool_quote_token_reserves: event.pool_quote_token_reserves,
                        quote_amount_out: event.quote_amount_out,
                        lp_fee_basis_points: event.lp_fee_basis_points,
                        lp_fee: event.lp_fee,
                        protocol_fee_basis_points: event.protocol_fee_basis_points,
                        protocol_fee: event.protocol_fee,
                        quote_amount_out_without_lp_fee: event.quote_amount_out_without_lp_fee,
                        user_quote_amount_out: event.user_quote_amount_out,
                        pool: event.pool.to_bytes().to_vec(),
                        user: event.user.to_bytes().to_vec(),
                        user_base_token_account: event.user_base_token_account.to_bytes().to_vec(),
                        user_quote_token_account: event.user_quote_token_account.to_bytes().to_vec(),
                        protocol_fee_recipient: event.protocol_fee_recipient.to_bytes().to_vec(),
                        protocol_fee_recipient_token_account: event.protocol_fee_recipient_token_account.to_bytes().to_vec(),
                        coin_creator: Some(event.coin_creator.to_bytes().to_vec()),
                        coin_creator_fee_basis_points: Some(event.coin_creator_fee_basis_points),
                        coin_creator_fee: Some(event.coin_creator_fee),
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- CreatePool V1 --
                Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::CreatePoolEventV1(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::CreatePoolEvent(pb::CreatePoolEvent {
                        timestamp: event.timestamp,
                        index: event.index as u32,
                        creator: event.creator.to_bytes().to_vec(),
                        base_mint: event.base_mint.to_bytes().to_vec(),
                        quote_mint: event.quote_mint.to_bytes().to_vec(),
                        base_mint_decimals: event.base_mint_decimals as u32,
                        quote_mint_decimals: event.quote_mint_decimals as u32,
                        base_amount_in: event.base_amount_in,
                        quote_amount_in: event.quote_amount_in,
                        pool_base_amount: event.pool_base_amount,
                        pool_quote_amount: event.pool_quote_amount,
                        minimum_liquidity: event.minimum_liquidity,
                        initial_liquidity: event.initial_liquidity,
                        lp_token_amount_out: event.lp_token_amount_out,
                        pool_bump: event.pool_bump as u32,
                        pool: event.pool.to_bytes().to_vec(),
                        lp_mint: event.lp_mint.to_bytes().to_vec(),
                        user_base_token_account: event.user_base_token_account.to_bytes().to_vec(),
                        user_quote_token_account: event.user_quote_token_account.to_bytes().to_vec(),
                        coin_creator: None,
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- CreatePool V2 --
                Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::CreatePoolEventV2(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::CreatePoolEvent(pb::CreatePoolEvent {
                        timestamp: event.timestamp,
                        index: event.index as u32,
                        creator: event.creator.to_bytes().to_vec(),
                        base_mint: event.base_mint.to_bytes().to_vec(),
                        quote_mint: event.quote_mint.to_bytes().to_vec(),
                        base_mint_decimals: event.base_mint_decimals as u32,
                        quote_mint_decimals: event.quote_mint_decimals as u32,
                        base_amount_in: event.base_amount_in,
                        quote_amount_in: event.quote_amount_in,
                        pool_base_amount: event.pool_base_amount,
                        pool_quote_amount: event.pool_quote_amount,
                        minimum_liquidity: event.minimum_liquidity,
                        initial_liquidity: event.initial_liquidity,
                        lp_token_amount_out: event.lp_token_amount_out,
                        pool_bump: event.pool_bump as u32,
                        pool: event.pool.to_bytes().to_vec(),
                        lp_mint: event.lp_mint.to_bytes().to_vec(),
                        user_base_token_account: event.user_base_token_account.to_bytes().to_vec(),
                        user_quote_token_account: event.user_quote_token_account.to_bytes().to_vec(),
                        coin_creator: Some(event.coin_creator.to_bytes().to_vec()),
                    }));
                    transaction.instructions.push(base.clone());
                }
                _ => {}
            }
        }
        if !transaction.instructions.is_empty() {
            events.transactions.push(transaction);
        }
    }
    Ok(events)
}

pub fn get_trade_accounts(instruction: &InstructionView) -> pb::TradeAccounts {
    pb::TradeAccounts {
        pool: instruction.accounts()[1 - 1].0.to_vec(),
        user: instruction.accounts()[2 - 1].0.to_vec(),
        global_config: instruction.accounts()[3 - 1].0.to_vec(),
        base_mint: instruction.accounts()[4 - 1].0.to_vec(),
        quote_mint: instruction.accounts()[5 - 1].0.to_vec(),
        user_base_token_account: instruction.accounts()[6 - 1].0.to_vec(),
        user_quote_token_account: instruction.accounts()[7 - 1].0.to_vec(),
        pool_base_token_account: instruction.accounts()[8 - 1].0.to_vec(),
        pool_quote_token_account: instruction.accounts()[9 - 1].0.to_vec(),
        protocol_fee_recipient: instruction.accounts()[10 - 1].0.to_vec(),
        protocol_fee_recipient_token_account: instruction.accounts()[11 - 1].0.to_vec(),
        coin_creator_vault_ata: instruction.accounts().get(18 - 1).map(|a| a.0.to_vec()),
        coin_creator_vault_authority: instruction.accounts().get(19 - 1).map(|a| a.0.to_vec()),
    }
}
