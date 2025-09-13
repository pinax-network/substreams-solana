use common::clickhouse::{common_key_v2, set_clock};
use proto::pb::meteora::dllm::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            if let Some(pb::instruction::Instruction::SwapInstruction(data)) = &ix.instruction {
                if let Some(event) = get_swap_event(tx, instruction_index) {
                    handle_swap(tables, clock, tx, ix, data.accounts.as_ref(), event, transaction_index, instruction_index);
                }
            }
        }
    }
}

fn get_swap_event(tx: &pb::Transaction, instruction_index: usize) -> Option<&pb::SwapEvent> {
    if instruction_index + 1 < tx.instructions.len() {
        if let Some(pb::instruction::Instruction::SwapEvent(ev)) = &tx.instructions[instruction_index + 1].instruction {
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
    accounts_opt: Option<&pb::SwapAccounts>,
    event: &pb::SwapEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match accounts_opt {
        Some(a) => a,
        None => return,
    };
    let (input_mint, output_mint) = if event.swap_for_y {
        (&accounts.token_x_mint, &accounts.token_y_mint)
    } else {
        (&accounts.token_y_mint, &accounts.token_x_mint)
    };
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("meteora_dllm_swap", key)
        .set("user", base58::encode(&accounts.user))
        .set("lb_pair", base58::encode(&accounts.lb_pair))
        .set("input_mint", base58::encode(input_mint))
        .set("output_mint", base58::encode(output_mint))
        .set("amount_in", event.amount_in)
        .set("amount_out", event.amount_out);
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
