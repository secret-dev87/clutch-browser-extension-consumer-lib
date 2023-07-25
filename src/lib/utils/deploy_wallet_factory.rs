use ethers::{
    abi::{self, Token},
    prelude::{k256::ecdsa::SigningKey, ContractFactory, SignerMiddleware},
    providers::Provider,
    types::Bytes,
};
use ethers::{prelude::*, types::Address};
use eyre::ContextCompat;
use std::sync::Arc;

// use crate::generated::bindings::contract_singletonfactory::singleton_factory;

#[derive(Clone, Debug)]
pub struct WalletFactoryConfig {
    pub abi: abi::Abi,
    pub bytecode: Bytes,
}

#[derive(Clone, Debug)]
pub struct WalletFactoryAddress {
    pub factory_address: Address,
    pub init_code_with_args: String,
}
#[derive(Clone, Debug)]
pub struct WalletFactory {
    singleton_factory_address: Address,
}

impl WalletFactory {
    pub fn new(singleton_factory_address: Address) -> Self {
        Self {
            singleton_factory_address,
        }
    }

    /// Calculates the wallet factory address and init code with args for use in the deploy_wallet_factory function
    ///
    /// # Arguments
    ///
    /// * `logic_contract_address` - The address of the logic contract
    /// * `singleton_factory_address` - The address of the singleton factory contract
    /// * `salt` - The salt
    /// * `wallet_factory_config` - The wallet factory config (abi and bytecode of the ClutchWalletFactory contract)
    ///
    /// # Returns
    ///
    /// * `Result<WalletFactoryAddress, eyre::Error>` - The result with the wallet factory address and the init code with args
    pub async fn get_wallet_factory(
        &self,
        logic_contract_address: Address,
        salt: Vec<u8>,
        wallet_factory_config: WalletFactoryConfig,
    ) -> eyre::Result<WalletFactoryAddress> {
        let (provider, _) = Provider::mocked();

        let abi = wallet_factory_config.abi;
        let bytecode = wallet_factory_config.bytecode;

        let factory = ContractFactory::new(abi, bytecode, Arc::new(provider));

        let constructor_args = Token::Tuple(vec![
            Token::Address(logic_contract_address),
            Token::Address(self.singleton_factory_address),
        ]);

        let deployer = factory.deploy(constructor_args)?;

        let wallet_factory_init_code_with_args = deployer
            .tx
            .data()
            .context("Failed to get transaction data")?;

        let wallet_factory_init_code_hash =
            ethers::utils::keccak256(wallet_factory_init_code_with_args);
        let wallet_factory_address = ethers::utils::get_create2_address_from_hash(
            self.singleton_factory_address,
            salt,
            wallet_factory_init_code_hash,
        );

        Ok(WalletFactoryAddress {
            factory_address: wallet_factory_address,
            init_code_with_args: wallet_factory_init_code_with_args.to_string(),
        })
    }

    pub async fn get_address(
        &self,
        logic_contract_address: Address,
        salt: Vec<u8>,
        wallet_factory_config: WalletFactoryConfig,
    ) -> eyre::Result<Address> {
        let wallet_factory_address = self
            .get_wallet_factory(logic_contract_address, salt.clone(), wallet_factory_config)
            .await?;

        Ok(wallet_factory_address.factory_address)
    }

    /// Deploys a wallet factory contract
    ///
    /// # Arguments
    ///
    /// * `logic_contract_address` - The address of the logic contract
    /// * `singleton_factory_address` - The address of the singleton factory contract
    /// * `salt` - The salt to use for the deployment
    /// * `wallet_factory_config` - The wallet factory config (abi and bytecode of the ClutchWalletFactory contract)
    ///
    /// # Returns
    ///
    /// * `Result<String, eyre::Error>` - The address of the deployed wallet factory contract
    pub async fn deploy(
        &self,
        logic_contract_address: Address,
        salt: Vec<u8>,
        wallet_factory_config: WalletFactoryConfig,
        client: Arc<SignerMiddleware<Provider<ethers::providers::Http>, Wallet<SigningKey>>>,
    ) -> eyre::Result<Address> {
        // let wallet_factory_address = self
        //     .get_wallet_factory(logic_contract_address, salt.clone(), wallet_factory_config)
        //     .await?;

        // let singleton_factory_contract =
        //     singleton_factory::SingletonFactory::new(self.singleton_factory_address, client);

        // let salt: [u8; 32] = salt
        //     .try_into()
        //     .map_err(|_| eyre::eyre!("Failed to convert salt to [u8;32]"))?;

        // singleton_factory_contract
        //     .deploy(
        //         wallet_factory_address
        //             .init_code_with_args
        //             .parse::<Bytes>()?,
        //         salt,
        //     )
        //     .send()
        //     .await?
        //     .confirmations(1)
        //     .await?;

        // Ok(wallet_factory_address.factory_address)
        unimplemented!()
    }
}
