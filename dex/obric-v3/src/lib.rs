use common::solana::{get_fee_payer, get_signers};
use proto::pb::obric::v3::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::obric;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    if instructions.is_empty() { return None; }
    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
        logs: vec![],
    })
}

fn process_instruction(ix: &InstructionView) -> Option<pb::Instruction> {
    let program_id = ix.program_id().0;
    if program_id != &obric::v3::PROGRAM_ID { return None; }

    match obric::v3::instructions::unpack(ix.data()) {
        Ok(obric::v3::instructions::ObricV3Instruction::SwapXToY(event)) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: ix.stack_height(),
            instruction: Some(pb::instruction::Instruction::SwapXToY(pb::SwapXToYInstruction {
                input_amount: event.input_x,
                min_output_amount: event.min_output_amt,
            })),
        }),
        Ok(obric::v3::instructions::ObricV3Instruction::SwapYToX(event)) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: ix.stack_height(),
            instruction: Some(pb::instruction::Instruction::SwapYToX(pb::SwapYToXInstruction {
                input_amount: event.input_y,
                min_output_amount: event.min_output_amt,
            })),
        }),
        _ => None,
    }
}
