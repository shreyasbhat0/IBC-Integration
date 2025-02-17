use anyhow::Error as AppError;
use common::constants::ICON_CLIENT_TYPE;
use common::ibc::events::IbcEventType;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use common::{icon::icon::lightclient::v1::ClientState as RawClientState, traits::AnyTypes};
use cosmwasm_std::{Addr, Empty};
use cw_common::raw_types::client::{RawMsgCreateClient, RawMsgUpdateClient};
use cw_common::{core_msg as CoreMsg, hex_string::HexString};
use cw_ibc_core::{execute, instantiate, query, reply};
use cw_icon_light_client;
use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
use prost::Message;
use test_utils::{get_event, get_event_name, get_test_signed_headers};

pub struct TestContext {
    pub app: App,
    pub lightclient: Addr,
    pub ibc_core: Addr,
    pub sender: Addr,
}

fn mock_app() -> App {
    App::default()
}

pub fn ibc_core_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    Box::new(contract)
}

pub fn lightclient_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_icon_light_client::contract::execute,
        cw_icon_light_client::contract::instantiate,
        cw_icon_light_client::contract::query,
    );
    Box::new(contract)
}

pub fn setup_test() -> TestContext {
    let mut router = mock_app();
    let sender = Addr::unchecked("sender");
    let light_client_code_id = router.store_code(lightclient_contract());
    let ibc_core_code_id = router.store_code(ibc_core_contract());

    let light_client_addr = router
        .instantiate_contract(
            light_client_code_id,
            sender.clone(),
            &cw_common::client_msg::InstantiateMsg::default(),
            &[],
            "LightClient",
            Some(sender.clone().to_string()),
        )
        .unwrap();

    let ibc_core_addr = router
        .instantiate_contract(
            ibc_core_code_id,
            sender.clone(),
            &cw_common::core_msg::InstantiateMsg {},
            &[],
            "IBCCore",
            Some(sender.clone().to_string()),
        )
        .unwrap();

    TestContext {
        app: router,
        lightclient: light_client_addr,
        ibc_core: ibc_core_addr,
        sender,
    }
}

pub fn call_register_client_type(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.ibc_core.clone(),
        &CoreMsg::ExecuteMsg::RegisterClient {
            client_type: ICON_CLIENT_TYPE.to_string(),
            client_address: ctx.lightclient.clone(),
        },
        &[],
    );
    res
}

pub fn call_create_client(
    ctx: &mut TestContext,
    signed_header: RawSignedHeader,
) -> Result<AppResponse, AppError> {
    let client_state: RawClientState = signed_header
        .header
        .clone()
        .unwrap()
        .to_client_state(1000000, 5);
    let consensus_state = signed_header.header.unwrap().to_consensus_state();
    let msg_raw = RawMsgCreateClient {
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consensus_state.to_any()),
        signer: "signer".to_owned(),
    };
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.ibc_core.clone(),
        &CoreMsg::ExecuteMsg::CreateClient {
            msg: HexString::from_bytes(&msg_raw.encode_to_vec()),
        },
        &[],
    );

    res
}

pub fn call_update_client(
    ctx: &mut TestContext,
    signed_header: RawSignedHeader,
    client_id: &str,
) -> Result<AppResponse, AppError> {
    let msg_raw = RawMsgUpdateClient {
        client_id: client_id.to_string(),
        header: Some(signed_header.to_any()),
        signer: "signer".to_owned(),
    };
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.ibc_core.clone(),
        &CoreMsg::ExecuteMsg::UpdateClient {
            msg: HexString::from_bytes(&msg_raw.encode_to_vec()),
        },
        &[],
    );

    res
}

#[test]
fn test_register_client() {
    let mut ctx = setup_test();
    let result = call_register_client_type(&mut ctx);
    assert!(result.is_ok());
}

#[test]
fn test_create_client() {
    let mut ctx = setup_test();
    call_register_client_type(&mut ctx).unwrap();
    let signed_headers: Vec<RawSignedHeader> = get_test_signed_headers();
    let result = call_create_client(&mut ctx, signed_headers[0].clone());
    println!("{:?}", &result);
    assert!(result.is_ok());
}
#[test]
fn test_update_client() {
    let mut ctx = setup_test();
    call_register_client_type(&mut ctx).unwrap();
    let signed_headers: Vec<RawSignedHeader> = get_test_signed_headers();
    let response = call_create_client(&mut ctx, signed_headers[0].clone()).unwrap();
    let event = get_event(&response, &get_event_name(IbcEventType::CreateClient)).unwrap();
    let client_id = event.get("client_id").unwrap();
    let result = call_update_client(&mut ctx, signed_headers[1].clone(), client_id);
    println!("{:?}", &result);
    assert!(result.is_ok());
}
