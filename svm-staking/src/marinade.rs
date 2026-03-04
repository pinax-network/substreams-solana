use common::db::{common_key_v2, set_clock};
use proto::pb::marinade::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(pb::log::Log::Deposit(event)) => {
                    handle_deposit(tables, clock, tx, log, event, transaction_index, log_index);
                }
                Some(pb::log::Log::DepositStakeAccount(event)) => {
                    handle_deposit_stake_account(tables, clock, tx, log, event, transaction_index, log_index);
                }
                Some(pb::log::Log::LiquidUnstake(event)) => {
                    handle_liquid_unstake(tables, clock, tx, log, event, transaction_index, log_index);
                }
                Some(pb::log::Log::AddLiquidity(event)) => {
                    handle_add_liquidity(tables, clock, tx, log, event, transaction_index, log_index);
                }
                Some(pb::log::Log::RemoveLiquidity(event)) => {
                    handle_remove_liquidity(tables, clock, tx, log, event, transaction_index, log_index);
                }
                Some(pb::log::Log::WithdrawStakeAccount(event)) => {
                    handle_withdraw_stake_account(tables, clock, tx, log, event, transaction_index, log_index);
                }
                None => {}
            }
        }
    }
}

fn handle_deposit(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    log: &pb::Log,
    event: &pb::DepositEvent,
    transaction_index: usize,
    log_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, log_index);
    let row = tables
        .create_row("marinade_deposit", key)
        .set("state", base58::encode(&event.state))
        .set("sol_owner", base58::encode(&event.sol_owner))
        .set("sol_swapped", event.sol_swapped)
        .set("msol_swapped", event.msol_swapped)
        .set("sol_deposited", event.sol_deposited)
        .set("msol_minted", event.msol_minted)
        .set("total_virtual_staked_lamports", event.total_virtual_staked_lamports)
        .set("msol_supply", event.msol_supply)
        .set("program_id", base58::encode(&log.program_id));
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_deposit_stake_account(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    log: &pb::Log,
    event: &pb::DepositStakeAccountEvent,
    transaction_index: usize,
    log_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, log_index);
    let row = tables
        .create_row("marinade_deposit_stake_account", key)
        .set("state", base58::encode(&event.state))
        .set("stake", base58::encode(&event.stake))
        .set("delegated", event.delegated)
        .set("withdrawer", base58::encode(&event.withdrawer))
        .set("validator", base58::encode(&event.validator))
        .set("msol_minted", event.msol_minted)
        .set("total_virtual_staked_lamports", event.total_virtual_staked_lamports)
        .set("msol_supply", event.msol_supply)
        .set("program_id", base58::encode(&log.program_id));
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_liquid_unstake(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    log: &pb::Log,
    event: &pb::LiquidUnstakeEvent,
    transaction_index: usize,
    log_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, log_index);
    let row = tables
        .create_row("marinade_liquid_unstake", key)
        .set("state", base58::encode(&event.state))
        .set("msol_owner", base58::encode(&event.msol_owner))
        .set("msol_amount", event.msol_amount)
        .set("msol_fee", event.msol_fee)
        .set("treasury_msol_cut", event.treasury_msol_cut)
        .set("sol_amount", event.sol_amount)
        .set("program_id", base58::encode(&log.program_id));
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_add_liquidity(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    log: &pb::Log,
    event: &pb::AddLiquidityEvent,
    transaction_index: usize,
    log_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, log_index);
    let row = tables
        .create_row("marinade_add_liquidity", key)
        .set("state", base58::encode(&event.state))
        .set("sol_owner", base58::encode(&event.sol_owner))
        .set("sol_added_amount", event.sol_added_amount)
        .set("lp_minted", event.lp_minted)
        .set("total_virtual_staked_lamports", event.total_virtual_staked_lamports)
        .set("msol_supply", event.msol_supply)
        .set("program_id", base58::encode(&log.program_id));
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_remove_liquidity(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    log: &pb::Log,
    event: &pb::RemoveLiquidityEvent,
    transaction_index: usize,
    log_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, log_index);
    let row = tables
        .create_row("marinade_remove_liquidity", key)
        .set("state", base58::encode(&event.state))
        .set("lp_burned", event.lp_burned)
        .set("sol_out_amount", event.sol_out_amount)
        .set("msol_out_amount", event.msol_out_amount)
        .set("program_id", base58::encode(&log.program_id));
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_withdraw_stake_account(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    log: &pb::Log,
    event: &pb::WithdrawStakeAccountEvent,
    transaction_index: usize,
    log_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, log_index);
    let row = tables
        .create_row("marinade_withdraw_stake_account", key)
        .set("state", base58::encode(&event.state))
        .set("stake", base58::encode(&event.stake))
        .set("validator", base58::encode(&event.validator))
        .set("user_msol_auth", base58::encode(&event.user_msol_auth))
        .set("msol_burned", event.msol_burned)
        .set("msol_fees", event.msol_fees)
        .set("beneficiary", base58::encode(&event.beneficiary))
        .set("split_lamports", event.split_lamports)
        .set("program_id", base58::encode(&log.program_id));
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
