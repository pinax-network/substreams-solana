use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::jupiter::v1 as pb;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta};
use substreams_solana_idls::jupiter;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

/// Process a transaction to extract Jupiter V4 instructions and events
fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let instructions = process_logs(tx_meta);
    if instructions.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
    })
}

/// Process transaction logs to extract Jupiter V4 instructions
fn process_logs(tx_meta: &TransactionStatusMeta) -> Vec<pb::Instruction> {
    let mut instructions = Vec::new();
    let mut is_invoked = false;
    let mut current_stack_height = 0;

    for log_message in &tx_meta.log_messages {
        let is_jupiter_program = parse_program_id(log_message).map_or(false, |id| id == jupiter::v4::PROGRAM_ID.to_vec());

        // Track program invocation and stack height
        if is_invoke(log_message) && is_jupiter_program {
            if let Some(height) = parse_invoke_depth(log_message) {
                current_stack_height = height - 1; // stack height is 1-based
                is_invoked = true;

                // Continue to next log message as invoke logs don't contain program data
                continue;
            }
        }

        // Skip processing if not in Jupiter V4 context
        if !is_invoked {
            continue;
        }

        // Try to parse program data from log
        if let Some(instruction) = parse_log_instruction(log_message, current_stack_height) {
            instructions.push(instruction);
        }
    }

    instructions
}

/// Parse a log message to extract Jupiter V4 instruction data
fn parse_log_instruction(log_message: &str, stack_height: u32) -> Option<pb::Instruction> {
    let data = parse_program_data(log_message)?;

    match jupiter::v4::anchor_self_cpi::unpack(data.as_slice()) {
        Ok(jupiter::v4::anchor_self_cpi::JupiterV4Event::Swap(event)) => Some(pb::Instruction {
            program_id: jupiter::v4::PROGRAM_ID.to_vec(),
            stack_height,
            instruction: Some(pb::instruction::Instruction::SwapEvent(pb::SwapEvent {
                amm: event.amm.to_bytes().to_vec(),
                input_mint: event.input_mint.to_bytes().to_vec(),
                input_amount: event.input_amount,
                output_mint: event.output_mint.to_bytes().to_vec(),
                output_amount: event.output_amount,
            })),
        }),
        _ => None,
    }
}
