use common::db::{common_key_v2, set_clock, set_spl_token_instruction_v2, set_spl_token_transaction_v2};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (i, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                // Metadata
                Some(pb::instruction::Instruction::InitializeTokenMetadata(data)) => {
                    handle_initialize_token_metadata(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::UpdateTokenMetadataField(data)) => {
                    handle_update_token_metadata_field(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::UpdateTokenMetadataAuthority(data)) => {
                    handle_update_token_metadata_authority(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::RemoveTokenMetadataField(data)) => {
                    handle_remove_token_metadata_field(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                _ => {}
            }
        }
    }
}

fn handle_initialize_token_metadata(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::InitializeTokenMetadata,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("initialize_token_metadata", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("mint", base58::encode(&data.mint))
        .set("mint_authority", base58::encode(&data.mint_authority))
        .set("name", &data.name)
        .set("symbol", &data.symbol)
        .set("uri", &data.uri);

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_update_token_metadata_field(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::UpdateTokenMetadataField,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("update_token_metadata_field", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("field", &data.field)
        .set("value", &data.value);

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_update_token_metadata_authority(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::UpdateTokenMetadataAuthority,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("update_token_metadata_authority", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("new_authority", base58::encode(&data.new_authority));

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_remove_token_metadata_field(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::RemoveTokenMetadataField,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("remove_token_metadata_field", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("key", &data.key)
        .set("idempotent", data.idempotent);

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
