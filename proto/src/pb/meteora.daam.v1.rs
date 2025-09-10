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
    #[prost(oneof="instruction::Instruction", tags="3,4,5")]
    pub instruction: ::core::option::Option<instruction::Instruction>,
}
/// Nested message and enum types in `Instruction`.
pub mod instruction {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Instruction {
        #[prost(message, tag="3")]
        AddLiquidity(super::AddLiquidityInstruction),
        #[prost(message, tag="4")]
        RemoveLiquidity(super::RemoveLiquidityInstruction),
        #[prost(message, tag="5")]
        Swap(super::SwapInstruction),
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityInstruction {
    #[prost(message, optional, tag="1")]
    pub accounts: ::core::option::Option<AddLiquidityAccounts>,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<AddLiquidityParameters>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityInstruction {
    #[prost(message, optional, tag="1")]
    pub accounts: ::core::option::Option<RemoveLiquidityAccounts>,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<RemoveLiquidityParameters>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapInstruction {
    #[prost(message, optional, tag="1")]
    pub accounts: ::core::option::Option<SwapAccounts>,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<SwapParameters>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityAccounts {
    #[prost(bytes="vec", tag="1")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub position: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub token_a_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub token_b_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub token_a_vault: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token_b_vault: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub token_a_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub token_b_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub position_nft_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="11")]
    pub token_a_program: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub token_b_program: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub event_authority: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub program: ::prost::alloc::vec::Vec<u8>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityAccounts {
    #[prost(bytes="vec", tag="1")]
    pub pool_authority: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub position: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub token_a_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub token_b_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token_a_vault: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub token_b_vault: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub token_a_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub token_b_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub position_nft_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="11")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub token_a_program: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub token_b_program: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub event_authority: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="15")]
    pub program: ::prost::alloc::vec::Vec<u8>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAccounts {
    #[prost(bytes="vec", tag="1")]
    pub pool_authority: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub input_token_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub output_token_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub token_a_vault: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token_b_vault: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub token_a_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub token_b_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub payer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub token_a_program: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="11")]
    pub token_b_program: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", optional, tag="12")]
    pub referral_token_account: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="13")]
    pub event_authority: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub program: ::prost::alloc::vec::Vec<u8>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityParameters {
    #[prost(string, tag="1")]
    pub liquidity_delta: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub token_a_amount_threshold: u64,
    #[prost(uint64, tag="3")]
    pub token_b_amount_threshold: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityParameters {
    #[prost(string, tag="1")]
    pub liquidity_delta: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub token_a_amount_threshold: u64,
    #[prost(uint64, tag="3")]
    pub token_b_amount_threshold: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapParameters {
    #[prost(uint64, tag="1")]
    pub amount_in: u64,
    #[prost(uint64, tag="2")]
    pub minimum_amount_out: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapResult {
    #[prost(uint64, tag="1")]
    pub output_amount: u64,
    #[prost(string, tag="2")]
    pub next_sqrt_price: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub lp_fee: u64,
    #[prost(uint64, tag="4")]
    pub protocol_fee: u64,
    #[prost(uint64, tag="5")]
    pub partner_fee: u64,
    #[prost(uint64, tag="6")]
    pub referral_fee: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub invoke_depth: u32,
    #[prost(oneof="log::Log", tags="3,4,5")]
    pub log: ::core::option::Option<log::Log>,
}
/// Nested message and enum types in `Log`.
pub mod log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Log {
        #[prost(message, tag="3")]
        AddLiquidity(super::AddLiquidityLog),
        #[prost(message, tag="4")]
        RemoveLiquidity(super::RemoveLiquidityLog),
        #[prost(message, tag="5")]
        Swap(super::SwapLog),
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityLog {
    #[prost(bytes="vec", tag="1")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub position: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub params: ::core::option::Option<AddLiquidityParameters>,
    #[prost(uint64, tag="5")]
    pub token_a_amount: u64,
    #[prost(uint64, tag="6")]
    pub token_b_amount: u64,
    #[prost(uint64, tag="7")]
    pub total_amount_a: u64,
    #[prost(uint64, tag="8")]
    pub total_amount_b: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityLog {
    #[prost(bytes="vec", tag="1")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub position: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub params: ::core::option::Option<RemoveLiquidityParameters>,
    #[prost(uint64, tag="5")]
    pub token_a_amount: u64,
    #[prost(uint64, tag="6")]
    pub token_b_amount: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapLog {
    #[prost(bytes="vec", tag="1")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub trade_direction: u32,
    #[prost(bool, tag="3")]
    pub has_referral: bool,
    #[prost(message, optional, tag="4")]
    pub params: ::core::option::Option<SwapParameters>,
    #[prost(message, optional, tag="5")]
    pub result: ::core::option::Option<SwapResult>,
    #[prost(uint64, tag="6")]
    pub actual_amount_in: u64,
    #[prost(uint64, tag="7")]
    pub current_timestamp: u64,
}
