use anchor_lang::prelude::borsh::BorshDeserialize;
use common::solana::{get_fee_payer, get_signers, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::raydium::cpmm::v1 as pb;
use raydium_cp_swap::states::events as cp_events;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();

    let logs = process_logs(tx_meta, &raydium_cp_swap::ID.to_bytes());

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

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;

    // CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
    if program_id != &raydium_cp_swap::ID.to_bytes() {
        return None;
    }

    let data = instruction.data();
    let parsed = parse_swap_base_input(data, instruction).or_else(|| parse_swap_base_output(data, instruction))?;

    Some(pb::Instruction {
        program_id: program_id.to_vec(),
        stack_height: instruction.stack_height(),
        instruction: Some(parsed),
    })
}

fn parse_swap_base_input(data: &[u8], ix: &InstructionView) -> Option<pb::instruction::Instruction> {
    let disc = compute_discriminator("swap_base_input");
    if data.len() < 24 || &data[..8] != disc {
        return None;
    }
    let amount_in = u64::from_le_bytes(data[8..16].try_into().ok()?);
    let minimum_amount_out = u64::from_le_bytes(data[16..24].try_into().ok()?);
    Some(pb::instruction::Instruction::SwapBaseInput(pb::SwapBaseInputInstruction {
        accounts: Some(get_swap_accounts(ix)),
        amount_in,
        minimum_amount_out,
    }))
}

fn parse_swap_base_output(data: &[u8], ix: &InstructionView) -> Option<pb::instruction::Instruction> {
    let disc = compute_discriminator("swap_base_output");
    if data.len() < 24 || &data[..8] != disc {
        return None;
    }
    let max_amount_in = u64::from_le_bytes(data[8..16].try_into().ok()?);
    let amount_out = u64::from_le_bytes(data[16..24].try_into().ok()?);
    Some(pb::instruction::Instruction::SwapBaseOutput(pb::SwapBaseOutputInstruction {
        accounts: Some(get_swap_accounts(ix)),
        max_amount_in,
        amount_out,
    }))
}

fn process_logs(tx_meta: &TransactionStatusMeta, program_id_bytes: &[u8]) -> Vec<pb::Log> {
    let mut logs = Vec::new();
    let mut is_invoked = false;
    let mut invoke_depth = 0;

    for log_message in tx_meta.log_messages.iter() {
        if is_invoke(log_message) {
            if let Some(id) = parse_program_id(log_message) {
                if id == program_id_bytes {
                    is_invoked = true;
                    invoke_depth = parse_invoke_depth(log_message).unwrap_or(0);
                }
            }
            continue;
        }
        if is_success(log_message) {
            if let Some(id) = parse_program_id(log_message) {
                if id == program_id_bytes {
                    is_invoked = false;
                }
            }
            continue;
        }
        if !is_invoked {
            continue;
        }
        if let Some(data) = parse_program_data(log_message) {
            if data.starts_with(&event_discriminator("SwapEvent")) {
                if let Ok(event) = cp_events::SwapEvent::try_from_slice(&data[8..]) {
                    logs.push(pb::Log {
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
                            input_mint: event.input_mint.to_bytes().to_vec(),
                            output_mint: event.output_mint.to_bytes().to_vec(),
                            trade_fee: event.trade_fee,
                            creator_fee: event.creator_fee,
                            creator_fee_on_input: event.creator_fee_on_input,
                        })),
                    });
                }
            } else if data.starts_with(&event_discriminator("LpChangeEvent")) {
                if let Ok(event) = cp_events::LpChangeEvent::try_from_slice(&data[8..]) {
                    logs.push(pb::Log {
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
                    });
                }
            }
        }
    }
    logs
}

fn compute_discriminator(name: &str) -> [u8; 8] {
    use anchor_lang::solana_program::hash::hashv;
    let hash = hashv(&[b"global", name.as_bytes()]);
    let mut disc = [0u8; 8];
    disc.copy_from_slice(&hash.to_bytes()[..8]);
    disc
}

fn event_discriminator(name: &str) -> [u8; 8] {
    use anchor_lang::solana_program::hash::hashv;
    let hash = hashv(&[b"event", name.as_bytes()]);
    let mut disc = [0u8; 8];
    disc.copy_from_slice(&hash.to_bytes()[..8]);
    disc
}

fn get_swap_accounts(ix: &InstructionView) -> pb::SwapAccounts {
    pb::SwapAccounts {
        payer: ix.accounts()[0].0.to_vec(),
        authority: ix.accounts()[1].0.to_vec(),
        amm_config: ix.accounts()[2].0.to_vec(),
        pool_state: ix.accounts()[3].0.to_vec(),
        input_token_account: ix.accounts()[4].0.to_vec(),
        output_token_account: ix.accounts()[5].0.to_vec(),
        input_vault: ix.accounts()[6].0.to_vec(),
        output_vault: ix.accounts()[7].0.to_vec(),
        input_token_program: ix.accounts()[8].0.to_vec(),
        output_token_program: ix.accounts()[9].0.to_vec(),
        input_token_mint: ix.accounts()[10].0.to_vec(),
        output_token_mint: ix.accounts()[11].0.to_vec(),
        observation_state: ix.accounts()[12].0.to_vec(),
    }
}
