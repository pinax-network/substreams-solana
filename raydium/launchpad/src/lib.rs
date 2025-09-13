use common::solana::{get_fee_payer, get_signers};
use proto::pb::raydium::launchpad::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
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

    if instructions.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
    })
}

fn process_instruction(ix: &InstructionView) -> Option<pb::Instruction> {
    let program_id = ix.program_id().0;
    if program_id != &raydium::launchpad::PROGRAM_ID {
        return None;
    }
    if let Ok(event) = raydium::launchpad::events::unpack(ix.data()) {
        return match event {
            raydium::launchpad::events::RaydiumLaunchpadEvent::TradeEventV1(event) => Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TradeEvent(pb::TradeEvent {
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
                    creator_fee: Some(event.creator_fee),
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
                    exact_in: Some(event.exact_in),
                })),
            }),
            raydium::launchpad::events::RaydiumLaunchpadEvent::TradeEventV2(event) => Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TradeEvent(pb::TradeEvent {
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
                    creator_fee: None,
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
                    exact_in: None,
                })),
            }),
            raydium::launchpad::events::RaydiumLaunchpadEvent::ClaimVestedEvent(event) => Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::ClaimVestedEvent(pb::ClaimVestedEvent {
                    pool_state: event.pool_state.to_bytes().to_vec(),
                    beneficiary: event.beneficiary.to_bytes().to_vec(),
                    claim_amount: event.claim_amount,
                })),
            }),
            raydium::launchpad::events::RaydiumLaunchpadEvent::CreateVestingEvent(event) => Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::CreateVestingEvent(pb::CreateVestingEvent {
                    pool_state: event.pool_state.to_bytes().to_vec(),
                    beneficiary: event.beneficiary.to_bytes().to_vec(),
                    share_amount: event.share_amount,
                })),
            }),
            raydium::launchpad::events::RaydiumLaunchpadEvent::PoolCreateEvent(event) => Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::PoolCreateEvent(pb::PoolCreateEvent {
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
            raydium::launchpad::events::RaydiumLaunchpadEvent::Unknown => None,
        };
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
