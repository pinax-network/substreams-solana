use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::meteora::dllm::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
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
    let logs = process_logs(tx_meta, &dllm::PROGRAM_ID.to_vec());

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

fn process_logs(tx_meta: &TransactionStatusMeta, program_id_bytes: &[u8]) -> Vec<pb::Log> {
    let mut logs = Vec::new();
    let mut is_invoked = false;

    for log_message in tx_meta.log_messages.iter() {
        let match_program_id = parse_program_id(log_message).map_or(false, |id| id == program_id_bytes.to_vec());

        if is_invoke(log_message) && match_program_id {
            if let Some(invoke_depth) = parse_invoke_depth(log_message) {
                is_invoked = true;
                if let Some(log_data) = parse_log_data(log_message, program_id_bytes, invoke_depth) {
                    logs.push(log_data);
                }
            }
        } else if is_invoked {
            if let Some(log_data) = parse_log_data(log_message, program_id_bytes, 0) {
                logs.push(log_data);
            }
        }
    }

    logs
}

fn parse_log_data(log_message: &str, program_id_bytes: &[u8], invoke_depth: u32) -> Option<pb::Log> {
    let data = parse_program_data(log_message)?;
    match dllm::events::unpack(data.as_slice()) {
        Ok(event) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            event: match event {
                dllm::events::MeteoraDllmEvent::AddLiquidity => pb::Event::EventAddLiquidity as i32,
                dllm::events::MeteoraDllmEvent::ClaimFee => pb::Event::EventClaimFee as i32,
                dllm::events::MeteoraDllmEvent::ClaimFee2 => pb::Event::EventClaimFee2 as i32,
                dllm::events::MeteoraDllmEvent::ClaimReward => pb::Event::EventClaimReward as i32,
                dllm::events::MeteoraDllmEvent::ClaimReward2 => pb::Event::EventClaimReward2 as i32,
                dllm::events::MeteoraDllmEvent::CompositionFee => pb::Event::EventCompositionFee as i32,
                dllm::events::MeteoraDllmEvent::DecreasePositionLength => pb::Event::EventDecreasePositionLength as i32,
                dllm::events::MeteoraDllmEvent::DynamicFeeParameterUpdate => pb::Event::EventDynamicFeeParameterUpdate as i32,
                dllm::events::MeteoraDllmEvent::FeeParameterUpdate => pb::Event::EventFeeParameterUpdate as i32,
                dllm::events::MeteoraDllmEvent::FundReward => pb::Event::EventFundReward as i32,
                dllm::events::MeteoraDllmEvent::GoToABin => pb::Event::EventGoToAbin as i32,
                dllm::events::MeteoraDllmEvent::IncreaseObservation => pb::Event::EventIncreaseObservation as i32,
                dllm::events::MeteoraDllmEvent::IncreasePositionLength => pb::Event::EventIncreasePositionLength as i32,
                dllm::events::MeteoraDllmEvent::InitializeReward => pb::Event::EventInitializeReward as i32,
                dllm::events::MeteoraDllmEvent::LbPairCreate => pb::Event::EventLbPairCreate as i32,
                dllm::events::MeteoraDllmEvent::PositionClose => pb::Event::EventPositionClose as i32,
                dllm::events::MeteoraDllmEvent::PositionCreate => pb::Event::EventPositionCreate as i32,
                dllm::events::MeteoraDllmEvent::Rebalancing => pb::Event::EventRebalancing as i32,
                dllm::events::MeteoraDllmEvent::RemoveLiquidity => pb::Event::EventRemoveLiquidity as i32,
                dllm::events::MeteoraDllmEvent::Swap => pb::Event::EventSwap as i32,
                dllm::events::MeteoraDllmEvent::UpdatePositionLockReleasePoint => pb::Event::EventUpdatePositionLockReleasePoint as i32,
                dllm::events::MeteoraDllmEvent::UpdatePositionOperator => pb::Event::EventUpdatePositionOperator as i32,
                dllm::events::MeteoraDllmEvent::UpdateRewardDuration => pb::Event::EventUpdateRewardDuration as i32,
                dllm::events::MeteoraDllmEvent::UpdateRewardFunder => pb::Event::EventUpdateRewardFunder as i32,
                dllm::events::MeteoraDllmEvent::WithdrawIneligibleReward => pb::Event::EventWithdrawIneligibleReward as i32,
                dllm::events::MeteoraDllmEvent::Unknown => pb::Event::EventUnknown as i32,
            },
        }),
        _ => None,
    }
}
