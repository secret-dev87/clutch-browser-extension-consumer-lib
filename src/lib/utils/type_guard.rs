use ethers::{
    abi::Address,
    types::{Bytes, H256, U256},
};

use std::str::*;
pub fn max_to_uint192(num: Bytes) -> eyre::Result<U256> {
    let bn = U256::from_str(&num.to_string()).unwrap();
    if bn > U256::from_str("0xffffffffffffffffffffffffffffffffffffffffffffffff").unwrap() {
        return Err(eyre::eyre!("num is too large"));
    }
    Ok(bn)
}

pub fn address_to_byte32(address: Address) -> Bytes {
    let addr: H256 = address.into();
    Bytes::from(addr.to_fixed_bytes())
}
