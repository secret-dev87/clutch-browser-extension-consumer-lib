use std::time::Duration;

use clutch_wallet_lib::{
    build::get_compiled_contracts,
    utils::{
        bundler::{self, BundlerClient, UserOperationTransport},
        clutch_wallet_lib::ClutchWalletLib,
        deploy_wallet_factory::{self, WalletFactory},
        guardians::Guardian,
        signatures::SignatureMode,
    },
};
use ethers::{
    prelude::{
        rand::{self, Rng},
        LocalWallet, Provider, SignerMiddleware,
    },
    providers::Middleware,
    signers::Signer,
    types::{Address, U256},
    utils::parse_units,
};

mod helpers;
use httpmock::{Method::POST, MockServer, Regex};
use serde_json::json;

use crate::helpers::{
    create_guardian_addresses, deploy_contract_no_args, deploy_price_oracle,
    deploy_token_paymaster, get_abi_and_bytecode, get_ganache_instance, guardian_factory_abi,
};

#[tokio::test]
async fn test_explore() {
    // Spawn a ganache instance
    let ganache = get_ganache_instance().await.lock().await;

    // A provider is an Ethereum JsonRPC client
    let provider: Provider<ethers::providers::Http> = Provider::try_from(ganache.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10));

    // Get the first wallet managed by ganache
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let wallet = wallet.clone().with_chain_id(chain_id);

    // Compile solidity project
    let project = get_compiled_contracts().await.lock().await;

    // Deploy SingletonFactory contract
    let singleton_factory_address = deploy_contract_no_args(
        "SingletonFactory",
        project.clone(),
        wallet.clone(),
        provider.clone(),
    )
    .await;

    println!(
        "SingletonFactory contract address {:?}",
        singleton_factory_address
    );

    // Deploy ClutchWallet contract
    let clutch_wallet_logic_address = deploy_contract_no_args(
        "ClutchWallet",
        project.clone(),
        wallet.clone(),
        provider.clone(),
    )
    .await;

    println!(
        "ClutchWallet contract address {:?}",
        clutch_wallet_logic_address
    );

    // Deploy Entrypoint contract
    let entrypoint_address = deploy_contract_no_args(
        "EntryPoint",
        project.clone(),
        wallet.clone(),
        provider.clone(),
    )
    .await;

    println!("EntryPoint contract address {:?}", entrypoint_address);

    // Deploy USDC Contract
    let usdc_contract_address =
        deploy_contract_no_args("USDCoin", project.clone(), wallet.clone(), provider.clone()).await;

    println!("USDCoin contract address {:?}", usdc_contract_address);

    // Deploy DAI Contract
    let dai_contract_address =
        deploy_contract_no_args("USDCoin", project.clone(), wallet.clone(), provider.clone()).await;

    println!("Dai contract address {:?}", dai_contract_address);

    // Deploy MockOracle Contract
    let mock_oracle_address = deploy_contract_no_args(
        "MockOracle",
        project.clone(),
        wallet.clone(),
        provider.clone(),
    )
    .await;

    println!("MockOracle contract address {:?}", mock_oracle_address);

    // Deploy PriceOracle Contract
    let price_oracle_address = deploy_price_oracle(
        project.clone(),
        wallet.clone(),
        provider.clone(),
        mock_oracle_address,
    )
    .await;

    println!("PriceOracle contract address {:?}", price_oracle_address);

    let salt = generate_bytes32_salt().to_vec();

    let (wallet_factory_abi, wallet_factory_bytecode) =
        get_abi_and_bytecode(String::from("ClutchWalletFactory"), project.clone());
    let wallet_factory_config = deploy_wallet_factory::WalletFactoryConfig {
        abi: wallet_factory_abi,
        bytecode: wallet_factory_bytecode,
    };

    let client = SignerMiddleware::new(provider.clone(), wallet.clone()).into();

    let wallet_factory = WalletFactory::new(singleton_factory_address);

    let wallet_factory_address = wallet_factory
        .deploy(
            clutch_wallet_logic_address,
            salt.clone(),
            wallet_factory_config.clone(),
            client,
        )
        .await
        .unwrap();

    println!("wallet factory address {:?}", wallet_factory_address);

    let mock_bundler_server = MockServer::start();
    let mock_bundler_server_url = mock_bundler_server.base_url();

    let eth_chain_id_mock = mock_bundler_server.mock(|when, then| {
        when.method(POST)
            .body_matches(Regex::new(r#""method":"eth_chainId""#).unwrap());
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"jsonrpc":"2.0","result":"0x0539","id":1}"#);
    });

    let expected_supported_entrypoints: Vec<Address> = vec![entrypoint_address];

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

    // Init bundler
    let mut bundler = BundlerClient::new(
        entrypoint_address.clone(),
        provider.clone(),
        mock_bundler_server_url,
    );

    bundler.init().await.unwrap();

    eth_chain_id_mock.assert();
    supported_entrypoints_mock.assert();

    // // Deploy TokenPaymaster
    // let token_paymaster_address = deploy_token_paymaster(
    //     project.clone(),
    //     wallet.clone(),
    //     provider.clone(),
    //     entrypoint_address.clone(),
    //     wallet_factory_address,
    // )
    // .await;

    // println!("TokenPaymaster address {:?}", token_paymaster_address);

    // // Connect to the TokenPaymaster contract
    // let client = SignerMiddleware::new(provider.clone(), wallet.clone()).into();
    // let token_paymaster_contract =
    //     token_paymaster::TokenPaymaster::new(token_paymaster_address, client);

    // // Set the token on the TokenPaymaster
    // token_paymaster_contract
    //     .set_token(
    //         vec![usdc_contract_address, dai_contract_address],
    //         vec![price_oracle_address, price_oracle_address],
    //     )
    //     .await
    //     .unwrap();

    // // Set the stake on the TokenPaymaster
    // // let stake_amount = U256::from(10).pow(U256::from(17));

    // let balance_from = provider.get_balance(wallet.address(), None).await.unwrap();
    // println!("balance from {:?}", balance_from);

    // let stake_amount = U256::from(10).pow(U256::from(17));
    // println!("stake amount {:?}", stake_amount);
    // let x = token_paymaster_contract
    //     .add_stake(1u32)
    //     .value(stake_amount)
    //     .from(wallet.address())
    //     .legacy()
    //     .send()
    //     .await.expect_err("should fail");
    //     // .confirmations(1)
    //     // .await.unwrap();

    // println!("x {:?}", x);

    // Set the deposit on the TokenPaymaster
    // let y = token_paymaster_contract
    //     .deposit()
    //     .value(stake_amount)
    //     .from(wallet.address())
    //     .await
    //     .expect_err("should fail");

    // println!("y {:?}", y);

    // Deploy Guardian logic contract
    let guardian_logic_address = deploy_contract_no_args(
        "GuardianMultiSigWallet",
        project.clone(),
        wallet.clone(),
        provider.clone(),
    )
    .await;

    println!(
        "GuardianMultiSigWallet contract address {:?}",
        guardian_logic_address
    );

    // Deploy Estimate Gas helper contract
    let estimate_gas_helper_address = deploy_contract_no_args(
        "EstimateGasHelper",
        project.clone(),
        wallet.clone(),
        provider.clone(),
    )
    .await;

    println!(
        "EstimateGasHelper contract address {:?}",
        estimate_gas_helper_address
    );

    // Deploy SignatureTest contract
    let signature_test_address = deploy_contract_no_args(
        "SignatureTest",
        project.clone(),
        wallet.clone(),
        provider.clone(),
    )
    .await;

    println!(
        "SignatureTest contract address {:?}",
        signature_test_address
    );

    // Activate with Eth
    // -------------------------------------------------------------------------------------- //

    // let upgrade_delay = 30;
    // let guardian_delay = 30;

    let (guardian_addresses, guardian_wallets) = create_guardian_addresses(10).await;

    let guardian = Guardian::new(singleton_factory_address);
    let threshold = U256::from(5);

    let guardian_proxy_config = guardian_factory_abi();

    let (guardian_address, guardian_init_code) = guardian
        .calculate_guardian_and_init_code(
            guardian_logic_address,
            guardian_addresses,
            threshold,
            salt.clone(),
            guardian_proxy_config.clone(),
        )
        .await
        .unwrap();

    println!("guardian address {:?}", guardian_address);

    // let clutch_wallet_lib = ClutchWalletLib::new(
    //     singleton_factory_address,
    //     wallet_factory_config,
    //     guardian_proxy_config,
    // );

    // let upgrade_delay = U256::from(30);
    // let guardian_delay = U256::from(30);

    // let wallet_address = clutch_wallet_lib
    //     .calculate_wallet_address(
    //         clutch_wallet_logic_address,
    //         entrypoint_address,
    //         wallet.address(),
    //         upgrade_delay,
    //         guardian_delay,
    //         guardian_address,
    //         salt.clone(),
    //     )
    //     .await
    //     .unwrap();

    // println!("wallet address {:?}", wallet_address);

    // let max_fee_per_gas = parse_units("100000000000", "wei").unwrap().into();
    // let max_priority_fee_per_gas = parse_units("100000000000", "wei").unwrap().into();
    // let paymaster_and_data = vec![];

    // let mut activate_op = clutch_wallet_lib
    //     .activate_wallet_op(
    //         clutch_wallet_logic_address,
    //         entrypoint_address,
    //         wallet.address(),
    //         upgrade_delay,
    //         guardian_delay,
    //         guardian_address,
    //         paymaster_and_data,
    //         max_fee_per_gas,
    //         max_priority_fee_per_gas,
    //         salt.clone(),
    //     )
    //     .await
    //     .unwrap();

    // let activate_op = estimate_user_operation_gas(&mut bundler, activate_op.clone()).await;

    unimplemented!()

    // let user_op_hash = activate_op.get_user_op_hash_with_time_range(
    //     entrypoint_address,
    //     chain_id,
    //     wallet.address(),
    //     SignatureMode::Owner,
    //     U256::from(0),
    //     U256::from(0),
    // ).unwrap();
}

async fn estimate_user_operation_gas(
    bundler: &mut BundlerClient,
    mut op: UserOperationTransport,
) -> UserOperationTransport {
    let gas_estimate = bundler
        .eth_estimate_user_operation_gas(op.clone())
        .await
        .expect("failed to estimate gas");
    op.pre_verification_gas = gas_estimate.pre_verification_gas;
    op.verification_gas_limit = gas_estimate.verification_gas;

    if op.call_gas_limit == 0.into() {
        op.call_gas_limit = gas_estimate.call_gas_limit;
    }

    op
}

fn generate_bytes32_salt() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut salt = [0u8; 32];
    rng.fill(&mut salt);
    salt
}
