use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::byreal::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::byreal;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &byreal::PROGRAM_ID.to_vec());
    if instructions.is_empty() && logs.is_empty() { return None; }
    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
        logs,
    })
}

fn process_instruction(ix: &InstructionView) -> Option<pb::Instruction> {
    let program_id = ix.program_id().0;
    if program_id != &byreal::PROGRAM_ID { return None; }

    match byreal::clmm::instructions::unpack(ix.data()) {
        Ok(byreal::clmm::instructions::ByRealInstruction::Swap(event)) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: ix.stack_height(),
            instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                amount_in: event.amount,
                minimum_amount_out: event.other_amount_threshold,
            })),
        }),
        _ => None,
    }
}

fn process_logs(tx_meta: &TransactionStatusMeta, program_id_bytes: &[u8]) -> Vec<pb::Log> {
    let mut logs = Vec::new();
    let mut is_invoked = false;
    for log_message in tx_meta.log_messages.iter() {
        let match_program_id = parse_program_id(log_message).map_or(false, |id| id == program_id_bytes.to_vec());
        if is_invoke(log_message) && match_program_id {
            if let Some(_invoke_depth) = parse_invoke_depth(log_message) { is_invoked = true; }
        } else if match_program_id && (is_success(log_message) || is_failed(log_message)) {
            is_invoked = false;
        } else if is_invoked {
            // ByReal CLMM events can be parsed here if needed
        }
    }
    logs
}
