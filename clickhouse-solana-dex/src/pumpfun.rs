use common::clickhouse::{common_key_v2, set_clock, set_pumpfun_instruction_v2, set_pumpfun_transaction_v2};
use proto::pb::pumpfun::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in transaction.instructions.iter().enumerate() {
            // Get TradeEvent from the next instruction
            let event = match transaction.instructions[instruction_index + 1].instruction {
                Some(pb::instruction::Instruction::Trade(ref event)) => event,
                _ => panic!("{} signature: {}", instruction_index, base58::encode(&transaction.signature)), // Skip if not a TradeEvent
            };

            match &instruction.instruction {
                Some(pb::instruction::Instruction::Buy(data)) => {
                    handle_buy(tables, clock, transaction, instruction, data, event, transaction_index, instruction_index);
                }

                _ => {}
            }
        }
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
        // accounts
        .set("global", base58::encode(&accounts.mint))
        .set("fee_recipient", base58::encode(&accounts.fee_recipient))
        .set("mint", base58::encode(&accounts.mint))
        .set("bonding_curve", base58::encode(&accounts.bonding_curve))
        .set("associated_bonding_curve", base58::encode(&accounts.associated_bonding_curve))
        .set("associated_user", base58::encode(&accounts.associated_user))
        .set("user", base58::encode(&accounts.user))
        .set("creator_vault", base58::encode(&accounts.creator_vault))
        // data
        .set("amount", data.amount)
        .set("max_sol_cost", data.max_sol_cost)
        // event
        .set("mint", base58::encode(&event.mint))
        .set("sol_amount", event.sol_amount)
        .set("token_amount", event.token_amount)
        .set("is_buy", event.is_buy)
        .set("user", base58::encode(&event.user))
        .set("timestamp", event.timestamp)
        .set("virtual_sol_reserves", event.virtual_sol_reserves)
        .set("virtual_token_reserves", event.virtual_token_reserves)
        .set("real_sol_reserves", event.real_sol_reserves)
        .set("real_token_reserves", event.real_token_reserves)
        // optional fields
        .set(
            "fee_recipient",
            event.fee_recipient.as_ref().map_or_else(|| "".to_string(), |c| base58::encode(c)),
        )
        .set("fee_basis_points", event.fee_basis_points.unwrap_or(0))
        .set("fee", event.fee.unwrap_or(0))
        // .set("creator", base58::encode(&event.creator))
        .set("creator", event.creator.as_ref().map_or_else(|| "".to_string(), |c| base58::encode(c)))
        .set("creator_fee_basis_points", event.creator_fee_basis_points.unwrap_or(0))
        .set("creator_fee", event.creator_fee.unwrap_or(0));

    set_pumpfun_instruction_v2(instruction, row);
    set_pumpfun_transaction_v2(transaction, row);
    set_clock(clock, row);
}

// // One emitted trade (buy or sell) on a Pump.fun bonding curve.
// message TradeEvent {
//   // 32-byte SPL-Token mint address.
//   bytes  mint                       = 1;

//   // Lamports moved (positive on buys, negative on sells).
//   uint64 sol_amount                 = 2;

//   // Token amount moved (positive on buys, negative on sells).
//   uint64 token_amount               = 3;

//   // true = buy (SOL→SPL), false = sell.
//   bool   is_buy                     = 4;

//   // Trader’s wallet (32 bytes).
//   bytes  user                       = 5;

//   // Unix-epoch seconds.
//   int64  timestamp                  = 6;

//   uint64 virtual_sol_reserves       = 7;
//   uint64 virtual_token_reserves     = 8;
//   uint64 real_sol_reserves          = 9;
//   uint64 real_token_reserves        = 10;

//   // Protocol-fee recipient (32 bytes).
//   optional bytes  fee_recipient              = 11;
//   optional uint64 fee_basis_points           = 12; // basis-points, 1 bp = 0.01 %
//   optional uint64 fee                        = 13; // lamports

//   // Pool creator wallet (32 bytes).
//   optional bytes  creator                    = 14;
//   optional uint64 creator_fee_basis_points   = 15;
//   optional uint64 creator_fee                = 16; // lamports
// }

// message BuyInstruction {
//     TradeAccounts accounts = 1;
//     uint64 amount = 2;
//     uint64 max_sol_cost = 3;
// }
