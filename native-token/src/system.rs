use proto::pb::solana::native::token::v1 as pb;
use solana_program::system_instruction::SystemInstruction;

use bincode::config;
use substreams_solana::block_view::InstructionView;

pub fn unpack_transfers(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    let cfg = config::standard()
        .with_fixed_int_encoding() // NOT variable‑int
        .with_little_endian();

    let (sys_ix, _): (SystemInstruction, usize) = bincode::serde::decode_from_slice(instruction.data(), cfg).ok()?;

    match sys_ix {
        SystemInstruction::Transfer { lamports } if lamports > 0 => {
            let source = instruction.accounts()[0].0.to_vec();
            let destination = instruction.accounts()[1].0.to_vec();

            Some(pb::instruction::Instruction::Transfer(pb::Transfer { source, destination, lamports }))
        }
        SystemInstruction::TransferWithSeed {
            lamports,
            from_owner,
            from_seed,
        } if lamports > 0 => {
            let source = instruction.accounts()[0].0.to_vec();
            let source_base = instruction.accounts()[1].0.to_vec();
            let destination = instruction.accounts()[2].0.to_vec();

            Some(pb::instruction::Instruction::TransferWithSeed(pb::TransferWithSeed {
                destination,
                lamports,
                source,
                source_owner: from_owner.to_bytes().to_vec(),
                source_base,
                source_seed: from_seed,
            }))
        }
        SystemInstruction::CreateAccount { space, owner, lamports } if lamports > 0 => {
            let source = instruction.accounts()[0].0.to_vec();
            let new_account = instruction.accounts()[1].0.to_vec();

            Some(pb::instruction::Instruction::CreateAccount(pb::CreateAccount {
                source,
                new_account,
                lamports,
                space,
                owner: owner.to_bytes().to_vec(),
            }))
        }
        SystemInstruction::CreateAccountWithSeed {
            base,
            seed,
            space,
            owner,
            lamports,
        } if lamports > 0 => {
            let source = instruction.accounts()[0].0.to_vec();
            let new_account = instruction.accounts()[1].0.to_vec();
            let base_account = instruction.accounts().get(2).map(|account| account.0.to_vec());

            Some(pb::instruction::Instruction::CreateAccountWithSeed(pb::CreateAccountWithSeed {
                source,
                new_account,
                lamports,
                space,
                owner: owner.to_bytes().to_vec(),
                base_account,
                base: base.to_bytes().to_vec(),
                seed,
            }))
        }
        SystemInstruction::WithdrawNonceAccount { 0: lamports } if lamports > 0 => {
            let accounts = instruction.accounts();

            let nonce_account = accounts[0].0.to_vec();
            let destination = accounts[1].0.to_vec();
            // If nonce_authority isn't specified (at index 4), use the nonce_account as the authority
            let nonce_authority = accounts.get(4).map_or(nonce_account.clone(), |account| account.0.to_vec());

            Some(pb::instruction::Instruction::WithdrawNonceAccount(pb::WithdrawNonceAccount {
                nonce_account,
                destination,
                lamports,
                nonce_authority,
            }))
        }
        _ => None,
    }
}
