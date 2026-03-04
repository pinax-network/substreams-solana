use proto::pb::solana::native::vote::v1 as pb;
use solana_program::vote::instruction::VoteInstruction;

use bincode::config;
use substreams_solana::block_view::InstructionView;

pub fn unpack_instruction(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();

    let (vote_ix, _): (VoteInstruction, usize) = bincode::serde::decode_from_slice(instruction.data(), cfg).ok()?;

    match vote_ix {
        VoteInstruction::InitializeAccount(vote_init) => {
            let accounts = instruction.accounts();
            if accounts.is_empty() {
                return None;
            }

            Some(pb::instruction::Instruction::InitializeAccount(pb::InitializeAccount {
                vote_account: accounts[0].0.to_vec(),
                node_pubkey: vote_init.node_pubkey.to_bytes().to_vec(),
                authorized_voter: vote_init.authorized_voter.to_bytes().to_vec(),
                authorized_withdrawer: vote_init.authorized_withdrawer.to_bytes().to_vec(),
                commission: vote_init.commission as u32,
            }))
        }
        VoteInstruction::Withdraw(lamports) => {
            let accounts = instruction.accounts();
            if accounts.len() < 2 {
                return None;
            }

            Some(pb::instruction::Instruction::Withdraw(pb::Withdraw {
                vote_account: accounts[0].0.to_vec(),
                destination: accounts[1].0.to_vec(),
                lamports,
                withdraw_authority: accounts.get(2).map_or(Vec::new(), |a| a.0.to_vec()),
            }))
        }
        VoteInstruction::UpdateCommission(commission) => {
            let accounts = instruction.accounts();
            if accounts.is_empty() {
                return None;
            }

            Some(pb::instruction::Instruction::UpdateCommission(pb::UpdateCommission {
                vote_account: accounts[0].0.to_vec(),
                commission: commission as u32,
                authorized_withdrawer: accounts.get(1).map_or(Vec::new(), |a| a.0.to_vec()),
            }))
        }
        VoteInstruction::UpdateValidatorIdentity => {
            let accounts = instruction.accounts();
            if accounts.len() < 2 {
                return None;
            }

            Some(pb::instruction::Instruction::UpdateValidatorIdentity(pb::UpdateValidatorIdentity {
                vote_account: accounts[0].0.to_vec(),
                node_pubkey: accounts[1].0.to_vec(),
                authorized_withdrawer: accounts.get(2).map_or(Vec::new(), |a| a.0.to_vec()),
            }))
        }
        _ => None,
    }
}
