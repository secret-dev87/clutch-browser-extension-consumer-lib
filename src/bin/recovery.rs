use std::str::FromStr;

use clutch_wallet_lib::utils::{guardians::*, type_guard::*};
use ethers::{
    prelude::{rand::thread_rng, *},
    signers::LocalWallet,
    types::{Address, Bytes, H256, U256},
    
};






#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc = "http://localhost:8545";
    let key_store_addr = Address::from_str("0x301f7cc88b5ae2a2f8698bd700ad24fa6729a619")
        .expect("Err on Address converting");
    let key_store_manager = L1KeyStore::new(rpc.to_string(), key_store_addr);

    let wallet_signer = LocalWallet::new(&mut thread_rng()).with_chain_id(1337u64);

    println!("new_wallet {:?}", hex::encode(wallet_signer.address()));

    let mut guardians = vec![];
    let mut guardian_addresses: Vec<Address> = vec![];
    let guardian_count = 10;
    let threshold = 5;

    let mut i = 0;
    while i < threshold {
        let item_wallet = LocalWallet::new(&mut thread_rng()).with_chain_id(1337u64);
        guardian_addresses.push(item_wallet.address());
        guardians.push(item_wallet);
        i += 1;
    }

    println!("guardian addresses {:?}", guardian_addresses);
    let initial_guardian_hash = key_store_manager.calc_guardian_hash(guardian_addresses, threshold, None);
    let initial_guardian_hash: H256 = H256::from_slice(&initial_guardian_hash.to_vec());
    let slot = key_store_manager.get_slot(wallet_signer.address().into(), initial_guardian_hash, None);

    println!("guardian hash {:?}", initial_guardian_hash);
    println!("slot {:?}", slot);
    unimplemented!()
}
