use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_id, parse_raydium_log};
use proto::pb::raydium::amm::v1 as pb;
use substreams::log;
use substreams_solana::{base58, block_view::InstructionView, pb::sf::solana::r#type::v1::Block};
use substreams_solana_idls::raydium;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();

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

        // -- Instructions --
        // Include instructions and events
        for instruction in tx.walk_instructions() {
            let program_id = instruction.program_id().0;

            // Skip instructions
            if program_id != &raydium::amm::v4::PROGRAM_ID.to_vec() {
                continue;
            }
            let mut base = pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: instruction.stack_height(),
                instruction: None,
            };
            // -- Events --
            match raydium::amm::v4::instructions::unpack(instruction.data()) {
                // -- SwapBaseIn --
                Ok(raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::SwapBaseIn(pb::SwapBaseInInstruction {
                        accounts: Some(get_swap_accounts(&instruction)),
                        amount_in: event.amount_in,
                        minimum_amount_out: event.minimum_amount_out,
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- SwapBaseOut --
                Ok(raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseOut(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::SwapBaseOut(pb::SwapBaseOutInstruction {
                        accounts: Some(get_swap_accounts(&instruction)),
                        amount_out: event.amount_out,
                        max_amount_in: event.max_amount_in,
                    }));
                    transaction.instructions.push(base.clone());
                }
                _ => {}
            }
        }

        // -- Logs --
        let mut is_invoked = false;
        for log_message in tx_meta.log_messages.iter() {
            // -- must match program ID --
            let match_program_id = match parse_program_id(log_message) {
                Some(id) => id == raydium::amm::v4::PROGRAM_ID.to_vec(),
                None => false,
            };

            let mut base = pb::Log {
                program_id: raydium::amm::v4::PROGRAM_ID.to_vec(),
                invoke_depth: 0,
                log: None,
            };

            // ─── NEW: track invoke / success & stack height ─────────────────────────────
            if is_invoke(log_message) && match_program_id {
                if let Some(i) = parse_invoke_depth(log_message) {
                    base.invoke_depth = i;
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
                        base.log = Some(pb::log::Log::SwapBaseIn(pb::SwapBaseInLog {
                            amount_in: event.amount_in,
                            minimum_out: event.minimum_out,
                            direction: event.direction,
                            user_source: event.user_source,
                            pool_coin: event.pool_coin,
                            pool_pc: event.pool_pc,
                            out_amount: event.out_amount,
                        }));
                        transaction.logs.push(base.clone());
                    }
                    // -- SwapBaseOut --
                    Ok(raydium::amm::v4::events::RaydiumV4Event::SwapBaseOut(event)) => {
                        base.log = Some(pb::log::Log::SwapBaseOut(pb::SwapBaseOutLog {
                            max_in: event.max_in,
                            amount_out: event.amount_out,
                            direction: event.direction,
                            user_source: event.user_source,
                            pool_coin: event.pool_coin,
                            pool_pc: event.pool_pc,
                            deduct_in: event.deduct_in,
                        }));
                        transaction.logs.push(base.clone());
                    }
                    // // -- InitLog --
                    // Ok(raydium::amm::v4::events::RaydiumV4Event::Init(event)) => {
                    //     base.log = Some(pb::log::Log::Init(pb::InitLog {
                    //         pc_decimals: event.pc_decimals as u32,
                    //         coin_decimals: event.coin_decimals as u32,
                    //         pc_lot_size: event.pc_lot_size,
                    //         coin_lot_size: event.coin_lot_size,
                    //         pc_amount: event.pc_amount,
                    //         coin_amount: event.coin_amount,
                    //         market: event.market.to_bytes().to_vec(),
                    //     }));
                    //     transaction.logs.push(base.clone());
                    // }
                    // // -- DepositLog --
                    // Ok(raydium::amm::v4::events::RaydiumV4Event::Deposit(event)) => {
                    //     base.log = Some(pb::log::Log::Deposit(pb::DepositLog {
                    //         max_coin: event.max_coin,
                    //         max_pc: event.max_pc,
                    //         base: event.base,
                    //         pool_coin: event.pool_coin,
                    //         pool_pc: event.pool_pc,
                    //         pool_lp: event.pool_lp,
                    //         calc_pnl_x: event.calc_pnl_x.to_string(),
                    //         calc_pnl_y: event.calc_pnl_y.to_string(),
                    //         deduct_coin: event.deduct_coin,
                    //         deduct_pc: event.deduct_pc,
                    //         mint_lp: event.mint_lp,
                    //     }));
                    //     transaction.logs.push(base.clone());
                    // }
                    // // -- WithdrawLog --
                    // Ok(raydium::amm::v4::events::RaydiumV4Event::Withdraw(event)) => {
                    //     base.log = Some(pb::log::Log::Withdraw(pb::WithdrawLog {
                    //         withdraw_lp: event.withdraw_lp,
                    //         user_lp: event.user_lp,
                    //         pool_coin: event.pool_coin,
                    //         pool_pc: event.pool_pc,
                    //         pool_lp: event.pool_lp,
                    //         calc_pnl_x: event.calc_pnl_x.to_string(),
                    //         calc_pnl_y: event.calc_pnl_y.to_string(),
                    //         out_coin: event.out_coin,
                    //         out_pc: event.out_pc,
                    //     }));
                    //     transaction.logs.push(base.clone());
                    // }
                    _ => {}
                }
            }
        }
        if !transaction.logs.is_empty() || !transaction.instructions.is_empty() {
            if transaction.logs.len() != transaction.instructions.len() {
                log::info!(
                    "Transaction logs and instructions count mismatch: {} logs, {} instructions - transaction:\n{}",
                    transaction.logs.len(),
                    transaction.instructions.len(),
                    base58::encode(transaction.clone().signature)
                );
                events.transactions.push(transaction);
            }
        }
    }
    Ok(events)
}

pub fn get_swap_accounts(instruction: &InstructionView) -> pb::SwapAccounts {
    pb::SwapAccounts {
        token_program: instruction.accounts()[0].0.to_vec(),
        amm_id: instruction.accounts()[1].0.to_vec(),
        amm_authority: instruction.accounts()[2].0.to_vec(),
        amm_open_orders: instruction.accounts()[3].0.to_vec(),
        pool_coin_token_account: instruction.accounts()[4].0.to_vec(),
        pool_pc_token_account: instruction.accounts()[5].0.to_vec(),
        serum_program_id: instruction.accounts()[6].0.to_vec(),
        serum_market: instruction.accounts()[7].0.to_vec(),
        serum_bids: instruction.accounts()[8].0.to_vec(),
        serum_asks: instruction.accounts()[8].0.to_vec(),
        serum_event_queue: instruction.accounts()[10].0.to_vec(),
        serum_coin_vault_account: instruction.accounts()[11].0.to_vec(),
        serum_pc_vault: instruction.accounts()[12].0.to_vec(),
        serum_vault_signer: instruction.accounts()[13].0.to_vec(),
        user_source_token_account: instruction.accounts()[14].0.to_vec(),
        user_dest_token_account: instruction.accounts()[15].0.to_vec(),
        user_owner: instruction.accounts()[16].0.to_vec(),
    }
}
