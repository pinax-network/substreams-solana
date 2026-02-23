use common::db::{common_key_v2, set_clock};
use proto::pb::orca::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            match &ix.instruction {
                Some(pb::instruction::Instruction::Swap(data)) => {
                    if let Some(event) = get_traded_event(tx, instruction_index) {
                        handle_swap_v1(tables, clock, tx, ix, data, event, transaction_index, instruction_index);
                    }
                }
                Some(pb::instruction::Instruction::SwapV2(data)) => {
                    if let Some(event) = get_traded_event(tx, instruction_index) {
                        handle_swap_v2(tables, clock, tx, ix, data, event, transaction_index, instruction_index);
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_traded_event(tx: &pb::Transaction, instruction_index: usize) -> Option<&pb::TradedEvent> {
    for i in instruction_index..tx.logs.len() {
        if let Some(pb::log::Log::Traded(ev)) = &tx.logs[i].log {
            return Some(ev);
        }
    }
    None
}

fn handle_swap_v1(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SwapInstruction,
    event: &pb::TradedEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match &data.accounts {
        Some(a) => a,
        None => return,
    };
    let (input_mint, output_mint) = if data.a_to_b {
        (&accounts.token_vault_a, &accounts.token_vault_b)
    } else {
        (&accounts.token_vault_b, &accounts.token_vault_a)
    };
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("orca_swap", key)
        .set("user", base58::encode(&accounts.token_authority))
        .set("whirlpool", base58::encode(&accounts.whirlpool))
        .set("input_mint", base58::encode(input_mint))
        .set("output_mint", base58::encode(output_mint))
        .set("amount_in", event.input_amount)
        .set("amount_out", event.output_amount);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_swap_v2(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SwapV2Instruction,
    event: &pb::TradedEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match &data.accounts {
        Some(a) => a,
        None => return,
    };
    let (input_mint, output_mint) = if data.a_to_b {
        (&accounts.token_mint_a, &accounts.token_mint_b)
    } else {
        (&accounts.token_mint_b, &accounts.token_mint_a)
    };
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("orca_swap", key)
        .set("user", base58::encode(&accounts.token_authority))
        .set("whirlpool", base58::encode(&accounts.whirlpool))
        .set("input_mint", base58::encode(input_mint))
        .set("output_mint", base58::encode(output_mint))
        .set("amount_in", event.input_amount)
        .set("amount_out", event.output_amount);
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
