use common::{
    clickhouse::{common_key, set_clock},
    to_global_sequence,
};
use proto::pb::raydium_amm;
use substreams::pb::substreams::Clock;

// Raydium Liquidity Pool V4
const PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &raydium_amm::RaydiumAmmBlockEvents) {
    let mut execution_index = 0;
    // -- Transactions --
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in transaction.events.iter().enumerate() {
            match &instruction.event {
                Some(raydium_amm::raydium_amm_event::Event::Initialize(event)) => {
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
                Some(raydium_amm::raydium_amm_event::Event::Deposit(event)) => {
                    handle_deposit(
                        tables,
                        clock,
                        event,
                        transaction_index as u32,
                        instruction_index as u32,
                        execution_index,
                        &transaction.signature,
                    );
                }
                Some(raydium_amm::raydium_amm_event::Event::Withdraw(event)) => {
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
                Some(raydium_amm::raydium_amm_event::Event::WithdrawPnl(event)) => {
                    handle_withdraw_pnl(
                        tables,
                        clock,
                        event,
                        transaction_index as u32,
                        instruction_index as u32,
                        execution_index,
                        &transaction.signature,
                    );
                }
                Some(raydium_amm::raydium_amm_event::Event::Swap(event)) => {
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

fn handle_swap(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &raydium_amm::SwapEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(&clock, execution_index as u64);

    let row = tables
        .create_row("raydium_amm_v4_swap", key)
        // -- transaction --
        .set("signature", signature)
        // -- ordering --
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // -- program ID --
        .set("program_id", PROGRAM_ID)
        // -- event --
        .set("amm", &event.amm)
        .set("user", &event.user)
        .set("mint_in", &event.mint_in)
        .set("mint_out", &event.mint_out)
        .set("amount_in", event.amount_in)
        .set("amount_out", event.amount_out)
        .set("direction", &event.direction)
        .set("pool_pc_amount", event.pool_pc_amount())
        .set("pool_coin_amount", event.pool_coin_amount())
        .set("pc_mint", &event.pc_mint)
        .set("coin_mint", &event.coin_mint)
        .set("user_pre_balance_in", event.user_pre_balance_in())
        .set("user_pre_balance_out", event.user_pre_balance_out());

    set_clock(clock, row);
}

fn handle_initialize(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &raydium_amm::InitializeEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("raydium_amm_v4_initialize", key)
        // ── transaction info ──────────────────────────────────────────────
        .set("signature", signature)
        // ── ordering  ─────────────────────────────────────────────────────
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // ── program id ────────────────────────────────────────────────────
        .set("program_id", PROGRAM_ID)
        // ── event ───────────────────────────────────────
        .set("amm", &event.amm)
        .set("user", &event.user)
        .set("pc_mint", &event.pc_mint)
        .set("coin_mint", &event.coin_mint)
        .set("lp_mint", &event.lp_mint)
        .set("pc_init_amount", event.pc_init_amount)
        .set("coin_init_amount", event.coin_init_amount)
        .set("lp_init_amount", event.lp_init_amount)
        .set("nonce", event.nonce)
        .set(
            "market",
            // `market` is optional in the protobuf; store empty string if absent.
            event.market.as_deref().unwrap_or_default(),
        )
        .set(
            "user_pc_pre_balance",
            // assuming helper returns `u64`, or adapt to `unwrap_or_default()`
            event.user_pc_pre_balance(),
        )
        .set("user_coin_pre_balance", event.user_coin_pre_balance());

    set_clock(clock, row);
}

fn handle_withdraw(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &raydium_amm::WithdrawEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("raydium_amm_v4_withdraw", key)
        // ── transaction info ──────────────────────────────────────────────
        .set("signature", signature)
        // ── ordering ──────────────────────────────────────────────────────
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // ── program id ────────────────────────────────────────────────────
        .set("program_id", PROGRAM_ID)
        // ── event ───────────────────────────────────────
        .set("amm", &event.amm)
        .set("user", &event.user)
        .set("pc_mint", &event.pc_mint)
        .set("coin_mint", &event.coin_mint)
        .set("lp_mint", &event.lp_mint)
        .set("pc_amount", event.pc_amount)
        .set("coin_amount", event.coin_amount)
        .set("lp_amount", event.lp_amount)
        .set("pool_pc_amount", event.pool_pc_amount.unwrap_or_default())
        .set("pool_coin_amount", event.pool_coin_amount.unwrap_or_default())
        .set("pool_lp_amount", event.pool_lp_amount.unwrap_or_default())
        .set("user_pc_pre_balance", event.user_pc_pre_balance.unwrap_or_default())
        .set("user_coin_pre_balance", event.user_coin_pre_balance.unwrap_or_default());

    set_clock(clock, row);
}

fn handle_deposit(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &raydium_amm::DepositEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("raydium_amm_v4_deposit", key)
        // ── transaction info ──────────────────────────────────────────────
        .set("signature", signature)
        // ── ordering ──────────────────────────────────────────────────────
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // ── program id ────────────────────────────────────────────────────
        .set("program_id", PROGRAM_ID)
        // ── event ───────────────────────────────────────
        .set("amm", &event.amm)
        .set("user", &event.user)
        .set("pc_mint", &event.pc_mint)
        .set("coin_mint", &event.coin_mint)
        .set("lp_mint", &event.lp_mint)
        .set("pc_amount", event.pc_amount)
        .set("coin_amount", event.coin_amount)
        .set("lp_amount", event.lp_amount)
        .set("pool_pc_amount", event.pool_pc_amount.unwrap_or_default())
        .set("pool_coin_amount", event.pool_coin_amount.unwrap_or_default())
        .set("pool_lp_amount", event.pool_lp_amount.unwrap_or_default())
        .set("user_pc_pre_balance", event.user_pc_pre_balance.unwrap_or_default())
        .set("user_coin_pre_balance", event.user_coin_pre_balance.unwrap_or_default());

    set_clock(clock, row);
}

fn handle_withdraw_pnl(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    event: &raydium_amm::WithdrawPnlEvent,
    transaction_index: u32,
    instruction_index: u32,
    execution_index: u32,
    signature: &str,
) {
    let key = common_key(clock, execution_index as u64);

    let row = tables
        .create_row("raydium_amm_v4_withdraw_pnl", key)
        // ── transaction info ──────────────────────────────────────────────
        .set("signature", signature)
        // ── ordering ──────────────────────────────────────────────────────
        .set("execution_index", execution_index)
        .set("transaction_index", transaction_index)
        .set("instruction_index", instruction_index)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64))
        // ── program id ────────────────────────────────────────────────────
        .set("program_id", PROGRAM_ID)
        // ── event ───────────────────────────────────────
        .set("amm", &event.amm)
        .set("user", &event.user)
        .set("pc_mint", event.pc_mint.as_deref().unwrap_or_default())
        .set("coin_mint", event.coin_mint.as_deref().unwrap_or_default())
        .set("pc_amount", event.pc_amount.unwrap_or_default())
        .set("coin_amount", event.coin_amount.unwrap_or_default());

    set_clock(clock, row);
}
