use ethers::{abi::FixedBytes, types::{U256}, abi::{encode_packed, encode, Token}, utils::keccak256, prelude::k256::pkcs8::der::Encode};

pub enum SignatureMode {
    Owner,
    Guardian,
}

pub fn pack_user_op_hash(
    user_op_hash: Vec<u8>,
    valid_after: Option<u64>,
    valid_until: Option<u64>,
) -> eyre::Result<(Vec<u8>, Vec<u8>)> {
    if user_op_hash.len() != 32 {
        return Err(eyre::eyre!("invalid userOpHash"));
    }

    if valid_after != None && valid_until != None {
        if valid_after.gt(&valid_until) {
            return Err(eyre::eyre!("invalid valid after and valid until"));
        }
    } else if  valid_after != None || valid_until != None {
        return Err(eyre::eyre!("invalid valid after and valid until"));
    } else {
        return Ok((user_op_hash, "0x0".as_bytes().to_vec()));
    }

    let valid_after = U256::from(valid_after.unwrap());
    let valid_until = U256::from(valid_until.unwrap());

    let validation_data = (valid_until<< U256::from(160)) + (valid_after << U256::from(160 + 48));    
    let packed_user_op_hash = encode_packed(&[
        Token::FixedBytes(user_op_hash),
        Token::Uint(validation_data)
    ]).unwrap();

    return Ok((keccak256(packed_user_op_hash).to_vec(), validation_data.to_string().as_bytes().to_vec()));
}
