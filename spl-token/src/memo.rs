use crate::is_spl_memo_program;
use proto::pb::solana::spl::token::v1 as pb;
use std::str::from_utf8;
use substreams_solana::block_view::InstructionView;

pub fn unpack_memo(instruction: &InstructionView, program_id: &[u8]) -> Option<pb::instruction::Instruction> {
    if !is_spl_memo_program(&program_id) {
        return None;
    }

    // Check if the instruction is from a Memo program
    let memo = from_utf8(instruction.data());
    if let Ok(memo) = memo {
        // Create a Memo instruction
        return Some(pb::instruction::Instruction::Memo(pb::Memo {
            data: instruction.data().to_vec(),
            memo: memo.to_string(),
        }));
    }
    None
}
