use common::solana::{get_fee_payer, get_signers};
use proto::pb::pumpfun::amm::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::pumpfun::amm as pumpfun;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    if instructions.is_empty() {
        return None;
    }
    Some(pb::Transaction {
        fee: tx.meta.as_ref()?.fee,
        compute_units_consumed: tx.meta.as_ref()?.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
    })
}

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;

    if program_id != &pumpfun::PROGRAM_ID.to_vec() {
        return None;
    }

    process_instruction1(instruction)
        .or_else(|| process_instruction2(instruction))
        .map(|parsed_instruction| pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: instruction.stack_height(),
            instruction: Some(parsed_instruction),
        })
}

fn process_instruction1(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match pumpfun::instructions::unpack(instruction.data()) {
        // -- Buy --
        Ok(pumpfun::instructions::PumpFunAmmInstruction::Buy(event)) => Some(pb::instruction::Instruction::BuyInstruction(pb::BuyInstruction {
            accounts: Some(get_trade_accounts(instruction)),
            base_amount_out: event.base_amount_out,
            max_quote_amount_in: event.max_quote_amount_in,
        })),
        // -- Sell --
        Ok(pumpfun::instructions::PumpFunAmmInstruction::Sell(event)) => Some(pb::instruction::Instruction::SellInstruction(pb::SellInstruction {
            accounts: Some(get_trade_accounts(instruction)),
            base_amount_in: event.base_amount_in,
            min_quote_amount_out: event.min_quote_amount_out,
        })),
        // -- CreatePool V2 --
        Ok(pumpfun::instructions::PumpFunAmmInstruction::CreatePoolV2(event)) => {
            Some(pb::instruction::Instruction::CreatePoolInstruction(pb::CreatePoolInstruction {
                index: event.index as u32,
                base_amount_in: event.base_amount_in,
                quote_amount_in: event.quote_amount_in,
                coin_creator: Some(event.coin_creator.to_bytes().to_vec()),
            }))
        }
        // -- CreatePool V1 --
        Ok(pumpfun::instructions::PumpFunAmmInstruction::CreatePoolV1(event)) => {
            Some(pb::instruction::Instruction::CreatePoolInstruction(pb::CreatePoolInstruction {
                index: event.index as u32,
                base_amount_in: event.base_amount_in,
                quote_amount_in: event.quote_amount_in,
                coin_creator: None,
            }))
        }
        _ => None,
    }
}
fn process_instruction2(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match pumpfun::anchor_self_cpi::unpack(instruction.data()) {
        // -- Buy V1 --
        Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::BuyEventV1(event)) => Some(pb::instruction::Instruction::BuyEvent(pb::BuyEvent {
            base_amount_out: event.base_amount_out,
            max_quote_amount_in: event.max_quote_amount_in,
            quote_amount_in: event.quote_amount_in,
            quote_amount_in_with_lp_fee: event.quote_amount_in_with_lp_fee,
            user_quote_amount_in: event.user_quote_amount_in,

            // Trade details
            trade: Some(pb::TradeDetails {
                user_base_token_reserves: event.user_base_token_reserves,
                user_quote_token_reserves: event.user_quote_token_reserves,
                pool_base_token_reserves: event.pool_base_token_reserves,
                pool_quote_token_reserves: event.pool_quote_token_reserves,
                lp_fee_basis_points: event.lp_fee_basis_points,
                lp_fee: event.lp_fee,
                protocol_fee_basis_points: event.protocol_fee_basis_points,
                protocol_fee: event.protocol_fee,
                pool: event.pool.to_bytes().to_vec(),
                user: event.user.to_bytes().to_vec(),
                user_base_token_account: event.user_base_token_account.to_bytes().to_vec(),
                user_quote_token_account: event.user_quote_token_account.to_bytes().to_vec(),
                protocol_fee_recipient: event.protocol_fee_recipient.to_bytes().to_vec(),
                protocol_fee_recipient_token_account: event.protocol_fee_recipient_token_account.to_bytes().to_vec(),
                coin_creator: None,
                coin_creator_fee_basis_points: None,
                coin_creator_fee: None,
            }),
        })),
        // -- Sell V1 --
        Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::SellEventV1(event)) => Some(pb::instruction::Instruction::SellEvent(pb::SellEvent {
            base_amount_in: event.base_amount_in,
            min_quote_amount_out: event.min_quote_amount_out,
            quote_amount_out: event.quote_amount_out,
            quote_amount_out_without_lp_fee: event.quote_amount_out_without_lp_fee,
            user_quote_amount_out: event.user_quote_amount_out,

            // Trade details
            trade: Some(pb::TradeDetails {
                user_base_token_reserves: event.user_base_token_reserves,
                user_quote_token_reserves: event.user_quote_token_reserves,
                pool_base_token_reserves: event.pool_base_token_reserves,
                pool_quote_token_reserves: event.pool_quote_token_reserves,
                lp_fee_basis_points: event.lp_fee_basis_points,
                lp_fee: event.lp_fee,
                protocol_fee_basis_points: event.protocol_fee_basis_points,
                protocol_fee: event.protocol_fee,
                pool: event.pool.to_bytes().to_vec(),
                user: event.user.to_bytes().to_vec(),
                user_base_token_account: event.user_base_token_account.to_bytes().to_vec(),
                user_quote_token_account: event.user_quote_token_account.to_bytes().to_vec(),
                protocol_fee_recipient: event.protocol_fee_recipient.to_bytes().to_vec(),
                protocol_fee_recipient_token_account: event.protocol_fee_recipient_token_account.to_bytes().to_vec(),
                coin_creator: None,
                coin_creator_fee_basis_points: None,
                coin_creator_fee: None,
            }),
        })),
        // -- Buy V2 --
        Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::BuyEventV2(event)) => Some(pb::instruction::Instruction::BuyEvent(pb::BuyEvent {
            base_amount_out: event.base_amount_out,
            max_quote_amount_in: event.max_quote_amount_in,
            quote_amount_in: event.quote_amount_in,
            quote_amount_in_with_lp_fee: event.quote_amount_in_with_lp_fee,
            user_quote_amount_in: event.user_quote_amount_in,

            // Trade details
            trade: Some(pb::TradeDetails {
                user_base_token_reserves: event.user_base_token_reserves,
                user_quote_token_reserves: event.user_quote_token_reserves,
                pool_base_token_reserves: event.pool_base_token_reserves,
                pool_quote_token_reserves: event.pool_quote_token_reserves,
                lp_fee_basis_points: event.lp_fee_basis_points,
                lp_fee: event.lp_fee,
                protocol_fee_basis_points: event.protocol_fee_basis_points,
                protocol_fee: event.protocol_fee,
                pool: event.pool.to_bytes().to_vec(),
                user: event.user.to_bytes().to_vec(),
                user_base_token_account: event.user_base_token_account.to_bytes().to_vec(),
                user_quote_token_account: event.user_quote_token_account.to_bytes().to_vec(),
                protocol_fee_recipient: event.protocol_fee_recipient.to_bytes().to_vec(),
                protocol_fee_recipient_token_account: event.protocol_fee_recipient_token_account.to_bytes().to_vec(),
                coin_creator: Some(event.coin_creator.to_bytes().to_vec()),
                coin_creator_fee_basis_points: Some(event.coin_creator_fee_basis_points),
                coin_creator_fee: Some(event.coin_creator_fee),
            }),
        })),
        // -- Sell V2 --
        Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::SellEventV2(event)) => Some(pb::instruction::Instruction::SellEvent(pb::SellEvent {
            base_amount_in: event.base_amount_in,
            min_quote_amount_out: event.min_quote_amount_out,
            quote_amount_out: event.quote_amount_out,
            quote_amount_out_without_lp_fee: event.quote_amount_out_without_lp_fee,
            user_quote_amount_out: event.user_quote_amount_out,

            // Trade details
            trade: Some(pb::TradeDetails {
                user_base_token_reserves: event.user_base_token_reserves,
                user_quote_token_reserves: event.user_quote_token_reserves,
                pool_base_token_reserves: event.pool_base_token_reserves,
                pool_quote_token_reserves: event.pool_quote_token_reserves,
                lp_fee_basis_points: event.lp_fee_basis_points,
                lp_fee: event.lp_fee,
                protocol_fee_basis_points: event.protocol_fee_basis_points,
                protocol_fee: event.protocol_fee,
                pool: event.pool.to_bytes().to_vec(),
                user: event.user.to_bytes().to_vec(),
                user_base_token_account: event.user_base_token_account.to_bytes().to_vec(),
                user_quote_token_account: event.user_quote_token_account.to_bytes().to_vec(),
                protocol_fee_recipient: event.protocol_fee_recipient.to_bytes().to_vec(),
                protocol_fee_recipient_token_account: event.protocol_fee_recipient_token_account.to_bytes().to_vec(),
                coin_creator: Some(event.coin_creator.to_bytes().to_vec()),
                coin_creator_fee_basis_points: Some(event.coin_creator_fee_basis_points),
                coin_creator_fee: Some(event.coin_creator_fee),
            }),
        })),
        // -- CreatePool V1 --
        Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::CreatePoolEventV1(event)) => Some(pb::instruction::Instruction::CreatePoolEvent(pb::CreatePoolEvent {
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
        })),
        // -- CreatePool V2 --
        Ok(pumpfun::anchor_self_cpi::PumpFunAmmEvent::CreatePoolEventV2(event)) => Some(pb::instruction::Instruction::CreatePoolEvent(pb::CreatePoolEvent {
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
        })),
        _ => None,
    }
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
