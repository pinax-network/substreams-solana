use base64::Engine;
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

        for log_message in tx_meta.log_messages.iter() {
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

pub fn parse_program_data(log_message: &String) -> Option<Vec<u8>> {
    if let Some(b64) = log_message.strip_prefix("Program data:") {
        // remove embedded whitespace, if any
        let clean: String = b64.chars().filter(|c| !c.is_whitespace()).collect();
        return Some(base64::engine::general_purpose::STANDARD.decode(clean).unwrap_or_default());
    }
    return None;
}
