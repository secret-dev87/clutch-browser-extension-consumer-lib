use clutch_wallet_lib::utils::bundler::{BundlerClient, UserOperationTransport};
use ethers::types::{Address, Bytes, U256};
mod helpers;
use crate::helpers::bundler_test_setup;
use expectest::prelude::*;
use httpmock::prelude::*;
use serde_json::json;

#[tokio::test]
async fn test_bundler_eth_get_chain_id() {
    let (mock_bundler_server, mock_bundler_server_url, provider, entrypoint_address) =
        bundler_test_setup().await;

    let bundler = BundlerClient::new(entrypoint_address, provider, mock_bundler_server_url);

    let eth_chain_id_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST)
            .body_matches(Regex::new(r#""method":"eth_chainId""#).unwrap());
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","result":"0x13881","id":1}"#);
    });

    expect!(bundler.eth_chain_id().await.unwrap()).to(be_equal_to(U256::from(80001)));

    eth_chain_id_mock.assert();
}

#[tokio::test]
async fn test_bundler_eth_supported_entry_points() {
    let (mock_bundler_server, mock_bundler_server_url, provider, entrypoint_address) =
        bundler_test_setup().await;

    let bundler = BundlerClient::new(
        entrypoint_address.clone(),
        provider,
        mock_bundler_server_url,
    );

    let expected_supported_entrypoints: Vec<Address> = vec![entrypoint_address.clone()];

    let json_body = json!(
        {"jsonrpc":"2.0","result":expected_supported_entrypoints,"id":1}
    );

    let supported_entrypoints_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST);
        then.status(200)
            .header("content-type", "application/json")
            .body(json_body.to_string());
    });

    expect!(bundler.eth_supported_entry_points().await.unwrap())
        .to(be_equal_to(expected_supported_entrypoints));

    supported_entrypoints_mock.assert();
}

#[tokio::test]
async fn test_bundler_init() {
    let (mock_bundler_server, mock_bundler_server_url, provider, entrypoint_address) =
        bundler_test_setup().await;

    let mut bundler = BundlerClient::new(
        entrypoint_address.clone(),
        provider,
        mock_bundler_server_url,
    );

    let eth_chain_id_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST)
            .body_matches(Regex::new(r#""method":"eth_chainId""#).unwrap());
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","result":"0x0539","id":1}"#);
    });

    let expected_supported_entrypoints: Vec<Address> = vec![entrypoint_address.clone()];

    let entrypoints_json_body = json!(
        {"jsonrpc":"2.0","result":expected_supported_entrypoints,"id":1}
    );

    let supported_entrypoints_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST)
            .body_matches(Regex::new(r#""method":"eth_supportedEntryPoints""#).unwrap());
        then.status(200)
            .header("content-type", "application/json")
            .body(entrypoints_json_body.to_string());
    });

    bundler.init().await.unwrap();

    expect!(bundler.chain_id).to(be_equal_to(U256::from(1337)));
    expect!(bundler.init).to(be_true());

    eth_chain_id_mock.assert();
    supported_entrypoints_mock.assert();
}

#[tokio::test]
async fn test_bundler_init_unsupported_chain() {
    let (mock_bundler_server, mock_bundler_server_url, provider, entrypoint_address) =
        bundler_test_setup().await;

    let mut bundler = BundlerClient::new(
        entrypoint_address.clone(),
        provider,
        mock_bundler_server_url,
    );

    let eth_chain_id_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST)
            .body_matches(Regex::new(r#""method":"eth_chainId""#).unwrap());
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","result":"0x13881","id":1}"#);
    });

    let error = bundler.init().await.unwrap_err();
    eth_chain_id_mock.assert();

    expect!(error.to_string()).to(be_equal_to(
        "Bundler chain id 80001 does not support target chain id 1337".to_string(),
    ));
}

