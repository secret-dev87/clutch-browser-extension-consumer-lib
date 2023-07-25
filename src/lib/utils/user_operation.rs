// use ethers::{types::{Address, U256}, utils::keccak256};
// use serde::{Serialize, Deserialize};

// use super::{bundler::UserOperationTransport, signatures::SignatureMode};

// pub struct UserOpTypeComponent {
//     pub _type: String,
//     pub name: String,
// }
// pub struct UserOpType {
//     components : Vec<UserOpTypeComponent>,
//     name: String,
//     _type: String
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct UserOperationTransportString {
//     pub sender: ::ethers::core::types::Address,
//     pub nonce: String,
//     pub init_code: ::ethers::core::types::Bytes,
//     pub call_data: ::ethers::core::types::Bytes,
//     pub call_gas_limit: String,
//     pub verification_gas_limit: String,
//     pub pre_verification_gas: String,
//     pub max_fee_per_gas: String,
//     pub max_priority_fee_per_gas: String,
//     pub paymaster_and_data: ::ethers::core::types::Bytes,
//     pub signature: ::ethers::core::types::Bytes,
// }

// impl UserOperationTransport {

//     pub fn get_user_op_hash_with_time_range(&self, entry_point_address: Address, chain_id: u64, wallet_owner_address: Address, signature_mode: SignatureMode, validAfter: U256, validUntil: U256) -> eyre::Result<()> {
//         if validUntil < validAfter {
//             return Err(eyre::eyre!("validUntil must be greater than validAfter"));
//         }

//         let hash = self.get_user_op_hash(entry_point_address, chain_id);
//         Ok(())
//     }

//     pub fn get_user_op_hash(&self, entry_point_address: Address, chain_id: u64) -> eyre::Result<String> {
//         let packed_op = self.pack_user_op(self, true)?;
//         let user_op_hash = keccak256(packed_op)?;

//     }

//     pub fn pack_user_op(&self, op: UserOperationTransport, for_signature: bool) -> eyre::Result<String> {
//       let user_op_type = UserOpType {
//         components: vec![
//             UserOpTypeComponent {
//                 _type: "address".to_string(),
//                 name: op.sender.to_string(),
//             },
//         ],
//         name: "userOp".to_string(),
//         _type: "tuple".to_string(),
//       };
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
