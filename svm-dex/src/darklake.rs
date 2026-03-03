use common::db::{common_key_v2, set_clock};
use proto::pb::darklake::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            if let Some(pb::instruction::Instruction::Swap(_data)) = &ix.instruction {
                if let Some(event) = get_swap_event(tx, instruction_index) {
                    handle_swap(tables, clock, tx, ix, event, transaction_index, instruction_index);
                }
            }
        }
    }
}

fn get_swap_event(tx: &pb::Transaction, instruction_index: usize) -> Option<&pb::SwapEvent> {
    for i in instruction_index..tx.logs.len() {
        if let Some(pb::log::Log::Swap(ev)) = &tx.logs[i].log {
            return Some(ev);
        }
    }
    None
}

fn handle_swap(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    event: &pb::SwapEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("darklake_swap", key)
        .set("trader", base58::encode(&event.trader))
        .set("amount_in", event.amount_in)
        .set("amount_out", event.amount_out)
        .set("token_mint_x", base58::encode(&event.token_mint_x))
        .set("token_mint_y", base58::encode(&event.token_mint_y))
        .set("direction", event.direction)
        .set("trade_fee", event.trade_fee)
        .set("protocol_fee", event.protocol_fee);
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
