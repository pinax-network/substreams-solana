use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

pub fn unpack_transfers(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match TokenInstruction::unpack(&instruction.data()) {
        Err(_err) => return None,

        // -- TransferChecked --
        Ok(token_instruction) => match token_instruction {
            // -- TransferChecked --
            TokenInstruction::TransferChecked { amount, decimals } => {
                if amount > 0 {
                    // accounts
                    let source = instruction.accounts()[0].0.to_vec();
                    let mint = instruction.accounts()[1].0.to_vec();
                    let destination = instruction.accounts()[2].0.to_vec();
                    let authority = instruction.accounts()[3].0.to_vec();
                    let multisig_authority = instruction.accounts()[4..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                    return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                        // authority
                        authority,
                        multisig_authority: multisig_authority.to_vec(),

                        // event
                        source,
                        destination,
                        amount,
                        mint: Some(mint),
                        decimals: Some(decimals as u32),
                    }));
                }
                return None;
            }
            // -- Transfer (DEPRECATED, but still active) --
            #[allow(deprecated)]
            TokenInstruction::Transfer { amount } => {
                if amount > 0 {
                    // accounts
                    let source = instruction.accounts()[0].0.to_vec();
                    let destination = instruction.accounts()[1].0.to_vec();
                    let authority = instruction.accounts()[2].0.to_vec();
                    let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                    return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                        // authority
                        authority,
                        multisig_authority: multisig_authority.to_vec(),

                        // event
                        source,
                        destination,
                        amount,
                        decimals: None,
                        mint: None,
                    }));
                }
                return None;
            }
            // -- Mint To --
            TokenInstruction::MintTo { amount } => {
                if amount > 0 {
                    // accounts
                    let mint = instruction.accounts()[0].0.to_vec();
                    let destination = instruction.accounts()[1].0.to_vec();
                    let authority = instruction.accounts()[2].0.to_vec();
                    let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                    return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                        // authority
                        authority,
                        multisig_authority: multisig_authority.to_vec(),

                        // event
                        source: mint.to_vec(),
                        destination,
                        amount,
                        decimals: None,
                        mint: Some(mint),
                    }));
                }
                return None;
            }
            // -- Mint To Checked --
            TokenInstruction::MintToChecked { amount, decimals } => {
                if amount > 0 {
                    // accounts
                    let mint = instruction.accounts()[0].0.to_vec();
                    let destination = instruction.accounts()[1].0.to_vec();
                    let authority = instruction.accounts()[2].0.to_vec();
                    let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                    return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                        // authority
                        authority,
                        multisig_authority: multisig_authority.to_vec(),

                        // event
                        source: mint.to_vec(),
                        destination,
                        amount,
                        decimals: Some(decimals as u32),
                        mint: Some(mint),
                    }));
                }
                return None;
            }
            // -- Burn --
            TokenInstruction::Burn { amount } => {
                if amount > 0 {
                    // accounts
                    let source = instruction.accounts()[0].0.to_vec();
                    let mint = instruction.accounts()[1].0.to_vec();
                    let authority = instruction.accounts()[2].0.to_vec();
                    let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                    return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                        // authority
                        authority,
                        multisig_authority: multisig_authority.to_vec(),

                        // event
                        source,
                        destination: mint.to_vec(),
                        amount,
                        decimals: None,
                        mint: Some(mint),
                    }));
                }
                return None;
            }
            // -- BurnChecked --
            TokenInstruction::BurnChecked { amount, decimals } => {
                if amount > 0 {
                    // accounts
                    let source = instruction.accounts()[0].0.to_vec();
                    let mint = instruction.accounts()[1].0.to_vec();
                    let authority = instruction.accounts()[2].0.to_vec();
                    let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                    return Some(pb::instruction::Instruction::Transfer(pb::Transfer {
                        // authority
                        authority,
                        multisig_authority: multisig_authority.to_vec(),

                        // event
                        source,
                        destination: mint.to_vec(),
                        amount,
                        decimals: Some(decimals as u32),
                        mint: Some(mint),
                    }));
                }
                return None;
            }
            _ => None,
        },
    }
}
