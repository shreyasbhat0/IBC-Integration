use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs::{self, read_dir, File},
    io::{self, ErrorKind, Read},
    path::PathBuf,
};

use common::ibc::events::IbcEventType;
use cw_multi_test::AppResponse;
use ibc_proto::ibc::core::channel::v1::Packet;
use serde::Deserialize;

use common::icon::icon::types::v1::BtpHeader;
use common::icon::icon::types::v1::MerkleNode;
use common::icon::icon::types::v1::SignedHeader;
use cosmwasm_std::{Attribute, Event};

pub mod constants;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TestHeader {
    pub main_height: u64,
    pub round: u32,
    pub next_proof_context_hash: String,
    pub network_section_to_root: Vec<TestMerkleNode>,
    pub network_id: u64,
    pub update_number: u64,
    pub prev_network_section_hash: String,
    pub message_count: u64,
    pub message_root: String,
    pub next_validators: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct TestHeaderData {
    pub signed_header: TestSignedHeader,
    pub encoded_protobuf: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TestSignedHeader {
    #[serde(rename(deserialize = "BTPHeader"))]
    pub btp_header: TestHeader,
    pub signature: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TestMerkleNode {
    pub dir: i32,
    pub value: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TestMessageData {
    #[serde(rename(deserialize = "signed_header"))]
    pub signed_header: TestSignedHeader,
    pub btp_header_encoded: String,
    pub commitment_key: String,
    pub commitment_path: String,
    pub height: u64,
    pub messages: Vec<String>,
    pub packet: TestPacket,
    pub packet_encoded: String,
    pub proof: Vec<TestMerkleNode>,
    pub validators: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TestPacket {
    pub data: String,
    pub destination_channel: String,
    pub destination_port: String,
    pub sequence: u64,
    pub source_channel: String,
    pub source_port: String,
}

impl TryFrom<&TestMerkleNode> for MerkleNode {
    type Error = hex::FromHexError;

    fn try_from(value: &TestMerkleNode) -> Result<Self, Self::Error> {
        let node = MerkleNode {
            dir: value.dir,
            value: hex::decode(value.value.replace("0x", "")).unwrap(),
        };
        Ok(node)
    }
}

impl TryFrom<TestHeader> for BtpHeader {
    type Error = hex::FromHexError;

    fn try_from(value: TestHeader) -> Result<Self, Self::Error> {
        let btp_header = BtpHeader {
            main_height: value.main_height,
            message_count: value.message_count,
            message_root: hex::decode(value.message_root.replace("0x", ""))?,
            network_id: value.network_id,
            network_section_to_root: value
                .network_section_to_root
                .into_iter()
                .map(|tn| {
                    let node: MerkleNode = (&tn).try_into().unwrap();
                    node
                })
                .collect(),
            next_proof_context_hash: hex::decode(value.next_proof_context_hash.replace("0x", ""))?,
            next_validators: value
                .next_validators
                .into_iter()
                .map(|v| hex::decode(v.replace("0x", "")).unwrap())
                .collect(),
            prev_network_section_hash: hex::decode(
                value.prev_network_section_hash.replace("0x", ""),
            )?,
            round: value.round,
            update_number: value.update_number,
        };
        Ok(btp_header)
    }
}

impl TryFrom<TestSignedHeader> for SignedHeader {
    type Error = hex::FromHexError;

    fn try_from(value: TestSignedHeader) -> Result<Self, Self::Error> {
        let btp_header: BtpHeader = value.btp_header.try_into()?;
        let signatures = value
            .signature
            .iter()
            .map(|s| hex::decode(s.replace("0x", "")).unwrap())
            .collect();
        return Ok(SignedHeader {
            header: Some(btp_header),
            signatures,
        });
    }
}

impl TryFrom<TestPacket> for Packet {
    type Error = hex::FromHexError;

    fn try_from(value: TestPacket) -> Result<Self, Self::Error> {
        let p = Packet {
            data: hex::decode(value.data).unwrap(),
            destination_channel: value.destination_channel,
            destination_port: value.destination_port,
            sequence: value.sequence,
            source_channel: value.source_channel,
            source_port: value.source_port,
            timeout_timestamp: 0,
            timeout_height: None,
        };
        Ok(p)
    }
}

pub fn load_test_headers() -> Vec<TestHeaderData> {
    return load_test_data::<TestHeaderData>("test_data/test_headers.json");
}

pub fn load_test_messages() -> Vec<TestMessageData> {
    return load_test_data::<TestMessageData>("test_data/test_messages.json");
}

pub fn load_test_data<T: for<'a> Deserialize<'a>>(path: &str) -> Vec<T> {
    let mut root = get_project_root().unwrap();
    root.push(path);
    let mut file = File::open(root).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let data: Vec<T> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    data
}

pub fn get_test_headers() -> Vec<BtpHeader> {
    return load_test_headers()
        .into_iter()
        .map(|th| {
            let btp: BtpHeader = th.signed_header.btp_header.try_into().unwrap();
            btp
        })
        .collect::<Vec<BtpHeader>>();
}

pub fn get_test_signed_headers() -> Vec<SignedHeader> {
    return load_test_headers()
        .into_iter()
        .map(|th| {
            let btp: SignedHeader = th.signed_header.try_into().unwrap();
            btp
        })
        .collect::<Vec<SignedHeader>>();
}

pub fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}

pub fn to_attribute_map(attrs: &Vec<Attribute>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for attr in attrs {
        map.insert(attr.key.clone(), attr.value.clone());
    }
    return map;
}

pub fn get_event(res: &AppResponse, event: &str) -> Option<HashMap<String, String>> {
    let event = res
        .events
        .iter()
        .filter(|e| e.ty == event)
        .collect::<Vec<&Event>>();
    if event.len() > 0 {
        let map = to_attribute_map(&event[0].attributes);
        return Some(map);
    }
    None
}

pub fn get_event_name(event_type: IbcEventType) -> String {
    format!("wasm-{}", event_type.as_str())
}
