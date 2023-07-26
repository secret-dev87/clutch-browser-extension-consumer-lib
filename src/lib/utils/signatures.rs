use std::str::from_utf8;

use ethers::{
    abi::FixedBytes,
    abi::{encode, encode_packed, Address, Token},
    prelude::k256::pkcs8::der::Encode,
    providers::Map,
    types::{Bytes, U256},
    utils::keccak256,
};

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
        return Ok((user_op_hash, "0x0".as_bytes().to_vec()));
    }

    let valid_after = U256::from(valid_after.unwrap());
    let valid_until = U256::from(valid_until.unwrap());

    let validation_data = (valid_until << U256::from(160)) + (valid_after << U256::from(160 + 48));
    let packed_user_op_hash = encode_packed(&[
        Token::FixedBytes(user_op_hash),
        Token::Uint(validation_data),
    ])
    .unwrap();

    return Ok((
        keccak256(packed_user_op_hash).to_vec(),
        validation_data.to_string().as_bytes().to_vec(),
    ));
}

pub fn pack_signature(
    signature: Vec<u8>,
    validation_data: Vec<u8>,
    hook_input_data: Option<HookInputData>,
) -> eyre::Result<Vec<u8>> {
    //check the signautre is only eoa signature
    let mut guard_hook_input_data_bytes: Vec<u8> = Vec::new();
    if signature.len() != 132 {
        return Err(eyre::eyre!("Invalid EOA signature"));
    }

    if let Some(hook_input_data) = hook_input_data {
        if hook_input_data.guard_hooks.len() == 0 {
            return Err(eyre::eyre!("Invalid guard hook input data"));
        }

        for guardian_hook_plugin_address in hook_input_data.guard_hooks.iter() {
            guard_hook_input_data_bytes
                .append(&mut guardian_hook_plugin_address.to_string().into_bytes());
            let guard_hook_input_data = (&hook_input_data)
                .input_data
                .get(&guardian_hook_plugin_address)
                .unwrap();
            let guard_hook_input_data_length = format!("{:012x}", guard_hook_input_data.len() / 2) ;
            guard_hook_input_data_bytes.append(&mut guard_hook_input_data_length.into_bytes());
            guard_hook_input_data_bytes.append(&mut guard_hook_input_data.clone());
        }
    }

    let validation_data = U256::from_str_radix(from_utf8(&validation_data).unwrap(), 16).unwrap();
    if guard_hook_input_data_bytes.len() > 0 || validation_data.gt(&U256::from(0)) {
        let sign_type = "01";
        let validation_data_hex = format!("{:030x}", validation_data);
        let packed_signature = format!("0x{sign_type}{validation_data_hex}{:?}{:?}", signature, guard_hook_input_data_bytes);
        Ok(packed_signature.into_bytes())
    }else {
        Ok(signature)
    }    
}
