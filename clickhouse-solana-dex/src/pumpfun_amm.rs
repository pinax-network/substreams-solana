use common::clickhouse::{common_key_v2, set_clock, set_pumpfun_amm_instruction_v2, set_pumpfun_amm_transaction_v2};
use proto::pb::pumpfun::amm::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Row;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::BuyInstruction(data)) => {
                    let Some(event) = get_buy_event(transaction, instruction_index) else {
                        continue;
                    };
                    handle_buy(tables, clock, transaction, instruction, data, event, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::SellInstruction(data)) => {
                    let Some(event) = get_sell_event(transaction, instruction_index) else {
                        continue;
                    };
                    handle_sell(tables, clock, transaction, instruction, data, event, transaction_index, instruction_index);
                }

                _ => {}
            }
        }
    }
}

fn get_buy_event(transaction: &pb::Transaction, instruction_index: usize) -> Option<&pb::BuyEvent> {
    if instruction_index + 1 < transaction.instructions.len() {
        match &transaction.instructions[instruction_index + 1].instruction {
            Some(pb::instruction::Instruction::BuyEvent(event)) => Some(event),
            _ => None,
        }
    } else {
        None
    }
}
fn get_sell_event(transaction: &pb::Transaction, instruction_index: usize) -> Option<&pb::SellEvent> {
    if instruction_index + 1 < transaction.instructions.len() {
        match &transaction.instructions[instruction_index + 1].instruction {
            Some(pb::instruction::Instruction::SellEvent(event)) => Some(event),
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
    event: &pb::BuyEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match &data.accounts {
        Some(accounts) => accounts,
        None => return,
    };
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("pumpfun_amm_buy", key)
        .set("base_amount_out", data.base_amount_out)
        .set("max_quote_amount_in", data.max_quote_amount_in)
        // -- event --
        .set("quote_amount_in", event.quote_amount_in)
        .set("quote_amount_in_with_lp_fee", event.quote_amount_in_with_lp_fee)
        .set("user_quote_amount_in", event.user_quote_amount_in);

    set_trade_account(accounts, row);
    set_pumpfun_amm_instruction_v2(instruction, row);
    set_pumpfun_amm_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_sell(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::SellInstruction,
    event: &pb::SellEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let accounts = match &data.accounts {
        Some(accounts) => accounts,
        None => return,
    };
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("pumpfun_amm_sell", key)
        // -- data --
        .set("base_amount_in", data.base_amount_in)
        .set("min_quote_amount_out", data.min_quote_amount_out)
        // -- event --
        .set("quote_amount_out", event.quote_amount_out)
        .set("quote_amount_out_without_lp_fee", event.quote_amount_out_without_lp_fee)
        .set("user_quote_amount_out", event.user_quote_amount_out);

    set_trade_account(accounts, row);
    set_pumpfun_amm_instruction_v2(instruction, row);
    set_pumpfun_amm_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn set_trade_account(accounts: &pb::TradeAccounts, row: &mut Row) {
    row.set("pool", base58::encode(accounts.pool.as_slice()))
        .set("user", base58::encode(accounts.user.as_slice()))
        .set("global_config", base58::encode(accounts.global_config.as_slice()))
        .set("base_mint", base58::encode(accounts.base_mint.as_slice()))
        .set("quote_mint", base58::encode(accounts.quote_mint.as_slice()))
        .set("user_base_token_account", base58::encode(accounts.user_base_token_account.as_slice()))
        .set("user_quote_token_account", base58::encode(accounts.user_quote_token_account.as_slice()))
        .set("pool_base_token_account", base58::encode(accounts.pool_base_token_account.as_slice()))
        .set("pool_quote_token_account", base58::encode(accounts.pool_quote_token_account.as_slice()))
        .set("protocol_fee_recipient", base58::encode(accounts.protocol_fee_recipient.as_slice()))
        .set(
            "protocol_fee_recipient_token_account",
            base58::encode(accounts.protocol_fee_recipient_token_account.as_slice()),
        )
        // optional fields
        .set(
            "coin_creator_vault_ata",
            accounts.coin_creator_vault_ata.as_ref().map(base58::encode).unwrap_or_default(),
        )
        .set(
            "coin_creator_vault_authority",
            accounts.coin_creator_vault_authority.as_ref().map(base58::encode).unwrap_or_default(),
        );
}
