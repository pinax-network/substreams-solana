use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

pub fn unpack_extensions(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match TokenInstruction::unpack(&instruction.data()) {
        Err(_err) => None,
        Ok(token_instruction) => match token_instruction {
            // -- MemoTransferExtension --
            TokenInstruction::MemoTransferExtension => {
                return Some(pb::instruction::Instruction::MemoTransferExtension(pb::MemoTransferExtension {
                    data: instruction.data().to_vec(),
                }));
            }
            // -- MetadataPointerExtension --
            TokenInstruction::MetadataPointerExtension => {
                // -- accounts --
                let metadata_address = instruction.accounts()[0].0.to_vec();

                return Some(pb::instruction::Instruction::InitializeMetadataPointer(pb::InitializeMetadataPointer {
                    metadata_address: metadata_address.to_vec(),
                    mint: metadata_address.to_vec(),      // TO-DO: not implemented yet
                    authority: metadata_address.to_vec(), // TO-DO: not implemented yet
                }));
            }
            _ => None,
        },
    }
}
