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
                    // -- InitializeMint --
                    TokenInstruction::InitializeMint {
                        decimals,
                        mint_authority,
                        freeze_authority,
                    } => {
                        // accounts
                        let mint = instruction.accounts()[0].0.to_vec();
                        base.instruction = Some(pb::instruction::Instruction::InitializeMint(pb::InitializeMint {
                            mint,
                            mint_authority: mint_authority.to_bytes().to_vec(),
                            freeze_authority: match freeze_authority {
                                COption::Some(key) => Some(key.to_bytes().to_vec()),
                                COption::None => None,
                            },
                            decimals: decimals as u32,
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    // -- InitializeMint2 --
                    TokenInstruction::InitializeMint2 {
                        decimals,
                        mint_authority,
                        freeze_authority,
                    } => {
                        // accounts
                        let mint = instruction.accounts()[0].0.to_vec();

                        base.instruction = Some(pb::instruction::Instruction::InitializeMint(pb::InitializeMint {
                            mint,
                            mint_authority: mint_authority.to_bytes().to_vec(),
                            freeze_authority: match freeze_authority {
                                COption::Some(key) => Some(key.to_bytes().to_vec()),
                                COption::None => None,
                            },
                            decimals: decimals as u32,
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
