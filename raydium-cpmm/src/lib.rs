use common::solana::{get_fee_payer, get_signers, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::raydium::cpmm::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};

// CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
const RAYDIUM_CP_SWAP_PROGRAM_ID: [u8; 32] = [
    169, 42, 90, 139, 79, 41, 89, 82, 132, 37, 80, 170, 147, 253, 91, 149, 181, 172, 230, 168, 235, 146, 12, 147, 148, 46, 67, 105, 12, 32, 236, 115,
];

const SWAP_BASE_INPUT_DISCRIMINATOR: [u8; 8] = [143, 190, 90, 218, 196, 30, 51, 222];
const SWAP_BASE_OUTPUT_DISCRIMINATOR: [u8; 8] = [55, 217, 98, 86, 163, 74, 180, 173];
const SWAP_EVENT_DISCRIMINATOR: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
const LP_CHANGE_EVENT_DISCRIMINATOR: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iv| process_instruction(&iv)).collect();

    let logs = process_logs(tx_meta, &RAYDIUM_CP_SWAP_PROGRAM_ID);

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

fn process_instruction(iv: &InstructionView) -> Option<pb::Instruction> {
    let program_id = iv.program_id().0;
    if program_id != &RAYDIUM_CP_SWAP_PROGRAM_ID {
        return None;
    }
    let data = iv.data();

    let instruction = decode_swap_base_input(data, iv).or_else(|| decode_swap_base_output(data, iv))?;

    Some(pb::Instruction {
        program_id: program_id.to_vec(),
        stack_height: iv.stack_height(),
        instruction: Some(instruction),
    })
}

fn decode_swap_base_input(data: &[u8], ix: &InstructionView) -> Option<pb::instruction::Instruction> {
    if data.len() < 24 || data[..8] != SWAP_BASE_INPUT_DISCRIMINATOR {
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

fn decode_swap_base_output(data: &[u8], ix: &InstructionView) -> Option<pb::instruction::Instruction> {
    if data.len() < 24 || data[..8] != SWAP_BASE_OUTPUT_DISCRIMINATOR {
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
            if data.starts_with(&SWAP_EVENT_DISCRIMINATOR) {
                if let Some(event) = decode_swap_event(&data[8..]) {
                    logs.push(pb::Log {
                        program_id: program_id_bytes.to_vec(),
                        invoke_depth,
                        log: Some(pb::log::Log::Swap(event)),
                    });
                }
            } else if data.starts_with(&LP_CHANGE_EVENT_DISCRIMINATOR) {
                if let Some(event) = decode_lp_change_event(&data[8..]) {
                    logs.push(pb::Log {
                        program_id: program_id_bytes.to_vec(),
                        invoke_depth,
                        log: Some(pb::log::Log::LpChange(event)),
                    });
                }
            }
        }
    }
    logs
}

fn decode_swap_event(data: &[u8]) -> Option<pb::SwapEvent> {
    let mut idx = 0;
    fn take<'a>(data: &'a [u8], idx: &mut usize, len: usize) -> Option<&'a [u8]> {
        if *idx + len > data.len() {
            None
        } else {
            let slice = &data[*idx..*idx + len];
            *idx += len;
            Some(slice)
        }
    }

    let pool_id = take(data, &mut idx, 32)?.to_vec();
    let input_vault_before = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let output_vault_before = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let input_amount = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let output_amount = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let input_transfer_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let output_transfer_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let base_input = take(data, &mut idx, 1)?[0] != 0;
    let input_mint = take(data, &mut idx, 32)?.to_vec();
    let output_mint = take(data, &mut idx, 32)?.to_vec();
    let trade_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let creator_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let creator_fee_on_input = take(data, &mut idx, 1)?[0] != 0;

    Some(pb::SwapEvent {
        pool_id,
        input_vault_before,
        output_vault_before,
        input_amount,
        output_amount,
        input_transfer_fee,
        output_transfer_fee,
        base_input,
        input_mint,
        output_mint,
        trade_fee,
        creator_fee,
        creator_fee_on_input,
    })
}

fn decode_lp_change_event(data: &[u8]) -> Option<pb::LpChangeEvent> {
    let mut idx = 0;
    fn take<'a>(data: &'a [u8], idx: &mut usize, len: usize) -> Option<&'a [u8]> {
        if *idx + len > data.len() {
            None
        } else {
            let slice = &data[*idx..*idx + len];
            *idx += len;
            Some(slice)
        }
    }

    let pool_id = take(data, &mut idx, 32)?.to_vec();
    let lp_amount_before = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let token_0_vault_before = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let token_1_vault_before = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let token_0_amount = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let token_1_amount = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let token_0_transfer_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let token_1_transfer_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let change_type = take(data, &mut idx, 1)?[0] as u32;

    Some(pb::LpChangeEvent {
        pool_id,
        lp_amount_before,
        token_0_vault_before,
        token_1_vault_before,
        token_0_amount,
        token_1_amount,
        token_0_transfer_fee,
        token_1_transfer_fee,
        change_type,
    })
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
