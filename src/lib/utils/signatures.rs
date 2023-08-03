use ethers::{
    abi::FixedBytes,
    abi::{encode, encode_packed, Address, Token},
    prelude::k256::pkcs8::der::Encode,
    providers::Map,
    types::{Bytes, H256, U256, U64},
    utils::keccak256,
};

use std::str::FromStr;

use super::guardians::{GuardHookInputData, HookInputData};

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
    } else if valid_after != None || valid_until != None {
        return Err(eyre::eyre!("invalid valid after and valid until"));
    } else {
        return Ok((user_op_hash, ethers::types::Bytes::from(b"").to_vec()));
    }

    let valid_after = U256::from(valid_after.unwrap());
    let valid_until = U256::from(valid_until.unwrap());

    let validation_data = (valid_until << U256::from(160)) + (valid_after << U256::from(160 + 48));

    println!("validation data{:?}", validation_data);
    let packed_user_op_hash = encode(&[
        Token::FixedBytes(FixedBytes::from(user_op_hash)),
        Token::Uint(validation_data),
    ]);

    return Ok((
        keccak256(packed_user_op_hash).to_vec(),
        H256(validation_data.try_into().unwrap())
            .to_fixed_bytes()
            .to_vec(),
    ));
}

pub fn pack_signature(
    signature: Vec<u8>,
    validation_data: U256,
    hook_input_data: Option<HookInputData>,
) -> eyre::Result<Vec<u8>> {
    //check the signautre is only eoa signature
    let mut guard_hook_input_data_bytes: Vec<u8> = Vec::new();
    if signature.len() != 65 {
        return Err(eyre::eyre!("Invalid EOA signature"));
    }

    if let Some(hook_input_data) = hook_input_data {
        if hook_input_data.guard_hooks.len() == 0 {
            return Err(eyre::eyre!("Invalid guard hook input data"));
        }

        for guardian_hook_plugin_address in hook_input_data.guard_hooks.iter() {
            guard_hook_input_data_bytes.extend_from_slice(guardian_hook_plugin_address.as_bytes());
            let guard_hook_input_data = (&hook_input_data)
                .input_data
                .get(&guardian_hook_plugin_address)
                .unwrap();
            let guard_hook_input_data_length = U64::from(guard_hook_input_data.len() / 2);
            guard_hook_input_data_bytes
                .extend_from_slice(guard_hook_input_data_length.to_string().as_bytes());
            guard_hook_input_data_bytes.extend_from_slice(guard_hook_input_data.as_slice());
        }
    }

    if guard_hook_input_data_bytes.len() > 0 || validation_data.gt(&U256::from(0)) {
        let sign_type = ethers::types::Bytes::from_str("0x01").unwrap();
        let validation_data_hex: H256 = H256(validation_data.try_into().unwrap());
        let mut packed_signature: Vec<u8> = Vec::new();
        packed_signature.extend_from_slice(&sign_type.to_vec());
        packed_signature.extend_from_slice(validation_data_hex.as_bytes());
        packed_signature.extend_from_slice(&signature);
        packed_signature.extend_from_slice(&guard_hook_input_data_bytes);

        Ok(packed_signature)
    } else {
        Ok(signature)
    }
}
