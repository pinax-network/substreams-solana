mod accounts;
mod balances;
mod memo;
mod metadata;
mod mints;
mod permissions;
mod transfers;
use common::solana::{get_fee_payer, get_signers};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::errors::Error;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Token Program KEG (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA)
pub const SOLANA_TOKEN_PROGRAM_KEG: [u8; 32] = [
    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
];

// Token Program ZQB (TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb)
pub const SOLANA_TOKEN_PROGRAM_ZQB: [u8; 32] = [
    6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218, 182, 26, 252, 77, 131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
];

// Memo Program V1 (Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo)
pub const SOLANA_MEMO_PROGRAM_V1: [u8; 32] = [
    5, 74, 83, 80, 248, 93, 200, 130, 214, 20, 165, 86, 114, 120, 138, 41, 109, 223, 30, 171, 171, 208, 166, 6, 120, 136, 73, 50, 244, 238, 246, 160,
];

// Memo Program V2 (MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr)
pub const SOLANA_MEMO_PROGRAM_V2: [u8; 32] = [
    5, 74, 83, 90, 153, 41, 33, 6, 77, 36, 232, 113, 96, 218, 56, 124, 124, 53, 181, 221, 188, 146, 187, 129, 228, 31, 168, 64, 65, 5, 68, 141,
];

pub fn is_spl_token_program(program_id: &[u8]) -> bool {
    program_id == &SOLANA_TOKEN_PROGRAM_KEG || program_id == &SOLANA_TOKEN_PROGRAM_ZQB
}

pub fn is_spl_memo_program(program_id: &[u8]) -> bool {
    program_id == &SOLANA_MEMO_PROGRAM_V1 || program_id == &SOLANA_MEMO_PROGRAM_V2
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let signers = get_signers(&tx).unwrap_or_default();

    // Process SPL-Token Balances
    let post_token_balances: Vec<_> = tx_meta
        .post_token_balances
        .iter()
        .filter_map(|balance| balances::get_token_balance(&tx, balance))
        .collect();

    let pre_token_balances: Vec<_> = tx_meta
        .pre_token_balances
        .iter()
        .filter_map(|balance| balances::get_token_balance(&tx, balance))
        .collect();

    let instructions: Vec<_> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();

    if instructions.is_empty() && pre_token_balances.is_empty() && post_token_balances.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers,
        instructions,
        post_token_balances,
        pre_token_balances,
    })
}

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;

    // Skip non-SPL-Token & SPL Memo instructions
    if !is_spl_token_program(program_id) && !is_spl_memo_program(program_id) {
        return None;
    }

    // Try each instruction parser in sequence ordered by frequency
    let parsed_instruction = transfers::unpack_transfers(instruction, program_id)
        .or_else(|| accounts::unpack_accounts(instruction, program_id))
        .or_else(|| permissions::unpack_permissions(instruction, program_id))
        .or_else(|| mints::unpack_mints(instruction, program_id))
        .or_else(|| metadata::unpack_metadata(instruction, program_id))
        .or_else(|| memo::unpack_memo(instruction, program_id));

    parsed_instruction.map(|parsed| pb::Instruction {
        program_id: program_id.to_vec(),
        stack_height: instruction.stack_height(),
        is_root: instruction.is_root(),
        instruction: Some(parsed),
    })
}
