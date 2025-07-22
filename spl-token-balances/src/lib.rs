use common::solana::{get_fee_payer, get_signers};
use proto::pb::solana::spl::token::balances::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TokenBalance},
};

pub const SOLANA_TOKEN_PROGRAM_KEG: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const SOLANA_TOKEN_PROGRAM_ZQB: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

pub fn is_spl_token_program(program_id: &str) -> bool {
    program_id == SOLANA_TOKEN_PROGRAM_KEG || program_id == SOLANA_TOKEN_PROGRAM_ZQB
}

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    let mut events = pb::Events::default();

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
        match &tx.meta {
            Some(meta) => {
                // PostTokenBalances
                for balance in meta.post_token_balances.iter() {
                    if let Some(token_balance) = get_token_balance(tx, balance) {
                        transaction.post_token_balances.push(token_balance);
                    }
                }
                // PreTokenBalances
                for balance in meta.pre_token_balances.iter() {
                    if let Some(token_balance) = get_token_balance(tx, balance) {
                        transaction.pre_token_balances.push(token_balance);
                    }
                }
            }
            None => continue,
        }
        if transaction.pre_token_balances.is_empty() && transaction.post_token_balances.is_empty() {
            continue; // Skip transactions without token balances
        }
        events.transactions.push(transaction);
    }
    Ok(events)
}

fn get_token_balance(tx: &ConfirmedTransaction, balance: &TokenBalance) -> Option<pb::TokenBalance> {
    let account = tx.account_at(balance.account_index as u8);

    // only include SPL-Token instructions
    if !is_spl_token_program(&balance.program_id) {
        return None;
    }
    let ui_token_amount = match &balance.ui_token_amount {
        Some(amount) => amount,
        None => return None, // skip if ui_token_amount is None
    };
    // convert ui_token_amount to a u64
    let amount = match ui_token_amount.amount.as_str().parse::<u64>() {
        Ok(amount) => amount,
        Err(_) => return None, // skip if parsing fails
    };
    Some(pb::TokenBalance {
        program_id: base58::decode(&balance.program_id).unwrap(),
        account: account.0.to_vec(),
        account_index: balance.account_index,
        mint: base58::decode(&balance.mint).unwrap(),
        amount,
        decimals: ui_token_amount.decimals as u32,
    })
}
