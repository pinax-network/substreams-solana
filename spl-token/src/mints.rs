use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::{option::COption, token_instruction_2022::TokenInstruction};

pub fn unpack_mints(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match TokenInstruction::unpack(&instruction.data()) {
        Err(_err) => return None,

        // -- TransferChecked --
        Ok(token_instruction) => match token_instruction {
            // -- InitializeMint --
            TokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                // accounts
                let mint = instruction.accounts()[0].0.to_vec();
                return Some(pb::instruction::Instruction::InitializeMint(pb::InitializeMint {
                    mint,
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
                // accounts
                let mint = instruction.accounts()[0].0.to_vec();

                return Some(pb::instruction::Instruction::InitializeMint(pb::InitializeMint {
                    mint,
                    mint_authority: mint_authority.to_bytes().to_vec(),
                    freeze_authority: match freeze_authority {
                        COption::Some(key) => Some(key.to_bytes().to_vec()),
                        COption::None => None,
                    },
                    decimals: decimals as u32,
                }));
            }
            _ => None,
        },
    }
}
