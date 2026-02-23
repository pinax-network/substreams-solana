use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::pumpswap::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::pumpswap;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &pumpswap::PROGRAM_ID.to_vec());

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

    if program_id != &pumpswap::PROGRAM_ID {
        return None;
    }

    match pumpswap::instructions::unpack(ix.data()) {
        Ok(pumpswap::instructions::PumpSwapInstruction::Buy(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    base_amount_out: event.base_amount_out,
                    max_quote_amount_in: event.max_quote_amount_in,
                })),
            })
        }
        Ok(pumpswap::instructions::PumpSwapInstruction::BuyExactQuoteIn(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::BuyExactQuoteIn(pb::BuyExactQuoteInInstruction {
                    spendable_quote_in: event.spendable_quote_in,
                    min_base_amount_out: event.min_base_amount_out,
                })),
            })
        }
        Ok(pumpswap::instructions::PumpSwapInstruction::Sell(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Sell(pb::SellInstruction {
                    base_amount_in: event.base_amount_in,
                    min_quote_amount_out: event.min_quote_amount_out,
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
    match pumpswap::events::unpack_event(data.as_slice()) {
        Ok(pumpswap::events::PumpSwapEvent::Buy(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Buy(pb::BuyEvent {
                timestamp: event.timestamp,
                base_amount_out: event.base_amount_out,
                max_quote_amount_in: event.max_quote_amount_in,
                user_base_token_reserves: event.user_base_token_reserves,
                user_quote_token_reserves: event.user_quote_token_reserves,
                pool_base_token_reserves: event.pool_base_token_reserves,
                pool_quote_token_reserves: event.pool_quote_token_reserves,
                quote_amount_in: event.quote_amount_in,
                lp_fee_basis_points: event.lp_fee_basis_points,
                lp_fee: event.lp_fee,
                protocol_fee_basis_points: event.protocol_fee_basis_points,
                protocol_fee: event.protocol_fee,
                quote_amount_in_with_lp_fee: event.quote_amount_in_with_lp_fee,
                user_quote_amount_in: event.user_quote_amount_in,
                pool: event.pool.to_bytes().to_vec(),
                user: event.user.to_bytes().to_vec(),
                coin_creator: event.coin_creator.to_bytes().to_vec(),
                coin_creator_fee_basis_points: event.coin_creator_fee_basis_points,
                coin_creator_fee: event.coin_creator_fee,
                min_base_amount_out: event.min_base_amount_out,
                ix_name: event.ix_name,
            })),
        }),
        Ok(pumpswap::events::PumpSwapEvent::Sell(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Sell(pb::SellEvent {
                timestamp: event.timestamp,
                base_amount_in: event.base_amount_in,
                min_quote_amount_out: event.min_quote_amount_out,
                user_base_token_reserves: event.user_base_token_reserves,
                user_quote_token_reserves: event.user_quote_token_reserves,
                pool_base_token_reserves: event.pool_base_token_reserves,
                pool_quote_token_reserves: event.pool_quote_token_reserves,
                quote_amount_out: event.quote_amount_out,
                lp_fee_basis_points: event.lp_fee_basis_points,
                lp_fee: event.lp_fee,
                protocol_fee_basis_points: event.protocol_fee_basis_points,
                protocol_fee: event.protocol_fee,
                quote_amount_out_without_lp_fee: event.quote_amount_out_without_lp_fee,
                user_quote_amount_out: event.user_quote_amount_out,
                pool: event.pool.to_bytes().to_vec(),
                user: event.user.to_bytes().to_vec(),
                coin_creator: event.coin_creator.to_bytes().to_vec(),
                coin_creator_fee_basis_points: event.coin_creator_fee_basis_points,
                coin_creator_fee: event.coin_creator_fee,
            })),
        }),
        Ok(pumpswap::events::PumpSwapEvent::CreatePool(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::CreatePool(pb::CreatePoolEvent {
                timestamp: event.timestamp,
                index: event.index as u32,
                creator: event.creator.to_bytes().to_vec(),
                base_mint: event.base_mint.to_bytes().to_vec(),
                quote_mint: event.quote_mint.to_bytes().to_vec(),
                base_mint_decimals: event.base_mint_decimals as u32,
                quote_mint_decimals: event.quote_mint_decimals as u32,
                base_amount_in: event.base_amount_in,
                quote_amount_in: event.quote_amount_in,
                pool_base_amount: event.pool_base_amount,
                pool_quote_amount: event.pool_quote_amount,
                minimum_liquidity: event.minimum_liquidity,
                initial_liquidity: event.initial_liquidity,
                lp_token_amount_out: event.lp_token_amount_out,
                pool_bump: event.pool_bump as u32,
                pool: event.pool.to_bytes().to_vec(),
                lp_mint: event.lp_mint.to_bytes().to_vec(),
                coin_creator: event.coin_creator.to_bytes().to_vec(),
            })),
        }),
        _ => None,
    }
}
