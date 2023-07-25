use std::{
    env,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};

use ethers::{
    abi::{self, encode, FixedBytes, Token},
    prelude::*,
    providers::Provider,
    types::{Address, U256},
};

use super::bundler::UserOperationTransport;

#[derive(Debug, Clone)]
pub struct WalletLib {
    provider: String,
    bundler: String,
    wallet_factory_address: Address, //_soulWalletFactoryAddress
    default_callback_handler_address: Address, //_defaultCallbackHandlerAddress
    key_store_module_address: Address, // _keyStoreModuleAddress
    security_control_module_address: Address, // _securityControlModuleAddress
}

impl WalletLib {
    pub fn new(
        provider: &str,
        bundler: &str,
        wallet_factory_address: &str,
        default_callback_handler_address: &str,
        key_store_module_address: &str,
        security_control_module_address: &str,
    ) -> Self {
        Self {
            provider: provider.to_string(),
            bundler: bundler.to_string(),
            wallet_factory_address: wallet_factory_address.parse::<Address>().unwrap(),
            default_callback_handler_address: default_callback_handler_address
                .parse::<Address>()
                .unwrap(),
            key_store_module_address: key_store_module_address.parse::<Address>().unwrap(),
            security_control_module_address: security_control_module_address
                .parse::<Address>()
                .unwrap(),
        }
    }

    pub async fn initialize_data(
        &self,
        initial_key: Address,
        initial_guardian_hash: FixedBytes,
        initial_guardian_safeperiod: &str,
    ) -> eyre::Result<Vec<u8>> {
        /*
            function initialize(
                address anOwner,
                address defalutCallbackHandler,
                bytes[] calldata modules,
                bytes[] calldata plugins
            )
        */
        let initial_guardian_safeperiod =
            U256::from_str_radix(initial_guardian_safeperiod, 16).unwrap();
        let guardian_safe_period_str = format!("{:030x}", initial_guardian_safeperiod); // remove '0x' so the length is 30
        let security_control_module_and_data = [
            self.security_control_module_address.as_bytes(),
            guardian_safe_period_str.as_bytes(),
        ]
        .concat();
        let key_store_init_data = encode(&[
            Token::Address(initial_key),
            Token::FixedBytes(initial_guardian_hash),
            Token::Uint(initial_guardian_safeperiod),
        ]);
        let abi = abi_soul_wallet();
        let initialize_data = abi
            .function("initialize")?
            .encode_input(&[
                Token::Address(initial_key),
                Token::Address(self.default_callback_handler_address),
                Token::Array(vec![
                    Token::Bytes(security_control_module_and_data),
                    Token::Bytes(key_store_init_data),
                ]),
                Token::Array(vec![]),
            ])
            .map_err(|e| {
                eyre::eyre!(
                    "Failed to encode input for ClutchWallet initialize function: {}",
                    e
                )
            })?;

        Ok(initialize_data)
    }

    pub async fn calc_wallet_address(
        &self,
        index: &str,
        initial_key: &str,
        initial_guard_hash: &str,
        initial_guardian_safeperiod: &str,
    ) -> eyre::Result<Address> {
        let initial_key = initial_key.parse::<Address>().unwrap();
        let initial_guard_hash = FixedBytes::from(initial_guard_hash);
        let initialize_data = self
            .initialize_data(initial_key, initial_guard_hash, initial_guardian_safeperiod)
            .await?;

        let provider = Provider::<Http>::try_from(self.provider.clone())?;
        // let wallet: LocalWallet = "9131cbad8e7e5369d670022b3ea8781d7ed83681a3720d3e551833816c2fb6a4".parse::<LocalWallet>()?.with_chain_id(1337 as u64);
        // let client = SignerMiddleware::new(provider.clone(), wallet.clone());
        let wallet_factory = super::abis::WalletFactoryContract::new(
            self.wallet_factory_address,
            Arc::new(provider),
        );

        let index = U256::from_str_radix(index, 16).unwrap();
        let salt: [u8; 32] = index.try_into().unwrap();
        let initialize_data = ethers::types::Bytes::from(initialize_data);

        let wallet_addr = wallet_factory
            .get_wallet_address(initialize_data, salt)
            .await?;

        Ok(wallet_addr)
    }

    pub async fn create_unsigned_deploy_wallet_user_op(
        &self,
        index: &str,
        initial_key: &str,
        initial_guard_hash: &str,
        initial_guardian_safeperiod: &str,
    ) -> eyre::Result<UserOperationTransport> {
        let sender = self.calc_wallet_address(index, initial_key, initial_guard_hash, initial_guardian_safeperiod).await?;

        let abi = abi_soul_wallet_factory();
        let initial_key = initial_key.parse::<Address>().unwrap();
        let initial_guard_hash = FixedBytes::from(initial_guard_hash);

        let initialize_data = self
            .initialize_data(initial_key, initial_guard_hash, initial_guardian_safeperiod)
            .await?;
        let index = U256::from_str_radix(index, 16).unwrap();
        let index = format!("{:030x}", index); //remove '0x' prefix
        let init_code = abi.function("createWallet")?.encode_input(&[
            Token::Bytes(initialize_data), Token::FixedBytes(index.clone().into_bytes())
        ]).map_err(|e| {
            eyre::eyre!(
                "Failed to encode input for ClutchWallet initialize function: {}",
                e
            )
        })?;
        let init_code = [self.wallet_factory_address.as_bytes(), init_code.as_ref()].concat();

        let user_operation = UserOperationTransport{
            sender,
            nonce: U256::from(0),
            init_code: ethers::types::Bytes::from(init_code),
            call_data: ethers::types::Bytes::from(b"0x"),
            call_gas_limit: U256::from(0), 
            verification_gas_limit: U256::from(0),
            pre_verification_gas: U256::from(10000000),
            max_fee_per_gas: U256::from(0),
            max_priority_fee_per_gas: U256::from(0),
            paymaster_and_data: ethers::types::Bytes::from(b"0x"),
            signature: ethers::types::Bytes::from(b"0x")
        };

        Ok(user_operation)        
    }
}

fn generated_contract_path(contract: &str) -> PathBuf {
    let current_file = file!();
    let path = Path::new(current_file).parent().unwrap();
    let contract_path = format!("../generated/abi/contract_{contract}.json");
    path.join(contract_path)
}

pub fn abi_soul_wallet() -> abi::Abi {
    let abi_path = generated_contract_path("soulwallet");
    let mut file = File::open(abi_path).unwrap();
    let mut abi_json = String::new();
    let _ = file.read_to_string(&mut abi_json).unwrap();
    serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap()
}

pub fn abi_soul_wallet_factory() -> abi::Abi {
    let abi_path = generated_contract_path("soulwalletfactory");
    let mut file = File::open(abi_path).unwrap();
    let mut abi_json = String::new();
    let _ = file.read_to_string(&mut abi_json).unwrap();
    serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap()
}
