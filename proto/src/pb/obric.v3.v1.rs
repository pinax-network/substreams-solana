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
