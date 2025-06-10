use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

pub fn is_confirmed_tx(tx: &ConfirmedTransaction) -> bool {
    match &tx.meta {
        Some(meta) => meta.err.is_none(),
        None => false,
    }
}
