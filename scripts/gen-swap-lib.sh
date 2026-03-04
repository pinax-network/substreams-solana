#!/bin/bash
# Generate simple swap-only src/lib.rs for a DEX
# Usage: ./scripts/gen-swap-lib.sh <crate_name> <idl_module> <instruction_enum> <swap_variant> <proto_path>
# e.g.: ./scripts/gen-swap-lib.sh sanctum sanctum SanctumInstruction "SwapViaStake { amount }" "sanctum::v1"
CRATE=$1
IDL=$2
ENUM=$3
SWAP=$4
PROTO=$5
BASE=/data/workspace/substreams-svm

cat > "$BASE/dex/$CRATE/src/lib.rs" << EOF
use common::solana::{get_fee_payer, get_signers};
use proto::pb::${PROTO} as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r\#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::${IDL};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    if instructions.is_empty() { return None; }
    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
        logs: vec![],
    })
}
EOF

echo "Generated dex/$CRATE/src/lib.rs (stub — needs process_instruction)"
