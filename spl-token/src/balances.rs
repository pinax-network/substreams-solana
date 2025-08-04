use proto::pb::solana::spl::token::v1 as pb;
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{ConfirmedTransaction, TokenBalance},
};

use crate::is_spl_token_program;

pub fn get_token_balance(tx: &ConfirmedTransaction, balance: &TokenBalance) -> Option<pb::TokenBalance> {
    let account = tx.account_at(balance.account_index as u8);

    // only include SPL-Token instructions
    if !is_spl_token_program(&balance.program_id) {
        return None;
    }
    let ui_token_amount = balance.ui_token_amount.as_ref()?;

    Some(pb::TokenBalance {
        program_id: base58::decode(&balance.program_id).expect("Invalid program ID"),
        account: account.0.to_vec(),
        mint: base58::decode(&balance.mint).expect("Invalid mint"),
        amount: ui_token_amount.amount.as_str().parse::<u64>().ok()?, // convert ui_token_amount to a u64
        decimals: ui_token_amount.decimals as u32,
    })
}
