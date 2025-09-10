use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::raydium::cpmm::v1 as pb;
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
    let logs = process_logs(tx_meta, &raydium::cpmm::PROGRAM_ID.to_vec());

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

    if program_id != &raydium::cpmm::PROGRAM_ID {
        return None;
    }

    match raydium::cpmm::instructions::unpack(ix.data()) {
        Ok(raydium::cpmm::instructions::RaydiumCpmmInstruction::SwapBaseInput(event)) => {
            let accounts = raydium::cpmm::accounts::get_swap_base_input_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::SwapBaseInput(pb::SwapBaseInputInstruction {
                    accounts: Some(pb::SwapAccounts {
                        payer: accounts.payer.to_bytes().to_vec(),
                        authority: accounts.authority.to_bytes().to_vec(),
                        amm_config: accounts.amm_config.to_bytes().to_vec(),
                        pool_state: accounts.pool_state.to_bytes().to_vec(),
                        input_token_account: accounts.input_token_account.to_bytes().to_vec(),
                        output_token_account: accounts.output_token_account.to_bytes().to_vec(),
                        input_vault: accounts.input_vault.to_bytes().to_vec(),
                        output_vault: accounts.output_vault.to_bytes().to_vec(),
                        input_token_program: accounts.input_token_program.to_bytes().to_vec(),
                        output_token_program: accounts.output_token_program.to_bytes().to_vec(),
                        input_token_mint: accounts.input_token_mint.to_bytes().to_vec(),
                        output_token_mint: accounts.output_token_mint.to_bytes().to_vec(),
                        observation_state: accounts.observation_state.to_bytes().to_vec(),
                    }),
                    amount_in: event.amount_in,
                    minimum_amount_out: event.minimum_amount_out,
                })),
            })
        }
        Ok(raydium::cpmm::instructions::RaydiumCpmmInstruction::SwapBaseOutput(event)) => {
            let accounts = raydium::cpmm::accounts::get_swap_base_output_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::SwapBaseOutput(pb::SwapBaseOutputInstruction {
                    accounts: Some(pb::SwapAccounts {
                        payer: accounts.payer.to_bytes().to_vec(),
                        authority: accounts.authority.to_bytes().to_vec(),
                        amm_config: accounts.amm_config.to_bytes().to_vec(),
                        pool_state: accounts.pool_state.to_bytes().to_vec(),
                        input_token_account: accounts.input_token_account.to_bytes().to_vec(),
                        output_token_account: accounts.output_token_account.to_bytes().to_vec(),
                        input_vault: accounts.input_vault.to_bytes().to_vec(),
                        output_vault: accounts.output_vault.to_bytes().to_vec(),
                        input_token_program: accounts.input_token_program.to_bytes().to_vec(),
                        output_token_program: accounts.output_token_program.to_bytes().to_vec(),
                        input_token_mint: accounts.input_token_mint.to_bytes().to_vec(),
                        output_token_mint: accounts.output_token_mint.to_bytes().to_vec(),
                        observation_state: accounts.observation_state.to_bytes().to_vec(),
                    }),
                    max_amount_in: event.max_amount_in,
                    amount_out: event.amount_out,
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
    match raydium::cpmm::events::unpack_event(data.as_slice()) {
        Ok(raydium::cpmm::events::RaydiumCpmmEvent::SwapEventV1(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Swap(pb::SwapEvent {
                pool_id: event.pool_id.to_bytes().to_vec(),
                input_vault_before: event.input_vault_before,
                output_vault_before: event.output_vault_before,
                input_amount: event.input_amount,
                output_amount: event.output_amount,
                input_transfer_fee: event.input_transfer_fee,
                output_transfer_fee: event.output_transfer_fee,
                base_input: event.base_input,
                input_mint: None,
                output_mint: None,
                trade_fee: None,
                creator_fee: None,
                creator_fee_on_input: None,
            })),
        }),
        Ok(raydium::cpmm::events::RaydiumCpmmEvent::SwapEventV2(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Swap(pb::SwapEvent {
                pool_id: event.pool_id.to_bytes().to_vec(),
                input_vault_before: event.input_vault_before,
                output_vault_before: event.output_vault_before,
                input_amount: event.input_amount,
                output_amount: event.output_amount,
                input_transfer_fee: event.input_transfer_fee,
                output_transfer_fee: event.output_transfer_fee,
                base_input: event.base_input,
                input_mint: Some(event.input_mint.to_bytes().to_vec()),
                output_mint: Some(event.output_mint.to_bytes().to_vec()),
                trade_fee: Some(event.trade_fee),
                creator_fee: Some(event.creator_fee),
                creator_fee_on_input: Some(event.creator_fee_on_input),
            })),
        }),
        Ok(raydium::cpmm::events::RaydiumCpmmEvent::LpChangeEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::LpChange(pb::LpChangeEvent {
                pool_id: event.pool_id.to_bytes().to_vec(),
                lp_amount_before: event.lp_amount_before,
                token_0_vault_before: event.token_0_vault_before,
                token_1_vault_before: event.token_1_vault_before,
                token_0_amount: event.token_0_amount,
                token_1_amount: event.token_1_amount,
                token_0_transfer_fee: event.token_0_transfer_fee,
                token_1_transfer_fee: event.token_1_transfer_fee,
                change_type: event.change_type as u32,
            })),
        }),
        _ => None,
    }
}
