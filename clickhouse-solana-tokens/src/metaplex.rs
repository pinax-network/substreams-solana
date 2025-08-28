use common::clickhouse::{common_key_v2, set_clock, set_metaplex_instruction_v2, set_metaplex_transaction_v2};
use proto::pb::solana::metaplex::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (i, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::CreateMetadataAccount(data)) => {
                    handle_create_metadata_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::UpdateMetadataAccount(data)) => {
                    handle_update_metadata_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                _ => {}
            }
        }
    }
}

fn handle_create_metadata_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::CreateMetadataAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("metaplex_create_metadata_account", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("mint", base58::encode(&data.mint))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("payer", base58::encode(&data.payer))
        .set("name", data.name.clone())
        .set("symbol", data.symbol.clone())
        .set("uri", data.uri.clone());

    set_metaplex_instruction_v2(instruction, row);
    set_metaplex_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_update_metadata_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::UpdateMetadataAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("metaplex_update_metadata_account", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("name_raw", data.name())
        .set("symbol_raw", data.symbol())
        .set("uri_raw", data.uri());

    set_metaplex_instruction_v2(instruction, row);
    set_metaplex_transaction_v2(transaction, row);
    set_clock(clock, row);
}
