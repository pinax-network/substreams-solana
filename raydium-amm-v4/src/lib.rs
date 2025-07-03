use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_height, parse_program_id, parse_raydium_log};
use proto::pb::raydium::v1 as pb;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana_idls::raydium;

#[substreams::handlers::map]
fn map_events(_params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();

    // let matcher = substreams::expr_matcher(&params);

    let mut base = pb::Instruction {
        program_id: raydium::amm::v4::PROGRAM_ID.to_vec(),
        stack_height: 0, // TO-DO: get stack height from log messages
        instruction: None,
    };

    // transactions
    for tx in block.transactions() {
        let mut transaction = pb::Transaction::default();
        let tx_meta = tx.meta.as_ref().expect("Transaction meta should be present");
        transaction.fee = tx_meta.fee;
        transaction.compute_units_consumed = tx_meta.compute_units_consumed();
        transaction.signature = tx.hash().to_vec();

        if let Some(fee_payer) = get_fee_payer(tx) {
            transaction.fee_payer = fee_payer;
        }
        if let Some(signers) = get_signers(tx) {
            transaction.signers = signers;
        }

        let mut is_invoked = false;

        for log_message in tx_meta.log_messages.iter() {
            // -- must match program ID --
            let match_program_id = match parse_program_id(log_message) {
                Some(id) => id == raydium::amm::v4::PROGRAM_ID.to_vec(),
                None => false,
            };
            // ─── NEW: track invoke / success & stack height ─────────────────────────────
            if is_invoke(log_message) && match_program_id {
                if let Some(h) = parse_invoke_height(log_message) {
                    base.stack_height = h - 1; // stack height is 1-based, so we subtract 1
                    is_invoked = true;
                }
            }

            // Not invoked, skip
            // makes sure we only process logs that are invoked by the program
            // in case of multiple invocations using the same Program Data
            if !is_invoked {
                continue;
            }

            if let Some(data) = parse_raydium_log(&log_message) {
                // -- Events --
                match raydium::amm::v4::events::unpack(data.as_slice()) {
                    // -- SwapBaseIn --
                    Ok(raydium::amm::v4::events::RaydiumV4Event::SwapBaseIn(event)) => {
                        base.instruction = Some(pb::instruction::Instruction::SwapBaseInLog(pb::SwapBaseInLog {
                            amount_in: event.amount_in,
                            minimum_out: event.minimum_out,
                            direction: event.direction,
                            user_source: event.user_source,
                            pool_coin: event.pool_coin,
                            pool_pc: event.pool_pc,
                            out_amount: event.out_amount,
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- SwapBaseOut --
                    Ok(raydium::amm::v4::events::RaydiumV4Event::SwapBaseOut(event)) => {
                        base.instruction = Some(pb::instruction::Instruction::SwapBaseOutLog(pb::SwapBaseOutLog {
                            max_in: event.max_in,
                            amount_out: event.amount_out,
                            direction: event.direction,
                            user_source: event.user_source,
                            pool_coin: event.pool_coin,
                            pool_pc: event.pool_pc,
                            deduct_in: event.deduct_in,
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- InitLog --
                    Ok(raydium::amm::v4::events::RaydiumV4Event::Init(event)) => {
                        base.instruction = Some(pb::instruction::Instruction::InitLog(pb::InitLog {
                            pc_decimals: event.pc_decimals as u32,
                            coin_decimals: event.coin_decimals as u32,
                            pc_lot_size: event.pc_lot_size,
                            coin_lot_size: event.coin_lot_size,
                            pc_amount: event.pc_amount,
                            coin_amount: event.coin_amount,
                            market: event.market.to_bytes().to_vec(),
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- DepositLog --
                    Ok(raydium::amm::v4::events::RaydiumV4Event::Deposit(event)) => {
                        base.instruction = Some(pb::instruction::Instruction::DepositLog(pb::DepositLog {
                            max_coin: event.max_coin,
                            max_pc: event.max_pc,
                            base: event.base,
                            pool_coin: event.pool_coin,
                            pool_pc: event.pool_pc,
                            pool_lp: event.pool_lp,
                            calc_pnl_x: event.calc_pnl_x.to_string(),
                            calc_pnl_y: event.calc_pnl_y.to_string(),
                            deduct_coin: event.deduct_coin,
                            deduct_pc: event.deduct_pc,
                            mint_lp: event.mint_lp,
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- WithdrawLog --
                    Ok(raydium::amm::v4::events::RaydiumV4Event::Withdraw(event)) => {
                        base.instruction = Some(pb::instruction::Instruction::WithdrawLog(pb::WithdrawLog {
                            withdraw_lp: event.withdraw_lp,
                            user_lp: event.user_lp,
                            pool_coin: event.pool_coin,
                            pool_pc: event.pool_pc,
                            pool_lp: event.pool_lp,
                            calc_pnl_x: event.calc_pnl_x.to_string(),
                            calc_pnl_y: event.calc_pnl_y.to_string(),
                            out_coin: event.out_coin,
                            out_pc: event.out_pc,
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    _ => {}
                }
            }
        }
        if !transaction.instructions.is_empty() {
            events.transactions.push(transaction);
        }
    }
    Ok(events)
}
