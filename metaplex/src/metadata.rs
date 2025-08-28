use borsh::BorshDeserialize;
use mpl_token_metadata::instructions::{CreateMetadataAccountV3InstructionArgs, UpdateMetadataAccountV2InstructionArgs};
use proto::pb::solana::metaplex::v1 as pb;
use substreams_solana::block_view::InstructionView;

use crate::is_metaplex_program;

pub fn unpack_metadata(instruction: &InstructionView, program_id: &[u8]) -> Option<pb::instruction::Instruction> {
    if !is_metaplex_program(program_id) {
        return None;
    }

    let data = instruction.data();
    let (discriminator, mut rest) = data.split_first()?;

    match discriminator {
        33 => {
            let args = CreateMetadataAccountV3InstructionArgs::deserialize(&mut rest).ok()?;
            let data = args.data;
            Some(pb::instruction::Instruction::CreateMetadataAccountV3(pb::CreateMetadataAccountV3 {
                metadata: instruction.accounts().get(0)?.0.to_vec(),
                mint: instruction.accounts().get(1)?.0.to_vec(),
                mint_authority: instruction.accounts().get(2)?.0.to_vec(),
                payer: instruction.accounts().get(3)?.0.to_vec(),
                update_authority: instruction.accounts().get(4)?.0.to_vec(),
                name: data.name,
                symbol: data.symbol,
                uri: data.uri,
            }))
        }
        15 => {
            let args = UpdateMetadataAccountV2InstructionArgs::deserialize(&mut rest).ok()?;
            let (name, symbol, uri) = if let Some(data) = args.data {
                (data.name, data.symbol, data.uri)
            } else {
                (String::new(), String::new(), String::new())
            };
            Some(pb::instruction::Instruction::UpdateMetadataAccountV2(pb::UpdateMetadataAccountV2 {
                metadata: instruction.accounts().get(0)?.0.to_vec(),
                update_authority: instruction.accounts().get(1)?.0.to_vec(),
                name,
                symbol,
                uri,
            }))
        }
        _ => None,
    }
}
