use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_id, parse_raydium_log};
use proto::pb::raydium::amm::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::raydium;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    // Process instructions first
    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();

    // Process logs
    let logs = process_logs(tx_meta, &raydium::amm::v4::PROGRAM_ID.to_vec());

    // Only return a transaction if it has either instructions or logs
    if instructions.is_empty() && logs.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
        logs,
    })
}

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;

    // Skip instructions that don't match our program ID
    if program_id != &raydium::amm::v4::PROGRAM_ID {
        return None;
    }

    // Try to unpack the instruction data
    match raydium::amm::v4::instructions::unpack(instruction.data()) {
        // -- SwapBaseIn --
        Ok(raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event)) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: instruction.stack_height(),
            instruction: Some(pb::instruction::Instruction::SwapBaseIn(pb::SwapBaseInInstruction {
                accounts: Some(get_swap_accounts(instruction)),
                amount_in: event.amount_in,
                minimum_amount_out: event.minimum_amount_out,
            })),
        }),
        // -- SwapBaseOut --
        Ok(raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseOut(event)) => Some(pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: instruction.stack_height(),
            instruction: Some(pb::instruction::Instruction::SwapBaseOut(pb::SwapBaseOutInstruction {
                accounts: Some(get_swap_accounts(instruction)),
                amount_out: event.amount_out,
                max_amount_in: event.max_amount_in,
            })),
        }),
        _ => None,
    }
}

fn process_logs(tx_meta: &TransactionStatusMeta, program_id_bytes: &[u8]) -> Vec<pb::Log> {
    let mut logs = Vec::new();
    let mut is_invoked = false;

    for log_message in tx_meta.log_messages.iter() {
        // Check if the log matches our program ID
        let match_program_id = parse_program_id(log_message).map_or(false, |id| id == program_id_bytes.to_vec());

        // Track invoke depth and program context
        if is_invoke(log_message) && match_program_id {
            if let Some(invoke_depth) = parse_invoke_depth(log_message) {
                is_invoked = true;

                if let Some(log_data) = parse_log_data(log_message, program_id_bytes, invoke_depth) {
                    logs.push(log_data);
                }
            }
        } else if is_invoked {
            // Process logs within an invoked context
            if let Some(log_data) = parse_log_data(log_message, program_id_bytes, 0) {
                logs.push(log_data);
            }
        }
    }

    logs
}

fn parse_log_data(log_message: &str, program_id_bytes: &[u8], invoke_depth: u32) -> Option<pb::Log> {
    let data = parse_raydium_log(log_message)?;

    // Create base log structure
    let mut log = pb::Log {
        program_id: program_id_bytes.to_vec(),
        invoke_depth,
        log: None,
    };

    // Process different log types
    match raydium::amm::v4::logs::unpack(data.as_slice()) {
        // -- SwapBaseIn --
        Ok(raydium::amm::v4::logs::RaydiumV4Log::SwapBaseIn(event)) => {
            log.log = Some(pb::log::Log::SwapBaseIn(pb::SwapBaseInLog {
                amount_in: event.amount_in,
                minimum_out: event.minimum_out,
                direction: event.direction,
                user_source: event.user_source,
                pool_coin: event.pool_coin,
                pool_pc: event.pool_pc,
                out_amount: event.out_amount,
            }));
            Some(log)
        }
        // -- SwapBaseOut --
        Ok(raydium::amm::v4::logs::RaydiumV4Log::SwapBaseOut(event)) => {
            log.log = Some(pb::log::Log::SwapBaseOut(pb::SwapBaseOutLog {
                max_in: event.max_in,
                amount_out: event.amount_out,
                direction: event.direction,
                user_source: event.user_source,
                pool_coin: event.pool_coin,
                pool_pc: event.pool_pc,
                deduct_in: event.deduct_in,
            }));
            Some(log)
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
        _ => None,
    }
}

/// Safely get the `idx`-th account's pubkey bytes or empty vector if not found.
///
/// * `ix`   – the fully-parsed instruction (`InstructionView`)
/// * `idx`  – the position in the account vector
#[inline]
fn account_bytes(ix: &InstructionView, idx: usize) -> Vec<u8> {
    ix.accounts()[idx].0.to_vec()
}

pub fn get_swap_accounts(ix: &InstructionView) -> pb::SwapAccounts {
    let with_target_orders = ix.accounts().len() == 18; // v4 = 18, legacy = 17
    let offset = if with_target_orders { 1 } else { 0 }; // how many slots to shift after we pass index 3

    pb::SwapAccounts {
        // fixed positions
        token_program: account_bytes(ix, 0),
        amm: account_bytes(ix, 1),
        amm_authority: account_bytes(ix, 2),
        amm_open_orders: account_bytes(ix, 3),
        // new in Raydium-v4
        amm_target_orders: if with_target_orders { Some(account_bytes(ix, 4)) } else { None },
        // everything after index 3 shifts by +1 when target-orders is present
        amm_coin_vault: account_bytes(ix, 4 + offset),
        amm_pc_vault: account_bytes(ix, 5 + offset),
        market_program: account_bytes(ix, 6 + offset),
        market: account_bytes(ix, 7 + offset),
        market_bids: account_bytes(ix, 8 + offset),
        market_asks: account_bytes(ix, 9 + offset),
        market_event_queue: account_bytes(ix, 10 + offset),
        market_coin_vault: account_bytes(ix, 11 + offset),
        market_pc_vault: account_bytes(ix, 12 + offset),
        market_vault_signer: account_bytes(ix, 13 + offset),
        user_token_source: account_bytes(ix, 14 + offset),
        user_token_destination: account_bytes(ix, 15 + offset),
        user_source_owner: account_bytes(ix, 16 + offset),
    }
}
