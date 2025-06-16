use substreams::pb::substreams::Clock;

pub fn to_timestamp(clock: &Clock) -> u64 {
    // 1584332940 is the genesis timestamp
    // 400ms per slot, so we multiply the slot number by 400 and divide by 1000 to get seconds
    1584332940 + (clock.number * 400) / 1000
}
