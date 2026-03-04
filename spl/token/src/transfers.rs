use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

use crate::is_spl_token_program;

pub fn unpack_transfers(instruction: &InstructionView, program_id: &[u8]) -> Option<pb::instruction::Instruction> {
    if !is_spl_token_program(&program_id) {
        return None;
    }
    match TokenInstruction::unpack(&instruction.data()).ok()? {
        // -- TransferChecked --
        TokenInstruction::TransferChecked { amount, decimals } => {
            if amount > 0 {
                return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                    // authority
                    authority: instruction.accounts()[3].0.to_vec(),
                    multisig_authority: instruction.accounts()[4..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                    // event
                    source: instruction.accounts()[0].0.to_vec(),
                    destination: instruction.accounts()[2].0.to_vec(),
                    amount,
                    mint: instruction.accounts()[1].0.to_vec(),
                    decimals: Some(decimals as u32),
                }));
            }
            return None;
        }
        // -- Transfer (DEPRECATED, but still active) --
        #[allow(deprecated)]
        TokenInstruction::Transfer { amount } => {
            if amount > 0 {
                return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                    // authority
                    authority: instruction.accounts()[2].0.to_vec(),
                    multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                    // event
                    source: instruction.accounts()[0].0.to_vec(),
                    destination: instruction.accounts()[2].0.to_vec(),
                    amount,
                    decimals: None,
                    mint: instruction.accounts()[1].0.to_vec(),
                }));
            }
            return None;
        }
        // -- Mint To --
        TokenInstruction::MintTo { amount } => {
            if amount > 0 {
                let mint = instruction.accounts()[0].0.to_vec();
                return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                    // authority
                    authority: instruction.accounts()[2].0.to_vec(),
                    multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                    // event
                    source: mint.clone(),
                    destination: instruction.accounts()[1].0.to_vec(),
                    amount,
                    mint,
                    decimals: None,
                }));
            }
            return None;
        }
        // -- Mint To Checked --
        TokenInstruction::MintToChecked { amount, decimals } => {
            if amount > 0 {
                // accounts
                let mint = instruction.accounts()[0].0.to_vec();
                return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                    // authority
                    authority: instruction.accounts()[2].0.to_vec(),
                    multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                    // event
                    source: mint.clone(),
                    destination: instruction.accounts()[1].0.to_vec(),
                    amount,
                    mint: mint,
                    decimals: Some(decimals as u32),
                }));
            }
            return None;
        }
        // -- Burn --
        TokenInstruction::Burn { amount } => {
            if amount > 0 {
                // accounts
                let mint = instruction.accounts()[1].0.to_vec();
                return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                    // authority
                    authority: instruction.accounts()[2].0.to_vec(),
                    multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                    // event
                    source: instruction.accounts()[0].0.to_vec(),
                    destination: mint.clone(),
                    amount,
                    mint,
                    decimals: None,
                }));
            }
            return None;
        }
        // -- BurnChecked --
        TokenInstruction::BurnChecked { amount, decimals } => {
            if amount > 0 {
                // accounts
                let mint = instruction.accounts()[1].0.to_vec();
                return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                    // authority
                    authority: instruction.accounts()[2].0.to_vec(),
                    multisig_authority: instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>(),

                    // event
                    source: instruction.accounts()[0].0.to_vec(),
                    destination: mint.clone(),
                    amount,
                    mint,
                    decimals: Some(decimals as u32),
                }));
            }
            return None;
        }
        _ => None,
    }
}
