use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::openbook::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::openbook;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &openbook::PROGRAM_ID.to_vec());

    if instructions.is_empty() && logs.is_empty() {
        return None;
    }

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

    if program_id != &openbook::PROGRAM_ID {
        return None;
    }

    match openbook::instructions::unpack(ix.data()) {
        Ok(openbook::instructions::OpenbookInstruction::PlaceOrder) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: ix.stack_height(),
            instruction: Some(pb::instruction::Instruction::PlaceOrder(pb::PlaceOrderInstruction {})),
        }),
        Ok(openbook::instructions::OpenbookInstruction::PlaceOrders) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: ix.stack_height(),
            instruction: Some(pb::instruction::Instruction::PlaceOrders(pb::PlaceOrdersInstruction {})),
        }),
        Ok(openbook::instructions::OpenbookInstruction::PlaceTakeOrder) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: ix.stack_height(),
            instruction: Some(pb::instruction::Instruction::PlaceTakeOrder(pb::PlaceTakeOrderInstruction {})),
        }),
        Ok(openbook::instructions::OpenbookInstruction::CancelAllAndPlaceOrders) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: ix.stack_height(),
            instruction: Some(pb::instruction::Instruction::CancelAllAndPlaceOrders(pb::CancelAllAndPlaceOrdersInstruction {})),
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
            if let Some(invoke_depth) = parse_invoke_depth(log_message) {
                is_invoked = true;
                if let Some(log_data) = parse_log_data(log_message, program_id_bytes, invoke_depth) {
                    logs.push(log_data);
                }
            }
        } else if match_program_id && (is_success(log_message) || is_failed(log_message)) {
            is_invoked = false;
        } else if is_invoked {
            if let Some(log_data) = parse_log_data(log_message, program_id_bytes, 0) {
                logs.push(log_data);
            }
        }
    }

    logs
}

fn parse_log_data(log_message: &str, program_id_bytes: &[u8], invoke_depth: u32) -> Option<pb::Log> {
    let data = parse_program_data(log_message)?;
    match openbook::events::unpack(data.as_slice()) {
        Ok(openbook::events::OpenbookEvent::FillLog(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::FillLog(pb::FillLogEvent {
                market: event.market.to_bytes().to_vec(),
                taker_side: event.taker_side as u32,
                maker_slot: event.maker_slot as u32,
                maker_out: event.maker_out,
                timestamp: event.timestamp,
                seq_num: event.seq_num,
                maker: event.maker.to_bytes().to_vec(),
                maker_client_order_id: event.maker_client_order_id,
                maker_fee: event.maker_fee,
                maker_timestamp: event.maker_timestamp,
                taker: event.taker.to_bytes().to_vec(),
                taker_client_order_id: event.taker_client_order_id,
                taker_fee_ceil: event.taker_fee_ceil,
                price: event.price,
                quantity: event.quantity,
            })),
        }),
        Ok(openbook::events::OpenbookEvent::TotalOrderFillEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::TotalOrderFill(pb::TotalOrderFillEvent {
                side: event.side as u32,
                taker: event.taker.to_bytes().to_vec(),
                total_quantity_paid: event.total_quantity_paid,
                total_quantity_received: event.total_quantity_received,
                fees: event.fees,
            })),
        }),
        _ => None,
    }
}
