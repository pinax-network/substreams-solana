use common::db::{common_key_v2, set_clock};
use proto::pb::magiceden::m3::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            match &ix.instruction {
                Some(pb::instruction::Instruction::SolFulfillBuy(data)) => {
                    handle_fulfill_buy(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::SolMip1FulfillBuy(data)) => {
                    handle_mip1_fulfill_buy(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::SolOcpFulfillBuy(data)) => {
                    handle_ocp_fulfill_buy(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::SolFulfillSell(data)) => {
                    handle_fulfill_sell(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::SolMip1FulfillSell(data)) => {
                    handle_mip1_fulfill_sell(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                Some(pb::instruction::Instruction::SolOcpFulfillSell(data)) => {
                    handle_ocp_fulfill_sell(tables, clock, tx, ix, data, transaction_index, instruction_index);
                }
                _ => {}
            }
        }
    }
}

fn handle_fulfill_buy(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SolFulfillBuyInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m3_fulfill_buy", key)
        .set("asset_amount", data.asset_amount)
        .set("min_payment_amount", data.min_payment_amount)
        .set("maker_fee_bp", data.maker_fee_bp)
        .set("taker_fee_bp", data.taker_fee_bp);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_mip1_fulfill_buy(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SolMip1FulfillBuyInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m3_fulfill_buy", key)
        .set("asset_amount", data.asset_amount)
        .set("min_payment_amount", data.min_payment_amount)
        .set("maker_fee_bp", data.maker_fee_bp)
        .set("taker_fee_bp", data.taker_fee_bp);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_ocp_fulfill_buy(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SolOcpFulfillBuyInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m3_fulfill_buy", key)
        .set("asset_amount", data.asset_amount)
        .set("min_payment_amount", data.min_payment_amount)
        .set("maker_fee_bp", data.maker_fee_bp)
        .set("taker_fee_bp", data.taker_fee_bp);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_fulfill_sell(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SolFulfillSellInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m3_fulfill_sell", key)
        .set("asset_amount", data.asset_amount)
        .set("max_payment_amount", data.max_payment_amount)
        .set("buyside_creator_royalty_bp", data.buyside_creator_royalty_bp)
        .set("maker_fee_bp", data.maker_fee_bp)
        .set("taker_fee_bp", data.taker_fee_bp);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_mip1_fulfill_sell(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SolMip1FulfillSellInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m3_fulfill_sell", key)
        .set("asset_amount", data.asset_amount)
        .set("max_payment_amount", data.max_payment_amount)
        .set("buyside_creator_royalty_bp", 0u32)
        .set("maker_fee_bp", data.maker_fee_bp)
        .set("taker_fee_bp", data.taker_fee_bp);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn handle_ocp_fulfill_sell(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    data: &pb::SolOcpFulfillSellInstruction,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("magiceden_m3_fulfill_sell", key)
        .set("asset_amount", data.asset_amount)
        .set("max_payment_amount", data.max_payment_amount)
        .set("buyside_creator_royalty_bp", 0u32)
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
