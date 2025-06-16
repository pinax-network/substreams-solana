use proto::pb::solana::clock::v1::SolanaClock;
use substreams::errors::Error;
use substreams::store::{StoreGet, StoreGetInt64, StoreNew, StoreSet, StoreSetInt64};
use substreams_solana::base58;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn map_clock(block: Block, store: StoreGetInt64) -> Result<SolanaClock, Error> {
    Ok(SolanaClock {
        blockhash: base58::decode(&block.blockhash).unwrap(),
        slot: block.slot,
        timestamp: 0,
    })
}

#[substreams::handlers::store]
pub fn store_solana_clock(block: Block, store: StoreSetInt64) {
    match block.block_time {
        Some(ts) => {
            // let ms = ts.seconds * 1000 + ts.nanos as i64 / 1_000_000;
            store.set(0, "", &ts.timestamp); // store the last clock timestamp in milliseconds
        }
        None => {}
    }
}
