use chrono::Utc;
use clutch_wallet_lib::utils::wallet_lib::WalletLib;
use ethers::{
    prelude::SignerMiddleware,
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
        "0x721ebda8f508e9de26d0a522d29679df34c7872b",
        "0xa195222110a5cda82934735a5cb1f3599df5f5a8",
        "0x6c3a9f19aa9c3c659fbf0ad6721ed48aba48f239",
        "0x9670a43e5e820e920c10d3bb2f018571fedb9b6e",
        "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789",
        "0xbd9fe2927251593d1073bb4e5538c76a3afd816c",
        1337,
    );

    let zero_hash: H256 = [0u8; 32].into();

    let mut user_op = wallet_lib
        .create_unsigned_deploy_wallet_user_op(
            0,
            "0xa98AEf09F2C28d858EeAC19B5611345DEF4188C3",
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
    let wallet = "5329363575be632f77c012647ecba5c2f1915617904214bbc57d92e4f4016007"
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(1337 as u64);
    let http = Provider::<Http>::try_from("http://localhost:8545")?;
    let provider = SignerMiddleware::new(http.clone(), wallet.clone());

    let tx = TransactionRequest::new()
        .to(Address::from_str("0x0e5Dfa093a7b8274cCbee4c27F2f08368ec99CE5").unwrap())
        .value(U256::from(utils::parse_ether(1)?));

    let tx = provider.send_transaction(tx, None).await?.await?;

    // println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);

    let dt = Utc::now();
    let valid_after = dt.timestamp() as u64;
    let valid_until = dt.timestamp() as u64 + 3600;

    let (packed_user_op_hash, validation_data) = wallet_lib
        .pack_user_op_hash(user_op.clone(), Some(valid_after), Some(valid_until))
        .await?;

    println!(
        "{:?}{:?}",
        ethers::types::Bytes::from(packed_user_op_hash),
        ethers::types::Bytes::from(validation_data)
    );
    Ok(())
}
