use std::str::FromStr;

use clutch_wallet_lib::utils::guardians::*;
use ethers::types::{Bytes, Address, U256};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    
    let key_store_addr = Bytes::from_str("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4").unwrap();
    let slot = Bytes::from_str("0xc83ce2ae5f0849408e8430aab8302ba83b8993ccec901ca1a9f5136a3892afbc").unwrap();
    let nonce = 2;
    let data = Bytes::from_str("0xf4bc49ca280d06067cb80250fb6d68e297fbf1bcabe8c633d2e27070278e9e75").unwrap();
    let expected_value = Bytes::from_str("0xa5eb3cebaea0e65f20767bc73febb028c24e68181a215203d805515db724d14c").unwrap();    
    
    let addr = Address::from_slice(&key_store_addr.to_vec());
    // println!("value {:?}", value);

    let value = L1KeyStore::get_sig_hash(addr, slot, U256::from(nonce), data);
    println!("value {:?}", value);
    Ok(())
}
