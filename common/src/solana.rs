use base64::Engine;
// use solana_sdk::pubkey::Pubkey;
// use std::str::FromStr;
use substreams::pb::substreams::Clock;
use substreams_solana::{base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

const GENESIS_TIMESTAMP: u64 = 1584332940; // Genesis timestamp in seconds
const SLOT_DURATION_MS: u64 = 400; // Slot duration in milliseconds

pub fn to_timestamp(clock: &Clock) -> u64 {
    // GENESIS_TIMESTAMP is the genesis timestamp
    // SLOT_DURATION_MS per slot, so we multiply the slot number by SLOT_DURATION_MS and divide by 1000 to get seconds
    GENESIS_TIMESTAMP + (clock.number * SLOT_DURATION_MS) / 1000
}

pub fn get_fee_payer(tx: &ConfirmedTransaction) -> Option<Vec<u8>> {
    // ConfirmedTransaction → Transaction → Message → account_keys[0]
    tx.transaction
        .as_ref() // Option<&Transaction>
        .and_then(|t| t.message.as_ref()) // Option<&Message>
        .and_then(|msg| msg.account_keys.get(0))
        .cloned() // Option<Vec<u8>>
}

/// Returns a vector of 32-byte pubkeys for every signer in the transaction
/// (index 0 is always the fee-payer).
/// `None` is returned only if the protobuf is missing the expected fields.
pub fn get_signers(tx: &ConfirmedTransaction) -> Option<Vec<Vec<u8>>> {
    tx.transaction
        .as_ref() // Option<&Transaction>
        .and_then(|t| t.message.as_ref()) // Option<&Message>
        .and_then(|msg| {
            msg.header.as_ref().map(|hdr| {
                let n = hdr.num_required_signatures as usize;
                msg.account_keys
                    .iter()
                    .take(n) // first `n` keys are all signers
                    .cloned()
                    .collect::<Vec<Vec<u8>>>()
            })
        })
}

pub fn parse_program_data(log_message: &String) -> Option<Vec<u8>> {
    if let Some(b64) = log_message.strip_prefix("Program data:") {
        // remove embedded whitespace, if any
        let clean: String = b64.chars().filter(|c| !c.is_whitespace()).collect();
        return Some(base64::engine::general_purpose::STANDARD.decode(clean).unwrap_or_default());
    }
    return None;
}

/// Matches:  Program <PK> invoke [123]
pub fn parse_invoke_height(log: &str) -> Option<u32> {
    let prefix = " invoke [";
    if let Some(pos) = log.find(prefix) {
        // fast path – cheap slice instead of regex
        let after = &log[pos + prefix.len()..];
        if let Some(end) = after.find(']') {
            return after[..end].parse::<u32>().ok();
        }
    }
    None
}

/// Extracts the base-58 program id.
///
/// Matches any of:
///   - "Program <PK> invoke [..]"
///   - "Program <PK> consumed .."
///   - "Program <PK> success"
///
/// Returns `Some(String)` holding the `<PK>` or `None` if the line
/// does not begin with the expected prefix.
///
/// Fast slice-based parsing; no regex needed.
pub fn parse_program_id(log: &str) -> Option<Vec<u8>> {
    const PREFIX: &str = "Program ";
    if !log.starts_with(PREFIX) {
        return None;
    }

    // slice off the prefix and grab the first whitespace-delimited token
    let rest = &log[PREFIX.len()..];
    let end = rest.find(' ')?; // bail if no space
    let candidate = &rest[..end];

    // Reject anything that is not a valid base-58 pubkey
    match base58::decode(candidate) {
        Ok(pk) if pk.len() == 32 => Some(pk),
        _ => None,
    }
}

/// Returns true for:  Program <PK> success
pub fn is_success(log: &str) -> bool {
    // success is always the last token
    log.trim_end().ends_with(" success")
}

/// Returns true for:  Program <PK> invoke [...]
pub fn is_invoke(log: &str) -> bool {
    log.contains(" invoke [")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_height_ok() {
        let log = "Program ABCD invoke [42]";
        assert_eq!(parse_invoke_height(log), Some(42));
    }

    #[test]
    fn parse_height_malformed() {
        // missing closing bracket
        let log = "Program ABCD invoke [42";
        assert_eq!(parse_invoke_height(log), None);
    }

    #[test]
    fn success_detection() {
        let log = "Program ABCD success";
        assert!(is_success(log));
        assert!(!is_invoke(log));
    }

    #[test]
    fn invoke_detection() {
        let log = "Program ABCD invoke [3]";
        assert!(is_invoke(log));
        assert!(!is_success(log));
    }

    #[test]
    fn program_data_decodes() {
        // "hello" in base64
        let msg = "Program data:aGVsbG8=";
        let decoded = parse_program_data(&msg.to_string()).unwrap();
        assert_eq!(decoded, b"hello");
    }

    #[test]
    fn contract_name_extracts() {
        let log = "Program Minimox7jqQmMpF6Z34DTNwE9iJyNkruzvvYQRaHpAP invoke [1]";
        assert_eq!(
            parse_program_id(log),
            Some(base58::decode("Minimox7jqQmMpF6Z34DTNwE9iJyNkruzvvYQRaHpAP").unwrap())
        );
    }

    #[test]
    fn contract_name_extracts_2() {
        let log = "Program 11111111111111111111111111111111 invoke [3]";
        assert_eq!(parse_program_id(log), Some(base58::decode("11111111111111111111111111111111").unwrap()));
    }

    #[test]
    fn contract_name_non_match() {
        let log = "Some unrelated line";
        assert!(parse_program_id(log).is_none());
    }

    #[test]
    fn program_id_rejects_log_sentinel() {
        let log = "Program log: Instruction: CloseAccount";
        assert!(parse_program_id(log).is_none());
    }
}
