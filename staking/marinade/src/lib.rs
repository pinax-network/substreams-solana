use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::marinade::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::marinade;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &marinade::PROGRAM_ID.to_vec());

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

    if program_id != &marinade::PROGRAM_ID {
        return None;
    }

    let instruction = match marinade::instructions::unpack(ix.data()) {
        Ok(marinade::instructions::MarinadeInstruction::Deposit(event)) => {
            pb::instruction::Instruction::Deposit(pb::DepositInstruction {
                lamports: event.lamports,
            })
        }
        Ok(marinade::instructions::MarinadeInstruction::DepositStakeAccount) => {
            pb::instruction::Instruction::DepositStakeAccount(pb::DepositStakeAccountInstruction {})
        }
        Ok(marinade::instructions::MarinadeInstruction::LiquidUnstake(event)) => {
            pb::instruction::Instruction::LiquidUnstake(pb::LiquidUnstakeInstruction {
                msol_amount: event.msol_amount,
            })
        }
        Ok(marinade::instructions::MarinadeInstruction::AddLiquidity(event)) => {
            pb::instruction::Instruction::AddLiquidity(pb::AddLiquidityInstruction {
                lamports: event.lamports,
            })
        }
        Ok(marinade::instructions::MarinadeInstruction::RemoveLiquidity(event)) => {
            pb::instruction::Instruction::RemoveLiquidity(pb::RemoveLiquidityInstruction {
                tokens: event.tokens,
            })
        }
        Ok(marinade::instructions::MarinadeInstruction::OrderUnstake(event)) => {
            pb::instruction::Instruction::OrderUnstake(pb::OrderUnstakeInstruction {
                msol_amount: event.msol_amount,
            })
        }
        Ok(marinade::instructions::MarinadeInstruction::Claim) => {
            pb::instruction::Instruction::Claim(pb::ClaimInstruction {})
        }
        Ok(marinade::instructions::MarinadeInstruction::WithdrawStakeAccount(event)) => {
            pb::instruction::Instruction::WithdrawStakeAccount(pb::WithdrawStakeAccountInstruction {
                msol_amount: event.msol_amount,
                stake_index: event.stake_index,
                validator_index: event.validator_index,
            })
        }
        _ => return None,
    };

    Some(pb::Instruction {
        program_id: program_id.to_vec(),
        stack_height: ix.stack_height(),
        instruction: Some(instruction),
    })
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
    match marinade::events::unpack(data.as_slice()) {
        Ok(marinade::events::MarinadeEvent::Deposit(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Deposit(pb::DepositEvent {
                state: event.state.to_bytes().to_vec(),
                sol_owner: event.sol_owner.to_bytes().to_vec(),
                sol_swapped: event.sol_swapped,
                msol_swapped: event.msol_swapped,
                sol_deposited: event.sol_deposited,
                msol_minted: event.msol_minted,
                total_virtual_staked_lamports: event.total_virtual_staked_lamports,
                msol_supply: event.msol_supply,
            })),
        }),
        Ok(marinade::events::MarinadeEvent::DepositStakeAccount(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::DepositStakeAccount(pb::DepositStakeAccountEvent {
                state: event.state.to_bytes().to_vec(),
                stake: event.stake.to_bytes().to_vec(),
                delegated: event.delegated,
                withdrawer: event.withdrawer.to_bytes().to_vec(),
                validator: event.validator.to_bytes().to_vec(),
                msol_minted: event.msol_minted,
                total_virtual_staked_lamports: event.total_virtual_staked_lamports,
                msol_supply: event.msol_supply,
            })),
        }),
        Ok(marinade::events::MarinadeEvent::LiquidUnstake(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::LiquidUnstake(pb::LiquidUnstakeEvent {
                state: event.state.to_bytes().to_vec(),
                msol_owner: event.msol_owner.to_bytes().to_vec(),
                msol_amount: event.msol_amount,
                msol_fee: event.msol_fee,
                treasury_msol_cut: event.treasury_msol_cut,
                sol_amount: event.sol_amount,
            })),
        }),
        Ok(marinade::events::MarinadeEvent::AddLiquidity(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::AddLiquidity(pb::AddLiquidityEvent {
                state: event.state.to_bytes().to_vec(),
                sol_owner: event.sol_owner.to_bytes().to_vec(),
                sol_added_amount: event.sol_added_amount,
                lp_minted: event.lp_minted,
                total_virtual_staked_lamports: event.total_virtual_staked_lamports,
                msol_supply: event.msol_supply,
            })),
        }),
        Ok(marinade::events::MarinadeEvent::RemoveLiquidity(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::RemoveLiquidity(pb::RemoveLiquidityEvent {
                state: event.state.to_bytes().to_vec(),
                lp_burned: event.lp_burned,
                sol_out_amount: event.sol_out_amount,
                msol_out_amount: event.msol_out_amount,
            })),
        }),
        Ok(marinade::events::MarinadeEvent::WithdrawStakeAccount(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::WithdrawStakeAccount(pb::WithdrawStakeAccountEvent {
                state: event.state.to_bytes().to_vec(),
                stake: event.stake.to_bytes().to_vec(),
                validator: event.validator.to_bytes().to_vec(),
                user_msol_auth: event.user_msol_auth.to_bytes().to_vec(),
                msol_burned: event.msol_burned,
                msol_fees: event.msol_fees,
                beneficiary: event.beneficiary.to_bytes().to_vec(),
                split_lamports: event.split_lamports,
            })),
        }),
        _ => None,
    }
}
