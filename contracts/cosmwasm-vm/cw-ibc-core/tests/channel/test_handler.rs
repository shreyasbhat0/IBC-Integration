use common::ibc::core::ics02_client::client_type::ClientType;
use common::ibc::core::ics24_host::identifier::PortId;
use cw_common::client_response::LightClientResponse;

use super::*;

#[test]
#[should_panic(expected = "UndefinedConnectionCounterparty")]
fn test_validate_open_try_channel_fail_missing_counterparty() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut msg = MsgChannelOpenTry::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();

    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::ibc_types::IbcModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();

    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    msg.connection_hops_on_b = vec![conn_id.clone()];
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client)
        .unwrap();
    let client_type = ClientType::from(IbcClientType::new("iconclient".to_string()));

    contract
        .store_client_into_registry(
            &mut deps.storage,
            client_type,
            "contractaddress".to_string(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_a;
    let consenus_state = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();

    contract
        .validate_channel_open_try(deps.as_mut(), info.clone(), &msg)
        .unwrap();
}

#[test]
fn test_execute_open_try_from_light_client() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut msg = MsgChannelOpenTry::try_from(raw.clone()).unwrap();

    let counter_party = Counterparty::new(msg.port_id_on_a.clone(), Some(msg.chan_id_on_a.clone()));
    let channel_id_on_b = ChannelId::new(0);
    let conn_id = ConnectionId::new(5);
    msg.connection_hops_on_b = vec![conn_id.clone()];
    let channel_end = ChannelEnd::new(
        State::Uninitialized,
        msg.ordering,
        counter_party,
        msg.connection_hops_on_b.clone(),
        msg.version_supported_on_a.clone(),
    );

    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();

    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::ibc_types::IbcModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();

    let message_info = cw_common::types::MessageInfo {
        sender: info.clone().sender,
        funds: info.clone().funds,
    };

    let expected_data = LightClientResponse {
        message_info,
        ibc_endpoint: cosmwasm_std::IbcEndpoint {
            port_id: PortId::from(msg.port_id_on_b.clone()).to_string(),
            channel_id: channel_id_on_b.clone().to_string(),
        },
    };
    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("action").add_attribute("action", "channel open try execution")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_TRY,
        result,
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            PortId::from(msg.port_id_on_b.clone()),
            channel_id_on_b.clone(),
            channel_end.clone(),
        )
        .unwrap();

    let expected = on_chan_open_try_submessage(
        &channel_end,
        &PortId::from(msg.port_id_on_b.clone()),
        &channel_id_on_b.clone(),
        &conn_id,
    );
    let data = cw_common::xcall_msg::ExecuteMsg::IbcChannelOpen { msg: expected };
    let data = to_binary(&data).unwrap();
    let on_chan_open_try = create_channel_submesssage(
        "contractaddress".to_string(),
        data,
        info.funds,
        EXECUTE_ON_CHANNEL_OPEN_TRY,
    );
    let res = contract.execute_open_try_from_light_client(deps.as_mut(), reply);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap().messages[0], on_chan_open_try)
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_execute_open_try_from_light_client_fail_missing_channel_end() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let msg = MsgChannelOpenTry::try_from(raw.clone()).unwrap();
    let channel_id_on_b = ChannelId::new(0);

    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();

    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::ibc_types::IbcModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();

    let message_info = cw_common::types::MessageInfo {
        sender: info.sender,
        funds: info.funds,
    };
    let expected_data = LightClientResponse {
        message_info,
        ibc_endpoint: cosmwasm_std::IbcEndpoint {
            port_id: PortId::from(msg.port_id_on_b.clone()).to_string(),
            channel_id: channel_id_on_b.clone().to_string(),
        },
    };
    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("action").add_attribute("action", "channel open try execution")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_TRY,
        result,
    };

    contract
        .execute_open_try_from_light_client(deps.as_mut(), reply)
        .unwrap();
}
