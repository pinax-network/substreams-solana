use common::db::{common_key_v2, set_clock};
use proto::pb::solana::native::stake::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in tx.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::Initialize(event)) => {
                    let key = common_key_v2(clock, transaction_index, instruction_index);
                    let row = tables
                        .create_row("native_stake_initialize", key)
                        .set("stake_account", base58::encode(&event.stake_account))
                        .set("staker", base58::encode(&event.staker))
                        .set("withdrawer", base58::encode(&event.withdrawer))
                        .set("lockup_unix_timestamp", event.lockup_unix_timestamp.unwrap_or_default())
                        .set("lockup_epoch", event.lockup_epoch.unwrap_or_default())
                        .set("lockup_custodian", event.lockup_custodian.as_ref().map(base58::encode).unwrap_or_default());
                    set_instruction(instruction, row);
                    set_transaction(tx, row);
                    set_clock(clock, row);
                }
                Some(pb::instruction::Instruction::Delegate(event)) => {
                    let key = common_key_v2(clock, transaction_index, instruction_index);
                    let row = tables
                        .create_row("native_stake_delegate", key)
                        .set("stake_account", base58::encode(&event.stake_account))
                        .set("vote_account", base58::encode(&event.vote_account))
                        .set("stake_authority", base58::encode(&event.stake_authority));
                    set_instruction(instruction, row);
                    set_transaction(tx, row);
                    set_clock(clock, row);
                }
                Some(pb::instruction::Instruction::Deactivate(event)) => {
                    let key = common_key_v2(clock, transaction_index, instruction_index);
                    let row = tables
                        .create_row("native_stake_deactivate", key)
                        .set("stake_account", base58::encode(&event.stake_account))
                        .set("stake_authority", base58::encode(&event.stake_authority));
                    set_instruction(instruction, row);
                    set_transaction(tx, row);
                    set_clock(clock, row);
                }
                Some(pb::instruction::Instruction::Withdraw(event)) => {
                    let key = common_key_v2(clock, transaction_index, instruction_index);
                    let row = tables
                        .create_row("native_stake_withdraw", key)
                        .set("stake_account", base58::encode(&event.stake_account))
                        .set("destination", base58::encode(&event.destination))
                        .set("lamports", event.lamports)
                        .set("withdraw_authority", base58::encode(&event.withdraw_authority))
                        .set("custodian", event.custodian.as_ref().map(base58::encode).unwrap_or_default());
                    set_instruction(instruction, row);
                    set_transaction(tx, row);
                    set_clock(clock, row);
                }
                Some(pb::instruction::Instruction::Merge(event)) => {
                    let key = common_key_v2(clock, transaction_index, instruction_index);
                    let row = tables
                        .create_row("native_stake_merge", key)
                        .set("destination_stake_account", base58::encode(&event.destination_stake_account))
                        .set("source_stake_account", base58::encode(&event.source_stake_account))
                        .set("stake_authority", base58::encode(&event.stake_authority));
                    set_instruction(instruction, row);
                    set_transaction(tx, row);
                    set_clock(clock, row);
                }
                Some(pb::instruction::Instruction::Split(event)) => {
                    let key = common_key_v2(clock, transaction_index, instruction_index);
                    let row = tables
                        .create_row("native_stake_split", key)
                        .set("stake_account", base58::encode(&event.stake_account))
                        .set("split_stake_account", base58::encode(&event.split_stake_account))
                        .set("lamports", event.lamports)
                        .set("stake_authority", base58::encode(&event.stake_authority));
                    set_instruction(instruction, row);
                    set_transaction(tx, row);
                    set_clock(clock, row);
                }
                None => {}
            }
        }
    }
}

fn set_transaction(tx: &pb::Transaction, row: &mut Row) {
    row.set("signature", base58::encode(&tx.signature))
        .set("fee_payer", base58::encode(&tx.fee_payer))
        .set("signers_raw", tx.signers.iter().map(base58::encode).collect::<Vec<_>>().join(","))
        .set("fee", tx.fee)
        .set("compute_units_consumed", tx.compute_units_consumed);
}

fn set_instruction(instruction: &pb::Instruction, row: &mut Row) {
    row.set("program_id", base58::encode(&instruction.program_id))
        .set("stack_height", instruction.stack_height)
        .set("is_root", instruction.is_root);
}
