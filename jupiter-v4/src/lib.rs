use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_height, parse_program_data, parse_program_id};
use proto::pb::jupiter::v1 as pb;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana_idls::jupiter;

#[substreams::handlers::map]
fn map_events(_params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();

    // let matcher = substreams::expr_matcher(&params);

    let mut base = pb::Instruction {
        program_id: jupiter::v4::PROGRAM_ID.to_vec(),
        stack_height: 0, // TO-DO: get stack height from log messages
        instruction: None,
    };

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

        let mut is_invoked = false;

        for log_message in tx_meta.log_messages.iter() {
            // -- must match Jupiter V4 program ID --
            let match_program_id = match parse_program_id(log_message) {
                Some(id) => id == jupiter::v4::PROGRAM_ID.to_vec(),
                None => false,
            };
            // ─── NEW: track invoke / success & stack height ─────────────────────────────
            if is_invoke(log_message) && match_program_id {
                if let Some(h) = parse_invoke_height(log_message) {
                    base.stack_height = h - 1; // stack height is 1-based, so we subtract 1
                    is_invoked = true;
                }
            }

            // Not invoked, skip
            // makes sure we only process logs that are invoked by the Jupiter V4 program
            // in case of multiple invocations using the same Program Data
            if !is_invoked {
                continue;
            }

            if let Some(data) = parse_program_data(&log_message) {
                // -- Events --
                match jupiter::v4::events::unpack(data.as_slice()) {
                    // -- Swap --
                    Ok(jupiter::v4::events::JupiterV4Event::Swap(event)) => {
                        base.instruction = Some(pb::instruction::Instruction::SwapEvent(pb::SwapEvent {
                            amm: event.amm.to_bytes().to_vec(),
                            input_mint: event.input_mint.to_bytes().to_vec(),
                            input_amount: event.input_amount,
                            output_mint: event.output_mint.to_bytes().to_vec(),
                            output_amount: event.output_amount,
                        }));
                        transaction.instructions.push(base.clone());
                    }
                    _ => {}
                }
            }
        }
        if !transaction.instructions.is_empty() {
            events.transactions.push(transaction);
        }
    }
    Ok(events)
}
