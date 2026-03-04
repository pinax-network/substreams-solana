use common::solana::{get_fee_payer, get_signers};
use proto::pb::tensor::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::tensor;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let mut instructions: Vec<pb::Instruction> = Vec::new();
    let mut logs: Vec<pb::Log> = Vec::new();

    for iview in tx.walk_instructions() {
        if let Some(result) = process_instruction(&iview) {
            match result {
                InstructionOrLog::Instruction(ix) => instructions.push(ix),
                InstructionOrLog::Log(log) => logs.push(log),
            }
        }
    }

    if instructions.is_empty() && logs.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
        logs,
    })
}

enum InstructionOrLog {
    Instruction(pb::Instruction),
    Log(pb::Log),
}

fn process_instruction(ix: &InstructionView) -> Option<InstructionOrLog> {
    let program_id = ix.program_id().0;

    if program_id != &tensor::PROGRAM_ID {
        return None;
    }

    match tensor::instructions::unpack(ix.data()) {
        // Tcomp noop CPI carries the event payload
        Ok(tensor::instructions::TensorInstruction::TcompNoop) => {
            match tensor::events::unpack(ix.data()) {
                Ok(tensor::events::TensorEvent::Maker(event)) => Some(InstructionOrLog::Log(pb::Log {
                    program_id: program_id.to_vec(),
                    invoke_depth: ix.stack_height(),
                    log: Some(pb::log::Log::Make(pb::MakeEvent {
                        maker: event.maker.to_bytes().to_vec(),
                        bid_id: event.bid_id.map(|b| b.to_bytes().to_vec()),
                        target: event.target as u32,
                        target_id: event.target_id.to_bytes().to_vec(),
                        amount: event.amount,
                        quantity: event.quantity,
                        currency: event.currency.map(|c| c.to_bytes().to_vec()),
                        expiry: event.expiry,
                        private_taker: event.private_taker.map(|p| p.to_bytes().to_vec()),
                        asset_id: event.asset_id.map(|a| a.to_bytes().to_vec()),
                    })),
                })),
                Ok(tensor::events::TensorEvent::Taker(event)) => Some(InstructionOrLog::Log(pb::Log {
                    program_id: program_id.to_vec(),
                    invoke_depth: ix.stack_height(),
                    log: Some(pb::log::Log::Take(pb::TakeEvent {
                        taker: event.taker.to_bytes().to_vec(),
                        bid_id: event.bid_id.map(|b| b.to_bytes().to_vec()),
                        target: event.target as u32,
                        target_id: event.target_id.to_bytes().to_vec(),
                        amount: event.amount,
                        quantity: event.quantity,
                        tcomp_fee: event.tcomp_fee,
                        taker_broker_fee: event.taker_broker_fee,
                        maker_broker_fee: event.maker_broker_fee,
                        creator_fee: event.creator_fee,
                        currency: event.currency.map(|c| c.to_bytes().to_vec()),
                        asset_id: event.asset_id.map(|a| a.to_bytes().to_vec()),
                    })),
                })),
                _ => None,
            }
        }
        Ok(tensor::instructions::TensorInstruction::List(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::List(pb::ListInstruction {
                    amount: event.amount,
                    expire_in_sec: event.expire_in_sec,
                    currency: event.currency.map(|c| c.to_bytes().to_vec()),
                    private_taker: event.private_taker.map(|p| p.to_bytes().to_vec()),
                    maker_broker: event.maker_broker.map(|m| m.to_bytes().to_vec()),
                    nft_standard: "cNFT".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::Delist) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Delist(pb::DelistInstruction {
                    nft_standard: "cNFT".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::Edit(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Edit(pb::EditInstruction {
                    amount: event.amount,
                    expire_in_sec: event.expire_in_sec,
                    currency: event.currency.map(|c| c.to_bytes().to_vec()),
                    private_taker: event.private_taker.map(|p| p.to_bytes().to_vec()),
                    maker_broker: event.maker_broker.map(|m| m.to_bytes().to_vec()),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::Buy(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    max_amount: event.max_amount,
                    nft_standard: "cNFT".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::BuySpl(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    max_amount: event.max_amount,
                    nft_standard: "cNFT_SPL".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::ListLegacy(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::List(pb::ListInstruction {
                    amount: event.amount,
                    expire_in_sec: event.expire_in_sec,
                    currency: event.currency.map(|c| c.to_bytes().to_vec()),
                    private_taker: event.private_taker.map(|p| p.to_bytes().to_vec()),
                    maker_broker: event.maker_broker.map(|m| m.to_bytes().to_vec()),
                    nft_standard: "pNFT".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::DelistLegacy) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Delist(pb::DelistInstruction {
                    nft_standard: "pNFT".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::BuyLegacy(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    max_amount: event.max_amount,
                    nft_standard: "pNFT".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::BuyLegacySpl(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    max_amount: event.max_amount,
                    nft_standard: "pNFT_SPL".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::ListT22(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::List(pb::ListInstruction {
                    amount: event.amount,
                    expire_in_sec: event.expire_in_sec,
                    currency: event.currency.map(|c| c.to_bytes().to_vec()),
                    private_taker: event.private_taker.map(|p| p.to_bytes().to_vec()),
                    maker_broker: event.maker_broker.map(|m| m.to_bytes().to_vec()),
                    nft_standard: "T22".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::DelistT22) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Delist(pb::DelistInstruction {
                    nft_standard: "T22".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::BuyT22(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    max_amount: event.max_amount,
                    nft_standard: "T22".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::ListWns(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::List(pb::ListInstruction {
                    amount: event.amount,
                    expire_in_sec: event.expire_in_sec,
                    currency: event.currency.map(|c| c.to_bytes().to_vec()),
                    private_taker: event.private_taker.map(|p| p.to_bytes().to_vec()),
                    maker_broker: event.maker_broker.map(|m| m.to_bytes().to_vec()),
                    nft_standard: "WNS".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::DelistWns) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Delist(pb::DelistInstruction {
                    nft_standard: "WNS".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::BuyWns(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    max_amount: event.max_amount,
                    nft_standard: "WNS".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::ListCore(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::List(pb::ListInstruction {
                    amount: event.amount,
                    expire_in_sec: event.expire_in_sec,
                    currency: event.currency.map(|c| c.to_bytes().to_vec()),
                    private_taker: event.private_taker.map(|p| p.to_bytes().to_vec()),
                    maker_broker: event.maker_broker.map(|m| m.to_bytes().to_vec()),
                    nft_standard: "Core".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::DelistCore) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Delist(pb::DelistInstruction {
                    nft_standard: "Core".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::BuyCore(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    max_amount: event.max_amount,
                    nft_standard: "Core".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::Bid(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Bid(pb::BidInstruction {
                    bid_id: event.bid_id.to_bytes().to_vec(),
                    target: event.target as u32,
                    target_id: event.target_id.to_bytes().to_vec(),
                    amount: event.amount,
                    quantity: event.quantity,
                    expire_in_sec: event.expire_in_sec,
                    currency: event.currency.map(|c| c.to_bytes().to_vec()),
                    private_taker: event.private_taker.map(|p| p.to_bytes().to_vec()),
                    maker_broker: event.maker_broker.map(|m| m.to_bytes().to_vec()),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::TakeBidMetaHash(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TakeBid(pb::TakeBidInstruction {
                    min_amount: event.min_amount,
                    nft_standard: "cNFT".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::TakeBidFullMeta(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TakeBid(pb::TakeBidInstruction {
                    min_amount: event.min_amount,
                    nft_standard: "cNFT_full".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::TakeBidLegacy(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TakeBid(pb::TakeBidInstruction {
                    min_amount: event.min_amount,
                    nft_standard: "pNFT".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::TakeBidT22(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TakeBid(pb::TakeBidInstruction {
                    min_amount: event.min_amount,
                    nft_standard: "T22".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::TakeBidWns(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TakeBid(pb::TakeBidInstruction {
                    min_amount: event.min_amount,
                    nft_standard: "WNS".to_string(),
                })),
            }))
        }
        Ok(tensor::instructions::TensorInstruction::TakeBidCore(event)) => {
            Some(InstructionOrLog::Instruction(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::TakeBid(pb::TakeBidInstruction {
                    min_amount: event.min_amount,
                    nft_standard: "Core".to_string(),
                })),
            }))
        }
        _ => None,
    }
}
