use common::db::{common_key_v2, set_clock};
use proto::pb::raydium::launchpad::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            match &ix.instruction {
                Some(pb::instruction::Instruction::BuyExactIn(data)) => {
                    if let Some(event) = get_trade_event(tx, instruction_index) {
                        handle_trade(
                            tables,
                            clock,
                            tx,
                            ix,
                            data.accounts.as_ref(),
                            event,
                            "raydium_launchpad_buy",
                            transaction_index,
                            instruction_index,
                        );
                    }
                }
                Some(pb::instruction::Instruction::BuyExactOut(data)) => {
                    if let Some(event) = get_trade_event(tx, instruction_index) {
                        handle_trade(
                            tables,
                            clock,
                            tx,
                            ix,
                            data.accounts.as_ref(),
                            event,
                            "raydium_launchpad_buy",
                            transaction_index,
                            instruction_index,
                        );
                    }
                }
                Some(pb::instruction::Instruction::SellExactIn(data)) => {
                    if let Some(event) = get_trade_event(tx, instruction_index) {
                        handle_trade(
                            tables,
                            clock,
                            tx,
                            ix,
                            data.accounts.as_ref(),
                            event,
                            "raydium_launchpad_sell",
                            transaction_index,
                            instruction_index,
                        );
                    }
                }
                Some(pb::instruction::Instruction::SellExactOut(data)) => {
                    if let Some(event) = get_trade_event(tx, instruction_index) {
                        handle_trade(
                            tables,
                            clock,
                            tx,
                            ix,
                            data.accounts.as_ref(),
                            event,
                            "raydium_launchpad_sell",
                            transaction_index,
                            instruction_index,
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_trade_event(tx: &pb::Transaction, instruction_index: usize) -> Option<&pb::TradeEvent> {
    if instruction_index + 1 < tx.instructions.len() {
        if let Some(pb::instruction::Instruction::TradeEvent(ev)) = &tx.instructions[instruction_index + 1].instruction {
            return Some(ev);
        }
    }
    None
}

fn handle_trade(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    accounts_opt: Option<&pb::TradeAccounts>,
    event: &pb::TradeEvent,
    table: &str,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match accounts_opt {
        Some(a) => a,
        None => return,
    };
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row(table, key)
        .set("payer", base58::encode(&accounts.payer))
        .set("pool_state", base58::encode(&accounts.pool_state))
        .set("base_token_mint", base58::encode(&accounts.base_token_mint))
        .set("quote_token_mint", base58::encode(&accounts.quote_token_mint))
        .set("amount_in", event.amount_in)
        .set("amount_out", event.amount_out)
        .set("exact_in", event.exact_in.unwrap_or(false));
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
