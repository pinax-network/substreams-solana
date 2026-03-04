use common::db::{common_key_v2, set_clock};
use proto::pb::magiceden::m2::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            match &ix.instruction {
                Some(pb::instruction::Instruction::Sell(data)) => {
                    handle_sell(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::ExecuteSaleV2(data)) => {
                    handle_execute_sale_v2(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::OcpExecuteSaleV2(data)) => {
                    handle_ocp_execute_sale_v2(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::Mip1ExecuteSaleV2(data)) => {
                    handle_mip1_execute_sale_v2(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                _ => {}
            }
        }
    }
}

fn handle_sell(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SellInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m2_sell", key)
        .set("buyer_price", data.buyer_price)
        .set("token_size", data.token_size)
        .set("seller_state_expiry", data.seller_state_expiry);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_execute_sale_v2(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::ExecuteSaleV2Instruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m2_execute_sale", key)
        .set("buyer_price", data.buyer_price)
        .set("token_size", data.token_size)
        .set("buyer_state_expiry", data.buyer_state_expiry)
        .set("seller_state_expiry", data.seller_state_expiry)
        .set("maker_fee_bp", data.maker_fee_bp)
        .set("taker_fee_bp", data.taker_fee_bp);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_ocp_execute_sale_v2(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::OcpExecuteSaleV2Instruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m2_execute_sale", key)
        .set("buyer_price", data.price)
        .set("token_size", 1u64)
        .set("buyer_state_expiry", 0i64)
        .set("seller_state_expiry", 0i64)
        .set("maker_fee_bp", data.maker_fee_bp)
        .set("taker_fee_bp", data.taker_fee_bp);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_mip1_execute_sale_v2(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::Mip1ExecuteSaleV2Instruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m2_execute_sale", key)
        .set("buyer_price", data.price)
        .set("token_size", 1u64)
        .set("buyer_state_expiry", 0i64)
        .set("seller_state_expiry", 0i64)
        .set("maker_fee_bp", data.maker_fee_bp)
        .set("taker_fee_bp", data.taker_fee_bp);
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
