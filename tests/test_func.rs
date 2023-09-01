use std::{
    env,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};

use clutch_wallet_lib::utils::{
    constants::ETHER_ZERO_HASH,
    wallet_lib::{WalletLib},
};
use ethers::{
    abi::{FixedBytes, Token},
    types::U256,
};

// #[test]
// fn test_json() {
//     let current = file!();
//     let current_directory = Path::new(current).parent().unwrap();
//     println!("{:?}", current_directory);
//     // let mut file = File::open(full_path).unwrap();
// }

#[tokio::test]
async fn test_create_wallet_user_op() -> eyre::Result<()> {
    let wallet_lib = WalletLib::new(
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

    // let zero_hash = format!("0x{}", "0".repeat(64));
    // let user_op = wallet_lib.create_unsigned_deploy_wallet_user_op(
    // 0,
    // "0x9Ab87E0BdDE47882d7b2De186Ae9A866A292dB7A",
    // zero_hash,
    // "0x",
    // None
    // ).await?;
    //check this value with the result of soulwallet javascript library
    // println!("{:?}", user_op);
    Ok(())
}

// #[test]
// fn test_ethereum_type() {
//     let valid_after = U256::from(11600);
//     let valid_until = U256::from(13700);

//     let validation_data = (valid_until << U256::from(160)) + (valid_after << U256::from(160 + 48));
//     println!("===={}", validation_data.to_string());
// }
