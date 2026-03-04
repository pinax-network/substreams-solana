use common::solana::{get_fee_payer, get_signers};
use proto::pb::solana::spl::token_swap::v1 as pb;
use substreams::errors::Error;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Token Swap Program (SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8)
pub const TOKEN_SWAP_PROGRAM: [u8; 32] = [
    6, 165, 58, 174, 54, 191, 72, 111, 181, 217, 56, 38, 78, 230, 69, 215, 75, 96, 22, 224, 244, 122, 235, 179, 236, 22, 67, 139, 247, 191, 251, 225,
];

pub fn is_token_swap_program(program_id: &[u8]) -> bool {
    program_id == &TOKEN_SWAP_PROGRAM
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<_> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();

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

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;

    if !is_token_swap_program(program_id) {
        return None;
    }

    // Token Swap instructions are identified by the first byte
    let data = instruction.data();
    if data.is_empty() {
        return None;
    }

    let parsed_instruction = match data[0] {
        0 => {
            // Initialize
            let accounts = instruction.accounts();
            if accounts.len() < 7 {
                return None;
            }
            Some(pb::instruction::Instruction::Initialize(pb::Initialize {
                swap_account: accounts[0].0.to_vec(),
                authority: accounts[1].0.to_vec(),
                token_a: accounts[2].0.to_vec(),
                token_b: accounts[3].0.to_vec(),
                pool_mint: accounts[4].0.to_vec(),
                fee_account: accounts[5].0.to_vec(),
                destination: accounts[6].0.to_vec(),
            }))
        }
        1 => {
            // Swap
            let accounts = instruction.accounts();
            if accounts.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::Swap(pb::Swap {
                swap_account: accounts[0].0.to_vec(),
                authority: accounts[1].0.to_vec(),
                user_transfer_authority: accounts[2].0.to_vec(),
                source: accounts[3].0.to_vec(),
                swap_source: accounts[4].0.to_vec(),
                swap_destination: accounts[5].0.to_vec(),
                destination: accounts[6].0.to_vec(),
                pool_mint: accounts[7].0.to_vec(),
                fee_account: accounts[8].0.to_vec(),
                amount_in: 0,
                minimum_amount_out: 0,
            }))
        }
        _ => None,
    };

    parsed_instruction.map(|parsed| pb::Instruction {
        program_id: program_id.to_vec(),
        stack_height: instruction.stack_height(),
        is_root: instruction.is_root(),
        instruction: Some(parsed),
    })
}
