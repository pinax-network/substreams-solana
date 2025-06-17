use substreams::pb::substreams::Clock;

const GENESIS_TIMESTAMP: u64 = 1584332940; // Genesis timestamp in seconds
const SLOT_DURATION_MS: u64 = 400; // Slot duration in milliseconds

pub fn to_timestamp(clock: &Clock) -> u64 {
    // GENESIS_TIMESTAMP is the genesis timestamp
    // SLOT_DURATION_MS per slot, so we multiply the slot number by SLOT_DURATION_MS and divide by 1000 to get seconds
    GENESIS_TIMESTAMP + (clock.number * SLOT_DURATION_MS) / 1000
}
