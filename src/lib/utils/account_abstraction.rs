use super::bundler::UserOperationTransport;
use ethers::{
    abi::{encode, encode_packed, FixedBytes, Token},
    types::{Address, U256},
    utils::keccak256,
};

pub fn get_user_op_hash(
    user_op: UserOperationTransport,
    entry_point_address: Address,
    chain_id: u64,
) -> eyre::Result<Vec<u8>> {
    let packed_op = pack_user_op(user_op, true)?;

    let user_op_hash = keccak256(packed_op);

    let enc = encode(&[
        Token::FixedBytes(FixedBytes::from(user_op_hash)),
        Token::Address(entry_point_address),
        Token::Uint(U256::from(chain_id)),
    ]);
    Ok(keccak256(enc).to_vec())
}

pub fn pack_user_op(op: UserOperationTransport, for_signature: bool) -> eyre::Result<Vec<u8>> {
    if for_signature == true {
        let ret = encode(&[
            Token::Address(op.sender),
            Token::Uint(op.nonce),
            Token::FixedBytes(keccak256(op.init_code).to_vec()),
            Token::FixedBytes(keccak256(op.call_data).to_vec()),
            Token::Uint(op.call_gas_limit),
            Token::Uint(op.verification_gas_limit),
            Token::Uint(op.pre_verification_gas),
            Token::Uint(op.max_fee_per_gas),
            Token::Uint(op.max_priority_fee_per_gas),
            Token::FixedBytes(keccak256(op.paymaster_and_data).to_vec()),
        ]);
        return Ok(ret);
    } else {
        let ret = encode(&[
            Token::Address(op.sender),
            Token::Uint(op.nonce),
            Token::Bytes(op.init_code.to_vec()),
            Token::Bytes(op.call_data.to_vec()),
            Token::Uint(op.call_gas_limit),
            Token::Uint(op.verification_gas_limit),
            Token::Uint(op.pre_verification_gas),
            Token::Uint(op.max_fee_per_gas),
            Token::Uint(op.max_priority_fee_per_gas),
            Token::Bytes(op.paymaster_and_data.to_vec()),
            Token::Bytes(op.signature.to_vec()),
        ]);
        return Ok(ret);
    };
}

// impl UserOperationTransport {

//     pub fn get_user_op_hash_with_time_range(&self, entry_point_address: Address, chain_id: u64, wallet_owner_address: Address, signature_mode: SignatureMode, validAfter: U256, validUntil: U256) -> eyre::Result<()> {
//         if validUntil < validAfter {
//             return Err(eyre::eyre!("validUntil must be greater than validAfter"));
//         }

//         let hash = self.get_user_op_hash(entry_point_address, chain_id);
//         Ok(())
//     }

//     pub fn alignment(&self) -> eyre::Result<UserOperationTransportString>{
//        Ok(UserOperationTransportString {
//         sender: self.sender,
//         nonce: hex::encode(format!("{:#x}", self.nonce)),
//         init_code: self.init_code,
//         call_data: self.call_data,
//         call_gas_limit: hex::encode(format!("{:#x}", self.call_data)),
//         verification_gas_limit: hex::encode(format!("{:#x}", self.verification_gas_limit)),
//         pre_verification_gas: hex::encode(format!("{:#x}", self.pre_verification_gas)),
//         max_fee_per_gas: hex::encode(format!("{:#x}", self.max_fee_per_gas)),
//         max_priority_fee_per_gas: hex::encode(format!("{:#x}", self.max_priority_fee_per_gas)),
//         paymaster_and_data: self.paymaster_and_data,
//         signature: self.signature,
//        })
//     }

// }
