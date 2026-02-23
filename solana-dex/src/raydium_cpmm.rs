use common::db::{common_key_v2, set_clock};
use proto::pb::raydium::cpmm::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Row;
use substreams_database_change::tables::Tables;
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        if tx.logs.len() != tx.instructions.len() {
            continue;
        }
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            match &ix.instruction {
                Some(pb::instruction::Instruction::SwapBaseInput(data)) => {
                    let log = match &tx.logs[instruction_index].log {
                        Some(pb::log::Log::Swap(l)) => l,
                        _ => continue,
                    };
                    let accounts = match &data.accounts {
                        Some(a) => a,
                        None => continue,
                    };
                    handle_swap(
                        tables,
                        clock,
                        tx,
                        ix,
                        accounts,
                        log,
                        "raydium_cpmm_swap_base_in",
                        transaction_index,
                        instruction_index,
                    );
                }
                Some(pb::instruction::Instruction::SwapBaseOutput(data)) => {
                    let log = match &tx.logs[instruction_index].log {
                        Some(pb::log::Log::Swap(l)) => l,
                        _ => continue,
                    };
                    let accounts = match &data.accounts {
                        Some(a) => a,
                        None => continue,
                    };
                    handle_swap(
                        tables,
                        clock,
                        tx,
                        ix,
                        accounts,
                        log,
                        "raydium_cpmm_swap_base_out",
                        transaction_index,
                        instruction_index,
                    );
                }
                _ => {}
            }
        }
    }
}

fn handle_swap(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    accounts: &pb::SwapAccounts,
    log: &pb::SwapEvent,
    table: &str,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row(table, key)
        .set("payer", base58::encode(&accounts.payer))
        .set("pool_state", base58::encode(&accounts.pool_state))
        .set("input_token_mint", base58::encode(&accounts.input_token_mint))
        .set("output_token_mint", base58::encode(&accounts.output_token_mint))
        .set("amount_in", log.input_amount)
        .set("amount_out", log.output_amount);
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
