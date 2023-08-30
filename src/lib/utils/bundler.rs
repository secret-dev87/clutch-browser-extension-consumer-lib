use ethers::{
    providers::{Http, JsonRpcClient, Middleware, Provider},
    types::{Address, Bytes, U256},
};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_tuple::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserOpReceipt {
    pub user_op_hash: ::ethers::core::types::Bytes,
    pub sender: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
    pub paymaster: Option<::ethers::core::types::Address>,
    pub actual_gas_cost: ::ethers::core::types::U256,
    pub actual_gas_used: ::ethers::core::types::U256,
    pub success: bool,
    pub reason: Option<::ethers::core::types::Bytes>,
    pub logs: Vec<::ethers::core::types::Log>,
    pub receipt: ::ethers::core::types::TransactionReceipt,
}

#[derive(Debug, Clone)]
pub enum UserOpErrCodes {
    UnknownError = -1,
    InvalidUserOp = -32602,
    EntryPointValidationFailed = -32500,
    PaymasterValidationFailed = -32501,
    OpcodeValidationFailed = -32502,
    TimeRangeValidationFailed = -32503,
    PaymasterThrottled = -32504,
    PaymasterStakeTooLow = -32505,
    UnsupportedSignatureAggregator = -32506,
    SignatureValidationFailed = -32507,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserOpGas {
    pub call_gas_limit: U256,
    pub verification_gas_limit: U256,
    pub pre_verification_gas: U256,
    pub valid_after: U256,
    pub valid_until: U256,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct UserOperationList {
    user_operation: UserOperationTransport,
    entry_point_address: Address,
}

#[derive(Debug, Clone)]
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
    ) -> eyre::Result<Bytes> {
        let provider = Http::from_str(self.bundler_api.as_str())?;

        let user_operation_hash: Bytes = provider
            .request(
                "eth_sendUserOperation",
                (user_operation_transport, self.entry_point_address),
            )
            .await?;

        // println!("{}", user_operation_hash);
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
        user_operation_transport: UserOperationTransport,
    ) -> eyre::Result<UserOpGas> {
        let provider = Http::from_str(self.bundler_api.as_str())?;

        let estimate_gas: UserOpGas = provider
            .request(
                "eth_estimateUserOperationGas",
                (user_operation_transport, self.entry_point_address),
            )
            .await?;

        Ok(estimate_gas)
    }

    pub async fn eth_get_user_operation_receipt(
        &self,
        user_op_hash: Bytes,
    ) -> eyre::Result<Option<UserOpReceipt>> {
        let provider = Http::from_str(self.bundler_api.as_str())?;
        let ret: Option<UserOpReceipt> = provider
            .request("eth_getUserOperationReceipt", [user_op_hash])
            .await?;
        Ok(ret)
    }
}
