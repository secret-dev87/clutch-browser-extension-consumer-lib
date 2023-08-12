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
