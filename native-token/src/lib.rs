mod system;

use common::solana::{get_fee_payer, get_signers};
use proto::pb::solana::native::token::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

pub const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";

pub fn is_system_program(program_id: &str) -> bool {
    program_id == SYSTEM_PROGRAM
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let resolved_accounts = tx.resolved_accounts();
    let mut transaction = pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        ..Default::default()
    };

    let create_balances = |balances: &[u64], tag: &str| -> Vec<pb::Balance> {
        if balances.len() != resolved_accounts.len() {
            substreams::log::info!(
                "Skipping {tag} balances update: {tag}_balances invalid length {} != {}",
                balances.len(),
                resolved_accounts.len()
            );
            return Vec::new();
        }
        balances
            .iter()
            .zip(resolved_accounts.iter())
            .map(|(&amount, account)| pb::Balance {
                account: account.to_vec(),
                amount,
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

    if transaction.instructions.is_empty() && transaction.post_balances.is_empty() && transaction.pre_balances.is_empty() {
        None
    } else {
        Some(transaction)
    }
}
