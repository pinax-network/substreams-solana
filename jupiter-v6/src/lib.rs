use common::solana::{get_fee_payer, get_signers};
use proto::pb::jupiter::v1 as pb;
use substreams_solana::{base58, pb::sf::solana::r#type::v1::Block};
use substreams_solana_idls::jupiter;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();

    let matcher = substreams::expr_matcher(&params);

    // transactions
    for tx in block.transactions() {
        let mut transaction = pb::Transaction::default();
        let tx_meta = tx.meta.as_ref().expect("Transaction meta should be present");
        transaction.fee = tx_meta.fee;
        transaction.compute_units_consumed = tx_meta.compute_units_consumed();
        transaction.signature = tx.hash().to_vec();

        if let Some(fee_payer) = get_fee_payer(tx) {
            transaction.fee_payer = fee_payer;
        }
        if let Some(signers) = get_signers(tx) {
            transaction.signers = signers;
        }

        // Include instructions and events
        for instruction in tx.walk_instructions() {
            let program_id = instruction.program_id().0;

            // Skip instructions
            if program_id != &jupiter::v6::PROGRAM_ID.to_vec() {
                continue;
            }
            let mut base = pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: instruction.stack_height(),
                instruction: None,
            };
            // -- Events --
            match jupiter::v6::anchor_self_cpi::unpack(instruction.data()) {
                // -- Swap --
                Ok(jupiter::v6::anchor_self_cpi::JupiterV6Event::Swap(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::SwapEvent(pb::SwapEvent {
                        amm: event.amm.to_bytes().to_vec(),
                        input_mint: event.input_mint.to_bytes().to_vec(),
                        input_amount: event.input_amount,
                        output_mint: event.output_mint.to_bytes().to_vec(),
                        output_amount: event.output_amount,
                    }));
                    transaction.instructions.push(base.clone());
                }
                // -- Fee --
                Ok(jupiter::v6::anchor_self_cpi::JupiterV6Event::Fee(event)) => {
                    base.instruction = Some(pb::instruction::Instruction::FeeEvent(pb::FeeEvent {
                        account: event.account.to_bytes().to_vec(),
                        mint: event.mint.to_bytes().to_vec(),
                        amount: event.amount,
                    }));
                    transaction.instructions.push(base.clone());
                }
                _ => {}
            }
        }
        if !transaction.instructions.is_empty() {
            events.transactions.push(transaction);
        }
    }
    Ok(events)
}
