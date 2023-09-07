use chrono::Utc;
use clutch_wallet_lib::utils::wallet_lib::{abi_entry_point, Transaction, WalletLib};
use ethers::{
    abi::{FixedBytes, Token},
    prelude::{rand::thread_rng, *},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Chain, TransactionRequest, H256, U256},
    utils,
};

use std::{str::FromStr, thread, time::Duration};

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

    let wallet_signer = LocalWallet::new(&mut thread_rng()).with_chain_id(1337u64);
    // let wallet_signer = "f7383a67d5e71ba14e2d64fb083e5cd58398b77466ed911631cdb4e792a9c3bc"
    //     .parse::<LocalWallet>()
    //     .unwrap()
    //     .with_chain_id(1337u64);

    println!("{:?}", wallet_signer.address().to_string());
    let mut user_op = wallet_lib
        .create_unsigned_deploy_wallet_user_op(0, wallet_signer.address(), zero_hash, "0x", None)
        .await?;

    let contract_addr = user_op.sender.clone();
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

    let wallet = "08266d3c24aaa41651c4b9bd7ca52c937afe6a535a0f735dd3c7168fe01741ee"
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(1337u64);
    let http = Provider::<Http>::try_from("http://localhost:8545")?;

    let provider = SignerMiddleware::new(http.clone(), wallet.clone());
    let tx = TransactionRequest::new()
        .to(user_op.clone().sender)
        .value(pre_fund_ret.missfund);

    let tx = provider.send_transaction(tx, None).await?.await?;

    let dt = Utc::now();
    let valid_after = dt.timestamp() as u64;
    let valid_until = dt.timestamp() as u64 + 3600;

    let (packed_user_op_hash, validation_data) = wallet_lib
        .pack_user_op_hash(user_op.clone(), Some(valid_after), Some(valid_until))
        .await?;

    // let key_as_bytes = wallet.signer().to_bytes();
    // let private_key = hex::encode(key_as_bytes);
    let signature = sign_message(packed_user_op_hash, wallet_signer.clone()).await?;
    let packed_signature_ret = wallet_lib
        .pack_user_op_signature(signature, validation_data, None)
        .await?;

    user_op.signature = ethers::types::Bytes::from(packed_signature_ret);

    let balance = provider.get_balance(user_op.sender.clone(), None).await?;
    println!(" before balance {:?}", balance);
    let ret = wallet_lib.send_user_operation(user_op.clone()).await?;

    loop {
        let code = provider.get_code(user_op.sender.clone(), None).await?;
        if code.ne(&Bytes::from(b"")) {
            break;
        }
        thread::sleep(Duration::from_secs(3));
        println!("Wait until the tx is minted");
    }
    println!("tx minted");
    let balance_after = provider.get_balance(user_op.sender.clone(), None).await?;
    println!(" after balance {:?}", balance_after);

    println!(
        "contract wallet is deployed to {}",
        hex::encode(contract_addr.clone())
    );
    // send transaction
    let abi_entrypoint = abi_entry_point();
    let new_wallet = LocalWallet::new(&mut thread_rng()).with_chain_id(1337u64);

    println!("new_wallet {:?}", hex::encode(new_wallet.address()));
    let call_data = encode_function_data(
        abi_entrypoint.function("depositTo")?,
        (Token::Address(new_wallet.address())),
    )
    .unwrap();

    let tx = TransactionRequest::new()
        .to(contract_addr.clone())
        .value(U256::from(utils::parse_ether(2)?));

    let tx = provider.send_transaction(tx, None).await?.await?;

    let tx = Transaction {
        to: wallet_lib.entrypoint(),
        value: Some(U256::from(utils::parse_ether(1)?)),
        data: Some(call_data),
        gas_limit: None,
    };

    let mut user_op_tx = wallet_lib
        .from_transaction(
            ethers::utils::parse_units(gas_price, "gwei")
                .unwrap()
                .into(),
            ethers::utils::parse_units(gas_price, "gwei")
                .unwrap()
                .into(),
            contract_addr.clone(),
            vec![tx],
            None,
        )
        .await?;

    let _ = wallet_lib
        .estimate_user_operation_gas(&mut user_op_tx, None)
        .await?;
    // println!("user_op_tx {:?}", user_op_tx);
    let (packed_user_op_hash, validation_data) = wallet_lib
        .pack_user_op_hash(user_op_tx.clone(), Some(valid_after), Some(valid_until))
        .await?;

    let signature = sign_message(packed_user_op_hash, wallet_signer).await?;
    let packed_signature_ret = wallet_lib
        .pack_user_op_signature(signature, validation_data, None)
        .await?;

    user_op_tx.signature = ethers::types::Bytes::from(packed_signature_ret);

    let balance = provider
        .get_balance(user_op_tx.sender.clone(), None)
        .await?;
    println!(" user_op_tx sender {:?}", user_op_tx.sender.clone());
    println!(" before balance {:?}", balance);
    let _ = wallet_lib.send_user_operation(user_op_tx.clone()).await?;
    let user_op_hash_tx = wallet_lib.user_op_hash(user_op_tx.clone()).await?;

    loop {
        let receipt = wallet_lib
            .bundler()
            .eth_get_user_operation_receipt(Bytes::from(user_op_hash_tx.clone()))
            .await?;
        if receipt.is_none() {
            thread::sleep(Duration::from_secs(3));
            println!("Wait until the tx is minted");
        } else {
            break;
        }
    }

    let balance_ = provider
        .get_balance(user_op_tx.sender.clone(), None)
        .await?;
    println!(" after balance {:?}", balance_);
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
