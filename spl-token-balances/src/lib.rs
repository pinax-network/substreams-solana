use proto::pb::{
    sf::{solana::r#type::v1::AccountBlock, substreams::solana::r#type::v1::FilteredAccounts},
    solana::spl::token::v1::{Approve, Events},
};
use substreams::{errors::Error, log};
use substreams_solana::{block_view::InstructionView, pb::sf::solana::r#type::v1::Block};

pub const SOLANA_TOKEN_PROGRAM_KEG: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const SOLANA_TOKEN_PROGRAM_ZQB: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

pub fn is_spl_token_program(instruction: &InstructionView) -> bool {
    let program_id = instruction.program_id().to_string();
    program_id == SOLANA_TOKEN_PROGRAM_KEG || program_id == SOLANA_TOKEN_PROGRAM_ZQB
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    for tx in block.transactions() {
        let accounts = tx.resolved_accounts();
        match &tx.meta {
            Some(meta) => {
                for balance in meta.post_token_balances.iter() {
                    balance.mint
                    balance.program_id
                    balance.ui_token_amount.unwrap().amount
                    balance.ui_token_amount.unwrap().decimals
                    balance.owner
                    let account = tx.account_at(balance.account_index as u8);
                    log::info!("Post token balance: {:?}, account = {}", balance, account);
                }
            }
            None => continue,
        }
    }
    Ok(events)
}
