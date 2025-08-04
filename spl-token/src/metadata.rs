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
                return Some(pb::instruction::Instruction::InitializeTokenMetadata(pb::InitializeTokenMetadata {
                    // accounts
                    metadata: instruction.accounts()[0].0.to_vec(),
                    update_authority: instruction.accounts()[1].0.to_vec(),
                    mint: instruction.accounts()[2].0.to_vec(),
                    mint_authority: instruction.accounts()[3].0.to_vec(),
                    // instruction data
                    name: data.name,
                    symbol: data.symbol,
                    uri: data.uri,
                }));
            }
            TokenMetadataInstruction::UpdateAuthority { 0: data } => {
                return Some(pb::instruction::Instruction::UpdateTokenMetadataAuthority(pb::UpdateTokenMetadataAuthority {
                    // accounts
                    metadata: instruction.accounts()[0].0.to_vec(),
                    update_authority: instruction.accounts()[1].0.to_vec(),
                    new_authority: data.new_authority.0.to_bytes().to_vec(),
                }));
            }
            TokenMetadataInstruction::UpdateField { 0: data } => {
                let field = match data.field {
                    Field::Name => "name".to_string(),
                    Field::Symbol => "symbol".to_string(),
                    Field::Uri => "uri".to_string(),
                    Field::Key(key) => key,
                };

                return Some(pb::instruction::Instruction::UpdateTokenMetadataField(pb::UpdateTokenMetadataField {
                    // accounts
                    metadata: instruction.accounts()[0].0.to_vec(),
                    update_authority: instruction.accounts()[1].0.to_vec(),
                    // instruction data
                    field,
                    value: data.value,
                }));
            }
            TokenMetadataInstruction::RemoveKey { 0: data } => {
                return Some(pb::instruction::Instruction::RemoveTokenMetadataField(pb::RemoveTokenMetadataField {
                    // accounts
                    metadata: instruction.accounts()[0].0.to_vec(),
                    update_authority: instruction.accounts()[1].0.to_vec(),
                    // instruction data
                    idempotent: data.idempotent,
                    key: data.key,
                }));
            }
            _ => None,
        },
    }
}
