#!/bin/bash
BASE=/data/workspace/substreams-svm

# Generate obric v2/v3 prost files (same structure, different package)
for ver in v2 v3; do
cat > "$BASE/proto/src/pb/obric.${ver}.v1.rs" << 'EOF'
// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub fee_payer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag="4")]
    pub fee: u64,
    #[prost(uint64, tag="5")]
    pub compute_units_consumed: u64,
    #[prost(message, repeated, tag="6")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    #[prost(message, repeated, tag="7")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub stack_height: u32,
    #[prost(oneof="instruction::Instruction", tags="3, 4")]
    pub instruction: ::core::option::Option<instruction::Instruction>,
}
pub mod instruction {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Instruction {
        #[prost(message, tag="3")]
        SwapXToY(super::SwapXToYInstruction),
        #[prost(message, tag="4")]
        SwapYToX(super::SwapYToXInstruction),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapXToYInstruction {
    #[prost(uint64, tag="1")]
    pub input_amount: u64,
    #[prost(uint64, tag="2")]
    pub min_output_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapYToXInstruction {
    #[prost(uint64, tag="1")]
    pub input_amount: u64,
    #[prost(uint64, tag="2")]
    pub min_output_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub invoke_depth: u32,
}
EOF
done

# Generate solfi proto (with swap events containing user + amounts)
for ver in v1 v2; do
cat > "$BASE/proto/v1/solfi-${ver}.proto" << EOF
syntax = "proto3";
package solfi.${ver}.v1;
message Events { repeated Transaction transactions = 1; }
message Transaction {
  bytes signature = 1; bytes fee_payer = 2; repeated bytes signers = 3;
  uint64 fee = 4; uint64 compute_units_consumed = 5;
  repeated Instruction instructions = 6; repeated Log logs = 7;
}
message Instruction {
  bytes program_id = 1; uint32 stack_height = 2;
  oneof instruction { SwapInstruction swap = 3; }
}
message SwapInstruction { uint64 amount_in = 1; uint64 minimum_out = 2; uint32 direction = 3; }
message Log {
  bytes program_id = 1; uint32 invoke_depth = 2;
  oneof log { SwapEvent swap = 3; }
}
message SwapEvent { bytes user = 1; uint64 amount_in = 2; uint64 amount_out = 3; }
EOF

# prost Rust for solfi
cat > "$BASE/proto/src/pb/solfi.${ver}.v1.rs" << 'PROST_EOF'
// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub fee_payer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag="4")]
    pub fee: u64,
    #[prost(uint64, tag="5")]
    pub compute_units_consumed: u64,
    #[prost(message, repeated, tag="6")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    #[prost(message, repeated, tag="7")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub stack_height: u32,
    #[prost(oneof="instruction::Instruction", tags="3")]
    pub instruction: ::core::option::Option<instruction::Instruction>,
}
pub mod instruction {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Instruction {
        #[prost(message, tag="3")]
        Swap(super::SwapInstruction),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapInstruction {
    #[prost(uint64, tag="1")]
    pub amount_in: u64,
    #[prost(uint64, tag="2")]
    pub minimum_out: u64,
    #[prost(uint32, tag="3")]
    pub direction: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub invoke_depth: u32,
    #[prost(oneof="log::Log", tags="3")]
    pub log: ::core::option::Option<log::Log>,
}
pub mod log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Log {
        #[prost(message, tag="3")]
        Swap(super::SwapEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvent {
    #[prost(bytes="vec", tag="1")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(uint64, tag="3")]
    pub amount_out: u64,
}
PROST_EOF
done

# Generate drift proto (BeginSwap/EndSwap)
cat > "$BASE/proto/v1/drift.proto" << 'EOF'
syntax = "proto3";
package drift.v1;
message Events { repeated Transaction transactions = 1; }
message Transaction {
  bytes signature = 1; bytes fee_payer = 2; repeated bytes signers = 3;
  uint64 fee = 4; uint64 compute_units_consumed = 5;
  repeated Instruction instructions = 6; repeated Log logs = 7;
}
message Instruction {
  bytes program_id = 1; uint32 stack_height = 2;
  oneof instruction { BeginSwapInstruction begin_swap = 3; EndSwapInstruction end_swap = 4; }
}
message BeginSwapInstruction { uint64 amount_in = 1; }
message EndSwapInstruction { uint32 reduce_only = 1; }
message Log {
  bytes program_id = 1; uint32 invoke_depth = 2;
  oneof log { SwapEvent swap = 3; }
}
message SwapEvent { bytes user = 1; uint64 amount_in = 2; uint64 amount_out = 3; }
EOF

cat > "$BASE/proto/src/pb/drift.v1.rs" << 'PROST_EOF'
// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub fee_payer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag="4")]
    pub fee: u64,
    #[prost(uint64, tag="5")]
    pub compute_units_consumed: u64,
    #[prost(message, repeated, tag="6")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    #[prost(message, repeated, tag="7")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub stack_height: u32,
    #[prost(oneof="instruction::Instruction", tags="3, 4")]
    pub instruction: ::core::option::Option<instruction::Instruction>,
}
pub mod instruction {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Instruction {
        #[prost(message, tag="3")]
        BeginSwap(super::BeginSwapInstruction),
        #[prost(message, tag="4")]
        EndSwap(super::EndSwapInstruction),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginSwapInstruction {
    #[prost(uint64, tag="1")]
    pub amount_in: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndSwapInstruction {
    #[prost(uint32, tag="1")]
    pub reduce_only: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub invoke_depth: u32,
    #[prost(oneof="log::Log", tags="3")]
    pub log: ::core::option::Option<log::Log>,
}
pub mod log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Log {
        #[prost(message, tag="3")]
        Swap(super::SwapEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvent {
    #[prost(bytes="vec", tag="1")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(uint64, tag="3")]
    pub amount_out: u64,
}
PROST_EOF

# Generate boop proto (buy/sell events)
cat > "$BASE/proto/v1/boop.proto" << 'EOF'
syntax = "proto3";
package boop.v1;
message Events { repeated Transaction transactions = 1; }
message Transaction {
  bytes signature = 1; bytes fee_payer = 2; repeated bytes signers = 3;
  uint64 fee = 4; uint64 compute_units_consumed = 5;
  repeated Instruction instructions = 6; repeated Log logs = 7;
}
message Instruction {
  bytes program_id = 1; uint32 stack_height = 2;
  oneof instruction { BuyTokenInstruction buy = 3; SellTokenInstruction sell = 4; }
}
message BuyTokenInstruction { uint64 buy_amount = 1; uint64 amount_out_min = 2; }
message SellTokenInstruction { uint64 sell_amount = 1; uint64 amount_out_min = 2; }
message Log {
  bytes program_id = 1; uint32 invoke_depth = 2;
  oneof log { TokenBoughtEvent bought = 3; TokenSoldEvent sold = 4; }
}
message TokenBoughtEvent {
  bytes mint = 1; uint64 amount_in = 2; uint64 amount_out = 3;
  uint64 swap_fee = 4; bytes buyer = 5; bytes recipient = 6;
}
message TokenSoldEvent {
  bytes mint = 1; uint64 amount_in = 2; uint64 amount_out = 3;
  uint64 swap_fee = 4; bytes seller = 5; bytes recipient = 6;
}
EOF

cat > "$BASE/proto/src/pb/boop.v1.rs" << 'PROST_EOF'
// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub fee_payer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag="4")]
    pub fee: u64,
    #[prost(uint64, tag="5")]
    pub compute_units_consumed: u64,
    #[prost(message, repeated, tag="6")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    #[prost(message, repeated, tag="7")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub stack_height: u32,
    #[prost(oneof="instruction::Instruction", tags="3, 4")]
    pub instruction: ::core::option::Option<instruction::Instruction>,
}
pub mod instruction {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Instruction {
        #[prost(message, tag="3")]
        Buy(super::BuyTokenInstruction),
        #[prost(message, tag="4")]
        Sell(super::SellTokenInstruction),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyTokenInstruction {
    #[prost(uint64, tag="1")]
    pub buy_amount: u64,
    #[prost(uint64, tag="2")]
    pub amount_out_min: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellTokenInstruction {
    #[prost(uint64, tag="1")]
    pub sell_amount: u64,
    #[prost(uint64, tag="2")]
    pub amount_out_min: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub invoke_depth: u32,
    #[prost(oneof="log::Log", tags="3, 4")]
    pub log: ::core::option::Option<log::Log>,
}
pub mod log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Log {
        #[prost(message, tag="3")]
        Bought(super::TokenBoughtEvent),
        #[prost(message, tag="4")]
        Sold(super::TokenSoldEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenBoughtEvent {
    #[prost(bytes="vec", tag="1")]
    pub mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(uint64, tag="3")]
    pub amount_out: u64,
    #[prost(uint64, tag="4")]
    pub swap_fee: u64,
    #[prost(bytes="vec", tag="5")]
    pub buyer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenSoldEvent {
    #[prost(bytes="vec", tag="1")]
    pub mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(uint64, tag="3")]
    pub amount_out: u64,
    #[prost(uint64, tag="4")]
    pub swap_fee: u64,
    #[prost(bytes="vec", tag="5")]
    pub seller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
}
PROST_EOF

# dflow proto (simple swap instruction, aggregator-style)
cat > "$BASE/proto/v1/dflow.proto" << 'EOF'
syntax = "proto3";
package dflow.v1;
message Events { repeated Transaction transactions = 1; }
message Transaction {
  bytes signature = 1; bytes fee_payer = 2; repeated bytes signers = 3;
  uint64 fee = 4; uint64 compute_units_consumed = 5;
  repeated Instruction instructions = 6; repeated Log logs = 7;
}
message Instruction {
  bytes program_id = 1; uint32 stack_height = 2;
  oneof instruction { SwapInstruction swap = 3; }
}
message SwapInstruction { uint64 amount_in = 1; uint64 minimum_amount_out = 2; }
message Log { bytes program_id = 1; uint32 invoke_depth = 2; }
EOF

cp "$BASE/proto/src/pb/sanctum.v1.rs" "$BASE/proto/src/pb/dflow.v1.rs"

echo "All batch 2 protos generated"
