use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::darklake::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::darklake;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &darklake::PROGRAM_ID.to_vec());

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

fn process_instruction(ix: &InstructionView) -> Option<pb::Instruction> {
    let program_id = ix.program_id().0;

    if program_id != &darklake::PROGRAM_ID {
        return None;
    }

    match darklake::instructions::unpack(ix.data()) {
        Ok(darklake::instructions::DarklakeInstruction::Swap(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    amount_in: event.amount_in,
                    is_swap_x_to_y: event.is_swap_x_to_y,
                    c_min: event.c_min.to_vec(),
                    label: event.label.map(|l| l.to_vec()),
                })),
            })
        }
        _ => None,
    }
}

fn process_logs(tx_meta: &TransactionStatusMeta, program_id_bytes: &[u8]) -> Vec<pb::Log> {
    let mut logs = Vec::new();
    let mut is_invoked = false;

    for log_message in tx_meta.log_messages.iter() {
        let match_program_id = parse_program_id(log_message).map_or(false, |id| id == program_id_bytes.to_vec());

        if is_invoke(log_message) && match_program_id {
            if let Some(invoke_depth) = parse_invoke_depth(log_message) {
                is_invoked = true;
                if let Some(log_data) = parse_log_data(log_message, program_id_bytes, invoke_depth) {
                    logs.push(log_data);
                }
            }
        } else if match_program_id && (is_success(log_message) || is_failed(log_message)) {
            is_invoked = false;
        } else if is_invoked {
            if let Some(log_data) = parse_log_data(log_message, program_id_bytes, 0) {
                logs.push(log_data);
            }
        }
    }

    logs
}

fn parse_log_data(log_message: &str, program_id_bytes: &[u8], invoke_depth: u32) -> Option<pb::Log> {
    let data = parse_program_data(log_message)?;
    match darklake::events::unpack_event(data.as_slice()) {
        Ok(darklake::events::DarklakeEvent::Swap(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Swap(pb::SwapEvent {
                trader: event.trader.to_bytes().to_vec(),
                direction: event.direction as u32,
                deadline: event.deadline,
                trade_fee: event.trade_fee,
                protocol_fee: event.protocol_fee,
                amount_in: event.amount_in,
                amount_out: event.amount_out,
                actual_amount_in: event.actual_amount_in,
                wsol_deposit: event.wsol_deposit,
                actual_amount_out: event.actual_amount_out,
                new_reserve_x: event.new_reserve_x,
                new_reserve_y: event.new_reserve_y,
                available_reserve_x: event.available_reserve_x,
                available_reserve_y: event.available_reserve_y,
                locked_x: event.locked_x,
                locked_y: event.locked_y,
                user_locked_x: event.user_locked_x,
                user_locked_y: event.user_locked_y,
                protocol_fee_x: event.protocol_fee_x,
                protocol_fee_y: event.protocol_fee_y,
                user_token_account_x: event.user_token_account_x.to_bytes().to_vec(),
                user_token_account_y: event.user_token_account_y.to_bytes().to_vec(),
                token_mint_lp: event.token_mint_lp.to_bytes().to_vec(),
                token_mint_x: event.token_mint_x.to_bytes().to_vec(),
                token_mint_y: event.token_mint_y.to_bytes().to_vec(),
                label: event.label,
            })),
        }),
        Ok(darklake::events::DarklakeEvent::InitializePool(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::InitializePool(pb::InitializePoolEvent {
                trader: event.trader.to_bytes().to_vec(),
                liquidity_minted: event.liquidity_minted,
                sol_create_pool_fee: event.sol_create_pool_fee,
                new_reserve_x: event.new_reserve_x,
                new_reserve_y: event.new_reserve_y,
                token_mint_x: event.token_mint_x.to_bytes().to_vec(),
                token_mint_y: event.token_mint_y.to_bytes().to_vec(),
                token_mint_lp: event.token_mint_lp.to_bytes().to_vec(),
                label: event.label,
            })),
        }),
        Ok(darklake::events::DarklakeEvent::AddLiquidity(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::AddLiquidity(pb::AddLiquidityEvent {
                supplier: event.supplier.to_bytes().to_vec(),
                max_amount_x: event.max_amount_x,
                max_amount_y: event.max_amount_y,
                transfer_in_x: event.transfer_in_x,
                transfer_in_y: event.transfer_in_y,
                liquidity_minted: event.liquidity_minted,
                user_token_lp_balance: event.user_token_lp_balance,
                new_reserve_x: event.new_reserve_x,
                new_reserve_y: event.new_reserve_y,
                available_reserve_x: event.available_reserve_x,
                available_reserve_y: event.available_reserve_y,
                token_mint_lp: event.token_mint_lp.to_bytes().to_vec(),
                token_mint_x: event.token_mint_x.to_bytes().to_vec(),
                token_mint_y: event.token_mint_y.to_bytes().to_vec(),
                ref_code: event.ref_code,
                label: event.label,
            })),
        }),
        Ok(darklake::events::DarklakeEvent::RemoveLiquidity(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::RemoveLiquidity(pb::RemoveLiquidityEvent {
                supplier: event.supplier.to_bytes().to_vec(),
                min_amount_x: event.min_amount_x,
                min_amount_y: event.min_amount_y,
                transfer_out_x: event.transfer_out_x,
                transfer_out_y: event.transfer_out_y,
                liquidity_burned: event.liquidity_burned,
                user_token_lp_balance: event.user_token_lp_balance,
                new_reserve_x: event.new_reserve_x,
                new_reserve_y: event.new_reserve_y,
                available_reserve_x: event.available_reserve_x,
                available_reserve_y: event.available_reserve_y,
                token_mint_lp: event.token_mint_lp.to_bytes().to_vec(),
                token_mint_x: event.token_mint_x.to_bytes().to_vec(),
                token_mint_y: event.token_mint_y.to_bytes().to_vec(),
                label: event.label,
            })),
        }),
        _ => None,
    }
}
