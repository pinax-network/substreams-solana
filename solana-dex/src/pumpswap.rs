use common::db::{common_key_v2, set_clock};
use proto::pb::pumpswap::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            let ix = if log_index < tx.instructions.len() { &tx.instructions[log_index] } else { continue };
            match &log.log {
                Some(pb::log::Log::Buy(event)) => {
                    handle_buy(tables, clock, tx, ix, event, transaction_index, log_index);
                }
                Some(pb::log::Log::Sell(event)) => {
                    handle_sell(tables, clock, tx, ix, event, transaction_index, log_index);
                }
                _ => {}
            }
        }
    }
}

fn handle_buy(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    event: &pb::BuyEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("pumpswap_buy", key)
        .set("pool", base58::encode(&event.pool))
        .set("user", base58::encode(&event.user))
        .set("base_amount_out", event.base_amount_out)
        .set("quote_amount_in", event.quote_amount_in)
        .set("lp_fee", event.lp_fee)
        .set("protocol_fee", event.protocol_fee)
        .set("coin_creator_fee", event.coin_creator_fee)
        .set("pool_base_token_reserves", event.pool_base_token_reserves)
        .set("pool_quote_token_reserves", event.pool_quote_token_reserves);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_sell(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    event: &pb::SellEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("pumpswap_sell", key)
        .set("pool", base58::encode(&event.pool))
        .set("user", base58::encode(&event.user))
        .set("base_amount_in", event.base_amount_in)
        .set("quote_amount_out", event.quote_amount_out)
        .set("lp_fee", event.lp_fee)
        .set("protocol_fee", event.protocol_fee)
        .set("coin_creator_fee", event.coin_creator_fee)
        .set("pool_base_token_reserves", event.pool_base_token_reserves)
        .set("pool_quote_token_reserves", event.pool_quote_token_reserves);
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
