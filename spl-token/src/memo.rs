use crate::is_spl_token_memo_program;
use proto::pb::solana::spl::token::v1 as pb;
use std::str::from_utf8;
use substreams_solana::{base58, block_view::InstructionView};

pub fn unpack_memo(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    // Check if the instruction is from a Memo program
    if is_spl_token_memo_program(&base58::encode(instruction.program_id().0)) {
        let memo = from_utf8(instruction.data()).ok();
        if let Some(memo) = memo {
            // Create a Memo instruction
            return Some(pb::instruction::Instruction::Memo(pb::Memo {
                data: instruction.data().to_vec(),
                memo: memo.to_string(),
            }));
        }
    }
    None
}
