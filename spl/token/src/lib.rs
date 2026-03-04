use proto::pb::solana::spl::token::v1 as pb;
use spl_token_common::{SOLANA_TOKEN_PROGRAM_KEG, SOLANA_TOKEN_PROGRAM_ZQB};
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

fn is_spl_token_program(program_id: &[u8]) -> bool {
    program_id == &SOLANA_TOKEN_PROGRAM_KEG || program_id == &SOLANA_TOKEN_PROGRAM_ZQB
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(|tx| spl_token_common::process_transaction(tx, is_spl_token_program)).collect(),
    })
}
