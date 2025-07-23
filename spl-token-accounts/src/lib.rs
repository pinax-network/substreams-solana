use common::solana::{get_fee_payer, get_signers, is_spl_token_program};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::{errors::Error, log};
use substreams_solana::{base58, pb::sf::solana::r#type::v1::Block};
use substreams_solana_program_instructions::{option::COption, token_instruction_2022::TokenInstruction};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    let mut events = pb::Events::default();

    // transactions
    for tx in block.transactions() {
        let mut transaction = pb::Transaction::default();
        let tx_meta = tx.meta.as_ref().expect("Transaction meta should be present");
        transaction.fee = tx_meta.fee;
        transaction.compute_units_consumed = tx_meta.compute_units_consumed();
        transaction.signature = tx.hash().to_vec();

        if let Some(fee_payer) = get_fee_payer(tx) {
            transaction.fee_payer = fee_payer;
        }
        if let Some(signers) = get_signers(tx) {
            transaction.signers = signers;
        }
        for instruction in tx.walk_instructions() {
            let program_id = instruction.program_id().0;

            // Skip instructions
            if !is_spl_token_program(&base58::encode(program_id)) {
                continue;
            }

            let mut base = pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: instruction.stack_height(),
                is_root: instruction.is_root(),
                instruction: None,
            };

            match TokenInstruction::unpack(&instruction.data()) {
                Err(err) => {
                    log::debug!("unpacking error: {}", err);
                }
                // -- TransferChecked --
                Ok(token_instruction) => match token_instruction {
                    // -- InitializeAccount --
                    TokenInstruction::InitializeAccount {} => {
                        // accounts
                        let account: Vec<u8> = instruction.accounts()[0].0.to_vec(); // The account to initialize.
                        let mint: Vec<u8> = instruction.accounts()[1].0.to_vec(); // The mint this account will be associated with.
                        let owner: Vec<u8> = instruction.accounts()[2].0.to_vec(); // The new account's owner/multisignature.

                        base.instruction = Some(pb::instruction::Instruction::InitializeAccount(pb::InitializeAccount { account, mint, owner }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- InitializeAccount2 --
                    TokenInstruction::InitializeAccount2 { owner } => {
                        // accounts
                        let account: Vec<u8> = instruction.accounts()[0].0.to_vec(); // The account to initialize.
                        let mint: Vec<u8> = instruction.accounts()[1].0.to_vec(); // The mint this account will be associated with.

                        base.instruction = Some(pb::instruction::Instruction::InitializeAccount(pb::InitializeAccount {
                            account,
                            mint,
                            owner: owner.to_bytes().to_vec(),
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- InitializeAccount3 --
                    TokenInstruction::InitializeAccount3 { owner } => {
                        // accounts
                        let account: Vec<u8> = instruction.accounts()[0].0.to_vec(); // The account to initialize.
                        let mint: Vec<u8> = instruction.accounts()[1].0.to_vec(); // The mint this account will be associated with.

                        base.instruction = Some(pb::instruction::Instruction::InitializeAccount(pb::InitializeAccount {
                            account,
                            mint,
                            owner: owner.to_bytes().to_vec(),
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- InitializeImmutableOwner --
                    TokenInstruction::InitializeImmutableOwner => {
                        // accounts
                        let account: Vec<u8> = instruction.accounts()[0].0.to_vec(); // The account to initialize.

                        base.instruction = Some(pb::instruction::Instruction::InitializeImmutableOwner(pb::InitializeImmutableOwner { account }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- CloseAccount --
                    TokenInstruction::CloseAccount {} => {
                        // accounts
                        let account = instruction.accounts()[0].0.to_vec();
                        let destination = instruction.accounts()[1].0.to_vec();
                        let authority = instruction.accounts()[2].0.to_vec();
                        let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                        base.instruction = Some(pb::instruction::Instruction::CloseAccount(pb::CloseAccount {
                            account,
                            destination,
                            authority,
                            multisig_authority,
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- SetAuthority --
                    TokenInstruction::SetAuthority { authority_type, new_authority } => {
                        // accounts
                        let account = instruction.accounts()[0].0.to_vec();
                        let authority = instruction.accounts()[1].0.to_vec();
                        let multisig_authority = instruction.accounts()[2..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                        base.instruction = Some(pb::instruction::Instruction::SetAuthority(pb::SetAuthority {
                            account,
                            authority_type: authority_type as i32 + 1,
                            authority: authority.to_vec(),
                            multisig_authority: multisig_authority.to_vec(),
                            new_authority: match new_authority {
                                COption::Some(key) => Some(key.to_bytes().to_vec()),
                                COption::None => None,
                            },
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    _ => {}
                },
            }
        }
        if !transaction.instructions.is_empty() {
            events.transactions.push(transaction);
        }
    }
    Ok(events)
}
