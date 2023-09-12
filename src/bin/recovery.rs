use std::{ops::Add, str::FromStr};

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

    // let mut guardians = vec![];
    // let mut guardian_addresses: Vec<Address> = vec![];
    let guardian_addresses: Vec<Address> = vec![
        "0x506b291e33880fb62383753dfabf7e90d84c7431"
            .parse::<Address>()
            .unwrap(),
        "0x49e9bb63eb14ab8c243b976d0c4ca00a32d45766"
            .parse()
            .unwrap(),
        "0xe97311682fe49adcd4002824697f1966cd982c1c"
            .parse()
            .unwrap(),
        "0x8fe960a674790a03db459f6c46910dd95a1afc5e"
            .parse()
            .unwrap(),
        "0x0605663ec0850b8c6dfc9a9962bc4413259138b1"
            .parse()
            .unwrap(),
    ];
    let guardian_count = 10;
    let threshold = 5;

    // let mut i = 0;
    // while i < threshold {
    //     let item_wallet = LocalWallet::new(&mut thread_rng()).with_chain_id(1337u64);
    //     guardian_addresses.push(item_wallet.address());
    //     guardians.push(item_wallet);
    //     i += 1;
    // }

    let initial_guardian_hash =
        key_store_manager.calc_guardian_hash(guardian_addresses, threshold, None);
    let initial_guardian_hash: H256 = H256::from_slice(&initial_guardian_hash.to_vec());
    let slot = key_store_manager.get_slot("0x9ceb725d9a643d75a6f885e06a1db7ab0eeb549b".parse::<Address>().unwrap().into()/*wallet_signer.address().into()*/, initial_guardian_hash, None);

    println!("guardian hash {:?}", initial_guardian_hash);
    println!("slot {:?}", slot);

    let key = key_store_manager.get_key(slot.clone()).await?;
    println!("key {:?}", key);

    let slot_info = key_store_manager.get_keystore_info(slot.clone()).await?;
    println!("slot info {:?}", slot_info);
    unimplemented!()
}
