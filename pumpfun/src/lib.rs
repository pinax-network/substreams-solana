use common::solana::{get_fee_payer, get_signers};
use proto::pb::pumpfun::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::pumpfun::bonding_curve as pumpfun;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    if instructions.is_empty() {
        return None;
    }
    Some(pb::Transaction {
        fee: tx.meta.as_ref()?.fee,
        compute_units_consumed: tx.meta.as_ref()?.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
    })
}

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;

    if program_id != &pumpfun::PROGRAM_ID.to_vec() {
        return None;
    }

    process_instruction1(instruction)
        .or_else(|| process_instruction2(instruction))
        .map(|parsed_instruction| pb::Instruction {
            program_id: program_id.to_vec(),
            stack_height: instruction.stack_height(),
            instruction: Some(parsed_instruction),
        })
}

fn process_instruction1(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match pumpfun::instructions::unpack(instruction.data()) {
        Ok(pumpfun::instructions::PumpFunInstruction::Buy(event)) => Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
            accounts: Some(get_trade_accounts(instruction)),
            amount: event.amount,
            max_sol_cost: event.max_sol_cost,
        })),
        Ok(pumpfun::instructions::PumpFunInstruction::Sell(event)) => Some(pb::instruction::Instruction::Sell(pb::SellInstruction {
            accounts: Some(get_trade_accounts(instruction)),
            amount: event.amount,
            min_sol_output: event.min_sol_output,
        })),
        Ok(pumpfun::instructions::PumpFunInstruction::Create(event)) => Some(pb::instruction::Instruction::Create(pb::CreateInstruction {
            name: event.name,
            symbol: event.symbol,
            uri: event.uri,
            creator: event.creator.to_bytes().to_vec(),
        })),
        Ok(pumpfun::instructions::PumpFunInstruction::Initialize) => Some(pb::instruction::Instruction::Initialize(pb::InitializeInstruction {})),
        Ok(pumpfun::instructions::PumpFunInstruction::Withdraw) => Some(pb::instruction::Instruction::Withdraw(pb::WithdrawInstruction {})),
        Ok(pumpfun::instructions::PumpFunInstruction::SetParams(event)) => Some(pb::instruction::Instruction::SetParams(pb::SetParamsInstruction {
            fee_recipient: event.fee_recipient.to_bytes().to_vec(),
            initial_virtual_token_reserves: event.initial_virtual_token_reserves,
            initial_virtual_sol_reserves: event.initial_virtual_sol_reserves,
            initial_real_token_reserves: event.initial_real_token_reserves,
            token_total_supply: event.token_total_supply,
            fee_basis_points: event.fee_basis_points,
        })),
        _ => None,
    }
}

fn process_instruction2(instruction: &InstructionView) -> Option<pb::instruction::Instruction> {
    match pumpfun::anchor_self_cpi::unpack(instruction.data()) {
        Ok(pumpfun::anchor_self_cpi::PumpFunEvent::TradeV0(event)) => Some(pb::instruction::Instruction::Trade(pb::TradeEvent {
            mint: event.mint.to_bytes().to_vec(),
            sol_amount: event.sol_amount,
            token_amount: event.token_amount,
            is_buy: event.is_buy,
            user: event.user.to_bytes().to_vec(),
            timestamp: event.timestamp,
            virtual_sol_reserves: event.virtual_sol_reserves,
            virtual_token_reserves: event.virtual_token_reserves,
            real_sol_reserves: None,
            real_token_reserves: None,
            fee_recipient: None,
            fee_basis_points: None,
            fee: None,
            creator: None,
            creator_fee_basis_points: None,
            creator_fee: None,
        })),
        Ok(pumpfun::anchor_self_cpi::PumpFunEvent::TradeV1(event)) => Some(pb::instruction::Instruction::Trade(pb::TradeEvent {
            mint: event.mint.to_bytes().to_vec(),
            sol_amount: event.sol_amount,
            token_amount: event.token_amount,
            is_buy: event.is_buy,
            user: event.user.to_bytes().to_vec(),
            timestamp: event.timestamp,
            virtual_sol_reserves: event.virtual_sol_reserves,
            virtual_token_reserves: event.virtual_token_reserves,
            real_sol_reserves: Some(event.real_sol_reserves),
            real_token_reserves: Some(event.real_token_reserves),
            fee_recipient: None,
            fee_basis_points: None,
            fee: None,
            creator: None,
            creator_fee_basis_points: None,
            creator_fee: None,
        })),
        Ok(pumpfun::anchor_self_cpi::PumpFunEvent::TradeV2(event)) => Some(pb::instruction::Instruction::Trade(pb::TradeEvent {
            mint: event.mint.to_bytes().to_vec(),
            sol_amount: event.sol_amount,
            token_amount: event.token_amount,
            is_buy: event.is_buy,
            user: event.user.to_bytes().to_vec(),
            timestamp: event.timestamp,
            virtual_sol_reserves: event.virtual_sol_reserves,
            virtual_token_reserves: event.virtual_token_reserves,
            real_sol_reserves: Some(event.real_sol_reserves),
            real_token_reserves: Some(event.real_token_reserves),
            fee_recipient: Some(event.fee_recipient.to_bytes().to_vec()),
            fee_basis_points: Some(event.fee_basis_points),
            fee: Some(event.fee),
            creator: Some(event.creator.to_bytes().to_vec()),
            creator_fee_basis_points: Some(event.creator_fee_basis_points),
            creator_fee: Some(event.creator_fee),
        })),
        _ => None,
    }
}

pub fn get_trade_accounts(instruction: &InstructionView) -> pb::TradeAccounts {
    pb::TradeAccounts {
        global: instruction.accounts()[0].0.to_vec(),
        fee_recipient: instruction.accounts()[1].0.to_vec(),
        mint: instruction.accounts()[2].0.to_vec(),
        bonding_curve: instruction.accounts()[3].0.to_vec(),
        associated_bonding_curve: instruction.accounts()[4].0.to_vec(),
        associated_user: instruction.accounts()[5].0.to_vec(),
        user: instruction.accounts()[6].0.to_vec(),
        creator_vault: instruction.accounts()[9].0.to_vec(),
    }
}
