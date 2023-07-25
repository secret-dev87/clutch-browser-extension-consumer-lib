use std::{
    env,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};

use clutch_wallet_lib::utils::wallet_lib::{abi_soul_wallet, WalletLib};

#[test]
fn test_json() {
    let current = file!();
    let current_directory = Path::new(current).parent().unwrap();
    println!("{:?}", current_directory);
    // let mut file = File::open(full_path).unwrap();
}

#[tokio::test]
async fn test_create_wallet() {
    let wallet_lib = WalletLib::new(
        "http://localhost:8545",
        "http://localhost:3000/rpc",
        "0x721ebda8f508e9de26d0a522d29679df34c7872b",
        "0x9670a43e5e820e920c10d3bb2f018571fedb9b6e",
        "0x6c3a9f19aa9c3c659fbf0ad6721ed48aba48f239",
        "0x9670a43e5e820e920c10d3bb2f018571fedb9b6e",
    );

    let abc = wallet_lib.create_unsigned_deploy_wallet_user_op("0xFFFF",
    "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789",
    "ABC",
    "0xFFFF").await;    
}