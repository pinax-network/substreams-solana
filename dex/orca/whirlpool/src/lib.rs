use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::orca::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::orca;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &orca::whirlpool::PROGRAM_ID.to_vec());

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

    if program_id != &orca::whirlpool::PROGRAM_ID {
        return None;
    }

    match orca::whirlpool::instructions::unpack(ix.data()) {
        Ok(orca::whirlpool::instructions::WhirlpoolInstruction::Swap(event)) => {
            let accounts = orca::whirlpool::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::SwapAccounts {
                        token_program: accounts.token_program.to_bytes().to_vec(),
                        token_authority: accounts.token_authority.to_bytes().to_vec(),
                        whirlpool: accounts.whirlpool.to_bytes().to_vec(),
                        token_owner_account_a: accounts.token_owner_account_a.to_bytes().to_vec(),
                        token_vault_a: accounts.token_vault_a.to_bytes().to_vec(),
                        token_owner_account_b: accounts.token_owner_account_b.to_bytes().to_vec(),
                        token_vault_b: accounts.token_vault_b.to_bytes().to_vec(),
                        tick_array0: accounts.tick_array0.to_bytes().to_vec(),
                        tick_array1: accounts.tick_array1.to_bytes().to_vec(),
                        tick_array2: accounts.tick_array2.to_bytes().to_vec(),
                        oracle: accounts.oracle.to_bytes().to_vec(),
                    }),
                    amount: event.amount,
                    other_amount_threshold: event.other_amount_threshold,
                    sqrt_price_limit: event.sqrt_price_limit.to_string(),
                    amount_specified_is_input: event.amount_specified_is_input,
                    a_to_b: event.a_to_b,
                })),
            })
        }
        Ok(orca::whirlpool::instructions::WhirlpoolInstruction::SwapV2(event)) => {
            let accounts = orca::whirlpool::accounts::get_swap_v2_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::SwapV2(pb::SwapV2Instruction {
                    accounts: Some(pb::SwapV2Accounts {
                        token_program_a: accounts.token_program_a.to_bytes().to_vec(),
                        token_program_b: accounts.token_program_b.to_bytes().to_vec(),
                        memo_program: accounts.memo_program.to_bytes().to_vec(),
                        token_authority: accounts.token_authority.to_bytes().to_vec(),
                        whirlpool: accounts.whirlpool.to_bytes().to_vec(),
                        token_mint_a: accounts.token_mint_a.to_bytes().to_vec(),
                        token_mint_b: accounts.token_mint_b.to_bytes().to_vec(),
                        token_owner_account_a: accounts.token_owner_account_a.to_bytes().to_vec(),
                        token_vault_a: accounts.token_vault_a.to_bytes().to_vec(),
                        token_owner_account_b: accounts.token_owner_account_b.to_bytes().to_vec(),
                        token_vault_b: accounts.token_vault_b.to_bytes().to_vec(),
                        tick_array0: accounts.tick_array0.to_bytes().to_vec(),
                        tick_array1: accounts.tick_array1.to_bytes().to_vec(),
                        tick_array2: accounts.tick_array2.to_bytes().to_vec(),
                        oracle: accounts.oracle.to_bytes().to_vec(),
                    }),
                    amount: event.amount,
                    other_amount_threshold: event.other_amount_threshold,
                    sqrt_price_limit: event.sqrt_price_limit.to_string(),
                    amount_specified_is_input: event.amount_specified_is_input,
                    a_to_b: event.a_to_b,
                })),
            })
        }
        Ok(orca::whirlpool::instructions::WhirlpoolInstruction::TwoHopSwap(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TwoHopSwap(pb::TwoHopSwapInstruction {
                    amount: event.amount,
                    other_amount_threshold: event.other_amount_threshold,
                    amount_specified_is_input: event.amount_specified_is_input,
                    a_to_b_one: event.a_to_b_one,
                    a_to_b_two: event.a_to_b_two,
                    sqrt_price_limit_one: event.sqrt_price_limit_one.to_string(),
                    sqrt_price_limit_two: event.sqrt_price_limit_two.to_string(),
                })),
            })
        }
        Ok(orca::whirlpool::instructions::WhirlpoolInstruction::TwoHopSwapV2(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TwoHopSwapV2(pb::TwoHopSwapV2Instruction {
                    amount: event.amount,
                    other_amount_threshold: event.other_amount_threshold,
                    amount_specified_is_input: event.amount_specified_is_input,
                    a_to_b_one: event.a_to_b_one,
                    a_to_b_two: event.a_to_b_two,
                    sqrt_price_limit_one: event.sqrt_price_limit_one.to_string(),
                    sqrt_price_limit_two: event.sqrt_price_limit_two.to_string(),
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
    match orca::whirlpool::events::parse_event(data.as_slice()) {
        Ok(orca::whirlpool::events::WhirlpoolEvent::Traded(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Traded(pb::TradedEvent {
                whirlpool: event.whirlpool.to_bytes().to_vec(),
                a_to_b: event.a_to_b,
                pre_sqrt_price: event.pre_sqrt_price.to_string(),
                post_sqrt_price: event.post_sqrt_price.to_string(),
                input_amount: event.input_amount,
                output_amount: event.output_amount,
                input_transfer_fee: event.input_transfer_fee,
                output_transfer_fee: event.output_transfer_fee,
                lp_fee: event.lp_fee,
                protocol_fee: event.protocol_fee,
            })),
        }),
        Ok(orca::whirlpool::events::WhirlpoolEvent::PoolInitialized(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::PoolInitialized(pb::PoolInitializedEvent {
                whirlpool: event.whirlpool.to_bytes().to_vec(),
                whirlpools_config: event.whirlpools_config.to_bytes().to_vec(),
                token_mint_a: event.token_mint_a.to_bytes().to_vec(),
                token_mint_b: event.token_mint_b.to_bytes().to_vec(),
                tick_spacing: event.tick_spacing as u32,
                token_program_a: event.token_program_a.to_bytes().to_vec(),
                token_program_b: event.token_program_b.to_bytes().to_vec(),
                decimals_a: event.decimals_a as u32,
                decimals_b: event.decimals_b as u32,
                initial_sqrt_price: event.initial_sqrt_price.to_string(),
            })),
        }),
        Ok(orca::whirlpool::events::WhirlpoolEvent::LiquidityIncreased(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::LiquidityIncreased(pb::LiquidityIncreasedEvent {
                whirlpool: event.whirlpool.to_bytes().to_vec(),
                position: event.position.to_bytes().to_vec(),
                tick_lower_index: event.tick_lower_index,
                tick_upper_index: event.tick_upper_index,
                liquidity: event.liquidity.to_string(),
                token_a_amount: event.token_a_amount,
                token_b_amount: event.token_b_amount,
                token_a_transfer_fee: event.token_a_transfer_fee,
                token_b_transfer_fee: event.token_b_transfer_fee,
            })),
        }),
        Ok(orca::whirlpool::events::WhirlpoolEvent::LiquidityDecreased(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::LiquidityDecreased(pb::LiquidityDecreasedEvent {
                whirlpool: event.whirlpool.to_bytes().to_vec(),
                position: event.position.to_bytes().to_vec(),
                tick_lower_index: event.tick_lower_index,
                tick_upper_index: event.tick_upper_index,
                liquidity: event.liquidity.to_string(),
                token_a_amount: event.token_a_amount,
                token_b_amount: event.token_b_amount,
                token_a_transfer_fee: event.token_a_transfer_fee,
                token_b_transfer_fee: event.token_b_transfer_fee,
            })),
        }),
        _ => None,
    }
}
