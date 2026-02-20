use common::db::{common_key_v2, set_clock};
use proto::pb::openbook::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(pb::log::Log::FillLog(event)) => {
                    let ix = if log_index < tx.instructions.len() { &tx.instructions[log_index] } else { continue };
                    handle_fill_log(tables, clock, tx, ix, event, transaction_index, log_index);
                }
                Some(pb::log::Log::TotalOrderFill(event)) => {
                    let ix = if log_index < tx.instructions.len() { &tx.instructions[log_index] } else { continue };
                    handle_total_order_fill(tables, clock, tx, ix, event, transaction_index, log_index);
                }
                _ => {}
            }
        }
    }
}

fn handle_fill_log(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    event: &pb::FillLogEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("openbook_fill", key)
        .set("market", base58::encode(&event.market))
        .set("maker", base58::encode(&event.maker))
        .set("taker", base58::encode(&event.taker))
        .set("price", event.price)
        .set("quantity", event.quantity)
        .set("taker_side", event.taker_side)
        .set("seq_num", event.seq_num);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_total_order_fill(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    event: &pb::TotalOrderFillEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("openbook_total_order_fill", key)
        .set("taker", base58::encode(&event.taker))
        .set("side", event.side)
        .set("total_quantity_paid", event.total_quantity_paid)
        .set("total_quantity_received", event.total_quantity_received)
        .set("fees", event.fees);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn set_transaction(tx: &pb::Transaction, row: &mut Row) {
    row.set("signature", base58::encode(&tx.signature))
        .set("fee_payer", base58::encode(&tx.fee_payer))
        .set("signers_raw", tx.signers.iter().map(base58::encode).collect::<Vec<_>>().join(","))
        .set("fee", tx.fee)
        .set("compute_units_consumed", tx.compute_units_consumed);
}

fn set_instruction(ix: &pb::Instruction, row: &mut Row) {
    row.set("program_id", base58::encode(&ix.program_id)).set("stack_height", ix.stack_height);
}
