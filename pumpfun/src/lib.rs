use proto::pb::pumpfun::v1::{instruction, BuyInstruction, Events, Instruction, SellInstruction, TradeEvent, Transaction};
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana_idls::pumpfun;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, Error> {
    let mut events: Events = Events::default();

    // transactions
    for tx in block.transactions() {
        let mut transaction = Transaction::default();
        transaction.signature = tx.hash().to_vec();

        // Include instructions and events
        for instruction in tx.walk_instructions() {
            let program_id = instruction.program_id().0;

            // Must be the PumpFun Bonding Curve program ID
            if program_id.to_vec() != pumpfun::PROGRAM_ID {
                continue;
            }
            let meta = instruction.meta();
            let base_instruction = Instruction {
                program_id: program_id.to_vec(),
                fee: meta.fee,
                stack_height: instruction.stack_height(),
                instruction: None,
            };

            // -- Instructions --
            match pumpfun::instructions::unpack(instruction.data()) {
                // -- Buy --
                Ok(pumpfun::instructions::PumpFunInstruction::Buy(event)) => {
                    let mut inst = base_instruction.clone();
                    inst.instruction = Some(instruction::Instruction::Buy(BuyInstruction {
                        amount: event.amount,
                        max_sol_cost: event.max_sol_cost,
                    }));
                    transaction.instructions.push(inst);
                }
                // -- Sell --
                Ok(pumpfun::instructions::PumpFunInstruction::Sell(event)) => {
                    let mut inst = base_instruction.clone();
                    inst.instruction = Some(instruction::Instruction::Sell(SellInstruction {
                        amount: event.amount,
                        min_sol_output: event.min_sol_output,
                    }));
                    transaction.instructions.push(inst);
                }
                _ => {}
            }

            // -- Events --
            match pumpfun::events::unpack(instruction.data()) {
                // -- TradeV1 --
                Ok(pumpfun::events::PumpFunEvent::TradeV1(event)) => {
                    let mut inst = base_instruction.clone();
                    inst.instruction = Some(instruction::Instruction::Trade(TradeEvent {
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
                    transaction.instructions.push(inst);
                }
                // -- TradeV2 --
                Ok(pumpfun::events::PumpFunEvent::TradeV2(event)) => {
                    let mut inst = base_instruction.clone();
                    inst.instruction = Some(instruction::Instruction::Trade(TradeEvent {
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
                    transaction.instructions.push(inst);
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
