use common::solana::{get_fee_payer, get_signers, is_spl_token_program};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::{errors::Error, log};
use substreams_solana::{base58, pb::sf::solana::r#type::v1::Block};
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

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
                substreams::log::info!("Skipping non-SPL Token instruction: {}", base58::encode(program_id));
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
                    TokenInstruction::TransferChecked { amount, decimals } => {
                        if amount > 0 {
                            // accounts
                            let source = instruction.accounts()[0].0.to_vec();
                            let mint = instruction.accounts()[1].0.to_vec();
                            let destination = instruction.accounts()[2].0.to_vec();
                            let authority = instruction.accounts()[3].0.to_vec();
                            let multisig_authority = instruction.accounts()[4..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                            base.instruction = Some(pb::instruction::Instruction::Transfer(pb::Transfer {
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
                            transaction.instructions.push(base.clone());
                        }
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

                            base.instruction = Some(pb::instruction::Instruction::Transfer(pb::Transfer {
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
                            transaction.instructions.push(base.clone());
                        }
                    }
                    // -- Mint To --
                    TokenInstruction::MintTo { amount } => {
                        if amount > 0 {
                            // accounts
                            let mint = instruction.accounts()[0].0.to_vec();
                            let destination = instruction.accounts()[1].0.to_vec();
                            let authority = instruction.accounts()[2].0.to_vec();
                            let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                            base.instruction = Some(pb::instruction::Instruction::Transfer(pb::Transfer {
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
                            transaction.instructions.push(base.clone());
                        }
                    }
                    // -- Mint To Checked --
                    TokenInstruction::MintToChecked { amount, decimals } => {
                        if amount > 0 {
                            // accounts
                            let mint = instruction.accounts()[0].0.to_vec();
                            let destination = instruction.accounts()[1].0.to_vec();
                            let authority = instruction.accounts()[2].0.to_vec();
                            let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                            base.instruction = Some(pb::instruction::Instruction::Transfer(pb::Transfer {
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
                            transaction.instructions.push(base.clone());
                        }
                    }
                    // -- Burn --
                    TokenInstruction::Burn { amount } => {
                        if amount > 0 {
                            // accounts
                            let source = instruction.accounts()[0].0.to_vec();
                            let mint = instruction.accounts()[1].0.to_vec();
                            let authority = instruction.accounts()[2].0.to_vec();
                            let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                            base.instruction = Some(pb::instruction::Instruction::Transfer(pb::Transfer {
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
                            transaction.instructions.push(base.clone());
                        }
                    }
                    // -- BurnChecked --
                    TokenInstruction::BurnChecked { amount, decimals } => {
                        if amount > 0 {
                            // accounts
                            let source = instruction.accounts()[0].0.to_vec();
                            let mint = instruction.accounts()[1].0.to_vec();
                            let authority = instruction.accounts()[2].0.to_vec();
                            let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                            base.instruction = Some(pb::instruction::Instruction::Transfer(pb::Transfer {
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
                            transaction.instructions.push(base.clone());
                        }
                    }
                    // -- Approve --
                    TokenInstruction::Approve { amount } => {
                        // accounts
                        let source = instruction.accounts()[0].0.to_vec();
                        let delegate = instruction.accounts()[1].0.to_vec();
                        let authority = instruction.accounts()[2].0.to_vec();
                        let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                        base.instruction = Some(pb::instruction::Instruction::Approve(pb::Approve {
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
                        transaction.instructions.push(base.clone());
                    }
                    // -- ApproveChecked --
                    TokenInstruction::ApproveChecked { amount, decimals } => {
                        // accounts
                        let source = instruction.accounts()[0].0.to_vec();
                        let mint = instruction.accounts()[1].0.to_vec();
                        let delegate = instruction.accounts()[2].0.to_vec();
                        let authority = instruction.accounts()[3].0.to_vec();
                        let multisig_authority = instruction.accounts()[4..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                        base.instruction = Some(pb::instruction::Instruction::Approve(pb::Approve {
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
                        transaction.instructions.push(base.clone());
                    }
                    // -- Revoke --
                    TokenInstruction::Revoke {} => {
                        // accounts
                        let source = instruction.accounts()[0].0.to_vec();
                        let authority = instruction.accounts()[1].0.to_vec();
                        let multisig_authority = instruction.accounts()[2..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                        base.instruction = Some(pb::instruction::Instruction::Revoke(pb::Revoke {
                            // authority
                            authority: authority.to_vec(),
                            multisig_authority: multisig_authority.to_vec(),

                            // event
                            source,
                            owner: authority,
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
