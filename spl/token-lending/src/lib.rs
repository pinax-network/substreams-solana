use common::solana::{get_fee_payer, get_signers};
use proto::pb::solana::spl::token_lending::v1 as pb;
use substreams::errors::Error;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Token Lending Program (LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi)
pub const TOKEN_LENDING_PROGRAM: [u8; 32] = [
    5, 8, 194, 206, 177, 181, 208, 92, 135, 73, 128, 172, 82, 207, 101, 151, 64, 231, 233, 185, 53, 106, 175, 42, 3, 98, 103, 50, 99, 82, 108, 21,
];

pub fn is_token_lending_program(program_id: &[u8]) -> bool {
    program_id == &TOKEN_LENDING_PROGRAM
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

    if !is_token_lending_program(program_id) {
        return None;
    }

    // Token Lending instructions are identified by the first byte
    let data = instruction.data();
    if data.is_empty() {
        return None;
    }

    let parsed_instruction = match data[0] {
        0 => {
            // InitLendingMarket
            let accounts = instruction.accounts();
            if accounts.len() < 2 {
                return None;
            }
            Some(pb::instruction::Instruction::InitLendingMarket(pb::InitLendingMarket {
                lending_market: accounts[0].0.to_vec(),
                owner: accounts[1].0.to_vec(),
                quote_currency: Vec::new(),
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
