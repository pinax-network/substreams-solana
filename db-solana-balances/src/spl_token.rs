use std::collections::HashMap;

use common::clickhouse::{common_key_v2, set_clock, set_spl_token_transaction_v2};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    // Only keep last balance change per block
    let mut post_token_balances_per_block = HashMap::new();
    let mut pre_token_balances_per_block = HashMap::new();
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        // Keep first pre balance and last post balance per account
        for (i, balance) in transaction.pre_token_balances.iter().enumerate() {
            let key = (balance.account.as_slice(), balance.mint.as_slice());
            if !pre_token_balances_per_block.contains_key(&key) {
                pre_token_balances_per_block.insert(key, (balance, transaction, transaction_index, i));
            }
        }
        for (i, balance) in transaction.post_token_balances.iter().enumerate() {
            let key = (balance.account.as_slice(), balance.mint.as_slice());
            post_token_balances_per_block.insert(key, (balance, transaction, transaction_index, i));
        }
    }
    let mut skipped = 0;
    for (post_balance, transaction, transaction_index, i) in post_token_balances_per_block.values() {
        // if balance not changed in the block - no need to include it - skip it
        if let Some((pre_balance, _, _, _)) = pre_token_balances_per_block.get(&(post_balance.account.as_slice(), post_balance.mint.as_slice())) {
            if pre_balance.amount == post_balance.amount {
                skipped += 1;
                continue;
            }
        }
        handle_token_balances("post_token_balances", tables, clock, transaction, post_balance, *transaction_index, *i);
    }
    substreams::log::info!("Skipped {} out of {} spl token balances", skipped, post_token_balances_per_block.len());
}

fn handle_token_balances(
    table_name: &str,
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    data: &pb::TokenBalance,
    transaction_index: usize,
    token_balance_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, token_balance_index);
    let row = tables
        .create_row(table_name, key)
        .set("program_id", base58::encode(&data.program_id))
        .set("account", base58::encode(&data.account))
        .set("mint", base58::encode(&data.mint))
        .set("amount", data.amount)
        .set("decimals", data.decimals);

    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
