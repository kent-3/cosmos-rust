#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuoteReport {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub version: u64,
    #[prost(string, tag="4")]
    pub isv_enclave_quote_status: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub platform_info_blob: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub isv_enclave_quote_body: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="7")]
    pub advisory_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuoteReportBody {
    #[prost(string, tag="1")]
    pub mr_enclave: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mr_signer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub report_data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuoteReportData {
    #[prost(uint64, tag="1")]
    pub version: u64,
    #[prost(uint64, tag="2")]
    pub sign_type: u64,
    #[prost(message, optional, tag="3")]
    pub report_body: ::core::option::Option<QuoteReportBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsedAttestationReport {
    #[prost(bytes="vec", tag="1")]
    pub report: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub signing_cert: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sgxec256Signature {
    #[prost(string, tag="1")]
    pub gx: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub gy: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformInfoBlob {
    #[prost(uint32, tag="1")]
    pub sgx_epid_group_flags: u32,
    #[prost(uint32, tag="2")]
    pub sgx_tcb_evaluation_flags: u32,
    #[prost(uint32, tag="3")]
    pub pse_evaluation_flags: u32,
    #[prost(string, tag="4")]
    pub latest_equivalent_tcb_psvn: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub latest_pse_isvsvn: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub latest_psda_svn: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub xeid: u32,
    #[prost(uint32, tag="8")]
    pub gid: u32,
    #[prost(message, optional, tag="9")]
    pub sgx_ec256_signature_t: ::core::option::Option<Sgxec256Signature>,
}
