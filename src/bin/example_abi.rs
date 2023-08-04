use chrono::Utc;
use clutch_wallet_lib::utils::wallet_lib::WalletLib;
use ethers::{
    abi::FixedBytes,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Chain, TransactionRequest, H256, U256},
    utils,
};

use std::str::FromStr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut wallet_lib = WalletLib::new(
        "http://localhost:8545",
        "http://localhost:3000/rpc",
        "0x6eca9bac37ba92908805c68c2de7106dd15fde28",
        "0xc4b4f2df5a4936aeda4df93ec203d6c6100bdb7f",
        "0xf16e8831312c0a4b884e49a639083c2ec9cfd4f1",
        "0x861adf70d644dfe2038775f648d2509190ee7579",
        "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789",
        "0x240c9cebe72a7f3010b40b5ef166be1ed56ddf44",
        1337,
    );

    let zero_hash: H256 = [0u8; 32].into();

    let mut user_op = wallet_lib
        .create_unsigned_deploy_wallet_user_op(
            0,
            "0xAcE9A8ff06F144e414D21933AF3c3eF021b2d25b",
            zero_hash,
            "0x",
            None,
        )
        .await?;

    let gas_price = "100"; // gwei
    user_op.max_fee_per_gas = ethers::utils::parse_units(gas_price, "gwei")
        .unwrap()
        .into();
    user_op.max_priority_fee_per_gas = ethers::utils::parse_units(gas_price, "gwei")
        .unwrap()
        .into();

    let _ = wallet_lib
        .estimate_user_operation_gas(&mut user_op, None)
        .await?;

    let pre_fund_ret = wallet_lib.pre_fund(user_op.clone()).await?;

    // let provider = Provider
    //0x0E18aAb6d107E59C46936eb53eB3D4ccfcF5b5AE

    let wallet = "9131cbad8e7e5369d670022b3ea8781d7ed83681a3720d3e551833816c2fb6a4"
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(1337u64);
    let http = Provider::<Http>::try_from("http://localhost:8545")?;

    let provider = SignerMiddleware::new(http.clone(), wallet.clone());
    let tx = TransactionRequest::new()
        .to(user_op.clone().sender)
        .value(U256::from(utils::parse_ether(1)?));

    let tx = provider.send_transaction(tx, None).await?.await?;

    // println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    let dt = Utc::now();
    let valid_after = dt.timestamp() as u64;
    let valid_until = dt.timestamp() as u64 + 3600;

    let (packed_user_op_hash, validation_data) = wallet_lib
        .pack_user_op_hash(user_op.clone(), Some(valid_after), Some(valid_until))
        .await?;

    // let key_as_bytes = wallet.signer().to_bytes();
    // let private_key = hex::encode(key_as_bytes);
    let signature = sign_message(packed_user_op_hash, wallet).await?;
    let packed_signature_ret = wallet_lib
        .pack_user_op_signature(signature, validation_data, None)
        .await?;

    user_op.signature = ethers::types::Bytes::from(packed_signature_ret);

    let balance = provider.get_balance(user_op.sender.clone(), None).await?;
    println!(" ===== {:?}", balance);
    // let ret = wallet_lib.send_user_operation(user_op).await?;
    // println!(" ===== {:?}", ret);
    Ok(())
}

async fn sign_message(msg: Vec<u8>, wallet: LocalWallet) -> eyre::Result<(Vec<u8>)> {
    let signature = wallet.sign_message(msg).await?;
    let mut signature_for_eth_sign = [
        H256(U256::from(signature.r).try_into().unwrap()).to_fixed_bytes(),
        H256(U256::from(signature.s).try_into().unwrap()).to_fixed_bytes(),
    ]
    .concat();
    signature_for_eth_sign.extend_from_slice(&[(signature.v as u8)]);
    Ok(signature_for_eth_sign)
}
