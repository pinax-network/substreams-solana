use common::clickhouse::{common_key_v2, set_clock, set_raydium_instruction_v2 as set_instruction_v2, set_raydium_transaction_v2 as set_transaction_v2};
use proto::pb::raydium::amm::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Row;
use substreams_solana::base58;

use crate::enums::Direction;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        // skip if log truncates (max size of 10KB and won't emit log events)
        // this is a workaround for the issue where the transaction logs are not emitted
        // if the transaction is too large, which can happen with large swap transactions.
        if transaction.logs.len() != transaction.instructions.len() {
            continue;
        }
        // assumes that logs & instruction sizes are equal
        // if not, it will skip the instruction
        for (instruction_index, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::SwapBaseIn(data)) => {
                    handle_swap_base_in(
                        tables,
                        clock,
                        transaction,
                        instruction,
                        data,
                        &transaction.logs[instruction_index],
                        transaction_index,
                        instruction_index,
                    );
                }
                Some(pb::instruction::Instruction::SwapBaseOut(data)) => {
                    handle_swap_base_out(
                        tables,
                        clock,
                        transaction,
                        instruction,
                        data,
                        &transaction.logs[instruction_index],
                        transaction_index,
                        instruction_index,
                    );
                }
                _ => {}
            }
        }
    }
}

fn handle_swap_base_in(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::SwapBaseInInstruction,
    log_cursor: &pb::Log,
    transaction_index: usize,
    instruction_index: usize,
) {
    // assumes that logs & instruction sizes are equal
    // if not, it will skip the instruction
    let log = match &log_cursor.log {
        Some(pb::log::Log::SwapBaseIn(l)) => l,
        _ => return,
    };
    let accounts = match &data.accounts {
        Some(accounts) => accounts,
        None => return,
    };
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("raydium_amm_v4_swap_base_in", key)
        // -- data --
        .set("amount_in", data.amount_in)
        .set("amount_out", log.out_amount)
        .set("minimum_amount_out", data.minimum_amount_out)
        // -- log --
        .set("direction", Direction::try_from(log.direction).unwrap().as_str())
        .set("user_source", log.user_source)
        .set("pool_coin", log.pool_coin)
        .set("pool_pc", log.pool_pc);

    set_swap_accounts(accounts, row);
    set_instruction_v2(instruction, row);
    set_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_swap_base_out(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::SwapBaseOutInstruction,
    log_cursor: &pb::Log,
    transaction_index: usize,
    instruction_index: usize,
) {
    // assumes that logs & instruction sizes are equal
    // if not, it will skip the instruction
    let log = match &log_cursor.log {
        Some(pb::log::Log::SwapBaseOut(l)) => l,
        _ => return,
    };
    let accounts = match &data.accounts {
        Some(accounts) => accounts,
        None => return,
    };
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("raydium_amm_v4_swap_base_out", key)
        // -- data --
        .set("amount_in", log.deduct_in)
        .set("amount_out", data.amount_out)
        .set("max_amount_in", data.max_amount_in)
        // -- log --
        .set("direction", Direction::try_from(log.direction).unwrap().as_str())
        .set("user_source", log.user_source)
        .set("pool_coin", log.pool_coin)
        .set("pool_pc", log.pool_pc);

    set_swap_accounts(accounts, row);
    set_instruction_v2(instruction, row);
    set_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn set_swap_accounts(accounts: &pb::SwapAccounts, row: &mut Row) {
    row.set("token_program", base58::encode(&accounts.token_program))
        .set("amm", base58::encode(&accounts.amm))
        .set("amm_authority", base58::encode(&accounts.amm_authority))
        .set("amm_open_orders", base58::encode(&accounts.amm_open_orders))
        .set("amm_coin_vault", base58::encode(&accounts.amm_coin_vault))
        .set("amm_pc_vault", base58::encode(&accounts.amm_pc_vault))
        .set("market_program", base58::encode(&accounts.market_program))
        .set("market", base58::encode(&accounts.market))
        .set("market_bids", base58::encode(&accounts.market_bids))
        .set("market_asks", base58::encode(&accounts.market_asks))
        .set("market_event_queue", base58::encode(&accounts.market_event_queue))
        .set("market_coin_vault", base58::encode(&accounts.market_coin_vault))
        .set("market_pc_vault", base58::encode(&accounts.market_pc_vault))
        .set("market_vault_signer", base58::encode(&accounts.market_vault_signer))
        .set("user_token_source", base58::encode(&accounts.user_token_source))
        .set("user_token_destination", base58::encode(&accounts.user_token_destination))
        .set("user_source_owner", base58::encode(&accounts.user_source_owner))
        // optional fields
        .set("amm_target_orders", accounts.amm_target_orders.as_ref().map(base58::encode).unwrap_or_default());
}
