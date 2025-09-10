use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::meteora::amm::v1 as pb;
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

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iv| process_instruction(&iv)).collect();
    let logs = process_logs(tx_meta, &meteora::amm::PROGRAM_ID.to_vec());

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
    if program_id != &meteora::amm::PROGRAM_ID {
        return None;
    }

    match meteora::amm::instructions::unpack(ix.data()) {
        Ok(meteora::amm::instructions::AmmInstruction::Swap(evt)) => {
            let accounts = meteora::amm::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::SwapAccounts {
                        pool: accounts.pool.to_bytes().to_vec(),
                        user_source_token: accounts.user_source_token.to_bytes().to_vec(),
                        user_destination_token: accounts.user_destination_token.to_bytes().to_vec(),
                        a_vault: accounts.a_vault.to_bytes().to_vec(),
                        b_vault: accounts.b_vault.to_bytes().to_vec(),
                        a_token_vault: accounts.a_token_vault.to_bytes().to_vec(),
                        b_token_vault: accounts.b_token_vault.to_bytes().to_vec(),
                        a_vault_lp_mint: accounts.a_vault_lp_mint.to_bytes().to_vec(),
                        b_vault_lp_mint: accounts.b_vault_lp_mint.to_bytes().to_vec(),
                        a_vault_lp: accounts.a_vault_lp.to_bytes().to_vec(),
                        b_vault_lp: accounts.b_vault_lp.to_bytes().to_vec(),
                        protocol_token_fee: accounts.protocol_token_fee.to_bytes().to_vec(),
                        user: accounts.user.to_bytes().to_vec(),
                        vault_program: accounts.vault_program.to_bytes().to_vec(),
                        token_program: accounts.token_program.to_bytes().to_vec(),
                    }),
                    in_amount: evt.in_amount,
                    minimum_out_amount: evt.minimum_out_amount,
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
    match meteora::amm::events::parse_event(data.as_slice()) {
        Ok(meteora::amm::events::AmmEvent::AddLiquidity(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::AddLiquidity(pb::AddLiquidityLog {
                lp_mint_amount: evt.lp_mint_amount,
                token_a_amount: evt.token_a_amount,
                token_b_amount: evt.token_b_amount,
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::RemoveLiquidity(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::RemoveLiquidity(pb::RemoveLiquidityLog {
                lp_unmint_amount: evt.lp_unmint_amount,
                token_a_out_amount: evt.token_a_out_amount,
                token_b_out_amount: evt.token_b_out_amount,
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::BootstrapLiquidity(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::BootstrapLiquidity(pb::BootstrapLiquidityLog {
                lp_mint_amount: evt.lp_mint_amount,
                token_a_amount: evt.token_a_amount,
                token_b_amount: evt.token_b_amount,
                pool: evt.pool.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::Swap(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Swap(pb::SwapLog {
                in_amount: evt.in_amount,
                out_amount: evt.out_amount,
                trade_fee: evt.trade_fee,
                protocol_fee: evt.protocol_fee,
                host_fee: evt.host_fee,
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::SetPoolFees(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::SetPoolFees(pb::SetPoolFeesLog {
                trade_fee_numerator: evt.trade_fee_numerator,
                trade_fee_denominator: evt.trade_fee_denominator,
                protocol_trade_fee_numerator: evt.protocol_trade_fee_numerator,
                protocol_trade_fee_denominator: evt.protocol_trade_fee_denominator,
                pool: evt.pool.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::PoolInfo(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::PoolInfo(pb::PoolInfoLog {
                token_a_amount: evt.token_a_amount,
                token_b_amount: evt.token_b_amount,
                virtual_price: evt.virtual_price,
                current_timestamp: evt.current_timestamp,
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::TransferAdmin(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::TransferAdmin(pb::TransferAdminLog {
                admin: evt.admin.to_bytes().to_vec(),
                new_admin: evt.new_admin.to_bytes().to_vec(),
                pool: evt.pool.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::OverrideCurveParam(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::OverrideCurveParam(pb::OverrideCurveParamLog {
                new_amp: evt.new_amp,
                updated_timestamp: evt.updated_timestamp,
                pool: evt.pool.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::PoolCreated(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::PoolCreated(pb::PoolCreatedLog {
                lp_mint: evt.lp_mint.to_bytes().to_vec(),
                token_a_mint: evt.token_a_mint.to_bytes().to_vec(),
                token_b_mint: evt.token_b_mint.to_bytes().to_vec(),
                pool_type: evt.pool_type as u32,
                pool: evt.pool.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::PoolEnabled(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::PoolEnabled(pb::PoolEnabledLog {
                pool: evt.pool.to_bytes().to_vec(),
                enabled: evt.enabled,
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::MigrateFeeAccount(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::MigrateFeeAccount(pb::MigrateFeeAccountLog {
                pool: evt.pool.to_bytes().to_vec(),
                new_admin_token_a_fee: evt.new_admin_token_a_fee.to_bytes().to_vec(),
                new_admin_token_b_fee: evt.new_admin_token_b_fee.to_bytes().to_vec(),
                token_a_amount: evt.token_a_amount,
                token_b_amount: evt.token_b_amount,
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::CreateLockEscrow(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::CreateLockEscrow(pb::CreateLockEscrowLog {
                pool: evt.pool.to_bytes().to_vec(),
                owner: evt.owner.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::Lock(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Lock(pb::LockLog {
                pool: evt.pool.to_bytes().to_vec(),
                owner: evt.owner.to_bytes().to_vec(),
                amount: evt.amount,
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::ClaimFee(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::ClaimFee(pb::ClaimFeeLog {
                pool: evt.pool.to_bytes().to_vec(),
                owner: evt.owner.to_bytes().to_vec(),
                amount: evt.amount,
                a_fee: evt.a_fee,
                b_fee: evt.b_fee,
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::CreateConfig(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::CreateConfig(pb::CreateConfigLog {
                trade_fee_numerator: evt.trade_fee_numerator,
                protocol_trade_fee_numerator: evt.protocol_trade_fee_numerator,
                config: evt.config.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::CloseConfig(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::CloseConfig(pb::CloseConfigLog {
                config: evt.config.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::WithdrawProtocolFees(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::WithdrawProtocolFees(pb::WithdrawProtocolFeesLog {
                pool: evt.pool.to_bytes().to_vec(),
                protocol_a_fee: evt.protocol_a_fee,
                protocol_b_fee: evt.protocol_b_fee,
                protocol_a_fee_owner: evt.protocol_a_fee_owner.to_bytes().to_vec(),
                protocol_b_fee_owner: evt.protocol_b_fee_owner.to_bytes().to_vec(),
            })),
        }),
        Ok(meteora::amm::events::AmmEvent::PartnerClaimFees(evt)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::PartnerClaimFees(pb::PartnerClaimFeesLog {
                pool: evt.pool.to_bytes().to_vec(),
                fee_a: evt.fee_a,
                fee_b: evt.fee_b,
                partner: evt.partner.to_bytes().to_vec(),
            })),
        }),
        _ => None,
    }
}
