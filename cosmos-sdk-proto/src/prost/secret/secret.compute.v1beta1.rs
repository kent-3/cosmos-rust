#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTypeParam {
    #[prost(enumeration = "AccessType", tag = "1")]
    pub value: i32,
}
/// CodeInfo is data for the uploaded contract WASM code
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub creator: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub builder: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractKey {
    #[prost(bytes = "vec", tag = "1")]
    pub og_contract_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub current_contract_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub current_contract_key_proof: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCustomInfo {
    #[prost(message, optional, tag = "1")]
    pub enclave_key: ::core::option::Option<ContractKey>,
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
}
/// ContractInfo stores a WASM contract instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfo {
    /// CodeID is the reference to the stored Wasm code
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// Creator address who initially instantiated the contract
    #[prost(bytes = "vec", tag = "2")]
    pub creator: ::prost::alloc::vec::Vec<u8>,
    /// Label is mandatory metadata to be stored with a contract instance.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// Created Tx position when the contract was instantiated.
    #[prost(message, optional, tag = "5")]
    pub created: ::core::option::Option<AbsoluteTxPosition>,
    #[prost(string, tag = "6")]
    pub ibc_port_id: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "7")]
    pub admin: ::prost::alloc::string::String,
    /// Proof that enclave executed the instantiate command
    #[prost(bytes = "vec", tag = "8")]
    pub admin_proof: ::prost::alloc::vec::Vec<u8>,
}
/// AbsoluteTxPosition can be used to sort contracts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbsoluteTxPosition {
    /// BlockHeight is the block the contract was created at
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    /// TxIndex is a monotonic counter within the block (actual transaction index, or gas consumed)
    #[prost(uint64, tag = "2")]
    pub tx_index: u64,
}
/// Model is a struct that holds a KV pair
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// hex-encode key to read it better (this is often ascii)
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// base64-encode raw value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// ContractCodeHistoryEntry metadata to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCodeHistoryEntry {
    #[prost(enumeration = "ContractCodeHistoryOperationType", tag = "1")]
    pub operation: i32,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "2")]
    pub code_id: u64,
    /// Updated Tx position when the operation was executed.
    #[prost(message, optional, tag = "3")]
    pub updated: ::core::option::Option<AbsoluteTxPosition>,
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessType {
    Undefined = 0,
    Nobody = 1,
    OnlyAddress = 2,
    Everybody = 3,
}
impl AccessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessType::Undefined => "UNDEFINED",
            AccessType::Nobody => "NOBODY",
            AccessType::OnlyAddress => "ONLY_ADDRESS",
            AccessType::Everybody => "EVERYBODY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNDEFINED" => Some(Self::Undefined),
            "NOBODY" => Some(Self::Nobody),
            "ONLY_ADDRESS" => Some(Self::OnlyAddress),
            "EVERYBODY" => Some(Self::Everybody),
            _ => None,
        }
    }
}
/// ContractCodeHistoryOperationType actions that caused a code change
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractCodeHistoryOperationType {
    /// ContractCodeHistoryOperationTypeUnspecified placeholder for empty value
    Unspecified = 0,
    /// ContractCodeHistoryOperationTypeInit on chain contract instantiation
    Init = 1,
    /// ContractCodeHistoryOperationTypeMigrate code migration
    Migrate = 2,
    /// ContractCodeHistoryOperationTypeGenesis based on genesis data
    Genesis = 3,
}
impl ContractCodeHistoryOperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractCodeHistoryOperationType::Unspecified => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED"
            }
            ContractCodeHistoryOperationType::Init => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT",
            ContractCodeHistoryOperationType::Migrate => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
            }
            ContractCodeHistoryOperationType::Genesis => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT" => Some(Self::Init),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE" => Some(Self::Migrate),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS" => Some(Self::Genesis),
            _ => None,
        }
    }
}
/// GenesisState - genesis state of x/wasm
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    ///     Params params = 1 \[(gogoproto.nullable) = false\];
    #[prost(message, repeated, tag = "2")]
    pub codes: ::prost::alloc::vec::Vec<Code>,
    #[prost(message, repeated, tag = "3")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
    #[prost(message, repeated, tag = "4")]
    pub sequences: ::prost::alloc::vec::Vec<Sequence>,
}
/// Code struct encompasses CodeInfo and CodeBytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Code {
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    #[prost(message, optional, tag = "2")]
    pub code_info: ::core::option::Option<CodeInfo>,
    #[prost(bytes = "vec", tag = "3")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// Contract struct encompasses ContractAddress, ContractInfo, and ContractState
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(bytes = "vec", tag = "1")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
    #[prost(message, repeated, tag = "3")]
    pub contract_state: ::prost::alloc::vec::Vec<Model>,
    #[prost(message, optional, tag = "4")]
    pub contract_custom_info: ::core::option::Option<ContractCustomInfo>,
}
/// Sequence id and value of a counter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sequence {
    #[prost(bytes = "vec", tag = "1")]
    pub id_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub value: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreCode {
    /// sender is the canonical address of the sender
    #[prost(bytes = "vec", tag = "1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "2")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// Source is a valid absolute HTTPS URI to the contract's source code, optional
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    /// Builder is a valid docker image name with tag, optional
    #[prost(string, tag = "4")]
    pub builder: ::prost::alloc::string::String,
}
/// MsgStoreCodeResponse returns store result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreCodeResponse {
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateContract {
    /// sender is the canonical address of the sender
    #[prost(bytes = "vec", tag = "1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub callback_code_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub code_id: u64,
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// init_msg is an encrypted input to pass to the contract on init
    #[prost(bytes = "vec", tag = "5")]
    pub init_msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "6")]
    pub init_funds: ::prost::alloc::vec::Vec<crate::cosmos::base::v1beta1::Coin>,
    /// used internally for encryption, should always be empty in a signed transaction
    #[prost(bytes = "vec", tag = "7")]
    pub callback_sig: ::prost::alloc::vec::Vec<u8>,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "8")]
    pub admin: ::prost::alloc::string::String,
}
/// MsgInstantiateContractResponse return instantiation result data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateContractResponse {
    /// Address is the bech32 address of the new contract instance.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Data contains base64-encoded bytes to returned from the contract
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContract {
    /// sender is the canonical address of the sender
    #[prost(bytes = "vec", tag = "1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    /// contract is the canonical address of the contract
    #[prost(bytes = "vec", tag = "2")]
    pub contract: ::prost::alloc::vec::Vec<u8>,
    /// msg is an encrypted input to pass to the contract on execute
    #[prost(bytes = "vec", tag = "3")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// used internally for encryption, should always be empty in a signed transaction
    #[prost(string, tag = "4")]
    pub callback_code_hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub sent_funds: ::prost::alloc::vec::Vec<crate::cosmos::base::v1beta1::Coin>,
    /// used internally for encryption, should always be empty in a signed transaction
    #[prost(bytes = "vec", tag = "6")]
    pub callback_sig: ::prost::alloc::vec::Vec<u8>,
}
/// MsgExecuteContractResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContractResponse {
    /// Data contains base64-encoded bytes to returned from the contract
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgMigrateContract runs a code upgrade/ downgrade for a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMigrateContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// CodeID references the new WASM code
    #[prost(uint64, tag = "3")]
    pub code_id: u64,
    /// msg is an encrypted input to pass to the contract on migration
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// used internally for encryption, should always be empty in a signed transaction
    #[prost(bytes = "vec", tag = "7")]
    pub callback_sig: ::prost::alloc::vec::Vec<u8>,
    /// used internally for encryption, should always be empty in a signed transaction
    #[prost(string, tag = "8")]
    pub callback_code_hash: ::prost::alloc::string::String,
}
/// MsgMigrateContractResponse returns contract migration result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMigrateContractResponse {
    /// Data contains same raw bytes returned as data from the wasm contract.
    /// (May be empty)
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUpdateAdmin sets a new admin for a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAdmin {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// NewAdmin address to be set
    #[prost(string, tag = "2")]
    pub new_admin: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
    /// used internally for encryption, should always be empty in a signed transaction
    #[prost(bytes = "vec", tag = "7")]
    pub callback_sig: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUpdateAdminResponse returns empty data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAdminResponse {}
/// MsgClearAdmin removes any admin stored for a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClearAdmin {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
    /// used internally for encryption, should always be empty in a signed transaction
    #[prost(bytes = "vec", tag = "7")]
    pub callback_sig: ::prost::alloc::vec::Vec<u8>,
}
/// MsgClearAdminResponse returns empty data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClearAdminResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the wasm Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// StoreCode to submit Wasm code to the system
        pub async fn store_code(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgStoreCode>,
        ) -> std::result::Result<tonic::Response<super::MsgStoreCodeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Msg/StoreCode");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("secret.compute.v1beta1.Msg", "StoreCode"));
            self.inner.unary(req, path, codec).await
        }
        ///  Instantiate creates a new smart contract instance for the given code id.
        pub async fn instantiate_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInstantiateContract>,
        ) -> std::result::Result<
            tonic::Response<super::MsgInstantiateContractResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Msg/InstantiateContract",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Msg",
                "InstantiateContract",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Execute submits the given message data to a smart contract
        pub async fn execute_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExecuteContract>,
        ) -> std::result::Result<tonic::Response<super::MsgExecuteContractResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Msg/ExecuteContract");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Msg",
                "ExecuteContract",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Migrate runs a code upgrade/ downgrade for a smart contract
        pub async fn migrate_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMigrateContract>,
        ) -> std::result::Result<tonic::Response<super::MsgMigrateContractResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Msg/MigrateContract");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Msg",
                "MigrateContract",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateAdmin sets a new   admin for a smart contract
        pub async fn update_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateAdmin>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateAdminResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Msg/UpdateAdmin");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("secret.compute.v1beta1.Msg", "UpdateAdmin"));
            self.inner.unary(req, path, codec).await
        }
        /// ClearAdmin removes any admin stored for a smart contract
        pub async fn clear_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgClearAdmin>,
        ) -> std::result::Result<tonic::Response<super::MsgClearAdminResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Msg/ClearAdmin");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("secret.compute.v1beta1.Msg", "ClearAdmin"));
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySecretContractRequest {
    /// address is the bech32 human readable address of the contract
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub query: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryByLabelRequest {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryByContractAddressRequest {
    /// address is the bech32 human readable address of the contract
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryByCodeIdRequest {
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySecretContractResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryContractInfoResponse is the response type for the Query/ContractInfo RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractInfoResponse {
    /// contract_address is the bech32 human readable address of the contract
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
}
/// ContractInfoWithAddress adds the contract address to the ContractInfo representation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfoWithAddress {
    /// contract_address is the bech32 human readable address of the contract
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractsByCodeIdResponse {
    #[prost(message, repeated, tag = "1")]
    pub contract_infos: ::prost::alloc::vec::Vec<ContractInfoWithAddress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeInfoResponse {
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// creator is the bech32 human readable address of the contract
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub code_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub builder: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeResponse {
    #[prost(message, optional, tag = "1")]
    pub code_info: ::core::option::Option<CodeInfoResponse>,
    #[prost(bytes = "vec", tag = "2")]
    pub wasm: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodesResponse {
    #[prost(message, repeated, tag = "1")]
    pub code_infos: ::prost::alloc::vec::Vec<CodeInfoResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractAddressResponse {
    /// address is the bech32 human readable address of the contract
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractLabelResponse {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeHashResponse {
    #[prost(string, tag = "1")]
    pub code_hash: ::prost::alloc::string::String,
}
/// DecryptedAnswer is a struct that represents a decrypted tx-query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptedAnswer {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub input: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub output_data: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub output_data_as_string: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptedAnswers {
    #[prost(message, repeated, tag = "1")]
    pub answers: ::prost::alloc::vec::Vec<DecryptedAnswer>,
    #[prost(message, repeated, tag = "2")]
    pub output_logs:
        ::prost::alloc::vec::Vec<crate::cosmos::base::abci::v1beta1::StringEvent>,
    #[prost(string, tag = "3")]
    pub output_error: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub plaintext_error: ::prost::alloc::string::String,
}
/// QueryContractHistoryRequest is the request type for the Query/ContractHistory
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractHistoryRequest {
    /// address is the address of the contract to query
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryContractHistoryResponse is the response type for the
/// Query/ContractHistory RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<ContractCodeHistoryEntry>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Query contract info by address
        pub async fn contract_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryByContractAddressRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryContractInfoResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Query/ContractInfo");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Query",
                "ContractInfo",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Query code info by id
        pub async fn contracts_by_code_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryByCodeIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryContractsByCodeIdResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/ContractsByCodeId",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Query",
                "ContractsByCodeId",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Query secret contract
        pub async fn query_secret_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySecretContractRequest>,
        ) -> std::result::Result<tonic::Response<super::QuerySecretContractResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/QuerySecretContract",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Query",
                "QuerySecretContract",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Query a specific contract code by id
        pub async fn code(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryByCodeIdRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryCodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Query/Code");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("secret.compute.v1beta1.Query", "Code"));
            self.inner.unary(req, path, codec).await
        }
        /// Query all contract codes on-chain
        pub async fn codes(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::QueryCodesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Query/Codes");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("secret.compute.v1beta1.Query", "Codes"));
            self.inner.unary(req, path, codec).await
        }
        /// Query code hash by contract address
        pub async fn code_hash_by_contract_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryByContractAddressRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryCodeHashResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/CodeHashByContractAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Query",
                "CodeHashByContractAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Query code hash by code id
        pub async fn code_hash_by_code_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryByCodeIdRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryCodeHashResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/CodeHashByCodeId",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Query",
                "CodeHashByCodeId",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Query contract label by address
        pub async fn label_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryByContractAddressRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryContractLabelResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/LabelByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Query",
                "LabelByAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Query contract address by label
        pub async fn address_by_label(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryByLabelRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryContractAddressResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/AddressByLabel",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Query",
                "AddressByLabel",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ContractHistory gets the contract code history
        pub async fn contract_history(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractHistoryRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryContractHistoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/ContractHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.compute.v1beta1.Query",
                "ContractHistory",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
