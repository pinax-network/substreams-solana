mod system;
use core::panic;

use common::solana::{get_fee_payer, get_signers, is_system_program};
use proto::pb::solana::native::token::v1 as pb;
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

        // Native Token Balances
        if let Some(meta) = &tx.meta {
            let resolved_accounts = tx.resolved_accounts();

            // Helper function to create balances from account/amount pairs
            let create_balances = |balances: &[u64], tag: &str| -> Vec<pb::Balance> {
                if balances.len() != resolved_accounts.len() {
                    substreams::log::info!(
                        "Skipping {tag} balances update: {tag}_balances length ({}) != resolved_accounts length ({})",
                        balances.len(),
                        resolved_accounts.len()
                    );
                    return Vec::new();
                }
                balances
                    .iter()
                    .enumerate()
                    .filter_map(|(i, amount)| {
                        resolved_accounts.get(i).map(|account| pb::Balance {
                            account: account.to_vec(),
                            amount: *amount,
                        })
                    })
                    .collect()
            };

            transaction.post_balances = create_balances(&meta.post_balances, "post");
            transaction.pre_balances = create_balances(&meta.pre_balances, "pre");
        }

        // Native Token Instructions
        for instruction in tx.walk_instructions() {
            let program_id = instruction.program_id().0;

            // Skip instructions
            if !is_system_program(&base58::encode(program_id)) {
                continue;
            }

            let mut base = pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: instruction.stack_height(),
                is_root: instruction.is_root(),
                instruction: None,
            };

            // Unpack transfers
            if let Some(instruction) = system::unpack_transfers(&instruction) {
                base.instruction = Some(instruction);
                transaction.instructions.push(base.clone());
            }
        }
        if !transaction.instructions.is_empty() || !transaction.post_balances.is_empty() || !transaction.pre_balances.is_empty() {
            events.transactions.push(transaction);
        }
    }
    Ok(events)
}
