use clutch_wallet_lib::utils::wallet_lib::WalletLib;
use ethers::types::H256;

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
    //check this value with the result of soulwallet javascript library
    // println!("User operation {:?}", user_op);

    let gas_price = "100"; // gwei
    user_op.max_fee_per_gas = ethers::utils::parse_units(gas_price, "gwei")
        .unwrap()
        .into();
    user_op.max_priority_fee_per_gas = ethers::utils::parse_units(gas_price, "gwei")
        .unwrap()
        .into();

    let ret = wallet_lib
        .estimate_user_operation_gas(user_op, None)
        .await?;

    println!("=============={ret}");
    Ok(())
}
