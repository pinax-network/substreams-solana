use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

use crate::is_spl_token_program;

pub fn unpack_permissions(instruction: &InstructionView, program_id: &[u8]) -> Option<pb::instruction::Instruction> {
    if !is_spl_token_program(&program_id) {
        return None;
    }
    match TokenInstruction::unpack(&instruction.data()).ok()? {
        // -- Approve --
        TokenInstruction::Approve { amount } => {
            // accounts
            let authority = instruction.accounts()[2].0.to_vec();
            return Some(pb::instruction::Instruction::Approve(pb::Approve {
                // authority
                authority: authority.clone(),
                multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                // event
                source: instruction.accounts()[0].0.to_vec(),
                mint: None,
                delegate: instruction.accounts()[1].0.to_vec(),
                owner: authority,
                amount,
                decimals: None,
            }));
        }
        // -- ApproveChecked --
        TokenInstruction::ApproveChecked { amount, decimals } => {
            // accounts
            let authority = instruction.accounts()[3].0.to_vec();
            return Some(pb::instruction::Instruction::Approve(pb::Approve {
                // authority
                authority: authority.clone(),
                multisig_authority: instruction.accounts()[4..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                // event
                source: instruction.accounts()[0].0.to_vec(),
                mint: Some(instruction.accounts()[1].0.to_vec()),
                delegate: instruction.accounts()[2].0.to_vec(),
                owner: authority,
                amount,
                decimals: Some(decimals as u32),
            }));
        }
        // -- Revoke --
        TokenInstruction::Revoke {} => {
            // accounts
            let authority = instruction.accounts()[1].0.to_vec();
            return Some(pb::instruction::Instruction::Revoke(pb::Revoke {
                // authority
                authority: authority.clone(),
                multisig_authority: instruction.accounts()[2..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                // event
                source: instruction.accounts()[0].0.to_vec(),
                owner: authority,
            }));
        }
        // -- FreezeAccount --
        TokenInstruction::FreezeAccount {} => {
            // accounts
            return Some(pb::instruction::Instruction::FreezeAccount(pb::FreezeAccount {
                // authority
                authority: instruction.accounts()[2].0.to_vec(),
                multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),
                account: instruction.accounts()[0].0.to_vec(),
                mint: instruction.accounts()[1].0.to_vec(),
            }));
        }
        // -- ThawAccount --
        TokenInstruction::ThawAccount {} => {
            // accounts
            return Some(pb::instruction::Instruction::ThawAccount(pb::ThawAccount {
                // authority
                authority: instruction.accounts()[2].0.to_vec(),
                multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),
                account: instruction.accounts()[0].0.to_vec(),
                mint: instruction.accounts()[1].0.to_vec(),
            }));
        }
        _ => None,
    }
}
