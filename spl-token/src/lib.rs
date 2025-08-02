mod accounts;
mod balances;
mod extensions;
mod metadata;
mod mints;
mod permissions;
mod transfers;
use common::solana::{get_fee_payer, get_signers, is_spl_token_program};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::errors::Error;
use substreams_solana::block_view::InstructionView;
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let mut transaction = pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
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
    let program_id = instruction.program_id().0;

    // Skip non-SPL-Token instructions
    if !is_spl_token_program(&base58::encode(program_id)) {
        return None;
    }

    // Try each instruction parser in sequence
    let parsed_instruction = transfers::unpack_transfers(instruction)
        .or_else(|| permissions::unpack_permissions(instruction))
        .or_else(|| mints::unpack_mints(instruction))
        .or_else(|| accounts::unpack_permissions(instruction))
        .or_else(|| extensions::unpack_extensions(instruction))
        .or_else(|| metadata::unpack_metadata(instruction));

    parsed_instruction.map(|parsed| pb::Instruction {
        program_id: program_id.to_vec(),
        stack_height: instruction.stack_height(),
        is_root: instruction.is_root(),
        instruction: Some(parsed),
    })
}
