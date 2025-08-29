use carbon_raydium_launchpad_decoder::{
    instructions::{self, buy_exact_in, buy_exact_out, sell_exact_in, sell_exact_out, RaydiumLaunchpadInstruction},
    types::{pool_status::PoolStatus, trade_direction::TradeDirection},
    RaydiumLaunchpadDecoder,
};
use common::solana::{get_fee_payer, get_signers};
use proto::pb::raydium::launchpad::v1 as pb;
use solana_instruction_v2::{AccountMeta, Instruction};
use solana_pubkey_v2::Pubkey;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

// LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
const RAYDIUM_LAUNCHPAD_PROGRAM_ID: [u8; 32] = Pubkey::new_from_array([
    107, 233, 173, 173, 146, 192, 112, 22, 32, 77, 88, 38, 82, 147, 208, 242, 43, 93, 75, 182, 27, 53, 92, 193, 117, 14, 82, 174, 77, 19, 51, 217,
]);

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iv| process_instruction(&iv)).collect();
    if instructions.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx.meta.as_ref()?.fee,
        compute_units_consumed: tx.meta.as_ref()?.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
    })
}

fn process_instruction(iv: &InstructionView) -> Option<pb::Instruction> {
    let accounts: Vec<AccountMeta> = iv
        .accounts()
        .iter()
        .map(|a| AccountMeta {
            pubkey: Pubkey::new(a.0),
            is_signer: a.1,
            is_writable: a.2,
        })
        .collect();

    let instruction = Instruction {
        program_id: Pubkey::new(iv.program_id().0),
        accounts: accounts.clone(),
        data: iv.data().to_vec(),
    };

    let decoder = RaydiumLaunchpadDecoder;
    let decoded = decoder.decode_instruction(&instruction)?;

    let stack_height = iv.stack_height();

    use RaydiumLaunchpadInstruction as RI;
    let instruction = match decoded.data {
        RI::BuyExactIn(data) => {
            let acc = buy_exact_in::BuyExactIn::arrange_accounts(&accounts)?;
            pb::instruction::Instruction::BuyExactIn(pb::BuyExactInInstruction {
                accounts: Some(pb::TradeAccounts {
                    payer: acc.payer.to_bytes().to_vec(),
                    authority: acc.authority.to_bytes().to_vec(),
                    global_config: acc.global_config.to_bytes().to_vec(),
                    platform_config: acc.platform_config.to_bytes().to_vec(),
                    pool_state: acc.pool_state.to_bytes().to_vec(),
                    user_base_token: acc.user_base_token.to_bytes().to_vec(),
                    user_quote_token: acc.user_quote_token.to_bytes().to_vec(),
                    base_vault: acc.base_vault.to_bytes().to_vec(),
                    quote_vault: acc.quote_vault.to_bytes().to_vec(),
                    base_token_mint: acc.base_token_mint.to_bytes().to_vec(),
                    quote_token_mint: acc.quote_token_mint.to_bytes().to_vec(),
                    base_token_program: acc.base_token_program.to_bytes().to_vec(),
                    quote_token_program: acc.quote_token_program.to_bytes().to_vec(),
                    event_authority: acc.event_authority.to_bytes().to_vec(),
                    program: acc.program.to_bytes().to_vec(),
                }),
                amount_in: data.amount_in,
                minimum_amount_out: data.minimum_amount_out,
                share_fee_rate: data.share_fee_rate,
            })
        }
        RI::BuyExactOut(data) => {
            let acc = buy_exact_out::BuyExactOut::arrange_accounts(&accounts)?;
            pb::instruction::Instruction::BuyExactOut(pb::BuyExactOutInstruction {
                accounts: Some(pb::TradeAccounts {
                    payer: acc.payer.to_bytes().to_vec(),
                    authority: acc.authority.to_bytes().to_vec(),
                    global_config: acc.global_config.to_bytes().to_vec(),
                    platform_config: acc.platform_config.to_bytes().to_vec(),
                    pool_state: acc.pool_state.to_bytes().to_vec(),
                    user_base_token: acc.user_base_token.to_bytes().to_vec(),
                    user_quote_token: acc.user_quote_token.to_bytes().to_vec(),
                    base_vault: acc.base_vault.to_bytes().to_vec(),
                    quote_vault: acc.quote_vault.to_bytes().to_vec(),
                    base_token_mint: acc.base_token_mint.to_bytes().to_vec(),
                    quote_token_mint: acc.quote_token_mint.to_bytes().to_vec(),
                    base_token_program: acc.base_token_program.to_bytes().to_vec(),
                    quote_token_program: acc.quote_token_program.to_bytes().to_vec(),
                    event_authority: acc.event_authority.to_bytes().to_vec(),
                    program: acc.program.to_bytes().to_vec(),
                }),
                amount_out: data.amount_out,
                maximum_amount_in: data.maximum_amount_in,
                share_fee_rate: data.share_fee_rate,
            })
        }
        RI::SellExactIn(data) => {
            let acc = sell_exact_in::SellExactIn::arrange_accounts(&accounts)?;
            pb::instruction::Instruction::SellExactIn(pb::SellExactInInstruction {
                accounts: Some(pb::TradeAccounts {
                    payer: acc.payer.to_bytes().to_vec(),
                    authority: acc.authority.to_bytes().to_vec(),
                    global_config: acc.global_config.to_bytes().to_vec(),
                    platform_config: acc.platform_config.to_bytes().to_vec(),
                    pool_state: acc.pool_state.to_bytes().to_vec(),
                    user_base_token: acc.user_base_token.to_bytes().to_vec(),
                    user_quote_token: acc.user_quote_token.to_bytes().to_vec(),
                    base_vault: acc.base_vault.to_bytes().to_vec(),
                    quote_vault: acc.quote_vault.to_bytes().to_vec(),
                    base_token_mint: acc.base_token_mint.to_bytes().to_vec(),
                    quote_token_mint: acc.quote_token_mint.to_bytes().to_vec(),
                    base_token_program: acc.base_token_program.to_bytes().to_vec(),
                    quote_token_program: acc.quote_token_program.to_bytes().to_vec(),
                    event_authority: acc.event_authority.to_bytes().to_vec(),
                    program: acc.program.to_bytes().to_vec(),
                }),
                amount_in: data.amount_in,
                minimum_amount_out: data.minimum_amount_out,
                share_fee_rate: data.share_fee_rate,
            })
        }
        RI::SellExactOut(data) => {
            let acc = sell_exact_out::SellExactOut::arrange_accounts(&accounts)?;
            pb::instruction::Instruction::SellExactOut(pb::SellExactOutInstruction {
                accounts: Some(pb::TradeAccounts {
                    payer: acc.payer.to_bytes().to_vec(),
                    authority: acc.authority.to_bytes().to_vec(),
                    global_config: acc.global_config.to_bytes().to_vec(),
                    platform_config: acc.platform_config.to_bytes().to_vec(),
                    pool_state: acc.pool_state.to_bytes().to_vec(),
                    user_base_token: acc.user_base_token.to_bytes().to_vec(),
                    user_quote_token: acc.user_quote_token.to_bytes().to_vec(),
                    base_vault: acc.base_vault.to_bytes().to_vec(),
                    quote_vault: acc.quote_vault.to_bytes().to_vec(),
                    base_token_mint: acc.base_token_mint.to_bytes().to_vec(),
                    quote_token_mint: acc.quote_token_mint.to_bytes().to_vec(),
                    base_token_program: acc.base_token_program.to_bytes().to_vec(),
                    quote_token_program: acc.quote_token_program.to_bytes().to_vec(),
                    event_authority: acc.event_authority.to_bytes().to_vec(),
                    program: acc.program.to_bytes().to_vec(),
                }),
                amount_out: data.amount_out,
                maximum_amount_in: data.maximum_amount_in,
                share_fee_rate: data.share_fee_rate,
            })
        }
        RI::TradeEvent(ev) => pb::instruction::Instruction::TradeEvent(pb::TradeEvent {
            pool_state: ev.pool_state.to_bytes().to_vec(),
            total_base_sell: ev.total_base_sell,
            virtual_base: ev.virtual_base,
            virtual_quote: ev.virtual_quote,
            real_base_before: ev.real_base_before,
            real_quote_before: ev.real_quote_before,
            real_base_after: ev.real_base_after,
            real_quote_after: ev.real_quote_after,
            amount_in: ev.amount_in,
            amount_out: ev.amount_out,
            protocol_fee: ev.protocol_fee,
            platform_fee: ev.platform_fee,
            creator_fee: ev.creator_fee,
            share_fee: ev.share_fee,
            trade_direction: match ev.trade_direction {
                TradeDirection::Buy => pb::TradeDirection::Buy as i32,
                TradeDirection::Sell => pb::TradeDirection::Sell as i32,
            },
            pool_status: match ev.pool_status {
                PoolStatus::Fund => pb::PoolStatus::Fund as i32,
                PoolStatus::Migrate => pb::PoolStatus::Migrate as i32,
                PoolStatus::Trade => pb::PoolStatus::Trade as i32,
            },
            exact_in: ev.exact_in,
        }),
        _ => return None,
    };

    Some(pb::Instruction {
        program_id: instruction.program_id.to_bytes().to_vec(),
        stack_height,
        instruction: Some(instruction),
    })
}
