use proto::pb::solana::spl::token::v1::{Approve, Events, InitializeAccount, InitializeMint, Instructions, Revoke, Transfer};
use substreams::{errors::Error, log};
use substreams_solana::{block_view::InstructionView, pb::sf::solana::r#type::v1::Block};
use substreams_solana_program_instructions::{option::COption, token_instruction_2022::TokenInstruction};

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
    for tx in block.transactions() {
        // position of the current instruction inside its transaction (root + inner), counted from the start of that transaction.
        let mut instruction_index = 0;
        // position of the instruction among inner (non-root) instructions within the same transaction; stays 0 for root instructions and increments only for nested ones.
        let mut inner_instruction_index = 0;

        // instructions
        // Iterates over all instructions, including inner instructions, of the transaction.
        // The iteration starts with the first compiled instruction and then goes through all its inner instructions, if any.
        // Then it moves to the next compiled instruction and so on recursively.
        for instruction in tx.walk_instructions() {
            // increment indexes
            // apply to all instructions to ensure consistent indexing across the block.
            execution_index += 1;
            instruction_index += 1;
            if !instruction.is_root() {
                inner_instruction_index += 1;
            }

            // only include SPL-Token instructions
            if !is_spl_token_program(&instruction) {
                continue;
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
                            let mint = instruction.accounts()[1].0.to_vec();
                            let destination = instruction.accounts()[2].0.to_vec();
                            let authority = instruction.accounts()[3].0.to_vec();
                            let multisig_authority = instruction.accounts()[4..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

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
                                instruction: Instructions::TransferChecked as i32,

                                // authority
                                authority,
                                multisig_authority: multisig_authority.to_vec(),

                                // event
                                source,
                                destination,
                                amount,
                                mint: Some(mint),
                                decimals: Some(decimals as u32),
                            })
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
                                instruction: Instructions::Transfer as i32,

                                // authority
                                authority,
                                multisig_authority: multisig_authority.to_vec(),

                                // event
                                source,
                                destination,
                                amount,
                                decimals: None,
                                mint: None,
                            })
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
                                instruction: Instructions::MintTo as i32,

                                // authority
                                authority,
                                multisig_authority: multisig_authority.to_vec(),

                                // event
                                source: mint.to_vec(),
                                destination,
                                amount,
                                decimals: None,
                                mint: Some(mint),
                            })
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
                                instruction: Instructions::MintToChecked as i32,

                                // authority
                                authority,
                                multisig_authority: multisig_authority.to_vec(),

                                // event
                                source: mint.to_vec(),
                                destination,
                                amount,
                                decimals: Some(decimals as u32),
                                mint: Some(mint),
                            })
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
                                instruction: Instructions::Burn as i32,

                                // authority
                                authority,
                                multisig_authority: multisig_authority.to_vec(),

                                // event
                                source,
                                destination: mint.to_vec(),
                                amount,
                                decimals: None,
                                mint: Some(mint),
                            })
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
                                instruction: Instructions::BurnChecked as i32,

                                // authority
                                authority,
                                multisig_authority: multisig_authority.to_vec(),

                                // event
                                source,
                                destination: mint.to_vec(),
                                amount,
                                decimals: Some(decimals as u32),
                                mint: Some(mint),
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
                            instruction: Instructions::InitializeMint as i32,

                            // event
                            mint,
                            mint_authority: mint_authority.to_bytes().to_vec(),
                            freeze_authority: match freeze_authority {
                                COption::Some(key) => Some(key.to_bytes().to_vec()),
                                COption::None => None,
                            },
                            decimals: decimals as u32,
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
                            instruction: Instructions::InitializeMint2 as i32,

                            // event
                            mint,
                            mint_authority: mint_authority.to_bytes().to_vec(),
                            freeze_authority: match freeze_authority {
                                COption::Some(key) => Some(key.to_bytes().to_vec()),
                                COption::None => None,
                            },
                            decimals: decimals as u32,
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
                            instruction: Instructions::InitializeAccount as i32,

                            // event
                            account,
                            mint,
                            owner,
                        })
                    }
                    // -- InitializeAccount2 --
                    TokenInstruction::InitializeAccount2 { owner } => {
                        // accounts
                        let account: Vec<u8> = instruction.accounts()[0].0.to_vec(); // The account to initialize.
                        let mint: Vec<u8> = instruction.accounts()[1].0.to_vec(); // The mint this account will be associated with.

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
                            instruction: Instructions::InitializeAccount2 as i32,

                            // event
                            account,
                            mint,
                            owner: owner.to_bytes().to_vec(),
                        })
                    }
                    // -- InitializeAccount3 --
                    TokenInstruction::InitializeAccount3 { owner } => {
                        // accounts
                        let account: Vec<u8> = instruction.accounts()[0].0.to_vec(); // The account to initialize.
                        let mint: Vec<u8> = instruction.accounts()[1].0.to_vec(); // The mint this account will be associated with.

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
                            instruction: Instructions::InitializeAccount3 as i32,

                            // event
                            account,
                            mint,
                            owner: owner.to_bytes().to_vec(),
                        })
                    }
                    // -- Approve --
                    TokenInstruction::Approve { amount } => {
                        // accounts
                        let source = instruction.accounts()[0].0.to_vec();
                        let delegate = instruction.accounts()[1].0.to_vec();
                        let authority = instruction.accounts()[2].0.to_vec();
                        let multisig_authority = instruction.accounts()[3..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                        events.approves.push(Approve {
                            // transaction
                            tx_hash,

                            // indexes
                            execution_index,
                            instruction_index,
                            inner_instruction_index,

                            // instruction
                            program_id,
                            stack_height,
                            instruction: Instructions::Approve as i32,

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
                        })
                    }
                    // -- ApproveChecked --
                    TokenInstruction::ApproveChecked { amount, decimals } => {
                        // accounts
                        let source = instruction.accounts()[0].0.to_vec();
                        let mint = instruction.accounts()[1].0.to_vec();
                        let delegate = instruction.accounts()[2].0.to_vec();
                        let authority = instruction.accounts()[3].0.to_vec();
                        let multisig_authority = instruction.accounts()[4..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                        events.approves.push(Approve {
                            // transaction
                            tx_hash,

                            // indexes
                            execution_index,
                            instruction_index,
                            inner_instruction_index,

                            // instruction
                            program_id,
                            stack_height,
                            instruction: Instructions::ApproveChecked as i32,

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
                        })
                    }

                    // -- Revoke --
                    TokenInstruction::Revoke {} => {
                        // accounts
                        let source = instruction.accounts()[0].0.to_vec();
                        let authority = instruction.accounts()[1].0.to_vec();
                        let multisig_authority = instruction.accounts()[2..].iter().map(|a| a.0.to_vec()).collect::<Vec<_>>();

                        events.revokes.push(Revoke {
                            // transaction
                            tx_hash,

                            // indexes
                            execution_index,
                            instruction_index,
                            inner_instruction_index,

                            // instruction
                            program_id,
                            stack_height,
                            instruction: Instructions::Revoke as i32,

                            // authority
                            authority: authority.to_vec(),
                            multisig_authority: multisig_authority.to_vec(),

                            // event
                            source,
                            owner: authority,
                        })
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(events)
}
