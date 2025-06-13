// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleBlockRequest {
    #[prost(message, repeated, tag="6")]
    pub transforms: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(oneof="single_block_request::Reference", tags="3, 4, 5")]
    pub reference: ::core::option::Option<single_block_request::Reference>,
}
/// Nested message and enum types in `SingleBlockRequest`.
pub mod single_block_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct BlockNumber {
        #[prost(uint64, tag="1")]
        pub num: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockHashAndNumber {
        #[prost(uint64, tag="1")]
        pub num: u64,
        #[prost(string, tag="2")]
        pub hash: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cursor {
        #[prost(string, tag="1")]
        pub cursor: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Reference {
        #[prost(message, tag="3")]
        BlockNumber(BlockNumber),
        #[prost(message, tag="4")]
        BlockHashAndNumber(BlockHashAndNumber),
        #[prost(message, tag="5")]
        Cursor(Cursor),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleBlockResponse {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<BlockMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(int64, tag="1")]
    pub start_block_num: i64,
    #[prost(string, tag="2")]
    pub cursor: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub stop_block_num: u64,
    #[prost(bool, tag="4")]
    pub final_blocks_only: bool,
    #[prost(message, repeated, tag="10")]
    pub transforms: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<::prost_types::Any>,
    #[prost(enumeration="ForkStep", tag="6")]
    pub step: i32,
    #[prost(string, tag="10")]
    pub cursor: ::prost::alloc::string::String,
    #[prost(message, optional, tag="12")]
    pub metadata: ::core::option::Option<BlockMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMetadata {
    #[prost(uint64, tag="1")]
    pub num: u64,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub parent_num: u64,
    #[prost(string, tag="4")]
    pub parent_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub lib_num: u64,
    #[prost(message, optional, tag="6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct InfoRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    #[prost(string, tag="1")]
    pub chain_name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub chain_name_aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag="3")]
    pub first_streamable_block_num: u64,
    #[prost(string, tag="4")]
    pub first_streamable_block_id: ::prost::alloc::string::String,
    #[prost(enumeration="info_response::BlockIdEncoding", tag="5")]
    pub block_id_encoding: i32,
    #[prost(string, repeated, tag="10")]
    pub block_features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `InfoResponse`.
pub mod info_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BlockIdEncoding {
        Unset = 0,
        Hex = 1,
        BlockIdEncoding0xHex = 2,
        Base58 = 3,
        Base64 = 4,
        Base64url = 5,
    }
    impl BlockIdEncoding {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BlockIdEncoding::Unset => "BLOCK_ID_ENCODING_UNSET",
                BlockIdEncoding::Hex => "BLOCK_ID_ENCODING_HEX",
                BlockIdEncoding::BlockIdEncoding0xHex => "BLOCK_ID_ENCODING_0X_HEX",
                BlockIdEncoding::Base58 => "BLOCK_ID_ENCODING_BASE58",
                BlockIdEncoding::Base64 => "BLOCK_ID_ENCODING_BASE64",
                BlockIdEncoding::Base64url => "BLOCK_ID_ENCODING_BASE64URL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BLOCK_ID_ENCODING_UNSET" => Some(Self::Unset),
                "BLOCK_ID_ENCODING_HEX" => Some(Self::Hex),
                "BLOCK_ID_ENCODING_0X_HEX" => Some(Self::BlockIdEncoding0xHex),
                "BLOCK_ID_ENCODING_BASE58" => Some(Self::Base58),
                "BLOCK_ID_ENCODING_BASE64" => Some(Self::Base64),
                "BLOCK_ID_ENCODING_BASE64URL" => Some(Self::Base64url),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ForkStep {
    StepUnset = 0,
    StepNew = 1,
    StepUndo = 2,
    StepFinal = 3,
}
impl ForkStep {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ForkStep::StepUnset => "STEP_UNSET",
            ForkStep::StepNew => "STEP_NEW",
            ForkStep::StepUndo => "STEP_UNDO",
            ForkStep::StepFinal => "STEP_FINAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STEP_UNSET" => Some(Self::StepUnset),
            "STEP_NEW" => Some(Self::StepNew),
            "STEP_UNDO" => Some(Self::StepUndo),
            "STEP_FINAL" => Some(Self::StepFinal),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