#[tokio::test]
async fn test_bundler_init_unsupported_entrypoint() {
    let (mock_bundler_server, mock_bundler_server_url, provider, entrypoint_address) =
        bundler_test_setup().await;

    let unsupported_address = Address::random();

    let mut bundler = BundlerClient::new(unsupported_address, provider, mock_bundler_server_url);

    let eth_chain_id_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST)
            .body_matches(Regex::new(r#""method":"eth_chainId""#).unwrap());
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","result":"0x0539","id":1}"#);
    });

    let expected_supported_entrypoints: Vec<Address> = vec![entrypoint_address.clone()];

    let entrypoints_json_body = json!(
        {"jsonrpc":"2.0","result":expected_supported_entrypoints,"id":1}
    );

    let supported_entrypoints_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST)
            .body_matches(Regex::new(r#""method":"eth_supportedEntryPoints""#).unwrap());
        then.status(200)
            .header("content-type", "application/json")
            .body(entrypoints_json_body.to_string());
    });

    let error = bundler.init().await.unwrap_err();
    eth_chain_id_mock.assert();
    supported_entrypoints_mock.assert();

    expect!(error.to_string()).to(be_equal_to(
        format!(
            "Bundler does not support entry point {}",
            unsupported_address
        )
        .to_string(),
    ));
}

#[tokio::test]
async fn test_bundler_eth_send_user_operation() {
    let (mock_bundler_server, mock_bundler_server_url, provider, entrypoint_address) =
        bundler_test_setup().await;

    let bundler = BundlerClient::new(
        entrypoint_address.clone(),
        provider,
        mock_bundler_server_url,
    );

    let test_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST)
            .body_matches(Regex::new(r#""method":"eth_sendUserOperation""#).unwrap());
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","result":"0x13881","id":1}"#);
    });

    let user_op_transport = UserOperationTransport {
        sender: "bb9abfb450cbc64e550b4e41a1b9b7ad8313450c"
            .parse::<Address>()
            .unwrap(),
        nonce: 0.into(),
        init_code: Bytes::new(),
        call_data: Bytes::new(),
        call_gas_limit: 0.into(),
        verification_gas_limit: 0.into(),
        pre_verification_gas: 0.into(),
        max_fee_per_gas: 0.into(),
        max_priority_fee_per_gas: 0.into(),
        paymaster_and_data: Bytes::new(),
        signature: Bytes::new(),
    };

    expect!(bundler
        .eth_send_user_operation(user_op_transport)
        .await
        .unwrap())
    .to(be_equal_to("0x13881".to_string()));

    test_mock.assert();
}

// https://docs.candidewallet.com/bundler/rpc-methods/#estimate-user-operation-gas
// https://github.com/stackup-wallet/stackup-bundler/blob/v0.6.4/pkg/client/client.go#L146
// #[tokio::test]
// async fn test_bundler_eth_estimate_user_operation_gas() {
//     let (mock_bundler_server, mock_bundler_server_url, provider, entrypoint_address) =
//         bundler_test_setup().await;

//     let bundler = BundlerClient::new(
//         entrypoint_address.clone(),
//         provider,
//         mock_bundler_server_url,
//     );

//     let test_mock = mock_bundler_server.mock(|when, then| {
//         when.method(POST)
//             .body_matches(Regex::new(r#""method":"eth_estimateUserOperationGas""#).unwrap());
//         then.status(200)
//             .header("content-type", "application/json")
//             .body(r#"{"jsonrpc":"2.0","result":"[0xCA1]","id":1}"#);
//     });

//     let user_op_transport = UserOperationTransport {
//         sender: "bb9abfb450cbc64e550b4e41a1b9b7ad8313450c"
//             .parse::<Address>()
//             .unwrap(),
//         nonce: 0.into(),
//         init_code: Bytes::new(),
//         call_data: Bytes::new(),
//         call_gas_limit: 0.into(),
//         verification_gas_limit: 0.into(),
//         pre_verification_gas: 0.into(),
//         max_fee_per_gas: 0.into(),
//         max_priority_fee_per_gas: 0.into(),
//         paymaster_and_data: Bytes::new(),
//         signature: Bytes::new(),
//     };

//      let x = bundler
//      .eth_estimate_user_operation_gas(user_op_transport)
//      .await
//      .unwrap();

//     println!("x: {:?}", x);

//     // expect!(bundler
//     //     .eth_estimate_user_operation_gas(user_op_transport)
//     //     .await
//     //     .unwrap())
//     // .to(be_equal_to("0x13881".to_string()));

//     // test_mock.assert();
// }
