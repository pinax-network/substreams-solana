use common::solana::{get_fee_payer, get_signers};
use proto::pb::solana::spl::token_lending::v1 as pb;
use substreams::errors::Error;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Token Lending Program (LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi)
pub const TOKEN_LENDING_PROGRAM: [u8; 32] = [
    5, 8, 194, 206, 177, 181, 208, 92, 135, 73, 128, 172, 82, 207, 101, 151, 64, 231, 233, 185, 53, 106, 175, 42, 3, 98, 103, 50, 99, 82, 108, 21,
];

pub fn is_token_lending_program(program_id: &[u8]) -> bool {
    program_id == &TOKEN_LENDING_PROGRAM
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<_> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();

    if instructions.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
    })
}

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;

    if !is_token_lending_program(program_id) {
        return None;
    }

    let data = instruction.data();
    if data.is_empty() {
        return None;
    }

    let accounts = instruction.accounts();

    let parsed_instruction = match data[0] {
        // InitLendingMarket
        0 => {
            if accounts.len() < 2 || data.len() < 65 {
                return None;
            }
            Some(pb::instruction::Instruction::InitLendingMarket(pb::InitLendingMarket {
                lending_market: accounts[0].0.to_vec(),
                owner: data[1..33].to_vec(),
                quote_currency: data[33..65].to_vec(),
            }))
        }
        // InitReserve
        2 => {
            if accounts.len() < 13 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::InitReserve(pb::InitReserve {
                lending_market: accounts[10].0.to_vec(),
                reserve: accounts[2].0.to_vec(),
                liquidity_mint: accounts[3].0.to_vec(),
                liquidity_supply: accounts[4].0.to_vec(),
                collateral_mint: accounts[6].0.to_vec(),
                collateral_supply: accounts[7].0.to_vec(),
                liquidity_amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        // DepositReserveLiquidity
        4 => {
            if accounts.len() < 7 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::DepositReserveLiquidity(pb::DepositReserveLiquidity {
                reserve: accounts[2].0.to_vec(),
                source_liquidity: accounts[0].0.to_vec(),
                destination_collateral: accounts[1].0.to_vec(),
                liquidity_amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        // RedeemReserveCollateral
        5 => {
            if accounts.len() < 7 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::RedeemReserveCollateral(pb::RedeemReserveCollateral {
                reserve: accounts[2].0.to_vec(),
                source_collateral: accounts[0].0.to_vec(),
                destination_liquidity: accounts[1].0.to_vec(),
                collateral_amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        // InitObligation
        6 => {
            if accounts.len() < 3 {
                return None;
            }
            Some(pb::instruction::Instruction::InitObligation(pb::InitObligation {
                obligation: accounts[0].0.to_vec(),
                lending_market: accounts[1].0.to_vec(),
                owner: accounts[2].0.to_vec(),
            }))
        }
        // DepositObligationCollateral
        8 => {
            if accounts.len() < 6 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::DepositObligationCollateral(pb::DepositObligationCollateral {
                obligation: accounts[3].0.to_vec(),
                source_collateral: accounts[0].0.to_vec(),
                reserve: accounts[2].0.to_vec(),
                collateral_amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        // WithdrawObligationCollateral
        9 => {
            if accounts.len() < 7 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::WithdrawObligationCollateral(pb::WithdrawObligationCollateral {
                obligation: accounts[3].0.to_vec(),
                destination_collateral: accounts[1].0.to_vec(),
                reserve: accounts[2].0.to_vec(),
                collateral_amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        // BorrowObligationLiquidity
        10 => {
            if accounts.len() < 8 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::BorrowObligationLiquidity(pb::BorrowObligationLiquidity {
                obligation: accounts[4].0.to_vec(),
                destination_liquidity: accounts[1].0.to_vec(),
                reserve: accounts[2].0.to_vec(),
                liquidity_amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        // RepayObligationLiquidity
        11 => {
            if accounts.len() < 6 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::RepayObligationLiquidity(pb::RepayObligationLiquidity {
                obligation: accounts[3].0.to_vec(),
                source_liquidity: accounts[0].0.to_vec(),
                reserve: accounts[2].0.to_vec(),
                liquidity_amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        // LiquidateObligation
        12 => {
            if accounts.len() < 10 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::LiquidateObligation(pb::LiquidateObligation {
                obligation: accounts[6].0.to_vec(),
                repay_reserve: accounts[2].0.to_vec(),
                withdraw_reserve: accounts[4].0.to_vec(),
                source_liquidity: accounts[0].0.to_vec(),
                destination_collateral: accounts[1].0.to_vec(),
                liquidity_amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        // FlashLoan
        13 => {
            if accounts.len() < 8 || data.len() < 9 {
                return None;
            }
            Some(pb::instruction::Instruction::FlashLoan(pb::FlashLoan {
                reserve: accounts[2].0.to_vec(),
                source_liquidity: accounts[0].0.to_vec(),
                destination_liquidity: accounts[1].0.to_vec(),
                amount: u64::from_le_bytes(data[1..9].try_into().ok()?),
            }))
        }
        _ => None,
    };

    parsed_instruction.map(|parsed| pb::Instruction {
        program_id: program_id.to_vec(),
        stack_height: instruction.stack_height(),
        is_root: instruction.is_root(),
        instruction: Some(parsed),
    })
}
