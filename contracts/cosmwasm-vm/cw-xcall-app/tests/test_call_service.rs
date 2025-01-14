mod account;
mod setup;
use cosmwasm_std::{
    testing::{mock_env, MOCK_CONTRACT_ADDR},
    Addr,
};

use cw_xcall_app::{instantiate, msg::InstantiateMsg, state::CwCallService};
use setup::test::*;

#[test]

fn proper_instantiate() {
    let mut mock_deps = deps();
    let mock_info = create_mock_info(MOCK_CONTRACT_ADDR, "umlg", 2000);
    let env = mock_env();
    let store = CwCallService::default();

    let res = instantiate(
        mock_deps.as_mut(),
        env,
        mock_info,
        InstantiateMsg {
            timeout_height: 10,
            connection_host: Addr::unchecked("someaddress"),
        },
    )
    .unwrap();

    assert_eq!(res.messages.len(), 0);

    let last_request_id = store
        .query_last_request_id(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(0, last_request_id);

    let owner = store.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(MOCK_CONTRACT_ADDR, owner.to_string())
}

#[test]
#[should_panic(expected = "NotFound")]
fn improper_instantiate() {
    let mock_deps = deps();

    let store = CwCallService::default();

    let last_request_id = store
        .query_last_request_id(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(0, last_request_id);
}
