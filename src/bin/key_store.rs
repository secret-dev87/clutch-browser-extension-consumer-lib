use std::str::FromStr;

use clutch_wallet_lib::utils::{guardians::*, type_guard::*};
use ethers::types::{Address, Bytes, H256, U256};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    {
        // let key_store_addr = Bytes::from_str("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4").unwrap();
        // let slot =
        //     Bytes::from_str("0xc83ce2ae5f0849408e8430aab8302ba83b8993ccec901ca1a9f5136a3892afbc")
        //         .unwrap();
        // let nonce = 2;
        // let data =
        //     Bytes::from_str("0xf4bc49ca280d06067cb80250fb6d68e297fbf1bcabe8c633d2e27070278e9e75")
        //         .unwrap();
        // let expected_value =
        //     Bytes::from_str("0xa5eb3cebaea0e65f20767bc73febb028c24e68181a215203d805515db724d14c")
        //         .unwrap();

        // let addr = Address::from_slice(&key_store_addr.to_vec());
        // // println!("value {:?}", value);

        // let value = L1KeyStore::get_sig_hash(addr, slot, U256::from(nonce), data);
        // println!("value {:?}, expected value {:?}", value, expected_value);
    }

    let rpc = "http://localhost:8545";
    let key_store_addr = Address::from_str("0x301f7cc88b5ae2a2f8698bd700ad24fa6729a619")
        .expect("Err on Address converting");
    let key_store_manager = L1KeyStore::new(rpc.to_string(), key_store_addr);

    // {
    //     let slot =
    //         Bytes::from_str("0xc83ce2ae5f0849408e8430aab8302ba83b8993ccec901ca1a9f5136a3892afbc")
    //             .unwrap();
    //     let new_key = Address::from_str("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4").unwrap();
    //     let value = key_store_manager
    //         .get_set_key_sig_hash(slot, new_key.into())
    //         .await?;
    //     println!("resut value {:?}", value);
    // }

    {
        let slot =
            Bytes::from_str("0xc83ce2ae5f0849408e8430aab8302ba83b8993ccec901ca1a9f5136a3892afbc")
                .unwrap();
        let new_guardian_hash: H256 = H256::from_str("0xf4bc49ca280d06067cb80250fb6d68e297fbf1bcabe8c633d2e27070278e9e75").unwrap();
        let value = key_store_manager.get_set_guardian_sig_hash(slot, new_guardian_hash).await?;
        println!("resut value {:?}", value);
    }
    Ok(())
}
