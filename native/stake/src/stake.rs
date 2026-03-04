use proto::pb::solana::native::stake::v1 as pb;
use solana_program::stake::instruction::StakeInstruction;

use bincode::config;
use substreams_solana::block_view::InstructionView;

pub fn unpack_instruction(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();

    let (stake_ix, _): (StakeInstruction, usize) = bincode::serde::decode_from_slice(instruction.data(), cfg).ok()?;

    match stake_ix {
        StakeInstruction::Initialize(authorized, lockup) => {
            let accounts = instruction.accounts();
            if accounts.is_empty() {
                return None;
            }

            Some(pb::instruction::Instruction::Initialize(pb::Initialize {
                stake_account: accounts[0].0.to_vec(),
                staker: authorized.staker.to_bytes().to_vec(),
                withdrawer: authorized.withdrawer.to_bytes().to_vec(),
                lockup_unix_timestamp: if lockup.unix_timestamp != 0 {
                    Some(lockup.unix_timestamp)
                } else {
                    None
                },
                lockup_epoch: if lockup.epoch != 0 { Some(lockup.epoch) } else { None },
                lockup_custodian: if lockup.custodian != solana_program::pubkey::Pubkey::default() {
                    Some(lockup.custodian.to_bytes().to_vec())
                } else {
                    None
                },
            }))
        }
        StakeInstruction::DelegateStake => {
            let accounts = instruction.accounts();
            if accounts.len() < 2 {
                return None;
            }

            Some(pb::instruction::Instruction::Delegate(pb::Delegate {
                stake_account: accounts[0].0.to_vec(),
                vote_account: accounts[1].0.to_vec(),
                stake_authority: accounts.get(5).map_or(Vec::new(), |a| a.0.to_vec()),
            }))
        }
        StakeInstruction::Deactivate => {
            let accounts = instruction.accounts();
            if accounts.is_empty() {
                return None;
            }

            Some(pb::instruction::Instruction::Deactivate(pb::Deactivate {
                stake_account: accounts[0].0.to_vec(),
                stake_authority: accounts.get(2).map_or(Vec::new(), |a| a.0.to_vec()),
            }))
        }
        StakeInstruction::Withdraw(lamports) => {
            let accounts = instruction.accounts();
            if accounts.len() < 2 {
                return None;
            }

            Some(pb::instruction::Instruction::Withdraw(pb::Withdraw {
                stake_account: accounts[0].0.to_vec(),
                destination: accounts[1].0.to_vec(),
                lamports,
                withdraw_authority: accounts.get(4).map_or(Vec::new(), |a| a.0.to_vec()),
                custodian: accounts.get(5).map(|a| a.0.to_vec()),
            }))
        }
        StakeInstruction::Merge => {
            let accounts = instruction.accounts();
            if accounts.len() < 2 {
                return None;
            }

            Some(pb::instruction::Instruction::Merge(pb::Merge {
                destination_stake_account: accounts[0].0.to_vec(),
                source_stake_account: accounts[1].0.to_vec(),
                stake_authority: accounts.get(4).map_or(Vec::new(), |a| a.0.to_vec()),
            }))
        }
        StakeInstruction::Split(lamports) => {
            let accounts = instruction.accounts();
            if accounts.len() < 2 {
                return None;
            }

            Some(pb::instruction::Instruction::Split(pb::Split {
                stake_account: accounts[0].0.to_vec(),
                split_stake_account: accounts[1].0.to_vec(),
                lamports,
                stake_authority: accounts.get(2).map_or(Vec::new(), |a| a.0.to_vec()),
            }))
        }
        _ => None,
    }
}
