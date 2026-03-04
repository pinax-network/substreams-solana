mod stake;

use common::solana::{get_fee_payer, get_signers};
use proto::pb::solana::native::stake::v1 as pb;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Stake Program (Stake11111111111111111111111111111111111111)
pub const STAKE_PROGRAM: [u8; 32] = [
    6, 161, 216, 23, 145, 55, 84, 42, 152, 52, 55, 189, 254, 42, 122, 178, 85, 127, 83, 92, 138, 120, 114, 43, 104, 164, 157, 192, 0, 0, 0, 0,
];

pub fn is_stake_program(program_id: &[u8]) -> bool {
    program_id == &STAKE_PROGRAM
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
            if !is_stake_program(&program_id) {
                return None;
            }

            stake::unpack_instruction(&iview).map(|instruction| pb::Instruction {
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
