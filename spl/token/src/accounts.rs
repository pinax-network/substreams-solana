use core::panic;

use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::{base58, block_view::InstructionView};
use substreams_solana_program_instructions::{option::COption, token_instruction_2022::TokenInstruction};

use crate::is_spl_token_program;

pub fn unpack_accounts(instruction: &InstructionView, program_id: &[u8]) -> Option<pb::instruction::Instruction> {
    if !is_spl_token_program(&program_id) {
        return None;
    }
    match TokenInstruction::unpack(&instruction.data()) {
        Err(_err) => return None,
        Ok(token_instruction) => match token_instruction {
            // -- InitializeAccount --
            TokenInstruction::InitializeAccount {} => {
                return Some(pb::instruction::Instruction::InitializeAccount(pb::InitializeAccount {
                    account: instruction.accounts()[0].0.to_vec(), // The account to initialize.
                    mint: instruction.accounts()[1].0.to_vec(),    // The mint this account will be associated with.
                    owner: instruction.accounts()[2].0.to_vec(),   // The new account's owner/multisignature.
                }));
            }
            // -- InitializeAccount2 --
            TokenInstruction::InitializeAccount2 { owner } => {
                return Some(pb::instruction::Instruction::InitializeAccount(pb::InitializeAccount {
                    account: instruction.accounts()[0].0.to_vec(), // The account to initialize.
                    mint: instruction.accounts()[1].0.to_vec(),    // The mint this account will be associated with.
                    owner: owner.to_bytes().to_vec(),
                }));
            }
            // -- InitializeAccount3 --
            TokenInstruction::InitializeAccount3 { owner } => {
                return Some(pb::instruction::Instruction::InitializeAccount(pb::InitializeAccount {
                    account: instruction.accounts()[0].0.to_vec(), // The account to initialize.
                    mint: instruction.accounts()[1].0.to_vec(),    // The mint this account will be associated with.
                    owner: owner.to_bytes().to_vec(),
                }));
            }
            // -- InitializeImmutableOwner --
            TokenInstruction::InitializeImmutableOwner => {
                return Some(pb::instruction::Instruction::InitializeImmutableOwner(pb::InitializeImmutableOwner {
                    account: instruction.accounts()[0].0.to_vec(), // The account to initialize.
                }));
            }
            // -- CloseAccount --
            TokenInstruction::CloseAccount {} => {
                if instruction.accounts().len() < 3 {
                    panic!("CloseAccount requires at least 3 accounts {}", base58::encode(instruction.transaction().hash()));
                }
                return Some(pb::instruction::Instruction::CloseAccount(pb::CloseAccount {
                    account: instruction.accounts()[0].0.to_vec(),     // The account to close.
                    destination: instruction.accounts()[1].0.to_vec(), // The destination to transfer the lamports to.
                    authority: instruction.accounts()[2].0.to_vec(),   // The authority to close the account.
                    multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),
                }));
            }
            // -- SetAuthority --
            TokenInstruction::SetAuthority { authority_type, new_authority } => {
                // accounts
                return Some(pb::instruction::Instruction::SetAuthority(pb::SetAuthority {
                    account: instruction.accounts()[0].0.to_vec(),
                    authority_type: authority_type as i32 + 1,
                    authority: instruction.accounts()[1].0.to_vec(),
                    multisig_authority: instruction.accounts()[2..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),
                    new_authority: match new_authority {
                        COption::Some(key) => Some(key.to_bytes().to_vec()),
                        COption::None => None,
                    },
                }));
            }
            _ => None,
        },
    }
}
