mod transfers;
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

        // // Native Token Balances
        // match &tx.meta {
        //     Some(meta) => {
        //         // Resolved accounts (same order as Balances)
        //         let resolved_accounts = tx.resolved_accounts();

        //         // PostTokenBalances
        //         for (i, amount) in meta.post_balances.iter().enumerate() {
        //             transaction.post_balances.push(pb::Balance {
        //                 account: resolved_accounts.get(i as usize).unwrap().to_vec(),
        //                 amount: *amount,
        //             });
        //         }
        //         // PreTokenBalances
        //         for (i, amount) in meta.pre_balances.iter().enumerate() {
        //             transaction.pre_balances.push(pb::Balance {
        //                 account: resolved_accounts.get(i as usize).unwrap().to_vec(),
        //                 amount: *amount,
        //             });
        //         }
        //     }
        //     None => {}
        // }

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
            if let Some(instruction) = transfers::unpack_transfers(&instruction) {
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
