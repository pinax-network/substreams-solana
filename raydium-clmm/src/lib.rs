use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::raydium::clmm::v1 as pb;
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

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &raydium::clmm::v3::PROGRAM_ID.to_vec());

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

    if program_id != &raydium::clmm::v3::PROGRAM_ID {
        return None;
    }

    match raydium::clmm::v3::instructions::unpack(ix.data()) {
        Ok(raydium::clmm::v3::instructions::RaydiumClmmInstruction::Swap(event)) => {
            let accounts = raydium::clmm::v3::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::swap_instruction::Accounts::V1Accounts(pb::SwapAccounts {
                        payer: accounts.payer.to_bytes().to_vec(),
                        amm_config: accounts.amm_config.to_bytes().to_vec(),
                        pool_state: accounts.pool_state.to_bytes().to_vec(),
                        input_token_account: accounts.input_token_account.to_bytes().to_vec(),
                        output_token_account: accounts.output_token_account.to_bytes().to_vec(),
                        input_vault: accounts.input_vault.to_bytes().to_vec(),
                        output_vault: accounts.output_vault.to_bytes().to_vec(),
                        observation_state: accounts.observation_state.to_bytes().to_vec(),
                        token_program: accounts.token_program.to_bytes().to_vec(),
                        tick_array: accounts.tick_array.to_bytes().to_vec(),
                    })),
                    amount: event.amount,
                    other_amount_threshold: event.other_amount_threshold,
                    sqrt_price_limit_x64: event.sqrt_price_limit_x64.to_string(),
                    is_base_input: event.is_base_input,
                })),
            })
        }
        Ok(raydium::clmm::v3::instructions::RaydiumClmmInstruction::SwapV2(event)) => {
            let accounts = raydium::clmm::v3::accounts::get_swap_v2_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::swap_instruction::Accounts::V2Accounts(pb::SwapV2Accounts {
                        payer: accounts.payer.to_bytes().to_vec(),
                        amm_config: accounts.amm_config.to_bytes().to_vec(),
                        pool_state: accounts.pool_state.to_bytes().to_vec(),
                        input_token_account: accounts.input_token_account.to_bytes().to_vec(),
                        output_token_account: accounts.output_token_account.to_bytes().to_vec(),
                        input_vault: accounts.input_vault.to_bytes().to_vec(),
                        output_vault: accounts.output_vault.to_bytes().to_vec(),
                        observation_state: accounts.observation_state.to_bytes().to_vec(),
                        token_program: accounts.token_program.to_bytes().to_vec(),
                        token_program_2022: accounts.token_program_2022.to_bytes().to_vec(),
                        memo_program: accounts.memo_program.to_bytes().to_vec(),
                        input_vault_mint: accounts.input_vault_mint.to_bytes().to_vec(),
                        output_vault_mint: accounts.output_vault_mint.to_bytes().to_vec(),
                    })),
                    amount: event.amount,
                    other_amount_threshold: event.other_amount_threshold,
                    sqrt_price_limit_x64: event.sqrt_price_limit_x64.to_string(),
                    is_base_input: event.is_base_input,
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
            // substreams::log::debug!("Log message: {}", log_message);
            if let Some(invoke_depth) = parse_invoke_depth(log_message) {
                is_invoked = true;
                if let Some(log_data) = parse_log_data(log_message, program_id_bytes, invoke_depth) {
                    logs.push(log_data);
                }
            }
        } else if is_invoked {
            // substreams::log::debug!("Invoked, Log message: {}", log_message);
            if let Some(log_data) = parse_log_data(log_message, program_id_bytes, 0) {
                logs.push(log_data);
            }
        }
    }

    logs
}

fn parse_log_data(log_message: &str, program_id_bytes: &[u8], invoke_depth: u32) -> Option<pb::Log> {
    let data = parse_program_data(log_message)?;

    match raydium::clmm::v3::events::unpack(data.as_slice()) {
        Ok(raydium::clmm::v3::events::RaydiumClmmEvent::SwapEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Swap(pb::SwapLog {
                pool_state: event.pool_state.to_bytes().to_vec(),
                sender: event.sender.to_bytes().to_vec(),
                token_account_0: event.token_account_0.to_bytes().to_vec(),
                token_account_1: event.token_account_1.to_bytes().to_vec(),
                amount_0: event.amount_0,
                transfer_fee_0: event.transfer_fee_0,
                amount_1: event.amount_1,
                transfer_fee_1: event.transfer_fee_1,
                zero_for_one: event.zero_for_one,
                sqrt_price_x64: event.sqrt_price_x64.to_string(),
                liquidity: event.liquidity.to_string(),
                tick: event.tick,
            })),
        }),
        _ => None,
    }
}
