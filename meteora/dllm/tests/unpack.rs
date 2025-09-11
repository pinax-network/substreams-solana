#![cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::meteora::dllm;

    #[test]
    fn unpack_dllm_swap_event() {
        // https://solscan.io/tx/<unknown? maybe but not necessary>. Maybe not.
        let bytes = hex!("e445a52e51cb9a1d516ce3becdd00ac471739c6f45944a0e56b3a0f9b2399f421b0a82b56c5b87acc78023b8ed27cc7cf74ba51f6820a847117a8fad48c80ae298e20bf37ff35e7ae8c7b85272b5010701feffff01feffff1f660a0000000000702c000000000000016037000000000000c40200000000000000623d010000000000000000000000000000000000000000");
        match dllm::events::unpack(&bytes).expect("decode event") {
            dllm::events::MeteoraDllmEvent::Swap(event) => {
                assert_eq!(event.lb_pair.to_string(), "8dsKNwMDMh1Vfr4YevzRY9oDS7N29fRHpPXcoDmUVSBd", "lb_pair");
                assert_eq!(event.from.to_string(), "HeLbvj7KM5fkduCdiufTa5SmVtuuVjySPps7dnp2pDZG", "from");
                assert_eq!(event.start_bin_id, -511, "start_bin_id");
                assert_eq!(event.end_bin_id, -511, "end_bin_id");
                assert_eq!(event.amount_in, 681_503, "amount_in");
                assert_eq!(event.amount_out, 11_376, "amount_out");
                assert!(event.swap_for_y, "swap_for_y");
                assert_eq!(event.fee, 14_176, "fee");
                assert_eq!(event.protocol_fee, 708, "protocol_fee");
                assert_eq!(event.fee_bps, 20_800_000, "fee_bps");
                assert_eq!(event.host_fee, 0, "host_fee");
            }
            _ => panic!("Expected Swap event"),
        }
    }
}
