#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreCode {
    #[prost(bytes = "vec", tag = "1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "2")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// Source is a valid absolute HTTPS URI to the contract's source code, optional
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    /// Builder is a valid docker image name with tag, optional
    ///
    /// InstantiatePermission to apply on contract creation, optional
    ///   AccessConfig InstantiatePermission = 5;
    #[prost(string, tag = "4")]
    pub builder: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateContract {
    #[prost(bytes = "vec", tag = "1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    /// Admin is an optional address that can execute migrations
    ///   bytes admin = 2 [(gogoproto.casttype) = "github.com/cosmos/cosmos-sdk/types.AccAddress"];
    #[prost(string, tag = "2")]
    pub callback_code_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub code_id: u64,
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub init_msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "6")]
    pub init_funds: ::prost::alloc::vec::Vec<crate::cosmos::base::v1beta1::Coin>,
    #[prost(bytes = "vec", tag = "7")]
    pub callback_sig: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContract {
    #[prost(bytes = "vec", tag = "1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub contract: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub callback_code_hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub sent_funds: ::prost::alloc::vec::Vec<crate::cosmos::base::v1beta1::Coin>,
    #[prost(bytes = "vec", tag = "6")]
    pub callback_sig: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTypeParam {
    #[prost(enumeration = "AccessType", tag = "1")]
    pub value: i32,
}
/// CodeInfo is data for the uploaded contract WASM code
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub creator: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    ///     AccessConfig instantiate_config = 5 [(gogoproto.nullable) = false];
    #[prost(string, tag = "4")]
    pub builder: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCustomInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub enclave_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
}
/// ContractInfo stores a WASM contract instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfo {
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub creator: ::prost::alloc::vec::Vec<u8>,
    ///     bytes admin = 3 [(gogoproto.casttype) = "github.com/cosmos/cosmos-sdk/types.AccAddress"];
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// never show this in query results, just use for sorting
    /// (Note: when using json tag "-" amino refused to serialize it...)
    ///
    ///     bytes init_msg = 5 [(gogoproto.casttype) = "encoding/json.RawMessage"];
    ///
    ///     AbsoluteTxPosition last_updated = 7;
    ///     uint64 previous_code_id = 8 [(gogoproto.customname) = "PreviousCodeID"];
    #[prost(message, optional, tag = "5")]
    pub created: ::core::option::Option<AbsoluteTxPosition>,
}
/// AbsoluteTxPosition can be used to sort contracts
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// hex-encode key to read it better (this is often ascii)
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// base64-encode raw value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
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
}
/// GenesisState - genesis state of x/wasm
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    ///     Params params = 1 [(gogoproto.nullable) = false];
    #[prost(message, repeated, tag = "2")]
    pub codes: ::prost::alloc::vec::Vec<Code>,
    #[prost(message, repeated, tag = "3")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
    #[prost(message, repeated, tag = "4")]
    pub sequences: ::prost::alloc::vec::Vec<Sequence>,
}
/// Code struct encompasses CodeInfo and CodeBytes
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sequence {
    #[prost(bytes = "vec", tag = "1")]
    pub id_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub value: u64,
}
/// QueryContractInfoRequest is the request type for the Query/ContractInfo RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractInfoRequest {
    /// address is the address of the contract to query
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
/// QueryContractInfoResponse is the response type for the Query/ContractInfo RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractInfoResponse {
    /// address is the address of the contract
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractHistoryRequest {
    /// address is the address of the contract to query
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractsByCodeRequest {
    /// grpc-gateway_out does not support Go style CodID
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
}
/// ContractInfoWithAddress adds the address (key) to the ContractInfo representation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfoWithAddress {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractsByCodeResponse {
    #[prost(message, repeated, tag = "1")]
    pub contract_infos: ::prost::alloc::vec::Vec<ContractInfoWithAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySmartContractStateRequest {
    /// address is the address of the contract
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub query_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractAddressByLabelRequest {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractKeyRequest {
    /// address is the address of the contract
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractHashRequest {
    /// address is the address of the contract
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySmartContractStateResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeRequest {
    /// grpc-gateway_out does not support Go style CodID
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeInfoResponse {
    /// id for legacy support
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub creator: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub builder: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeResponse {
    #[prost(message, optional, tag = "1")]
    pub code_info: ::core::option::Option<CodeInfoResponse>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodesResponse {
    #[prost(message, repeated, tag = "1")]
    pub code_infos: ::prost::alloc::vec::Vec<CodeInfoResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractAddressByLabelResponse {
    /// address is the address of the contract
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractKeyResponse {
    /// address is the address of the contract
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractHashResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
}
/// DecryptedAnswer is a struct that represents a decrypted tx-query
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
    #[prost(message, repeated, tag = "5")]
    pub output_logs: ::prost::alloc::vec::Vec<crate::cosmos::base::abci::v1beta1::StringEvent>,
    #[prost(bytes = "vec", tag = "6")]
    pub output_error: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub plaintext_error: ::prost::alloc::string::String,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query provides defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        /// Query contract
        pub async fn contract_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractInfoRequest>,
        ) -> Result<tonic::Response<super::QueryContractInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Query/ContractInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query contract
        pub async fn contracts_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractsByCodeRequest>,
        ) -> Result<tonic::Response<super::QueryContractsByCodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/ContractsByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query contract
        pub async fn smart_contract_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySmartContractStateRequest>,
        ) -> Result<tonic::Response<super::QuerySmartContractStateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.compute.v1beta1.Query/SmartContractState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query a specific contract code
        pub async fn code(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCodeRequest>,
        ) -> Result<tonic::Response<super::QueryCodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Query/Code");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query all contract codes on-chain
        pub async fn codes(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::QueryCodesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/secret.compute.v1beta1.Query/Codes");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
