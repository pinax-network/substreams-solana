use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

pub fn unpack_permissions(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match TokenInstruction::unpack(&instruction.data()) {
        Err(_err) => return None,
        Ok(token_instruction) => match token_instruction {
            // -- Approve --
            TokenInstruction::Approve { amount } => {
                // accounts
                let source = instruction.accounts()[0].0.to_vec();
                let delegate = instruction.accounts()[1].0.to_vec();
                let authority = instruction.accounts()[2].0.to_vec();
                let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                return Some(pb::instruction::Instruction::Approve(pb::Approve {
                    // authority
                    authority: authority.to_vec(),
                    multisig_authority: multisig_authority.to_vec(),

                    // event
                    source,
                    mint: None,
                    delegate,
                    owner: authority,
                    amount,
                    decimals: None,
                }));
            }
            // -- ApproveChecked --
            TokenInstruction::ApproveChecked { amount, decimals } => {
                // accounts
                let source = instruction.accounts()[0].0.to_vec();
                let mint = instruction.accounts()[1].0.to_vec();
                let delegate = instruction.accounts()[2].0.to_vec();
                let authority = instruction.accounts()[3].0.to_vec();
                let multisig_authority = instruction.accounts()[4..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                return Some(pb::instruction::Instruction::Approve(pb::Approve {
                    // authority
                    authority: authority.to_vec(),
                    multisig_authority: multisig_authority.to_vec(),

                    // event
                    source,
                    mint: Some(mint),
                    delegate,
                    owner: authority,
                    amount,
                    decimals: Some(decimals as u32),
                }));
            }
            // -- Revoke --
            TokenInstruction::Revoke {} => {
                // accounts
                let source = instruction.accounts()[0].0.to_vec();
                let authority = instruction.accounts()[1].0.to_vec();
                let multisig_authority = instruction.accounts()[2..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                return Some(pb::instruction::Instruction::Revoke(pb::Revoke {
                    // authority
                    authority: authority.to_vec(),
                    multisig_authority: multisig_authority.to_vec(),

                    // event
                    source,
                    owner: authority,
                }));
            }
            // -- FreezeAccount --
            TokenInstruction::FreezeAccount {} => {
                // accounts
                let account = instruction.accounts()[0].0.to_vec();
                let mint = instruction.accounts()[1].0.to_vec();
                let authority = instruction.accounts()[2].0.to_vec();
                let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                return Some(pb::instruction::Instruction::FreezeAccount(pb::FreezeAccount {
                    // authority
                    authority,
                    multisig_authority,
                    account,
                    mint,
                }));
            }
            // -- ThawAccount --
            TokenInstruction::ThawAccount {} => {
                // accounts
                let account = instruction.accounts()[0].0.to_vec();
                let mint = instruction.accounts()[1].0.to_vec();
                let authority = instruction.accounts()[2].0.to_vec();
                let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                return Some(pb::instruction::Instruction::ThawAccount(pb::ThawAccount {
                    authority,
                    multisig_authority,
                    account,
                    mint,
                }));
            }
            _ => None,
        },
    }
}
