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
        "0x9cef0d6889154f56fc266c9e54250cbc5c0c9bfe",
        "0x861adf70d644dfe2038775f648d2509190ee7579",
        "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789",
        "0x240c9cebe72a7f3010b40b5ef166be1ed56ddf44",
        1337,
    );

    // let paymaster_and_data = WalletLib::paymaster_and_data(
    //     Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f").unwrap(),
    //     Address::from_str("0xee4d0d07318dd076d588bccdf2383275b499f29f").unwrap(),
    // ).await?;
    // println!("paymaster {:?}", paymaster_and_data);
    unimplemented!()
}
