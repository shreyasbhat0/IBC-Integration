use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::{ibc::prelude::*, ibc::Height};

use ibc_proto::protobuf::Protobuf;

use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenConfirm as RawMsgConnectionOpenConfirm;

use crate::ibc::core::ics03_connection::error::ConnectionError;
use crate::ibc::core::ics24_host::identifier::ConnectionId;
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;

pub const TYPE_URL: &str = "/ibc.core.connection.v1.MsgConnectionOpenConfirm";

/// Per our convention, this message is sent to chain B.
/// The handler will check proofs of chain A.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgConnectionOpenConfirm {
    /// ConnectionId that chain B has chosen for it's ConnectionEnd
    pub conn_id_on_b: ConnectionId,
    /// proof of ConnectionEnd stored on Chain A during ConnOpenInit
    pub proof_conn_end_on_a: CommitmentProofBytes,
    /// Height at which `proof_conn_end_on_a` in this message was taken
    pub proof_height_on_a: Height,
    pub signer: Signer,
}

impl Msg for MsgConnectionOpenConfirm {
    type Raw = RawMsgConnectionOpenConfirm;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgConnectionOpenConfirm> for MsgConnectionOpenConfirm {}

impl TryFrom<RawMsgConnectionOpenConfirm> for MsgConnectionOpenConfirm {
    type Error = ConnectionError;

    fn try_from(msg: RawMsgConnectionOpenConfirm) -> Result<Self, Self::Error> {
        Ok(Self {
            conn_id_on_b: msg
                .connection_id
                .parse()
                .map_err(ConnectionError::InvalidIdentifier)?,
            proof_conn_end_on_a: msg
                .proof_ack
                .try_into()
                .map_err(|_| ConnectionError::InvalidProof)?,
            proof_height_on_a: msg
                .proof_height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or(ConnectionError::MissingProofHeight)?,
            signer: msg.signer.parse().map_err(ConnectionError::Signer)?,
        })
    }
}

impl From<MsgConnectionOpenConfirm> for RawMsgConnectionOpenConfirm {
    fn from(msg: MsgConnectionOpenConfirm) -> Self {
        RawMsgConnectionOpenConfirm {
            connection_id: msg.conn_id_on_b.as_str().to_string(),
            proof_ack: msg.proof_conn_end_on_a.into(),
            proof_height: Some(msg.proof_height_on_a.into()),
            signer: msg.signer.to_string(),
        }
    }
}
