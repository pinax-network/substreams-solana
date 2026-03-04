use common::db::{common_key_v2, set_clock};
use proto::pb::tensor::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        // Process log-based take events (sales)
        for (log_index, log) in tx.logs.iter().enumerate() {
            if let Some(pb::log::Log::Take(event)) = &log.log {
                handle_take(tables, clock, tx, log, event, transaction_index, log_index);
            }
        }
        // Process instruction-based list events
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            match &ix.instruction {
                Some(pb::instruction::Instruction::List(data)) => {
                    handle_list(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::Buy(data)) => {
                    handle_buy(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::Bid(data)) => {
                    handle_bid(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                _ => {}
            }
        }
    }
}

fn handle_take(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    log: &pb::Log,
    event: &pb::TakeEvent,
    transaction_index: usize,
    log_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, log_index);
    let row = tables
        .create_row("tensor_take", key)
        .set("taker", base58::encode(&event.taker))
        .set("bid_id", event.bid_id.as_deref().map(base58::encode).unwrap_or_default())
        .set("target", event.target)
        .set("target_id", base58::encode(&event.target_id))
        .set("amount", event.amount)
        .set("quantity", event.quantity)
        .set("tcomp_fee", event.tcomp_fee)
        .set("taker_broker_fee", event.taker_broker_fee)
        .set("maker_broker_fee", event.maker_broker_fee)
        .set("creator_fee", event.creator_fee)
        .set("currency", event.currency.as_deref().map(base58::encode).unwrap_or_default())
        .set("asset_id", event.asset_id.as_deref().map(base58::encode).unwrap_or_default())
        .set("program_id", base58::encode(&log.program_id));
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_list(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::ListInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("tensor_list", key)
        .set("amount", data.amount)
        .set("expire_in_sec", data.expire_in_sec.unwrap_or(0))
        .set("currency", data.currency.as_deref().map(base58::encode).unwrap_or_default())
        .set("nft_standard", &data.nft_standard);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_buy(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::BuyInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("tensor_buy", key)
        .set("max_amount", data.max_amount)
        .set("nft_standard", &data.nft_standard);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_bid(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::BidInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("tensor_bid", key)
        .set("bid_id", base58::encode(&data.bid_id))
        .set("target", data.target)
        .set("target_id", base58::encode(&data.target_id))
        .set("amount", data.amount)
        .set("quantity", data.quantity)
        .set("expire_in_sec", data.expire_in_sec.unwrap_or(0))
        .set("currency", data.currency.as_deref().map(base58::encode).unwrap_or_default());
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
