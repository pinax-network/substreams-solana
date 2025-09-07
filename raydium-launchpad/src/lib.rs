use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_id, parse_raydium_log};
use proto::pb::raydium::launchpad::v1 as pb;
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

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iv| process_instruction(&iv)).collect();
    let logs = process_logs(tx_meta, &raydium::launchpad::PROGRAM_ID.to_vec());

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
    if program_id != &raydium::launchpad::PROGRAM_ID {
        return None;
    }

    match raydium::launchpad::instructions::unpack(ix.data()) {
        Ok(raydium::launchpad::instructions::RaydiumLaunchpadInstruction::BuyExactIn(evt)) => {
            let accounts = raydium::launchpad::accounts::get_buy_exact_in_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::BuyExactIn(pb::BuyExactInInstruction {
                    accounts: Some(pb::TradeAccounts {
                        payer: accounts.payer.to_bytes().to_vec(),
                        authority: accounts.authority.to_bytes().to_vec(),
                        global_config: accounts.global_config.to_bytes().to_vec(),
                        platform_config: accounts.platform_config.to_bytes().to_vec(),
                        pool_state: accounts.pool_state.to_bytes().to_vec(),
                        user_base_token: accounts.user_base_token.to_bytes().to_vec(),
                        user_quote_token: accounts.user_quote_token.to_bytes().to_vec(),
                        base_vault: accounts.base_vault.to_bytes().to_vec(),
                        quote_vault: accounts.quote_vault.to_bytes().to_vec(),
                        base_token_mint: accounts.base_token_mint.to_bytes().to_vec(),
                        quote_token_mint: accounts.quote_token_mint.to_bytes().to_vec(),
                        base_token_program: accounts.base_token_program.to_bytes().to_vec(),
                        quote_token_program: accounts.quote_token_program.to_bytes().to_vec(),
                        event_authority: accounts.event_authority.to_bytes().to_vec(),
                        program: accounts.program.to_bytes().to_vec(),
                    }),
                    amount_in: evt.amount_in,
                    minimum_amount_out: evt.minimum_amount_out,
                    share_fee_rate: evt.share_fee_rate,
                })),
            })
        }
        Ok(raydium::launchpad::instructions::RaydiumLaunchpadInstruction::BuyExactOut(evt)) => {
            let accounts = raydium::launchpad::accounts::get_buy_exact_out_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::BuyExactOut(pb::BuyExactOutInstruction {
                    accounts: Some(pb::TradeAccounts {
                        payer: accounts.payer.to_bytes().to_vec(),
                        authority: accounts.authority.to_bytes().to_vec(),
                        global_config: accounts.global_config.to_bytes().to_vec(),
                        platform_config: accounts.platform_config.to_bytes().to_vec(),
                        pool_state: accounts.pool_state.to_bytes().to_vec(),
                        user_base_token: accounts.user_base_token.to_bytes().to_vec(),
                        user_quote_token: accounts.user_quote_token.to_bytes().to_vec(),
                        base_vault: accounts.base_vault.to_bytes().to_vec(),
                        quote_vault: accounts.quote_vault.to_bytes().to_vec(),
                        base_token_mint: accounts.base_token_mint.to_bytes().to_vec(),
                        quote_token_mint: accounts.quote_token_mint.to_bytes().to_vec(),
                        base_token_program: accounts.base_token_program.to_bytes().to_vec(),
                        quote_token_program: accounts.quote_token_program.to_bytes().to_vec(),
                        event_authority: accounts.event_authority.to_bytes().to_vec(),
                        program: accounts.program.to_bytes().to_vec(),
                    }),
                    amount_out: evt.amount_out,
                    maximum_amount_in: evt.maximum_amount_in,
                    share_fee_rate: evt.share_fee_rate,
                })),
            })
        }
        Ok(raydium::launchpad::instructions::RaydiumLaunchpadInstruction::SellExactIn(evt)) => {
            let accounts = raydium::launchpad::accounts::get_sell_exact_in_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::SellExactIn(pb::SellExactInInstruction {
                    accounts: Some(pb::TradeAccounts {
                        payer: accounts.payer.to_bytes().to_vec(),
                        authority: accounts.authority.to_bytes().to_vec(),
                        global_config: accounts.global_config.to_bytes().to_vec(),
                        platform_config: accounts.platform_config.to_bytes().to_vec(),
                        pool_state: accounts.pool_state.to_bytes().to_vec(),
                        user_base_token: accounts.user_base_token.to_bytes().to_vec(),
                        user_quote_token: accounts.user_quote_token.to_bytes().to_vec(),
                        base_vault: accounts.base_vault.to_bytes().to_vec(),
                        quote_vault: accounts.quote_vault.to_bytes().to_vec(),
                        base_token_mint: accounts.base_token_mint.to_bytes().to_vec(),
                        quote_token_mint: accounts.quote_token_mint.to_bytes().to_vec(),
                        base_token_program: accounts.base_token_program.to_bytes().to_vec(),
                        quote_token_program: accounts.quote_token_program.to_bytes().to_vec(),
                        event_authority: accounts.event_authority.to_bytes().to_vec(),
                        program: accounts.program.to_bytes().to_vec(),
                    }),
                    amount_in: evt.amount_in,
                    minimum_amount_out: evt.minimum_amount_out,
                    share_fee_rate: evt.share_fee_rate,
                })),
            })
        }
        Ok(raydium::launchpad::instructions::RaydiumLaunchpadInstruction::SellExactOut(evt)) => {
            let accounts = raydium::launchpad::accounts::get_sell_exact_out_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::SellExactOut(pb::SellExactOutInstruction {
                    accounts: Some(pb::TradeAccounts {
                        payer: accounts.payer.to_bytes().to_vec(),
                        authority: accounts.authority.to_bytes().to_vec(),
                        global_config: accounts.global_config.to_bytes().to_vec(),
                        platform_config: accounts.platform_config.to_bytes().to_vec(),
                        pool_state: accounts.pool_state.to_bytes().to_vec(),
                        user_base_token: accounts.user_base_token.to_bytes().to_vec(),
                        user_quote_token: accounts.user_quote_token.to_bytes().to_vec(),
                        base_vault: accounts.base_vault.to_bytes().to_vec(),
                        quote_vault: accounts.quote_vault.to_bytes().to_vec(),
                        base_token_mint: accounts.base_token_mint.to_bytes().to_vec(),
                        quote_token_mint: accounts.quote_token_mint.to_bytes().to_vec(),
                        base_token_program: accounts.base_token_program.to_bytes().to_vec(),
                        quote_token_program: accounts.quote_token_program.to_bytes().to_vec(),
                        event_authority: accounts.event_authority.to_bytes().to_vec(),
                        program: accounts.program.to_bytes().to_vec(),
                    }),
                    amount_out: evt.amount_out,
                    maximum_amount_in: evt.maximum_amount_in,
                    share_fee_rate: evt.share_fee_rate,
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
    let data = parse_raydium_log(log_message)?;
    match raydium::launchpad::events::unpack(data.as_slice()) {
        Ok(raydium::launchpad::events::RaydiumLaunchpadEvent::TradeEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Trade(pb::TradeLog {
                pool_state: event.pool_state.to_bytes().to_vec(),
                total_base_sell: event.total_base_sell,
                virtual_base: event.virtual_base,
                virtual_quote: event.virtual_quote,
                real_base_before: event.real_base_before,
                real_quote_before: event.real_quote_before,
                real_base_after: event.real_base_after,
                real_quote_after: event.real_quote_after,
                amount_in: event.amount_in,
                amount_out: event.amount_out,
                protocol_fee: event.protocol_fee,
                platform_fee: event.platform_fee,
                creator_fee: event.creator_fee,
                share_fee: event.share_fee,
                trade_direction: match event.trade_direction {
                    raydium::launchpad::events::TradeDirection::Buy => pb::TradeDirection::Buy as i32,
                    raydium::launchpad::events::TradeDirection::Sell => pb::TradeDirection::Sell as i32,
                },
                pool_status: match event.pool_status {
                    raydium::launchpad::events::PoolStatus::Fund => pb::PoolStatus::Fund as i32,
                    raydium::launchpad::events::PoolStatus::Migrate => pb::PoolStatus::Migrate as i32,
                    raydium::launchpad::events::PoolStatus::Trade => pb::PoolStatus::Trade as i32,
                },
                exact_in: event.exact_in,
            })),
        }),
        Ok(raydium::launchpad::events::RaydiumLaunchpadEvent::ClaimVestedEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::ClaimVested(pb::ClaimVestedLog {
                pool_state: event.pool_state.to_bytes().to_vec(),
                beneficiary: event.beneficiary.to_bytes().to_vec(),
                claim_amount: event.claim_amount,
            })),
        }),
        Ok(raydium::launchpad::events::RaydiumLaunchpadEvent::CreateVestingEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::CreateVesting(pb::CreateVestingLog {
                pool_state: event.pool_state.to_bytes().to_vec(),
                beneficiary: event.beneficiary.to_bytes().to_vec(),
                share_amount: event.share_amount,
            })),
        }),
        Ok(raydium::launchpad::events::RaydiumLaunchpadEvent::PoolCreateEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::PoolCreate(pb::PoolCreateLog {
                pool_state: event.pool_state.to_bytes().to_vec(),
                creator: event.creator.to_bytes().to_vec(),
                config: event.config.to_bytes().to_vec(),
                base_mint_param: Some(pb::MintParams {
                    decimals: event.base_mint_param.decimals as u32,
                    name: event.base_mint_param.name,
                    symbol: event.base_mint_param.symbol,
                    uri: event.base_mint_param.uri,
                }),
                curve_param: Some(match event.curve_param {
                    raydium::launchpad::events::CurveParams::Constant { data } => pb::CurveParams {
                        curve: Some(pb::curve_params::Curve::Constant(pb::ConstantCurve {
                            supply: data.supply,
                            total_base_sell: data.total_base_sell,
                            total_quote_fund_raising: data.total_quote_fund_raising,
                            migrate_type: data.migrate_type as u32,
                        })),
                    },
                    raydium::launchpad::events::CurveParams::Fixed { data } => pb::CurveParams {
                        curve: Some(pb::curve_params::Curve::Fixed(pb::FixedCurve {
                            supply: data.supply,
                            total_quote_fund_raising: data.total_quote_fund_raising,
                            migrate_type: data.migrate_type as u32,
                        })),
                    },
                    raydium::launchpad::events::CurveParams::Linear { data } => pb::CurveParams {
                        curve: Some(pb::curve_params::Curve::Linear(pb::LinearCurve {
                            supply: data.supply,
                            total_quote_fund_raising: data.total_quote_fund_raising,
                            migrate_type: data.migrate_type as u32,
                        })),
                    },
                }),
                vesting_param: Some(pb::VestingParams {
                    total_locked_amount: event.vesting_param.total_locked_amount,
                    cliff_period: event.vesting_param.cliff_period,
                    unlock_period: event.vesting_param.unlock_period,
                }),
                amm_fee_on: match event.amm_fee_on {
                    raydium::launchpad::events::AmmCreatorFeeOn::QuoteToken => pb::AmmCreatorFeeOn::QuoteToken as i32,
                    raydium::launchpad::events::AmmCreatorFeeOn::BothToken => pb::AmmCreatorFeeOn::BothToken as i32,
                },
            })),
        }),
        _ => None,
    }
}
