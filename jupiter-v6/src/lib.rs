use common::solana::{get_fee_payer, get_signers};
use proto::pb::jupiter::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::jupiter;

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

    if program_id != &jupiter::v6::PROGRAM_ID.to_vec() {
        return None;
    }

    match jupiter::v6::anchor_self_cpi::unpack(instruction.data()) {
        Ok(jupiter::v6::anchor_self_cpi::JupiterV6Event::Swap(event)) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: instruction.stack_height(),
            instruction: Some(pb::instruction::Instruction::SwapEvent(pb::SwapEvent {
                amm: event.amm.to_bytes().to_vec(),
                input_mint: event.input_mint.to_bytes().to_vec(),
                input_amount: event.input_amount,
                output_mint: event.output_mint.to_bytes().to_vec(),
                output_amount: event.output_amount,
            })),
        }),
        Ok(jupiter::v6::anchor_self_cpi::JupiterV6Event::Fee(event)) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: instruction.stack_height(),
            instruction: Some(pb::instruction::Instruction::FeeEvent(pb::FeeEvent {
                account: event.account.to_bytes().to_vec(),
                mint: event.mint.to_bytes().to_vec(),
                amount: event.amount,
            })),
        }),
        _ => None,
    }
}
