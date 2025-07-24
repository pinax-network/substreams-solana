use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::block_view::InstructionView;
use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

pub fn unpack_memo(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match TokenInstruction::unpack(&instruction.data()) {
        Err(_err) => return None,
        Ok(token_instruction) => match token_instruction {
            // -- MemoTransferExtension --
            TokenInstruction::MemoTransferExtension => {
                return Some(pb::instruction::Instruction::MemoTransferExtension(pb::MemoTransferExtension {
                    data: instruction.data().to_vec(),
                }));
            }
            // -- MetadataPointerExtension --
            TokenInstruction::MetadataPointerExtension => {
                return Some(pb::instruction::Instruction::MetadataPointerExtension(pb::MetadataPointerExtension {
                    data: instruction.data().to_vec(),
                }));
            }
            // // -- InitializeTokenMetadata --
            // TokenInstruction::InitializeTokenMetadata => {
            //     return Some(pb::instruction::Instruction::InitializeTokenMetadata(pb::InitializeTokenMetadata {
            //         data: instruction.data().to_vec(),
            //     }));
            // }
            _ => None,
        },
    }
}
