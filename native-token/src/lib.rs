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
    for tx in block.transactions_owned() {
        let mut transaction = pb::Transaction::default();
        let tx_meta = tx.meta.as_ref().expect("Transaction meta should be present");
        transaction.fee = tx_meta.fee;
        transaction.compute_units_consumed = tx_meta.compute_units_consumed();
        transaction.signature = tx.hash().to_vec();
        transaction.fee_payer = get_fee_payer(&tx).unwrap_or_default();
        transaction.signers = get_signers(&tx).unwrap_or_default();

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

        transaction.post_balances = create_balances(&tx_meta.post_balances, "post");
        transaction.pre_balances = create_balances(&tx_meta.pre_balances, "pre");

        transaction.instructions = tx
            .walk_instructions()
            .filter_map(|iview| {
                let program_id = iview.program_id().0;
                if !is_system_program(&base58::encode(program_id)) {
                    return None;
                }
                system::unpack_transfers(&iview).map(|instruction| pb::Instruction {
                    program_id: program_id.to_vec(),
                    stack_height: iview.stack_height(),
                    is_root: iview.is_root(),
                    instruction: Some(instruction),
                })
            })
            .collect();

        if !transaction.instructions.is_empty() || !transaction.post_balances.is_empty() || !transaction.pre_balances.is_empty() {
            events.transactions.push(transaction);
        }
    }
    Ok(events)
}
