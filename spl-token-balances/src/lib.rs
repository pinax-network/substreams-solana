use proto::pb::solana::spl::token::balances::v1::{Balance, Events};
use substreams::errors::Error;
use substreams_solana::{base58, pb::sf::solana::r#type::v1::Block};

pub const SOLANA_TOKEN_PROGRAM_KEG: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const SOLANA_TOKEN_PROGRAM_ZQB: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

pub fn is_spl_token_program(program_id: &str) -> bool {
    program_id == SOLANA_TOKEN_PROGRAM_KEG || program_id == SOLANA_TOKEN_PROGRAM_ZQB
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    // running counter of every SPL-Token account balance encountered across the whole block, incremented each time one is processed.
    let mut execution_index = 0;

    for tx in block.transactions() {
        match &tx.meta {
            Some(meta) => {
                for balance in meta.post_token_balances.iter() {
                    let account = tx.account_at(balance.account_index as u8);

                    execution_index += 1;

                    // only include SPL-Token instructions
                    if !is_spl_token_program(&balance.program_id) {
                        continue;
                    }
                    let ui_token_amount = match &balance.ui_token_amount {
                        Some(amount) => amount,
                        None => continue, // skip if ui_token_amount is None
                    };
                    // convert ui_token_amount to a u64
                    let amount = match ui_token_amount.amount.as_str().parse::<u64>() {
                        Ok(amount) => amount,
                        Err(_) => continue, // skip if parsing fails
                    };

                    events.balances.push(Balance {
                        // transaction
                        tx_hash: tx.hash().to_vec(),

                        // indexes
                        execution_index,

                        // account
                        program_id: base58::decode(&balance.program_id).unwrap(),

                        // event
                        owner: account.0.to_vec(),
                        amount,
                        decimals: ui_token_amount.decimals,
                        mint: base58::decode(&balance.mint).unwrap(),
                    })
                }
            }
            None => continue,
        }
    }
    Ok(events)
}
