use ethers::{
    providers::{Http, JsonRpcClient, Middleware, Provider},
    types::{Address, U256},
};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_tuple::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserOperationTransport {
    pub sender: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
    pub init_code: ::ethers::core::types::Bytes,
    pub call_data: ::ethers::core::types::Bytes,
    pub call_gas_limit: ::ethers::core::types::U256,
    pub verification_gas_limit: ::ethers::core::types::U256,
    pub pre_verification_gas: ::ethers::core::types::U256,
    pub max_fee_per_gas: ::ethers::core::types::U256,
    pub max_priority_fee_per_gas: ::ethers::core::types::U256,
    pub paymaster_and_data: ::ethers::core::types::Bytes,
    pub signature: ::ethers::core::types::Bytes,
}

pub struct GasEstimates {
    pub call_gas_limit: U256,
    pub verification_gas: U256,
    pub pre_verification_gas: U256,
    pub valid_after: U256,
    pub valid_before: U256,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct UserOperationList {
    user_operation: UserOperationTransport,
    entry_point_address: Address,
}

pub struct BundlerClient {
    pub entry_point_address: Address,
    provider: Provider<Http>,
    pub bundler_api: String,
    pub chain_id: U256,
    pub init: bool,
}

impl BundlerClient {
    /// Create a new BundlerClient
    ///
    /// # Arguments
    ///
    /// * `entry_point_address` - The address of the entry point contract
    /// * `provider` - The provider to use for the client
    /// * `bundler_api` - The bundler api
    ///
    /// # Returns
    ///
    /// * `BundlerClient` - The BundlerClient
    pub fn new(
        entry_point_address: Address,
        provider: Provider<Http>,
        bundler_api: String,
    ) -> Self {
        Self {
            entry_point_address,
            provider,
            bundler_api,
            chain_id: U256::from(0),
            init: false,
        }
    }

    /// Initialize the BundlerClient
    ///
    /// # Returns
    ///
    /// * `Result<(), eyre::Error>` - The result of the initialization
    ///
    pub async fn init(&mut self) -> eyre::Result<()> {
        // if already initialized, return Ok
        if self.init {
            return Ok(());
        }
        // set chain_id
        let chain_id = self.provider.get_chainid().await?;
        self.chain_id = chain_id;

        // check bundler supports the chain by it's chain_id
        let bundler_chain_id = self.eth_chain_id().await?;
        if bundler_chain_id != self.chain_id {
            return Err(eyre::eyre!(
                "Bundler chain id {} does not support target chain id {}",
                bundler_chain_id,
                self.chain_id
            ));
        }

        // check bundler supports the entry point
        let supported_entry_points = self.eth_supported_entry_points().await?;
        if !supported_entry_points.contains(&self.entry_point_address) {
            return Err(eyre::eyre!(
                "Bundler does not support entry point {}",
                self.entry_point_address
            ));
        }

        self.init = true;

        Ok(())
    }

    /// Get the chain id
    ///
    /// # Returns
    ///
    /// * `Result<U256, eyre::Error>` - The chain id
    pub async fn eth_chain_id(&self) -> eyre::Result<U256> {
        let provider = Http::from_str(self.bundler_api.as_str())?;
        let params: Vec<String> = vec![];
        let chain_id: U256 = provider.request("eth_chainId", params).await?;
        Ok(chain_id)
    }

    /// Get the supported entry points
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Address>, eyre::Error>` - The supported entry points
    pub async fn eth_supported_entry_points(&self) -> eyre::Result<Vec<Address>> {
        let provider = Http::from_str(self.bundler_api.as_str())?;
        let params: Vec<String> = vec![];
        let supported_entry_points: Vec<Address> =
            provider.request("eth_supportedEntryPoints", params).await?;
        Ok(supported_entry_points)
    }

    /// Send user operations to the bundler
    ///
    /// # Arguments
    ///
    /// * `user_operation_transport` - The user operation transport
    ///
    /// # Returns
    ///
    /// * `Result<String, eyre::Error>` - The user operation hash
    pub async fn eth_send_user_operation(
        &self,
        user_operation_transport: UserOperationTransport,
    ) -> eyre::Result<String> {
        let provider = Http::from_str(self.bundler_api.as_str())?;

        let user_op_list = UserOperationList {
            entry_point_address: self.entry_point_address,
            user_operation: user_operation_transport,
        };

        println!("{:?}", user_op_list);

        let user_operation_hash: String = provider
            .request("eth_sendUserOperation", user_op_list)
            .await?;

        Ok(user_operation_hash)
    }

    /// Estimate the gas for a user operation
    ///
    /// # Arguments
    ///
    /// * `user_operation_transport` - The user operation transport
    ///
    /// # Returns
    ///
    /// * `Result<U256, eyre::Error>` - The gas estimate
    pub async fn eth_estimate_user_operation_gas(
        &self,
        _user_operation_transport: UserOperationTransport,
    ) -> eyre::Result<GasEstimates> {
        // let provider = Http::from_str(self.bundler_api.as_str())?;

        // let user_op_list = UserOperationList {
        //     entry_point_address: self.entry_point_address,
        //     user_operation: user_operation_transport,
        // };

        // not sure the format this returns - need to try for real to understand
        // let gas: ??? = provider
        //     .request("eth_estimateUserOperationGas", user_op_list)
        //     .await?;

        // {
        //     preVerificationGas: '0xc51c',
        //     verificationGas: '0x06d076',
        //     validAfter: '0x0fffffff',
        //     validUntil: '0x0fffffff',
        //     callGasLimit: '0xb479'
        //   }
        Ok(GasEstimates {
            call_gas_limit: U256::from(1),
            verification_gas: U256::from(2),
            pre_verification_gas: U256::from(3),
            valid_after: U256::from(4),
            valid_before: U256::from(5),
        })
    }
}
