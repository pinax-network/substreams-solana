mod vote;

use common::solana::{get_fee_payer, get_signers};
use proto::pb::solana::native::vote::v1 as pb;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Vote Program (Vote111111111111111111111111111111111111111)
pub const VOTE_PROGRAM: [u8; 32] = [
    7, 97, 72, 29, 53, 116, 116, 187, 124, 77, 118, 36, 235, 211, 189, 179, 216, 53, 94, 115, 209, 16, 67, 252, 13, 163, 83, 128, 0, 0, 0, 0,
];

pub fn is_vote_program(program_id: &[u8]) -> bool {
    program_id == &VOTE_PROGRAM
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<_> = tx
        .walk_instructions()
        .filter_map(|iview| {
            let program_id = iview.program_id().0;
            if !is_vote_program(&program_id) {
                return None;
            }

            vote::unpack_instruction(&iview).map(|instruction| pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: iview.stack_height(),
                is_root: iview.is_root(),
                instruction: Some(instruction),
            })
        })
        .collect();

    if instructions.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
    })
}
