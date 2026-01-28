use common::clickhouse::{common_key_v2, set_authority, set_clock, set_spl_token_instruction_v2, set_spl_token_transaction_v2};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (i, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                // Transfers
                Some(pb::instruction::Instruction::Transfer(data)) => {
                    handle_transfer(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::Mint(data)) => {
                    handle_transfer(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::Burn(data)) => {
                    handle_transfer(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                _ => {}
            }
        }
    }
}

fn handle_transfer(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::Transfer,
    transaction_index: usize,
    instruction_index: usize,
) {
    // Skip transfers to self
    if data.source == data.destination {
        return;
    }
    let decimals_raw = data.decimals.map(|d| d.to_string()).unwrap_or_default();
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("spl_transfer", key)
        .set("source", base58::encode(&data.source))
        .set("destination", base58::encode(&data.destination))
        .set("amount", data.amount)
        .set("mint", base58::encode(&data.mint))
        // -- SPL Token-2022 --
        .set("decimals_raw", decimals_raw);

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
