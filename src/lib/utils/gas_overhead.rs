use ethers::types::U256;

use super::bundler::UserOperationTransport;

pub fn calc_gas_overhead(user_op: &mut UserOperationTransport) -> eyre::Result<()> {
    user_op.pre_verification_gas = user_op
        .pre_verification_gas
        .checked_add(U256::from(10000))
        .unwrap();

    if user_op.paymaster_and_data.eq("".as_bytes()) {
        user_op.verification_gas_limit = user_op
            .verification_gas_limit
            .checked_add(U256::from(100000))
            .unwrap();
    }

    Ok(())
}
