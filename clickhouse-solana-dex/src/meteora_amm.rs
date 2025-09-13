use common::clickhouse::{common_key_v2, set_clock};
use proto::pb::meteora::amm::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        if tx.logs.len() != tx.instructions.len() {
            continue;
        }
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            if let Some(pb::instruction::Instruction::Swap(data)) = &ix.instruction {
                let log = match &tx.logs[instruction_index].log {
                    Some(pb::log::Log::Swap(l)) => l,
                    _ => continue,
                };
                handle_swap(tables, clock, tx, ix, data.accounts.as_ref(), log, transaction_index, instruction_index);
            }
        }
    }
}

fn handle_swap(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    accounts_opt: Option<&pb::SwapAccounts>,
    log: &pb::SwapLog,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match accounts_opt {
        Some(a) => a,
        None => return,
    };
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("meteora_amm_swap", key)
        .set("user", base58::encode(&accounts.user))
        .set("pool", base58::encode(&accounts.pool))
        .set("input_mint", base58::encode(&accounts.user_source_token))
        .set("output_mint", base58::encode(&accounts.user_destination_token))
        .set("amount_in", log.in_amount)
        .set("amount_out", log.out_amount);
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
