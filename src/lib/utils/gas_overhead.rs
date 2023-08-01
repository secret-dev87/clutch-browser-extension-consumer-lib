use ethers::types::U256;

use super::bundler::UserOperationTransport;

pub fn calc_gas_overhead(user_op: UserOperationTransport) -> eyre::Result<()> {
    let mut user_op = user_op.clone();
    user_op.pre_verification_gas = user_op
        .pre_verification_gas
        .checked_add(U256::from(10000))
        .unwrap();
    //TODO: soul wallet library is not implemented yet
    Ok(())
}
