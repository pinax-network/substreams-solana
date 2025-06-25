use proto::pb::pumpfun::v1 as pb;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana_idls::pumpfun;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    let mut events = pb::Events::default();

    // transactions
    for tx in block.transactions() {
        let mut transaction = pb::Transaction::default();
        transaction.signature = tx.hash().to_vec();

        // Include instructions and events
        for instruction in tx.walk_instructions() {
            let program_id = instruction.program_id().0;

            // Must be the PumpFun Bonding Curve program ID
            if program_id.to_vec() != pumpfun::PROGRAM_ID {
                continue;
            }
            let meta = instruction.meta();
            let mut base = pb::Instruction {
                program_id: program_id.to_vec(),
                fee: meta.fee,
                stack_height: instruction.stack_height(),
                compute_units_consumed: meta.compute_units_consumed(),
                instruction: None,
            };

            // -- Instructions --
            match pumpfun::instructions::unpack(instruction.data()) {
                // -- Buy --
                Ok(pumpfun::instructions::PumpFunInstruction::Buy(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                        amount: event.amount,
                        max_sol_cost: event.max_sol_cost,
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- Sell --
                Ok(pumpfun::instructions::PumpFunInstruction::Sell(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::Sell(pb::SellInstruction {
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
