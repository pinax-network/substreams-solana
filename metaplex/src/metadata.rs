use borsh::BorshDeserialize;
use mpl_token_metadata::instructions::{CreateMetadataAccountV3InstructionArgs, UpdateMetadataAccountV2InstructionArgs};
use mpl_token_metadata::types::{Data, DataV2};
use proto::pb::solana::metaplex::token_metadata::v1 as pb;
use substreams_solana::block_view::InstructionView;

use crate::is_metaplex_program;

pub fn unpack_metadata(instruction: &InstructionView, program_id: &[u8]) -> Option<pb::instruction::Instruction> {
    if !is_metaplex_program(program_id) {
        return None;
    }

    let data = instruction.data();
    let (discriminator, mut rest) = data.split_first()?;

    match discriminator {
        0 => {
            #[derive(BorshDeserialize)]
            struct Args {
                data: Data,
                is_mutable: bool,
            }
            let args: Args = Args::deserialize(&mut rest).ok()?;
            Some(pb::instruction::Instruction::CreateMetadataAccountV1(pb::CreateMetadataAccountV1 {
                metadata: instruction.accounts().get(0)?.0.to_vec(),
                mint: instruction.accounts().get(1)?.0.to_vec(),
                mint_authority: instruction.accounts().get(2)?.0.to_vec(),
                payer: instruction.accounts().get(3)?.0.to_vec(),
                update_authority: instruction.accounts().get(4)?.0.to_vec(),
                name: args.data.name,
                symbol: args.data.symbol,
                uri: args.data.uri,
            }))
        }
        16 => {
            #[derive(BorshDeserialize)]
            struct Args {
                data: DataV2,
                is_mutable: bool,
            }
            let args: Args = Args::deserialize(&mut rest).ok()?;
            Some(pb::instruction::Instruction::CreateMetadataAccountV2(pb::CreateMetadataAccountV2 {
                metadata: instruction.accounts().get(0)?.0.to_vec(),
                mint: instruction.accounts().get(1)?.0.to_vec(),
                mint_authority: instruction.accounts().get(2)?.0.to_vec(),
                payer: instruction.accounts().get(3)?.0.to_vec(),
                update_authority: instruction.accounts().get(4)?.0.to_vec(),
                name: args.data.name,
                symbol: args.data.symbol,
                uri: args.data.uri,
            }))
        }
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
        1 => {
            #[derive(BorshDeserialize)]
            struct Args {
                data: Option<Data>,
                update_authority: Option<[u8; 32]>,
                primary_sale_happened: Option<bool>,
            }
            let args: Args = Args::deserialize(&mut rest).ok()?;
            let (name, symbol, uri) = if let Some(data) = args.data {
                (Some(data.name), Some(data.symbol), Some(data.uri))
            } else {
                (None, None, None)
            };
            Some(pb::instruction::Instruction::UpdateMetadataAccountV1(pb::UpdateMetadataAccountV1 {
                metadata: instruction.accounts().get(0)?.0.to_vec(),
                update_authority: instruction.accounts().get(1)?.0.to_vec(),
                name,
                symbol,
                uri,
            }))
        }
        15 => {
            let args = UpdateMetadataAccountV2InstructionArgs::deserialize(&mut rest).ok()?;
            let (name, symbol, uri) = if let Some(data) = args.data {
                (Some(data.name), Some(data.symbol), Some(data.uri))
            } else {
                (None, None, None)
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
