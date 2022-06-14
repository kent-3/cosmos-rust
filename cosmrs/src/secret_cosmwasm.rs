use crate::{
    proto,
    tx::{Msg, MsgProto},
    AccountId, ErrorReport, Result,
};
use proto::cosmwasm::secret::compute::v1beta1 as cosmwasm_proto;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// MsgStoreCode submit Wasm code to the system
pub struct MsgStoreCode {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,
    /// WASMByteCode can be raw or gzip compressed
    pub wasm_byte_code: Vec<u8>,
    /// Source is a valid absolute HTTPS URI to the contract's source code, optional
    pub source: Option<String>,
    /// Builder is a valid docker image name with tag, optional
    pub builder: Option<String>,
}

impl MsgProto for cosmwasm_proto::MsgStoreCode {
    const TYPE_URL: &'static str = "/secret.compute.v1beta1.MsgStoreCode";
}

impl Msg for MsgStoreCode {
    type Proto = cosmwasm_proto::MsgStoreCode;
}

impl TryFrom<cosmwasm_proto::MsgStoreCode> for MsgStoreCode {
    type Error = ErrorReport;

    fn try_from(proto: cosmwasm_proto::MsgStoreCode) -> Result<MsgStoreCode> {
        Ok(MsgStoreCode {
            sender: AccountId::new("secret", &proto.sender)?,
            wasm_byte_code: proto.wasm_byte_code,
            source: if proto.source.is_empty() {
                None
            } else {
                Some(proto.source)
            },
            builder: if proto.builder.is_empty() {
                None
            } else {
                Some(proto.builder)
            },
        })
    }
}

impl From<MsgStoreCode> for cosmwasm_proto::MsgStoreCode {
    fn from(msg: MsgStoreCode) -> cosmwasm_proto::MsgStoreCode {
        cosmwasm_proto::MsgStoreCode {
            sender: msg.sender.to_bytes(),
            wasm_byte_code: msg.wasm_byte_code,
            source: msg.source.unwrap_or_default(),
            builder: msg.builder.unwrap_or_default(),
        }
    }
}

/// MsgInstantiateContract initialise a contract from some stored code
#[derive(Debug, Clone)]
pub struct MsgInstantiateContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,
    /// The code id of the stored contract code
    pub code_id: u64,
    /// The label to give this contract instance
    pub label: String,
    /// The initialisation message to pass to the contract init method
    pub init_msg: Vec<u8>,
}

impl MsgProto for cosmwasm_proto::MsgInstantiateContract {
    const TYPE_URL: &'static str = "/secret.compute.v1beta1.MsgInstantiateContract";
}

impl Msg for MsgInstantiateContract {
    type Proto = cosmwasm_proto::MsgInstantiateContract;
}

impl TryFrom<cosmwasm_proto::MsgInstantiateContract> for MsgInstantiateContract {
    type Error = ErrorReport;

    fn try_from(proto: cosmwasm_proto::MsgInstantiateContract) -> Result<MsgInstantiateContract> {
        Ok(MsgInstantiateContract {
            sender: AccountId::new("secret", &proto.sender)?,
            code_id: proto.code_id,
            label: proto.label,
            init_msg: proto.init_msg,
        })
    }
}

impl From<MsgInstantiateContract> for cosmwasm_proto::MsgInstantiateContract {
    fn from(msg: MsgInstantiateContract) -> cosmwasm_proto::MsgInstantiateContract {
        cosmwasm_proto::MsgInstantiateContract {
            sender: msg.sender.to_bytes(),
            callback_code_hash: "".to_string(),
            code_id: msg.code_id,
            label: msg.label,
            init_msg: msg.init_msg,
            init_funds: vec![],
            callback_sig: vec![],
        }
    }
}

/// MsgExecuteContract execute a contract handle function
#[derive(Debug, Clone)]
pub struct MsgExecuteContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,
    /// The contract instance to execute the message on
    pub contract: AccountId,
    /// The message to pass to the contract handle method
    pub msg: Vec<u8>,
}

impl MsgProto for cosmwasm_proto::MsgExecuteContract {
    const TYPE_URL: &'static str = "/secret.compute.v1beta1.MsgExecuteContract";
}

impl Msg for MsgExecuteContract {
    type Proto = cosmwasm_proto::MsgExecuteContract;
}

impl TryFrom<cosmwasm_proto::MsgExecuteContract> for MsgExecuteContract {
    type Error = ErrorReport;

    fn try_from(proto: cosmwasm_proto::MsgExecuteContract) -> Result<MsgExecuteContract> {
        Ok(MsgExecuteContract {
            sender: AccountId::new("secret", &proto.sender)?,
            contract: AccountId::new("secret", &proto.contract)?,
            msg: proto.msg,
        })
    }
}

impl From<MsgExecuteContract> for cosmwasm_proto::MsgExecuteContract {
    fn from(msg: MsgExecuteContract) -> cosmwasm_proto::MsgExecuteContract {
        cosmwasm_proto::MsgExecuteContract {
            sender: msg.sender.to_bytes(),
            contract: msg.contract.to_bytes(),
            msg: msg.msg,
            callback_code_hash: "".to_string(),
            sent_funds: vec![],
            callback_sig: vec![],
        }
    }
}
