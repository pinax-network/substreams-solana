use common::solana::{get_fee_payer, get_signers};
use proto::pb::meteora::dllm::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::meteora::dllm;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iv| process_instruction(&iv)).collect();
    let logs: Vec<pb::Log> = tx.walk_instructions().filter_map(|iv| process_event_instruction(&iv)).collect();

    if instructions.is_empty() && logs.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
        logs,
    })
}

fn process_instruction(ix: &InstructionView) -> Option<pb::Instruction> {
    let program_id = ix.program_id().0;
    if program_id != &dllm::PROGRAM_ID {
        return None;
    }

    match dllm::instructions::unpack(ix.data()) {
        Ok(dllm::instructions::MeteoraDllmInstruction::Swap(evt)) => {
            let accounts = dllm::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::SwapAccounts {
                        lb_pair: accounts.lb_pair.to_bytes().to_vec(),
                        bin_array_bitmap_extension: accounts.bin_array_bitmap_extension.map(|a| a.to_bytes().to_vec()).unwrap_or_default(),
                        reserve_x: accounts.reserve_x.to_bytes().to_vec(),
                        reserve_y: accounts.reserve_y.to_bytes().to_vec(),
                        user_token_in: accounts.user_token_in.to_bytes().to_vec(),
                        user_token_out: accounts.user_token_out.to_bytes().to_vec(),
                        token_x_mint: accounts.token_x_mint.to_bytes().to_vec(),
                        token_y_mint: accounts.token_y_mint.to_bytes().to_vec(),
                        oracle: accounts.oracle.to_bytes().to_vec(),
                        host_fee_in: accounts.host_fee_in.map(|a| a.to_bytes().to_vec()).unwrap_or_default(),
                        user: accounts.user.to_bytes().to_vec(),
                        token_x_program: accounts.token_x_program.to_bytes().to_vec(),
                        token_y_program: accounts.token_y_program.to_bytes().to_vec(),
                        event_authority: accounts.event_authority.to_bytes().to_vec(),
                        program: accounts.program.to_bytes().to_vec(),
                    }),
                    amount_in: evt.amount_in,
                    min_amount_out: evt.min_amount_out,
                })),
            })
        }
        _ => None,
    }
}

fn process_event_instruction(ix: &InstructionView) -> Option<pb::Log> {
    let program_id = ix.program_id().0;
    if program_id != &dllm::PROGRAM_ID {
        return None;
    }

    match dllm::events::unpack(ix.data()) {
        Ok(event) => Some(pb::Log {
            program_id: program_id.to_vec(),
            invoke_depth: ix.stack_height(),
            event: match event {
                dllm::events::MeteoraDllmEvent::AddLiquidity => pb::Event::AddLiquidity as i32,
                dllm::events::MeteoraDllmEvent::ClaimFee => pb::Event::ClaimFee as i32,
                dllm::events::MeteoraDllmEvent::ClaimFee2 => pb::Event::ClaimFee2 as i32,
                dllm::events::MeteoraDllmEvent::ClaimReward => pb::Event::ClaimReward as i32,
                dllm::events::MeteoraDllmEvent::ClaimReward2 => pb::Event::ClaimReward2 as i32,
                dllm::events::MeteoraDllmEvent::CompositionFee => pb::Event::CompositionFee as i32,
                dllm::events::MeteoraDllmEvent::DecreasePositionLength => pb::Event::DecreasePositionLength as i32,
                dllm::events::MeteoraDllmEvent::DynamicFeeParameterUpdate => pb::Event::DynamicFeeParameterUpdate as i32,
                dllm::events::MeteoraDllmEvent::FeeParameterUpdate => pb::Event::FeeParameterUpdate as i32,
                dllm::events::MeteoraDllmEvent::FundReward => pb::Event::FundReward as i32,
                dllm::events::MeteoraDllmEvent::GoToABin => pb::Event::GoToAbin as i32,
                dllm::events::MeteoraDllmEvent::IncreaseObservation => pb::Event::IncreaseObservation as i32,
                dllm::events::MeteoraDllmEvent::IncreasePositionLength => pb::Event::IncreasePositionLength as i32,
                dllm::events::MeteoraDllmEvent::InitializeReward => pb::Event::InitializeReward as i32,
                dllm::events::MeteoraDllmEvent::LbPairCreate => pb::Event::LbPairCreate as i32,
                dllm::events::MeteoraDllmEvent::PositionClose => pb::Event::PositionClose as i32,
                dllm::events::MeteoraDllmEvent::PositionCreate => pb::Event::PositionCreate as i32,
                dllm::events::MeteoraDllmEvent::Rebalancing => pb::Event::Rebalancing as i32,
                dllm::events::MeteoraDllmEvent::RemoveLiquidity => pb::Event::RemoveLiquidity as i32,
                dllm::events::MeteoraDllmEvent::Swap => pb::Event::Swap as i32,
                dllm::events::MeteoraDllmEvent::UpdatePositionLockReleasePoint => pb::Event::UpdatePositionLockReleasePoint as i32,
                dllm::events::MeteoraDllmEvent::UpdatePositionOperator => pb::Event::UpdatePositionOperator as i32,
                dllm::events::MeteoraDllmEvent::UpdateRewardDuration => pb::Event::UpdateRewardDuration as i32,
                dllm::events::MeteoraDllmEvent::UpdateRewardFunder => pb::Event::UpdateRewardFunder as i32,
                dllm::events::MeteoraDllmEvent::WithdrawIneligibleReward => pb::Event::WithdrawIneligibleReward as i32,
                dllm::events::MeteoraDllmEvent::Unknown => pb::Event::Unknown as i32,
            },
        }),
        _ => None,
    }
}
