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
use substreams_solana::{base58, pb::sf::solana::r#type::v1::Block};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    let mut events = pb::Events::default();

    // transactions
    for tx in block.transactions() {
        let mut transaction = pb::Transaction::default();
        let tx_meta = tx.meta.as_ref().expect("Transaction meta should be present");
        transaction.fee = tx_meta.fee;
        transaction.compute_units_consumed = tx_meta.compute_units_consumed();
        transaction.signature = tx.hash().to_vec();

        if let Some(fee_payer) = get_fee_payer(tx) {
            transaction.fee_payer = fee_payer;
        }
        if let Some(signers) = get_signers(tx) {
            transaction.signers = signers;
        }

        // SPL-Token Balances
        match &tx.meta {
            Some(meta) => {
                // PostTokenBalances
                for balance in meta.post_token_balances.iter() {
                    if let Some(token_balance) = balances::get_token_balance(tx, balance) {
                        transaction.post_token_balances.push(token_balance);
                    }
                }
                // PreTokenBalances
                for balance in meta.pre_token_balances.iter() {
                    if let Some(token_balance) = balances::get_token_balance(tx, balance) {
                        transaction.pre_token_balances.push(token_balance);
                    }
                }
            }
            None => {}
        }

        // SPL-Token Instructions
        for instruction in tx.walk_instructions() {
            let program_id = instruction.program_id().0;

            // Skip instructions
            if !is_spl_token_program(&base58::encode(program_id)) {
                continue;
            }

            let mut base = pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: instruction.stack_height(),
                is_root: instruction.is_root(),
                instruction: None,
            };

            // Unpack transfers
            if let Some(instruction) = transfers::unpack_transfers(&instruction) {
                base.instruction = Some(instruction);
                transaction.instructions.push(base.clone());
            }
            // Unpack permissions
            else if let Some(instruction) = permissions::unpack_permissions(&instruction) {
                base.instruction = Some(instruction);
                transaction.instructions.push(base.clone());
            }
            // Unpack mints
            else if let Some(instruction) = mints::unpack_mints(&instruction) {
                base.instruction = Some(instruction);
                transaction.instructions.push(base.clone());
            }
            // Unpack accounts
            else if let Some(instruction) = accounts::unpack_permissions(&instruction) {
                base.instruction = Some(instruction);
                transaction.instructions.push(base.clone());
            }
            // Unpack extensions
            else if let Some(instruction) = extensions::unpack_extensions(&instruction) {
                base.instruction = Some(instruction);
                transaction.instructions.push(base.clone());
            }
            // Unpack metadata
            else if let Some(instruction) = metadata::unpack_metadata(&instruction) {
                base.instruction = Some(instruction);
                transaction.instructions.push(base.clone());
            }
        }
        if !transaction.instructions.is_empty() || !transaction.pre_token_balances.is_empty() || !transaction.post_token_balances.is_empty() {
            events.transactions.push(transaction);
        }
    }
    Ok(events)
}
