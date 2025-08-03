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
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

pub const SOLANA_TOKEN_PROGRAM_KEG: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const SOLANA_TOKEN_PROGRAM_ZQB: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
pub const SOLANA_MEMO_PROGRAM_V1: &str = "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo";
pub const SOLANA_MEMO_PROGRAM_V2: &str = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr";

pub fn is_spl_token_program(program_id: &str) -> bool {
    program_id == SOLANA_TOKEN_PROGRAM_KEG || program_id == SOLANA_TOKEN_PROGRAM_ZQB
}

pub fn is_spl_memo_program(program_id: &str) -> bool {
    program_id == SOLANA_MEMO_PROGRAM_V1 || program_id == SOLANA_MEMO_PROGRAM_V2
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
    let mut transaction = pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers,
        ..Default::default()
    };

    // Process SPL-Token Balances
    transaction.post_token_balances = tx_meta
        .post_token_balances
        .iter()
        .filter_map(|balance| balances::get_token_balance(&tx, balance))
        .collect();

    transaction.pre_token_balances = tx_meta
        .pre_token_balances
        .iter()
        .filter_map(|balance| balances::get_token_balance(&tx, balance))
        .collect();

    transaction.instructions = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();

    if transaction.instructions.is_empty() && transaction.pre_token_balances.is_empty() && transaction.post_token_balances.is_empty() {
        None
    } else {
        Some(transaction)
    }
}

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = base58::encode(instruction.program_id().0);

    // Skip non-SPL-Token & SPL Memo instructions
    if !is_spl_token_program(&program_id) && !is_spl_memo_program(&program_id) {
        return None;
    }

    // Try each instruction parser in sequence
    let parsed_instruction = transfers::unpack_transfers(instruction, &program_id)
        .or_else(|| permissions::unpack_permissions(instruction, &program_id))
        .or_else(|| mints::unpack_mints(instruction, &program_id))
        .or_else(|| accounts::unpack_accounts(instruction, &program_id))
        .or_else(|| metadata::unpack_metadata(instruction, &program_id))
        .or_else(|| memo::unpack_memo(instruction, &program_id));

    parsed_instruction.map(|parsed| pb::Instruction {
        program_id: instruction.program_id().0.to_vec(),
        stack_height: instruction.stack_height(),
        is_root: instruction.is_root(),
        instruction: Some(parsed),
    })
}
