use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::meteora::daam::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::meteora;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx
        .walk_instructions()
        .filter_map(|iv| process_instruction(&iv))
        .collect();
    let logs = process_logs(tx_meta, &meteora::daam::PROGRAM_ID.to_vec());

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
    if program_id != &meteora::daam::PROGRAM_ID {
        return None;
    }

    match meteora::daam::instructions::unpack(ix.data()) {
        Ok(meteora::daam::instructions::MeteoraDammInstruction::AddLiquidity(instr)) => {
            let accounts = meteora::daam::accounts::get_add_liquidity_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::AddLiquidity(pb::AddLiquidityInstruction {
                    accounts: Some(pb::AddLiquidityAccounts {
                        pool: accounts.pool.to_bytes().to_vec(),
                        position: accounts.position.to_bytes().to_vec(),
                        token_a_account: accounts.token_a_account.to_bytes().to_vec(),
                        token_b_account: accounts.token_b_account.to_bytes().to_vec(),
                        token_a_vault: accounts.token_a_vault.to_bytes().to_vec(),
                        token_b_vault: accounts.token_b_vault.to_bytes().to_vec(),
                        token_a_mint: accounts.token_a_mint.to_bytes().to_vec(),
                        token_b_mint: accounts.token_b_mint.to_bytes().to_vec(),
                        position_nft_account: accounts.position_nft_account.to_bytes().to_vec(),
                        owner: accounts.owner.to_bytes().to_vec(),
                        token_a_program: accounts.token_a_program.to_bytes().to_vec(),
                        token_b_program: accounts.token_b_program.to_bytes().to_vec(),
                        event_authority: accounts.event_authority.to_bytes().to_vec(),
                        program: accounts.program.to_bytes().to_vec(),
                    }),
                    params: Some(pb::AddLiquidityParameters {
                        liquidity_delta: instr.params.liquidity_delta.to_string(),
                        token_a_amount_threshold: instr.params.token_a_amount_threshold,
                        token_b_amount_threshold: instr.params.token_b_amount_threshold,
                    }),
                })),
            })
        }
        Ok(meteora::daam::instructions::MeteoraDammInstruction::RemoveLiquidity(instr)) => {
            let accounts = meteora::daam::accounts::get_remove_liquidity_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::RemoveLiquidity(pb::RemoveLiquidityInstruction {
                    accounts: Some(pb::RemoveLiquidityAccounts {
                        pool_authority: accounts.pool_authority.to_bytes().to_vec(),
                        pool: accounts.pool.to_bytes().to_vec(),
                        position: accounts.position.to_bytes().to_vec(),
                        token_a_account: accounts.token_a_account.to_bytes().to_vec(),
                        token_b_account: accounts.token_b_account.to_bytes().to_vec(),
                        token_a_vault: accounts.token_a_vault.to_bytes().to_vec(),
                        token_b_vault: accounts.token_b_vault.to_bytes().to_vec(),
                        token_a_mint: accounts.token_a_mint.to_bytes().to_vec(),
                        token_b_mint: accounts.token_b_mint.to_bytes().to_vec(),
                        position_nft_account: accounts.position_nft_account.to_bytes().to_vec(),
                        owner: accounts.owner.to_bytes().to_vec(),
                        token_a_program: accounts.token_a_program.to_bytes().to_vec(),
                        token_b_program: accounts.token_b_program.to_bytes().to_vec(),
                        event_authority: accounts.event_authority.to_bytes().to_vec(),
                        program: accounts.program.to_bytes().to_vec(),
                    }),
                    params: Some(pb::RemoveLiquidityParameters {
                        liquidity_delta: instr.params.liquidity_delta.to_string(),
                        token_a_amount_threshold: instr.params.token_a_amount_threshold,
                        token_b_amount_threshold: instr.params.token_b_amount_threshold,
                    }),
                })),
            })
        }
        Ok(meteora::daam::instructions::MeteoraDammInstruction::Swap(instr)) => {
            let accounts = meteora::daam::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::SwapAccounts {
                        pool_authority: accounts.pool_authority.to_bytes().to_vec(),
                        pool: accounts.pool.to_bytes().to_vec(),
                        input_token_account: accounts.input_token_account.to_bytes().to_vec(),
                        output_token_account: accounts.output_token_account.to_bytes().to_vec(),
                        token_a_vault: accounts.token_a_vault.to_bytes().to_vec(),
                        token_b_vault: accounts.token_b_vault.to_bytes().to_vec(),
                        token_a_mint: accounts.token_a_mint.to_bytes().to_vec(),
                        token_b_mint: accounts.token_b_mint.to_bytes().to_vec(),
                        payer: accounts.payer.to_bytes().to_vec(),
                        token_a_program: accounts.token_a_program.to_bytes().to_vec(),
                        token_b_program: accounts.token_b_program.to_bytes().to_vec(),
                        referral_token_account: accounts.referral_token_account.map(|p| p.to_bytes().to_vec()),
                        event_authority: accounts.event_authority.to_bytes().to_vec(),
                        program: accounts.program.to_bytes().to_vec(),
                    }),
                    params: Some(pb::SwapParameters {
                        amount_in: instr.params.amount_in,
                        minimum_amount_out: instr.params.minimum_amount_out,
                    }),
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
    match meteora::daam::events::unpack(data.as_slice()) {
        Ok(meteora::daam::events::MeteoraDammEvent::EvtAddLiquidity(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::AddLiquidity(pb::AddLiquidityLog {
                pool: event.pool.to_bytes().to_vec(),
                position: event.position.to_bytes().to_vec(),
                owner: event.owner.to_bytes().to_vec(),
                params: Some(pb::AddLiquidityParameters {
                    liquidity_delta: event.params.liquidity_delta.to_string(),
                    token_a_amount_threshold: event.params.token_a_amount_threshold,
                    token_b_amount_threshold: event.params.token_b_amount_threshold,
                }),
                token_a_amount: event.token_a_amount,
                token_b_amount: event.token_b_amount,
                total_amount_a: event.total_amount_a,
                total_amount_b: event.total_amount_b,
            })),
        }),
        Ok(meteora::daam::events::MeteoraDammEvent::EvtRemoveLiquidity(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::RemoveLiquidity(pb::RemoveLiquidityLog {
                pool: event.pool.to_bytes().to_vec(),
                position: event.position.to_bytes().to_vec(),
                owner: event.owner.to_bytes().to_vec(),
                params: Some(pb::RemoveLiquidityParameters {
                    liquidity_delta: event.params.liquidity_delta.to_string(),
                    token_a_amount_threshold: event.params.token_a_amount_threshold,
                    token_b_amount_threshold: event.params.token_b_amount_threshold,
                }),
                token_a_amount: event.token_a_amount,
                token_b_amount: event.token_b_amount,
            })),
        }),
        Ok(meteora::daam::events::MeteoraDammEvent::EvtSwap(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Swap(pb::SwapLog {
                pool: event.pool.to_bytes().to_vec(),
                trade_direction: event.trade_direction as u32,
                has_referral: event.has_referral,
                params: Some(pb::SwapParameters {
                    amount_in: event.params.amount_in,
                    minimum_amount_out: event.params.minimum_amount_out,
                }),
                result: Some(pb::SwapResult {
                    output_amount: event.swap_result.output_amount,
                    next_sqrt_price: event.swap_result.next_sqrt_price.to_string(),
                    lp_fee: event.swap_result.lp_fee,
                    protocol_fee: event.swap_result.protocol_fee,
                    partner_fee: event.swap_result.partner_fee,
                    referral_fee: event.swap_result.referral_fee,
                }),
                actual_amount_in: event.actual_amount_in,
                current_timestamp: event.current_timestamp,
            })),
        }),
        _ => None,
    }
}
