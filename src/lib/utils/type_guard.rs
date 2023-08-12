use ethers::types::{Bytes, U256};
// use std::convert::*;
use std::str::*;
pub fn max_to_uint192(num: Bytes) -> eyre::Result<U256> {
    let bn = U256::from_str(&num.to_string()).unwrap();
    if bn > U256::from_str("0xffffffffffffffffffffffffffffffffffffffffffffffff").unwrap() {
        return Err(eyre::eyre!("num is too large"));
    }
    Ok(bn)
}
