use common::solana::is_confirmed_tx;
use proto::pb::solana::spl::token::v1::{Events, InitializeAccount, InitializeMint, Instructions, Transfer};
use substreams::{errors::Error, log};
use substreams_solana::{block_view::InstructionView, pb::sf::solana::r#type::v1::Block};
use substreams_solana_program_instructions::{option::COption, pubkey::Pubkey, token_instruction_2022::TokenInstruction};

pub const SOLANA_TOKEN_PROGRAM_KEG: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const SOLANA_TOKEN_PROGRAM_ZQB: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

pub fn is_spl_token_program(instruction: &InstructionView) -> bool {
    let program_id = instruction.program_id().to_string();
    program_id == SOLANA_TOKEN_PROGRAM_KEG || program_id == SOLANA_TOKEN_PROGRAM_ZQB
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, Error> {
    let mut events: Events = Events::default();

    // running counter of every SPL-Token instruction encountered across the whole block, incremented each time one is processed.
    let mut execution_index = 0;

    // transactions
    for tx in block.transactions {
        if is_confirmed_tx(&tx) == false {
            continue;
        }
        // position of the current instruction inside its transaction (root + inner), counted from the start of that transaction.
        let mut instruction_index = 0;
        // position of the instruction among inner (non-root) instructions within the same transaction; stays 0 for root instructions and increments only for nested ones.
        let mut inner_instruction_index = 0;

        // instructions
        // Iterates over all instructions, including inner instructions, of the transaction.
        // The iteration starts with the first compiled instruction and then goes through all its inner instructions, if any.
        // Then it moves to the next compiled instruction and so on recursively.
        for instruction in tx.walk_instructions() {
            if !is_spl_token_program(&instruction) {
                continue;
            }

            // increment indexes
            execution_index += 1;
            instruction_index += 1;
            if !instruction.is_root() {
                inner_instruction_index += 1;
            }
            let stack_height = instruction.maybe_stack_height().unwrap_or(instruction.stack_height());
            let tx_hash = tx.hash().to_vec();
            let program_id = instruction.program_id().0.to_vec();

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
                            let destination = instruction.accounts()[2].0.to_vec();
                            let authority = [instruction.accounts()[3].0.to_vec()];
                            // TO-DO: handle Multisig authority

                            events.transfers.push(Transfer {
                                // transaction
                                tx_hash,

                                // indexes
                                execution_index,
                                instruction_index,
                                inner_instruction_index,

                                // instruction
                                program_id,
                                stack_height,

                                // event
                                authority: authority.to_vec(),
                                source,
                                destination,
                                amount,
                                decimals: Some(decimals as u32),
                                instruction: Instructions::TransferChecked as i32,
                            })
                        }
                    }
                    // -- Transfer (DEPRECATED, but still active) --
                    TokenInstruction::Transfer { amount } => {
                        if amount > 0 {
                            // accounts
                            let source = instruction.accounts()[0].0.to_vec();
                            let destination = instruction.accounts()[1].0.to_vec();
                            let authority = [instruction.accounts()[2].0.to_vec()];
                            // TO-DO: handle Multisig authority

                            events.transfers.push(Transfer {
                                // transaction
                                tx_hash,

                                // indexes
                                execution_index,
                                instruction_index,
                                inner_instruction_index,

                                // instruction
                                program_id,
                                stack_height,

                                // event
                                source,
                                destination,
                                authority: authority.to_vec(),
                                amount,
                                decimals: None,
                                instruction: Instructions::Transfer as i32,
                            })
                        }
                    }

                    // -- Mint To --
                    TokenInstruction::MintTo { amount } => {
                        if amount > 0 {
                            // accounts
                            let mint = instruction.accounts()[0].0.to_vec();
                            let destination = instruction.accounts()[1].0.to_vec();
                            let authority = [instruction.accounts()[2].0.to_vec()];
                            // TO-DO: handle Multisig authority

                            events.mints.push(Transfer {
                                // transaction
                                tx_hash,

                                // indexes
                                execution_index,
                                instruction_index,
                                inner_instruction_index,

                                // instruction
                                program_id,
                                stack_height,

                                // event
                                source: mint,
                                destination,
                                authority: authority.to_vec(),
                                amount,
                                decimals: None,
                                instruction: Instructions::MintTo as i32,
                            })
                        }
                    }

                    // -- Mint To Checked --
                    TokenInstruction::MintToChecked { amount, decimals } => {
                        if amount > 0 {
                            // accounts
                            let mint = instruction.accounts()[0].0.to_vec();
                            let destination = instruction.accounts()[1].0.to_vec();
                            let authority = [instruction.accounts()[2].0.to_vec()];
                            // TO-DO: handle Multisig authority

                            events.mints.push(Transfer {
                                // transaction
                                tx_hash,

                                // indexes
                                execution_index,
                                instruction_index,
                                inner_instruction_index,

                                // instruction
                                program_id,
                                stack_height,

                                // event
                                source: mint,
                                destination,
                                authority: authority.to_vec(),
                                amount,
                                decimals: Some(decimals as u32),
                                instruction: Instructions::MintToChecked as i32,
                            })
                        }
                    }

                    // -- Burn --
                    TokenInstruction::Burn { amount } => {
                        if amount > 0 {
                            // accounts
                            let source = instruction.accounts()[0].0.to_vec();
                            let mint = instruction.accounts()[1].0.to_vec();
                            let authority = [instruction.accounts()[2].0.to_vec()];
                            // TO-DO: handle Multisig authority

                            events.burns.push(Transfer {
                                // transaction
                                tx_hash,

                                // indexes
                                execution_index,
                                instruction_index,
                                inner_instruction_index,

                                // instruction
                                program_id,
                                stack_height,

                                // event
                                source,
                                destination: mint,
                                authority: authority.to_vec(),
                                amount,
                                decimals: None,
                                instruction: Instructions::Burn as i32,
                            })
                        }
                    }

                    // -- BurnChecked --
                    TokenInstruction::BurnChecked { amount, decimals } => {
                        if amount > 0 {
                            // accounts
                            let source = instruction.accounts()[0].0.to_vec();
                            let mint = instruction.accounts()[1].0.to_vec();
                            let authority = [instruction.accounts()[2].0.to_vec()];
                            // TO-DO: handle Multisig authority

                            events.burns.push(Transfer {
                                // transaction
                                tx_hash,

                                // indexes
                                execution_index,
                                instruction_index,
                                inner_instruction_index,

                                // instruction
                                program_id,
                                stack_height,

                                // event
                                source,
                                destination: mint,
                                authority: authority.to_vec(),
                                amount,
                                decimals: Some(decimals as u32),
                                instruction: Instructions::Burn as i32,
                            })
                        }
                    }

                    // -- InitializeMint --
                    TokenInstruction::InitializeMint {
                        decimals,
                        mint_authority,
                        freeze_authority,
                    } => {
                        // accounts
                        let mint = instruction.accounts()[0].0.to_vec();

                        events.initialize_mints.push(InitializeMint {
                            // transaction
                            tx_hash,

                            // indexes
                            execution_index,
                            instruction_index,
                            inner_instruction_index,

                            // instruction
                            program_id,
                            stack_height,

                            // event
                            mint,
                            mint_authority: mint_authority.to_bytes().to_vec(),
                            freeze_authority: match freeze_authority {
                                COption::Some(key) => Some(key.to_bytes().to_vec()),
                                COption::None => None,
                            },
                            decimals: decimals as u32,
                            instruction: Instructions::InitializeMint as i32,
                        })
                    }

                    // -- InitializeMint2 --
                    TokenInstruction::InitializeMint2 {
                        decimals,
                        mint_authority,
                        freeze_authority,
                    } => {
                        // accounts
                        let mint = instruction.accounts()[0].0.to_vec();

                        events.initialize_mints.push(InitializeMint {
                            // transaction
                            tx_hash,

                            // indexes
                            execution_index,
                            instruction_index,
                            inner_instruction_index,

                            // instruction
                            program_id,
                            stack_height,

                            // event
                            mint,
                            mint_authority: mint_authority.to_bytes().to_vec(),
                            freeze_authority: match freeze_authority {
                                COption::Some(key) => Some(key.to_bytes().to_vec()),
                                COption::None => None,
                            },
                            decimals: decimals as u32,
                            instruction: Instructions::InitializeMint2 as i32,
                        })
                    }

                    // -- InitializeAccount --
                    TokenInstruction::InitializeAccount {} => {
                        // accounts
                        let account: Vec<u8> = instruction.accounts()[0].0.to_vec(); // The account to initialize.
                        let mint: Vec<u8> = instruction.accounts()[1].0.to_vec(); // The mint this account will be associated with.
                        let owner: Vec<u8> = instruction.accounts()[2].0.to_vec(); // The new account's owner/multisignature.

                        events.initialize_accounts.push(InitializeAccount {
                            // transaction
                            tx_hash,

                            // indexes
                            execution_index,
                            instruction_index,
                            inner_instruction_index,

                            // instruction
                            program_id,
                            stack_height,

                            // event
                            account,
                            mint,
                            owner,
                        })
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(events)
}
