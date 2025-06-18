use common::{
    clickhouse::{common_key, set_clock},
    to_global_sequence,
};
use proto::pb::raydium_amm;
use substreams::pb::substreams::Clock;

const PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &raydium_amm::RaydiumAmmBlockEvents) {
    let mut execution_index = 0;
    // -- Transactions --
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in transaction.events.iter().enumerate() {
            match &instruction.event {
                Some(raydium_amm::raydium_amm_event::Event::Initialize(initialize)) => {
                    // handle_initialize(tables, clock, initialize);
                }
                Some(raydium_amm::raydium_amm_event::Event::Deposit(deposit)) => {
                    // handle_deposit(tables, clock, deposit);
                }
                Some(raydium_amm::raydium_amm_event::Event::Withdraw(withdraw)) => {
                    // handle_withdraw(tables, clock, withdraw);
                }
                Some(raydium_amm::raydium_amm_event::Event::WithdrawPnl(withdraw_pnl)) => {
                    // handle_withdraw_pnl(tables, clock, withdraw_pnl);
                }
                Some(raydium_amm::raydium_amm_event::Event::Swap(swap)) => {
                    handle_swap(
                        tables,
                        clock,
                        swap,
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
    tx_hash: &str,
) {
    let key = common_key(&clock, execution_index as u64);

    let row = tables
        .create_row("raydium_amm_swaps", key)
        // -- transaction --
        .set("tx_hash", tx_hash)
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
        .set("user_pre_balance_in", event.user_pre_balance_in())
        .set("user_pre_balance_out", event.user_pre_balance_out());

    set_clock(clock, row);
}
