use common::clickhouse::{common_key_v2, set_clock, set_instruction_v2, set_transaction_v2};
use proto::pb::raydium::v1 as pb;
use substreams::pb::substreams::Clock;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::SwapBaseInLog(event)) => {
                    handle_swap_base_in(tables, clock, transaction, instruction, event, transaction_index, instruction_index);
                }
                _ => {}
            }
        }
    }
}

fn handle_swap_base_in(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    event: &pb::SwapBaseInLog,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("raydium_amm_v4_swap_base_in", key)
        .set("amount_in", event.amount_in.to_string())
        .set("minimum_out", event.minimum_out.to_string())
        .set("direction", event.direction.to_string())
        .set("user_source", event.user_source.to_string())
        .set("pool_coin", event.pool_coin.to_string())
        .set("pool_pc", event.pool_pc.to_string())
        .set("out_amount", event.out_amount.to_string());

    set_instruction_v2(instruction, row);
    set_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_swap_base_out(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    event: &pb::SwapBaseOutLog,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("raydium_amm_v4_swap_base_out", key)
        .set("max_in", event.max_in.to_string())
        .set("amount_out", event.amount_out.to_string())
        .set("direction", event.direction.to_string())
        .set("user_source", event.user_source.to_string())
        .set("pool_coin", event.pool_coin.to_string())
        .set("pool_pc", event.pool_pc.to_string())
        .set("deduct_in", event.deduct_in.to_string());

    set_instruction_v2(instruction, row);
    set_transaction_v2(transaction, row);
    set_clock(clock, row);
}

// fn handle_initialize(
//     tables: &mut substreams_database_change::tables::Tables,
//     clock: &Clock,
//     event: &raydium_amm::InitializeEvent,
//     transaction_index: u32,
//     instruction_index: u32,
//     execution_index: u32,
//     signature: &str,
// ) {
//     let key = common_key(clock, execution_index as u64);

//     let row = tables
//         .create_row("raydium_amm_v4_initialize", key)
//         // ── transaction info ──────────────────────────────────────────────
//         .set("signature", signature)
//         // ── ordering  ─────────────────────────────────────────────────────
//         .set("execution_index", execution_index)
//         .set("transaction_index", transaction_index)
//         .set("instruction_index", instruction_index)
//         .set("global_sequence", to_global_sequence(clock, execution_index as u64))
//         // ── program id ────────────────────────────────────────────────────
//         .set("program_id", PROGRAM_ID)
//         // ── event ───────────────────────────────────────
//         .set("amm", &event.amm)
//         .set("user", &event.user)
//         .set("pc_mint", &event.pc_mint)
//         .set("coin_mint", &event.coin_mint)
//         .set("lp_mint", &event.lp_mint)
//         .set("pc_init_amount", event.pc_init_amount)
//         .set("coin_init_amount", event.coin_init_amount)
//         .set("lp_init_amount", event.lp_init_amount)
//         .set("nonce", event.nonce)
//         .set(
//             "market",
//             // `market` is optional in the protobuf; store empty string if absent.
//             event.market.as_deref().unwrap_or_default(),
//         )
//         .set(
//             "user_pc_pre_balance",
//             // assuming helper returns `u64`, or adapt to `unwrap_or_default()`
//             event.user_pc_pre_balance(),
//         )
//         .set("user_coin_pre_balance", event.user_coin_pre_balance());

//     set_clock(clock, row);
// }

// fn handle_withdraw(
//     tables: &mut substreams_database_change::tables::Tables,
//     clock: &Clock,
//     event: &raydium_amm::WithdrawEvent,
//     transaction_index: u32,
//     instruction_index: u32,
//     execution_index: u32,
//     signature: &str,
// ) {
//     let key = common_key(clock, execution_index as u64);

//     let row = tables
//         .create_row("raydium_amm_v4_withdraw", key)
//         // ── transaction info ──────────────────────────────────────────────
//         .set("signature", signature)
//         // ── ordering ──────────────────────────────────────────────────────
//         .set("execution_index", execution_index)
//         .set("transaction_index", transaction_index)
//         .set("instruction_index", instruction_index)
//         .set("global_sequence", to_global_sequence(clock, execution_index as u64))
//         // ── program id ────────────────────────────────────────────────────
//         .set("program_id", PROGRAM_ID)
//         // ── event ───────────────────────────────────────
//         .set("amm", &event.amm)
//         .set("user", &event.user)
//         .set("pc_mint", &event.pc_mint)
//         .set("coin_mint", &event.coin_mint)
//         .set("lp_mint", &event.lp_mint)
//         .set("pc_amount", event.pc_amount)
//         .set("coin_amount", event.coin_amount)
//         .set("lp_amount", event.lp_amount)
//         .set("pool_pc_amount", event.pool_pc_amount.unwrap_or_default())
//         .set("pool_coin_amount", event.pool_coin_amount.unwrap_or_default())
//         .set("pool_lp_amount", event.pool_lp_amount.unwrap_or_default())
//         .set("user_pc_pre_balance", event.user_pc_pre_balance.unwrap_or_default())
//         .set("user_coin_pre_balance", event.user_coin_pre_balance.unwrap_or_default());

//     set_clock(clock, row);
// }

// fn handle_deposit(
//     tables: &mut substreams_database_change::tables::Tables,
//     clock: &Clock,
//     event: &raydium::v1::DepositLog,
//     transaction_index: u32,
//     instruction_index: u32,
//     execution_index: u32,
//     transaction: &Transaction,
// ) {
//     let key = common_key(clock, execution_index as u64);

//     let row = tables
//         .create_row("raydium_amm_v4_deposit", key)
//         // ── transaction info ──────────────────────────────────────────────
//         .set("signature", signature)
//         // ── ordering ──────────────────────────────────────────────────────
//         .set("execution_index", execution_index)
//         .set("transaction_index", transaction_index)
//         .set("instruction_index", instruction_index)
//         .set("global_sequence", to_global_sequence(clock, execution_index as u64))
//         // ── program id ────────────────────────────────────────────────────
//         .set("program_id", PROGRAM_ID)
//         // ── event ───────────────────────────────────────
//         .set("amm", &event.amm)
//         .set("user", &event.user)
//         .set("pc_mint", &event.pc_mint)
//         .set("coin_mint", &event.coin_mint)
//         .set("lp_mint", &event.lp_mint)
//         .set("pc_amount", event.pc_amount)
//         .set("coin_amount", event.coin_amount)
//         .set("lp_amount", event.lp_amount)
//         .set("pool_pc_amount", event.pool_pc_amount.unwrap_or_default())
//         .set("pool_coin_amount", event.pool_coin_amount.unwrap_or_default())
//         .set("pool_lp_amount", event.pool_lp_amount.unwrap_or_default())
//         .set("user_pc_pre_balance", event.user_pc_pre_balance.unwrap_or_default())
//         .set("user_coin_pre_balance", event.user_coin_pre_balance.unwrap_or_default());

//     set_clock(clock, row);
// }

// fn handle_withdraw_pnl(
//     tables: &mut substreams_database_change::tables::Tables,
//     clock: &Clock,
//     event: &raydium_amm::WithdrawPnlEvent,
//     transaction_index: u32,
//     instruction_index: u32,
//     execution_index: u32,
//     signature: &str,
// ) {
//     let key = common_key(clock, execution_index as u64);

//     let row = tables
//         .create_row("raydium_amm_v4_withdraw_pnl", key)
//         // ── transaction info ──────────────────────────────────────────────
//         .set("signature", signature)
//         // ── ordering ──────────────────────────────────────────────────────
//         .set("execution_index", execution_index)
//         .set("transaction_index", transaction_index)
//         .set("instruction_index", instruction_index)
//         .set("global_sequence", to_global_sequence(clock, execution_index as u64))
//         // ── program id ────────────────────────────────────────────────────
//         .set("program_id", PROGRAM_ID)
//         // ── event ───────────────────────────────────────
//         .set("amm", &event.amm)
//         .set("user", &event.user)
//         .set("pc_mint", event.pc_mint.as_deref().unwrap_or_default())
//         .set("coin_mint", event.coin_mint.as_deref().unwrap_or_default())
//         .set("pc_amount", event.pc_amount.unwrap_or_default())
//         .set("coin_amount", event.coin_amount.unwrap_or_default());

//     set_clock(clock, row);
// }
