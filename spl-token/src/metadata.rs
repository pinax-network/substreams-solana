use proto::pb::solana::spl::token::v1 as pb;
use spl_token_metadata_interface::{instruction::TokenMetadataInstruction, state::Field};
use substreams_solana::block_view::InstructionView;

use crate::is_spl_token_program;

pub fn unpack_metadata(instruction: &InstructionView, program_id: &str) -> Option<pb::instruction::Instruction> {
    if !is_spl_token_program(&program_id) {
        return None;
    }
    match TokenMetadataInstruction::unpack(&instruction.data()) {
        Err(_err) => return None,
        Ok(token_instruction) => match token_instruction {
            TokenMetadataInstruction::Initialize { 0: data } => {
                // -- accounts --
                let metadata = instruction.accounts()[0].0.to_vec();
                let update_authority = instruction.accounts()[1].0.to_vec();
                let mint = instruction.accounts()[2].0.to_vec();
                let mint_authority = instruction.accounts()[3].0.to_vec();

                return Some(pb::instruction::Instruction::InitializeTokenMetadata(pb::InitializeTokenMetadata {
                    // accounts
                    metadata,
                    update_authority,
                    mint,
                    mint_authority,
                    // instruction data
                    name: data.name,
                    symbol: data.symbol,
                    uri: data.uri,
                }));
            }
            TokenMetadataInstruction::UpdateAuthority { 0: data } => {
                // -- accounts --
                let metadata = instruction.accounts()[0].0.to_vec();
                let update_authority = instruction.accounts()[1].0.to_vec();

                return Some(pb::instruction::Instruction::UpdateTokenMetadataAuthority(pb::UpdateTokenMetadataAuthority {
                    // accounts
                    metadata,
                    update_authority,
                    new_authority: data.new_authority.0.to_bytes().to_vec(),
                }));
            }
            TokenMetadataInstruction::UpdateField { 0: data } => {
                // -- accounts --
                let metadata = instruction.accounts()[0].0.to_vec();
                let update_authority = instruction.accounts()[1].0.to_vec();
                let field = match data.field {
                    Field::Name => "name".to_string(),
                    Field::Symbol => "symbol".to_string(),
                    Field::Uri => "uri".to_string(),
                    Field::Key(key) => key,
                };

                return Some(pb::instruction::Instruction::UpdateTokenMetadataField(pb::UpdateTokenMetadataField {
                    // accounts
                    metadata,
                    update_authority,
                    // instruction data
                    field,
                    value: data.value,
                }));
            }
            TokenMetadataInstruction::RemoveKey { 0: data } => {
                // -- accounts --
                let metadata = instruction.accounts()[0].0.to_vec();
                let update_authority = instruction.accounts()[1].0.to_vec();

                return Some(pb::instruction::Instruction::RemoveTokenMetadataField(pb::RemoveTokenMetadataField {
                    // accounts
                    metadata,
                    update_authority,
                    // instruction data
                    idempotent: data.idempotent,
                    key: data.key,
                }));
            }
            _ => None,
        },
    }
}
