// @generated
// This file is @generated by prost-build.
/// -----------------------------------------------------------------------------
/// Top-level containers
/// -----------------------------------------------------------------------------
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
    /// Fee-payer account address
    #[prost(bytes="vec", tag="2")]
    pub fee_payer: ::prost::alloc::vec::Vec<u8>,
    /// Signers of the tx
    #[prost(bytes="vec", repeated, tag="3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Lamports paid
    #[prost(uint64, tag="4")]
    pub fee: u64,
    /// CU used
    #[prost(uint64, tag="5")]
    pub compute_units_consumed: u64,
    /// Executed instructions
    #[prost(message, repeated, tag="6")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
}
/// -----------------------------------------------------------------------------
/// Instruction + typed payloads
/// -----------------------------------------------------------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub stack_height: u32,
    #[prost(oneof="instruction::Instruction", tags="3, 4, 5, 6, 7")]
    pub instruction: ::core::option::Option<instruction::Instruction>,
}
/// Nested message and enum types in `Instruction`.
pub mod instruction {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Instruction {
        #[prost(message, tag="3")]
        InitLog(super::InitLog),
        #[prost(message, tag="4")]
        DepositLog(super::DepositLog),
        #[prost(message, tag="5")]
        WithdrawLog(super::WithdrawLog),
        #[prost(message, tag="6")]
        SwapBaseInLog(super::SwapBaseInLog),
        #[prost(message, tag="7")]
        SwapBaseOutLog(super::SwapBaseOutLog),
    }
}
/// -----------------------------------------------------------------------------
/// Raydium event payloads (renumbered from 1)
/// -----------------------------------------------------------------------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitLog {
    #[prost(uint32, tag="1")]
    pub pc_decimals: u32,
    #[prost(uint32, tag="2")]
    pub coin_decimals: u32,
    #[prost(uint64, tag="3")]
    pub pc_lot_size: u64,
    #[prost(uint64, tag="4")]
    pub coin_lot_size: u64,
    #[prost(uint64, tag="5")]
    pub pc_amount: u64,
    #[prost(uint64, tag="6")]
    pub coin_amount: u64,
    /// 32-byte Pubkey
    #[prost(bytes="vec", tag="7")]
    pub market: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositLog {
    #[prost(uint64, tag="1")]
    pub max_coin: u64,
    #[prost(uint64, tag="2")]
    pub max_pc: u64,
    #[prost(uint64, tag="3")]
    pub base: u64,
    #[prost(uint64, tag="4")]
    pub pool_coin: u64,
    #[prost(uint64, tag="5")]
    pub pool_pc: u64,
    #[prost(uint64, tag="6")]
    pub pool_lp: u64,
    /// UInt128 as decimal string
    #[prost(string, tag="7")]
    pub calc_pnl_x: ::prost::alloc::string::String,
    /// UInt128 as decimal string
    #[prost(string, tag="8")]
    pub calc_pnl_y: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub deduct_coin: u64,
    #[prost(uint64, tag="10")]
    pub deduct_pc: u64,
    #[prost(uint64, tag="11")]
    pub mint_lp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLog {
    #[prost(uint64, tag="1")]
    pub withdraw_lp: u64,
    #[prost(uint64, tag="2")]
    pub user_lp: u64,
    #[prost(uint64, tag="3")]
    pub pool_coin: u64,
    #[prost(uint64, tag="4")]
    pub pool_pc: u64,
    #[prost(uint64, tag="5")]
    pub pool_lp: u64,
    /// UInt128 as decimal string
    #[prost(string, tag="6")]
    pub calc_pnl_x: ::prost::alloc::string::String,
    /// UInt128 as decimal string
    #[prost(string, tag="7")]
    pub calc_pnl_y: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub out_coin: u64,
    #[prost(uint64, tag="9")]
    pub out_pc: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SwapBaseInLog {
    #[prost(uint64, tag="1")]
    pub amount_in: u64,
    #[prost(uint64, tag="2")]
    pub minimum_out: u64,
    #[prost(uint64, tag="3")]
    pub direction: u64,
    #[prost(uint64, tag="4")]
    pub user_source: u64,
    #[prost(uint64, tag="5")]
    pub pool_coin: u64,
    #[prost(uint64, tag="6")]
    pub pool_pc: u64,
    #[prost(uint64, tag="7")]
    pub out_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SwapBaseOutLog {
    #[prost(uint64, tag="1")]
    pub max_in: u64,
    #[prost(uint64, tag="2")]
    pub amount_out: u64,
    #[prost(uint64, tag="3")]
    pub direction: u64,
    #[prost(uint64, tag="4")]
    pub user_source: u64,
    #[prost(uint64, tag="5")]
    pub pool_coin: u64,
    #[prost(uint64, tag="6")]
    pub pool_pc: u64,
    #[prost(uint64, tag="7")]
    pub deduct_in: u64,
}
// @@protoc_insertion_point(module)
