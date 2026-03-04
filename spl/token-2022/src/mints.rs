use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::{option::COption, token_instruction_2022::TokenInstruction};

use crate::is_spl_token_program;

pub fn unpack_mints(instruction: &InstructionView, program_id: &[u8]) -> Option<pb::instruction::Instruction> {
    if !is_spl_token_program(&program_id) {
        return None;
    }
    match TokenInstruction::unpack(&instruction.data()).ok()? {
        // -- InitializeMint --
        TokenInstruction::InitializeMint {
            decimals,
            mint_authority,
            freeze_authority,
        } => {
            return Some(pb::instruction::Instruction::InitializeMint(pb::InitializeMint {
                mint: instruction.accounts()[0].0.to_vec(),
                mint_authority: mint_authority.to_bytes().to_vec(),
                freeze_authority: match freeze_authority {
                    COption::Some(key) => Some(key.to_bytes().to_vec()),
                    COption::None => None,
                },
                decimals: decimals as u32,
            }));
        }
        // -- InitializeMint2 --
        TokenInstruction::InitializeMint2 {
            decimals,
            mint_authority,
            freeze_authority,
        } => {
            return Some(pb::instruction::Instruction::InitializeMint(pb::InitializeMint {
                mint: instruction.accounts()[0].0.to_vec(),
                mint_authority: mint_authority.to_bytes().to_vec(),
                freeze_authority: match freeze_authority {
                    COption::Some(key) => Some(key.to_bytes().to_vec()),
                    COption::None => None,
                },
                decimals: decimals as u32,
            }));
        }
        _ => None,
    }
}
