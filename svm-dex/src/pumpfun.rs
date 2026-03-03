use common::db::{common_key_v2, set_clock, set_pumpfun_instruction_v2, set_pumpfun_transaction_v2};
use proto::pb::pumpfun::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Row;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::Buy(data)) => {
                    let Some(event) = get_trade_event(transaction, instruction_index) else {
                        continue;
                    };
                    handle_buy(tables, clock, transaction, instruction, data, event, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::Sell(data)) => {
                    let Some(event) = get_trade_event(transaction, instruction_index) else {
                        continue;
                    };
                    handle_sell(tables, clock, transaction, instruction, data, event, transaction_index, instruction_index);
                }

                _ => {}
            }
        }
    }
}

fn get_trade_event(transaction: &pb::Transaction, instruction_index: usize) -> Option<&pb::TradeEvent> {
    if instruction_index + 1 < transaction.instructions.len() {
        match &transaction.instructions[instruction_index + 1].instruction {
            Some(pb::instruction::Instruction::Trade(event)) => Some(event),
            _ => None,
        }
    } else {
        None
    }
}

fn handle_buy(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::BuyInstruction,
    event: &pb::TradeEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match &data.accounts {
        Some(accounts) => accounts,
        None => return,
    };
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("pumpfun_buy", key)
        .set("amount", data.amount)
        .set("max_sol_cost", data.max_sol_cost);

    set_trade_event(event, accounts, row);
    set_pumpfun_instruction_v2(instruction, row);
    set_pumpfun_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_sell(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::SellInstruction,
    event: &pb::TradeEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match &data.accounts {
        Some(accounts) => accounts,
        None => return,
    };
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("pumpfun_sell", key)
        .set("amount", data.amount)
        .set("min_sol_output", data.min_sol_output);

    set_trade_event(event, accounts, row);
    set_pumpfun_instruction_v2(instruction, row);
    set_pumpfun_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn set_trade_event(event: &pb::TradeEvent, accounts: &pb::TradeAccounts, row: &mut Row) {
    row.set("global", base58::encode(&accounts.mint))
        .set("fee_recipient", base58::encode(&accounts.fee_recipient))
        .set("mint", base58::encode(&accounts.mint))
        .set("bonding_curve", base58::encode(&accounts.bonding_curve))
        .set("associated_bonding_curve", base58::encode(&accounts.associated_bonding_curve))
        .set("associated_user", base58::encode(&accounts.associated_user))
        .set("user", base58::encode(&accounts.user))
        .set("creator_vault", base58::encode(&accounts.creator_vault))
        // event
        .set("mint", base58::encode(&event.mint))
        .set("sol_amount", event.sol_amount)
        .set("token_amount", event.token_amount)
        .set("is_buy", event.is_buy)
        .set("user", base58::encode(&event.user))
        .set("timestamp", event.timestamp)
        .set("virtual_sol_reserves", event.virtual_sol_reserves)
        .set("virtual_token_reserves", event.virtual_token_reserves)
        // (optional) TradeEventV1
        .set("real_sol_reserves", event.real_sol_reserves.unwrap_or(0))
        .set("real_token_reserves", event.real_token_reserves.unwrap_or(0))
        // (optional) TradeEventV2
        .set("protocol_fee_recipient", event.fee_recipient.as_ref().map(base58::encode).unwrap_or_default())
        .set("protocol_fee_basis_points", event.fee_basis_points.unwrap_or(0))
        .set("protocol_fee", event.fee.unwrap_or(0))
        .set("creator", event.creator.as_ref().map(base58::encode).unwrap_or_default())
        .set("creator_fee_basis_points", event.creator_fee_basis_points.unwrap_or(0))
        .set("creator_fee", event.creator_fee.unwrap_or(0));
}
