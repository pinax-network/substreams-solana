use common::{
    clickhouse::{common_key, set_clock},
    to_global_sequence,
};
use proto::pb::pumpfun;
use substreams::pb::substreams::Clock;

const PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
// Pump.fun AMM: pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pumpfun::PumpfunBlockEvents) {
    let mut execution_index = 0;
    // -- Transactions --
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in transaction.events.iter().enumerate() {
            match &instruction.event {
                Some(pumpfun::pumpfun_event::Event::Create(event)) => {
                    handle_create(
                        tables,
                        clock,
                        event,
                        transaction_index as u32,
                        instruction_index as u32,
                        execution_index,
                        &transaction.signature,
                    );
                }
                Some(pumpfun::pumpfun_event::Event::Initialize(event)) => {
                    handle_initialize(
                        tables,
                        clock,
                        event,
                        transaction_index as u32,
                        instruction_index as u32,
                        execution_index,
                        &transaction.signature,
                    );
                }
                Some(pumpfun::pumpfun_event::Event::Withdraw(event)) => {
                    handle_withdraw(
                        tables,
                        clock,
                        event,
                        transaction_index as u32,
                        instruction_index as u32,
                        execution_index,
                        &transaction.signature,
                    );
                }
                Some(pumpfun::pumpfun_event::Event::SetParams(event)) => {
                    handle_set_params(
                        tables,
                        clock,
                        event,
                        transaction_index as u32,
                        instruction_index as u32,
                        execution_index,
                        &transaction.signature,
                    );
                }
                Some(pumpfun::pumpfun_event::Event::Swap(event)) => {
                    handle_swap(
                        tables,
                        clock,
                        event,
                        transaction_index as u32,
                        instruction_index as u32,
                        execution_index,
                        &transaction.signature,
                    );
                }
                None => {}
            }
            execution_index += 1;
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 1 ▪ CreateEvent
// ─────────────────────────────────────────────────────────────────────────────
fn handle_create(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &pumpfun::CreateEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("pumpfun_create", key)
        // transaction / ordering
        .set("signature", signature)
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // program id
        .set("program_id", PROGRAM_ID)
        // event fields
        .set("user", &event.user)
        .set("name", &event.name)
        .set("symbol", &event.symbol)
        .set("uri", &event.uri)
        .set("mint", &event.mint)
        .set("bonding_curve", &event.bonding_curve)
        .set("associated_bonding_curve", &event.associated_bonding_curve)
        .set("metadata", &event.metadata);

    set_clock(clock, row);
}

// ─────────────────────────────────────────────────────────────────────────────
// 2 ▪ InitializeEvent
// ─────────────────────────────────────────────────────────────────────────────
fn handle_initialize(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &pumpfun::InitializeEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("pumpfun_initialize", key)
        // transaction / ordering
        .set("signature", signature)
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // program id
        .set("program_id", PROGRAM_ID)
        // event fields
        .set("user", &event.user);

    set_clock(clock, row);
}

// ─────────────────────────────────────────────────────────────────────────────
// 3 ▪ SetParamsEvent
// ─────────────────────────────────────────────────────────────────────────────
fn handle_set_params(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &pumpfun::SetParamsEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("pumpfun_set_params", key)
        // transaction / ordering
        .set("signature", signature)
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // program id
        .set("program_id", PROGRAM_ID)
        // event fields
        .set("user", &event.user)
        .set("fee_recipient", &event.fee_recipient)
        .set("initial_virtual_token_reserves", event.initial_virtual_token_reserves)
        .set("initial_virtual_sol_reserves", event.initial_virtual_sol_reserves)
        .set("initial_real_token_reserves", event.initial_real_token_reserves)
        .set("token_total_supply", event.token_total_supply)
        .set("fee_basis_points", event.fee_basis_points);

    set_clock(clock, row);
}

// ─────────────────────────────────────────────────────────────────────────────
// 4 ▪ SwapEvent
// ─────────────────────────────────────────────────────────────────────────────
fn handle_swap(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &pumpfun::SwapEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("pumpfun_swap", key)
        // transaction / ordering
        .set("signature", signature)
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // program id
        .set("program_id", PROGRAM_ID)
        // event fields
        .set("user", &event.user)
        .set("mint", &event.mint)
        .set("bonding_curve", &event.bonding_curve)
        .set("sol_amount", event.sol_amount.unwrap_or_default())
        .set("token_amount", event.token_amount)
        .set("direction", &event.direction)
        .set("virtual_sol_reserves", event.virtual_sol_reserves.unwrap_or_default())
        .set("virtual_token_reserves", event.virtual_token_reserves.unwrap_or_default())
        .set("real_sol_reserves", event.real_sol_reserves.unwrap_or_default())
        .set("real_token_reserves", event.real_token_reserves.unwrap_or_default())
        .set("user_token_pre_balance", event.user_token_pre_balance.unwrap_or_default());

    set_clock(clock, row);
}

// ─────────────────────────────────────────────────────────────────────────────
// 5 ▪ WithdrawEvent
// ─────────────────────────────────────────────────────────────────────────────
fn handle_withdraw(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &pumpfun::WithdrawEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("pumpfun_withdraw", key)
        // transaction / ordering
        .set("signature", signature)
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // program id
        .set("program_id", PROGRAM_ID)
        // event fields
        .set("mint", &event.mint);

    set_clock(clock, row);
}
