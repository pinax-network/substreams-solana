use common::solana::{get_fee_payer, get_signers};
use proto::pb::pumpfun::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{base58, block_view::InstructionView, pb::sf::solana::r#type::v1::Block};
use substreams_solana_idls::pumpfun::bonding_curve as pumpfun;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, Error> {
    let mut events = pb::Events::default();

    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&params);

    // transactions
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
            if !matcher.matches_keys(&vec![format!("program:{}", base58::encode(&program_id))]) {
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
                Ok(pumpfun::instructions::PumpFunInstruction::Buy(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                        accounts: Some(get_trade_accounts(&instruction)),
                        amount: event.amount,
                        max_sol_cost: event.max_sol_cost,
                    }));
                    transaction.instructions.push(base.clone());

                    // TO-DO: include SPL-Tokens & Native SOL transfers
                    // for inner in instruction.inner_instructions() {
                    // }
                }
                // -- Sell --
                Ok(pumpfun::instructions::PumpFunInstruction::Sell(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::Sell(pb::SellInstruction {
                        accounts: Some(get_trade_accounts(&instruction)),
                        amount: event.amount,
                        min_sol_output: event.min_sol_output,
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- Create --
                Ok(pumpfun::instructions::PumpFunInstruction::Create(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::Create(pb::CreateInstruction {
                        name: event.name,
                        symbol: event.symbol,
                        uri: event.uri,
                        creator: event.creator.to_bytes().to_vec(),
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- Initialize --
                Ok(pumpfun::instructions::PumpFunInstruction::Initialize) => {
                    base.instruction = Some(pb::instruction::Instruction::Initialize(pb::InitializeInstruction {}));
                    transaction.instructions.push(base.clone());
                }
                // -- Withdraw --
                Ok(pumpfun::instructions::PumpFunInstruction::Withdraw) => {
                    base.instruction = Some(pb::instruction::Instruction::Withdraw(pb::WithdrawInstruction {}));
                    transaction.instructions.push(base.clone());
                }
                // -- SetParams --
                Ok(pumpfun::instructions::PumpFunInstruction::SetParams(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::SetParams(pb::SetParamsInstruction {
                        fee_recipient: event.fee_recipient.to_bytes().to_vec(),
                        initial_virtual_token_reserves: event.initial_virtual_token_reserves,
                        initial_virtual_sol_reserves: event.initial_virtual_sol_reserves,
                        initial_real_token_reserves: event.initial_real_token_reserves,
                        token_total_supply: event.token_total_supply,
                        fee_basis_points: event.fee_basis_points,
                    }));
                    transaction.instructions.push(base.clone());
                }
                _ => {}
            }
            // -- Events --
            match pumpfun::events::unpack(instruction.data()) {
                // -- TradeV1 --
                Ok(pumpfun::events::PumpFunEvent::TradeV1(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::Trade(pb::TradeEvent {
                        mint: event.mint.to_bytes().to_vec(),
                        sol_amount: event.sol_amount,
                        token_amount: event.token_amount,
                        is_buy: event.is_buy,
                        user: event.user.to_bytes().to_vec(),
                        timestamp: event.timestamp,
                        virtual_sol_reserves: event.virtual_sol_reserves,
                        virtual_token_reserves: event.virtual_token_reserves,
                        real_sol_reserves: event.real_sol_reserves,
                        real_token_reserves: event.real_token_reserves,
                        fee_recipient: None,
                        fee_basis_points: None,
                        fee: None,
                        creator: None,
                        creator_fee_basis_points: None,
                        creator_fee: None,
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- TradeV2 --
                Ok(pumpfun::events::PumpFunEvent::TradeV2(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::Trade(pb::TradeEvent {
                        mint: event.mint.to_bytes().to_vec(),
                        sol_amount: event.sol_amount,
                        token_amount: event.token_amount,
                        is_buy: event.is_buy,
                        user: event.user.to_bytes().to_vec(),
                        timestamp: event.timestamp,
                        virtual_sol_reserves: event.virtual_sol_reserves,
                        virtual_token_reserves: event.virtual_token_reserves,
                        real_sol_reserves: event.real_sol_reserves,
                        real_token_reserves: event.real_token_reserves,
                        fee_recipient: Some(event.fee_recipient.to_bytes().to_vec()),
                        fee_basis_points: Some(event.fee_basis_points),
                        fee: Some(event.fee),
                        creator: Some(event.creator.to_bytes().to_vec()),
                        creator_fee_basis_points: Some(event.creator_fee_basis_points),
                        creator_fee: Some(event.creator_fee),
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
        global: instruction.accounts()[0].0.to_vec(),
        fee_recipient: instruction.accounts()[1].0.to_vec(),
        mint: instruction.accounts()[2].0.to_vec(),
        bonding_curve: instruction.accounts()[3].0.to_vec(),
        associated_bonding_curve: instruction.accounts()[4].0.to_vec(),
        associated_user: instruction.accounts()[5].0.to_vec(),
        user: instruction.accounts()[6].0.to_vec(),
        creator_vault: instruction.accounts()[9].0.to_vec(),
    }
}
