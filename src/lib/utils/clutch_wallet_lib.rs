use std::sync::Arc;

use ethers::{
    abi::{self, encode, FixedBytes, Token},
    prelude::*,
    providers::Provider,
    types::{Address, U256},
};

use super::{
    bundler::UserOperationTransport,
    deploy_wallet_factory::{WalletFactory, WalletFactoryConfig},
    guardians::GuardianProxyConfig,
};
use eyre::ContextCompat;
use std::str::FromStr;
#[derive(Debug, Clone)]
pub struct ClutchWalletLib {
    singleton_factory_address: Address, //_soulWalletFactoryAddress
    default_callback_handler_address: Address, //_defaultCallbackHandlerAddress
    key_store_module_address: Address,  // _keyStoreModuleAddress
    security_control_module_address: Address, // _securityControlModuleAddress
    wallet_factory: WalletFactory,
    wallet_factory_config: WalletFactoryConfig,
    guardian_proxy_config: GuardianProxyConfig,
}

impl ClutchWalletLib {
    pub fn new(
        singleton_factory_address: Address,
        default_callback_handler_address: Address,
        key_store_module_address: Address,
        security_control_module_address: Address,
        wallet_factory_config: WalletFactoryConfig,
        guardian_proxy_config: GuardianProxyConfig,
    ) -> Self {
        Self {
            singleton_factory_address,
            default_callback_handler_address,
            key_store_module_address,
            security_control_module_address,
            wallet_factory: WalletFactory::new(singleton_factory_address),
            wallet_factory_config,
            guardian_proxy_config,
        }
    }

    pub async fn get_initialize_data(
        &self,
        owner_address: Address,     //initialKey
        guardian_hash: FixedBytes,  //initialGuardianHash
        guardian_safe_period: U256, //initialGuardianSafePeriod
    ) -> eyre::Result<Vec<u8>> {
        /*
            function initialize(
                address anOwner,
                address defalutCallbackHandler,
                bytes[] calldata modules,
                bytes[] calldata plugins
            )
        */
        let abi = abi_soul_wallet();
        let guardian_safe_period_str = format!("{:032x}", guardian_safe_period);

        let security_control_module_and_data = [
            self.security_control_module_address.as_bytes(),
            guardian_safe_period_str.as_bytes(),
        ]
        .concat();

        let key_store_init_data = encode(&[
            Token::Address(owner_address),
            Token::FixedBytes(guardian_hash),
            Token::Uint(guardian_safe_period),
        ]);
        let initialize_data = abi
            .function("initialize")?
            .encode_input(&[
                Token::Address(owner_address),
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

    pub async fn get_wallet_code(
        &self,
        wallet_logic_address: Address,
        owner_address: Address,
        guardian_delay: U256,
    ) -> eyre::Result<Vec<u8>> {
        let (provider, _) = Provider::mocked();

        let abi = self.wallet_factory_config.abi.clone();
        let bytecode = self.wallet_factory_config.bytecode.clone();

        let initialize_data = self
            .get_initialize_data(owner_address, FixedBytes::from("abcdef"), guardian_delay)
            .await?;

        let factory = ContractFactory::new(abi, bytecode, Arc::new(provider));

        let constructor_args = Token::Tuple(vec![
            Token::Address(wallet_logic_address),
            Token::Bytes(initialize_data),
        ]);

        let deployer = factory.deploy(constructor_args)?;

        let wallet_factory_init_code_with_args = deployer
            .tx
            .data()
            .context("Failed to get transaction data")?;

        Ok(wallet_factory_init_code_with_args.to_vec())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn calculate_wallet_address(
        &self,
        wallet_logic_address: Address,
        owner_address: Address,
        upgrade_delay: U256,
        salt: Vec<u8>,
    ) -> eyre::Result<Address> {
        let initialize_data = self
            .get_initialize_data(owner_address, FixedBytes::from("abcdef"), upgrade_delay)
            .await?;

        abigen!(
            WalletFactoryContract,
            r#"
        [
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "_walletImpl",
                "type": "address"
              }
            ],
            "stateMutability": "nonpayable",
            "type": "constructor"
          },
          {
            "inputs": [],
            "name": "VERSION",
            "outputs": [
              {
                "internalType": "string",
                "name": "",
                "type": "string"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes",
                "name": "_initializer",
                "type": "bytes"
              },
              {
                "internalType": "bytes32",
                "name": "_salt",
                "type": "bytes32"
              }
            ],
            "name": "createWallet",
            "outputs": [
              {
                "internalType": "address",
                "name": "proxy",
                "type": "address"
              }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes",
                "name": "_initializer",
                "type": "bytes"
              },
              {
                "internalType": "bytes32",
                "name": "_salt",
                "type": "bytes32"
              }
            ],
            "name": "getWalletAddress",
            "outputs": [
              {
                "internalType": "address",
                "name": "proxy",
                "type": "address"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [],
            "name": "proxyCode",
            "outputs": [
              {
                "internalType": "bytes",
                "name": "",
                "type": "bytes"
              }
            ],
            "stateMutability": "pure",
            "type": "function"
          },
          {
            "inputs": [],
            "name": "walletImpl",
            "outputs": [
              {
                "internalType": "address",
                "name": "",
                "type": "address"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          }
        ]
        "#
        );

        let (provider, _) = Provider::mocked();
        let contract =
            WalletFactoryContract::new(self.singleton_factory_address, Arc::new(provider));
        // contract.get_wallet_address(initializer, salt)
        unimplemented!()
        // let init_code_hash = ethers::utils::keccak256(&init_code_with_args);
        // let wallet_address = self
        //     .calculate_wallet_address_by_code_hash(init_code_hash.to_vec(), salt)
        //     .await?;
        // Ok(wallet_address)
    }

    pub async fn calculate_wallet_address_by_code_hash(
        &self,
        init_code_hash: Vec<u8>,
        salt: Vec<u8>,
    ) -> eyre::Result<Address> {
        Ok(ethers::utils::get_create2_address_from_hash(
            self.singleton_factory_address,
            salt,
            init_code_hash,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_packed_init_code_using_wallet_factory(
        &self,
        wallet_logic_address: Address,
        entrypoint_address: Address,
        owner_address: Address,
        upgrade_delay: U256,
        guardian_delay: U256,
        guardian_address: Address,
        salt: Vec<u8>,
    ) -> eyre::Result<Vec<u8>> {
        // let (abi, _) = abi_and_bytecode_for_contract("ClutchWalletFactory").await?;
        let abi = abi_soul_wallet_factory();

        let packed_init_data = abi
            .function("createWallet")?
            .encode_input(&[
                Token::Address(entrypoint_address),
                Token::Address(owner_address),
                Token::Uint(upgrade_delay),
                Token::Uint(guardian_delay),
                Token::Address(guardian_address),
                Token::FixedBytes(salt.clone()),
            ])
            .map_err(|e| {
                eyre::eyre!(
                    "Failed to encode input for ClutchWalletFactory createWallet function: {}",
                    e
                )
            })?;

        let wallet_factory_address = self
            .wallet_factory
            .get_address(
                wallet_logic_address,
                salt,
                self.wallet_factory_config.clone(),
            )
            .await?;
        let mut data = wallet_factory_address.as_bytes().to_vec();
        data.extend(&packed_init_data);

        Ok(data)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn activate_wallet_op(
        &self,
        wallet_logic_address: Address,
        entrypoint_address: Address,
        owner_address: Address,
        upgrade_delay: U256,
        guardian_delay: U256,
        guardian_address: Address,
        paymaster_and_data: Vec<u8>,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: U256,
        salt: Vec<u8>,
    ) -> eyre::Result<UserOperationTransport> {
        let wallet_address = self
            .calculate_wallet_address(
                wallet_logic_address,
                owner_address,
                upgrade_delay,
                salt.clone(),
            )
            .await?;
        let init_code = self
            .get_packed_init_code_using_wallet_factory(
                wallet_logic_address,
                entrypoint_address,
                owner_address,
                upgrade_delay,
                guardian_delay,
                guardian_address,
                salt,
            )
            .await?;

        let user_operation = UserOperationTransport {
            sender: wallet_address,
            nonce: U256::from(0),
            init_code: init_code.into(),
            call_data: ethers::types::Bytes::from(vec![]),
            call_gas_limit: U256::from(0),
            max_fee_per_gas,
            max_priority_fee_per_gas,
            paymaster_and_data: paymaster_and_data.into(),
            verification_gas_limit: U256::from(0),
            pre_verification_gas: U256::from(0),
            signature: ethers::types::Bytes::from(vec![]),
        };
        Ok(user_operation)
    }
}

pub fn wallet_factory_conf() -> WalletFactoryConfig {
    let bytecode = "0x60a03461008057601f61074438819003918201601f19168301916001600160401b038311848410176100855780849260209460405283398101031261008057516001600160a01b03811690819003610080578015610080576080526040516106a8908161009c823960805181818161016d015281816102bb015261035b0152f35b600080fd5b634e487b7160e01b600052604160045260246000fdfe60806040818152600436101561001457600080fd5b600091823560e01c9081633943c030146103115750806341d900581461024a5780636fa59bbc146101e8578063a1aafc9e146101185763ffa1ad741461005957600080fd5b3461011457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101145780519080820182811067ffffffffffffffff8211176100e7576100e393508152600582527f302e302e310000000000000000000000000000000000000000000000000000006020830152519182916020835260208301906104c8565b0390f35b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b5080fd5b5090346101e157610128366103ef565b92909161012b8251906101a36020966101438884018561037f565b82845287840192610548843961019d86805180966101698c8301978892519283916104a5565b81017f00000000000000000000000000000000000000000000000000000000000000008c820152038a81018752018561037f565b8661050b565b91519083f59273ffffffffffffffffffffffffffffffffffffffff84169384156101e4578183929183888194519301915af1156101e1575051908152f35b80fd5b8280fd5b503461011457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101145780516100e39161012b61022d602082018461037f565b8083526105486020840139519182916020835260208301906104c8565b5034610114576102b69073ffffffffffffffffffffffffffffffffffffffff6055600b610276366103ef565b6102f096919661012b918751986102eb896020809c6102978289018261037f565b87815281810197610548893982519889928301988992519283916104a5565b81018d7f000000000000000000000000000000000000000000000000000000000000000090820152038c81018852018661037f565b61050b565b915190209085519186830152868201523081520160ff815320915191168152f35b83903461011457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101145760209073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176103c057604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc8201126104a05767ffffffffffffffff6004358181116104a057826023820112156104a05780600401359182116103c0576040519261047960207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f860116018561037f565b828452602483830101116104a0578160009260246020930183860137830101529060243590565b600080fd5b60005b8381106104b85750506000910152565b81810151838201526020016104a8565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602093610504815180928187528780880191016104a5565b0116010190565b602081519101209060405190602082019283526040820152604081526060810181811067ffffffffffffffff8211176103c0576040525190209056fe60803461007f57601f61012b38819003918201601f19168301916001600160401b038311848410176100845780849260209460405283398101031261007f57516001600160a01b038116810361007f577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc556040516090908161009b8239f35b600080fd5b634e487b7160e01b600052604160045260246000fdfe608060405273ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc54166000808092368280378136915af43d82803e156056573d90f35b3d90fdfea26469706673582212209b32a2f4a7feb9a7377de80388e0846b98e065aab7dee844dc0210dd36ae47ea64736f6c63430008140033a26469706673582212202edddb01ef5d8ece61c0f9c653198519f7db591f8882d24100ac1ac57017affb64736f6c63430008140033";
    let bytecode = Bytes::from_str(bytecode).unwrap();
    WalletFactoryConfig {
        abi: abi_soul_wallet_factory(),
        bytecode: bytecode,
    }
}

pub fn abi_soul_wallet_factory() -> abi::Abi {
    let abi_json = r#"
    [
        {
            "inputs": [
              {
                "internalType": "address",
                "name": "_walletImpl",
                "type": "address"
              }
            ],
            "stateMutability": "nonpayable",
            "type": "constructor"
          },
          {
            "inputs": [],
            "name": "VERSION",
            "outputs": [
              {
                "internalType": "string",
                "name": "",
                "type": "string"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes",
                "name": "_initializer",
                "type": "bytes"
              },
              {
                "internalType": "bytes32",
                "name": "_salt",
                "type": "bytes32"
              }
            ],
            "name": "createWallet",
            "outputs": [
              {
                "internalType": "address",
                "name": "proxy",
                "type": "address"
              }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes",
                "name": "_initializer",
                "type": "bytes"
              },
              {
                "internalType": "bytes32",
                "name": "_salt",
                "type": "bytes32"
              }
            ],
            "name": "getWalletAddress",
            "outputs": [
              {
                "internalType": "address",
                "name": "proxy",
                "type": "address"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [],
            "name": "proxyCode",
            "outputs": [
              {
                "internalType": "bytes",
                "name": "",
                "type": "bytes"
              }
            ],
            "stateMutability": "pure",
            "type": "function"
          },
          {
            "inputs": [],
            "name": "walletImpl",
            "outputs": [
              {
                "internalType": "address",
                "name": "",
                "type": "address"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          }
]
    "#;
    serde_json::from_str::<ethers::abi::Contract>(abi_json).unwrap()
}

pub fn abi_soul_wallet() -> abi::Abi {
    let abi_json = r#"
    [
        {
            "inputs": [
              {
                "internalType": "contract IEntryPoint",
                "name": "_EntryPoint",
                "type": "address"
              }
            ],
            "stateMutability": "nonpayable",
            "type": "constructor"
          },
          {
            "inputs": [],
            "name": "ADDRESS_ALREADY_EXISTS",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "ADDRESS_NOT_EXISTS",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "CALLER_MUST_BE_ENTRYPOINT",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "CALLER_MUST_BE_MODULE",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "CALLER_MUST_BE_SELF_OR_MODULE",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "HASH_ALREADY_APPROVED",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "HASH_ALREADY_REJECTED",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "INVALID_ADDRESS",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "INVALID_GUARD_HOOK_DATA",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "INVALID_LOGIC_ADDRESS",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "INVALID_SELECTOR",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "INVALID_SIGNTYPE",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "MODULE_ADDRESS_EMPTY",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "MODULE_EXECUTE_FROM_MODULE_RECURSIVE",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "MODULE_NOT_SUPPORT_INTERFACE",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "MODULE_SELECTORS_EMPTY",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "NOT_IMPLEMENTED",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "NO_OWNER",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "PLUGIN_ADDRESS_EMPTY",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "PLUGIN_HOOK_TYPE_ERROR",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "PLUGIN_INIT_FAILED",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "PLUGIN_NOT_REGISTERED",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "PLUGIN_NOT_SUPPORT_INTERFACE",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "PLUGIN_POST_HOOK_FAILED",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "PLUGIN_PRE_HOOK_FAILED",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "SAME_LOGIC_ADDRESS",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "SELECTOR_ALREADY_EXISTS",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "UNSUPPORTED_SIGNTYPE",
            "type": "error"
          },
          {
            "inputs": [],
            "name": "UPGRADE_FAILED",
            "type": "error"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "bytes32",
                "name": "hash",
                "type": "bytes32"
              }
            ],
            "name": "ApproveHash",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "fallbackContract",
                "type": "address"
              }
            ],
            "name": "FallbackChanged",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": false,
                "internalType": "uint8",
                "name": "version",
                "type": "uint8"
              }
            ],
            "name": "Initialized",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "module",
                "type": "address"
              }
            ],
            "name": "ModuleAdded",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "module",
                "type": "address"
              }
            ],
            "name": "ModuleRemoved",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "module",
                "type": "address"
              }
            ],
            "name": "ModuleRemovedWithError",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "owner",
                "type": "address"
              }
            ],
            "name": "OwnerAdded",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [],
            "name": "OwnerCleared",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "owner",
                "type": "address"
              }
            ],
            "name": "OwnerRemoved",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "plugin",
                "type": "address"
              }
            ],
            "name": "PluginAdded",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "plugin",
                "type": "address"
              }
            ],
            "name": "PluginRemoved",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "plugin",
                "type": "address"
              }
            ],
            "name": "PluginRemovedWithError",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "bytes32",
                "name": "hash",
                "type": "bytes32"
              }
            ],
            "name": "RejectHash",
            "type": "event"
          },
          {
            "anonymous": false,
            "inputs": [
              {
                "indexed": true,
                "internalType": "address",
                "name": "oldImplementation",
                "type": "address"
              },
              {
                "indexed": true,
                "internalType": "address",
                "name": "newImplementation",
                "type": "address"
              }
            ],
            "name": "Upgraded",
            "type": "event"
          },
          {
            "stateMutability": "payable",
            "type": "fallback"
          },
          {
            "inputs": [
              {
                "internalType": "bytes",
                "name": "moduleAndData",
                "type": "bytes"
              }
            ],
            "name": "addModule",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "owner",
                "type": "address"
              }
            ],
            "name": "addOwner",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address[]",
                "name": "owners",
                "type": "address[]"
              }
            ],
            "name": "addOwners",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes",
                "name": "pluginAndData",
                "type": "bytes"
              }
            ],
            "name": "addPlugin",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes32",
                "name": "hash",
                "type": "bytes32"
              }
            ],
            "name": "approveHash",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [],
            "name": "entryPoint",
            "outputs": [
              {
                "internalType": "contract IEntryPoint",
                "name": "",
                "type": "address"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "dest",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "value",
                "type": "uint256"
              },
              {
                "internalType": "bytes",
                "name": "func",
                "type": "bytes"
              }
            ],
            "name": "execute",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address[]",
                "name": "dest",
                "type": "address[]"
              },
              {
                "internalType": "bytes[]",
                "name": "func",
                "type": "bytes[]"
              }
            ],
            "name": "executeBatch",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address[]",
                "name": "dest",
                "type": "address[]"
              },
              {
                "internalType": "uint256[]",
                "name": "value",
                "type": "uint256[]"
              },
              {
                "internalType": "bytes[]",
                "name": "func",
                "type": "bytes[]"
              }
            ],
            "name": "executeBatch",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "to",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "value",
                "type": "uint256"
              },
              {
                "internalType": "bytes",
                "name": "data",
                "type": "bytes"
              }
            ],
            "name": "executeFromModule",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [],
            "name": "getNonce",
            "outputs": [
              {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "anOwner",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "defalutCallbackHandler",
                "type": "address"
              },
              {
                "internalType": "bytes[]",
                "name": "modules",
                "type": "bytes[]"
              },
              {
                "internalType": "bytes[]",
                "name": "plugins",
                "type": "bytes[]"
              }
            ],
            "name": "initialize",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "module",
                "type": "address"
              }
            ],
            "name": "isAuthorizedModule",
            "outputs": [
              {
                "internalType": "bool",
                "name": "",
                "type": "bool"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "plugin",
                "type": "address"
              }
            ],
            "name": "isAuthorizedPlugin",
            "outputs": [
              {
                "internalType": "bool",
                "name": "",
                "type": "bool"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "addr",
                "type": "address"
              }
            ],
            "name": "isOwner",
            "outputs": [
              {
                "internalType": "bool",
                "name": "",
                "type": "bool"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes32",
                "name": "hash",
                "type": "bytes32"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "name": "isValidSignature",
            "outputs": [
              {
                "internalType": "bytes4",
                "name": "magicValue",
                "type": "bytes4"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [],
            "name": "listModule",
            "outputs": [
              {
                "internalType": "address[]",
                "name": "modules",
                "type": "address[]"
              },
              {
                "internalType": "bytes4[][]",
                "name": "selectors",
                "type": "bytes4[][]"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [],
            "name": "listOwner",
            "outputs": [
              {
                "internalType": "address[]",
                "name": "owners",
                "type": "address[]"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "uint8",
                "name": "hookType",
                "type": "uint8"
              }
            ],
            "name": "listPlugin",
            "outputs": [
              {
                "internalType": "address[]",
                "name": "plugins",
                "type": "address[]"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "plugin",
                "type": "address"
              },
              {
                "internalType": "bytes32",
                "name": "key",
                "type": "bytes32"
              }
            ],
            "name": "pluginDataLoad",
            "outputs": [
              {
                "internalType": "bytes",
                "name": "",
                "type": "bytes"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes32",
                "name": "key",
                "type": "bytes32"
              },
              {
                "internalType": "bytes",
                "name": "value",
                "type": "bytes"
              }
            ],
            "name": "pluginDataStore",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "bytes32",
                "name": "hash",
                "type": "bytes32"
              }
            ],
            "name": "rejectHash",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "module",
                "type": "address"
              }
            ],
            "name": "removeModule",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "owner",
                "type": "address"
              }
            ],
            "name": "removeOwner",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "plugin",
                "type": "address"
              }
            ],
            "name": "removePlugin",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "newOwner",
                "type": "address"
              }
            ],
            "name": "resetOwner",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address[]",
                "name": "newOwners",
                "type": "address[]"
              }
            ],
            "name": "resetOwners",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "fallbackContract",
                "type": "address"
              }
            ],
            "name": "setFallbackHandler",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "oldImplementation",
                "type": "address"
              }
            ],
            "name": "upgradeFrom",
            "outputs": [],
            "stateMutability": "pure",
            "type": "function"
          },
          {
            "inputs": [
              {
                "internalType": "address",
                "name": "newImplementation",
                "type": "address"
              }
            ],
            "name": "upgradeTo",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "inputs": [
              {
                "components": [
                  {
                    "internalType": "address",
                    "name": "sender",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "nonce",
                    "type": "uint256"
                  },
                  {
                    "internalType": "bytes",
                    "name": "initCode",
                    "type": "bytes"
                  },
                  {
                    "internalType": "bytes",
                    "name": "callData",
                    "type": "bytes"
                  },
                  {
                    "internalType": "uint256",
                    "name": "callGasLimit",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "verificationGasLimit",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "preVerificationGas",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "maxFeePerGas",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "maxPriorityFeePerGas",
                    "type": "uint256"
                  },
                  {
                    "internalType": "bytes",
                    "name": "paymasterAndData",
                    "type": "bytes"
                  },
                  {
                    "internalType": "bytes",
                    "name": "signature",
                    "type": "bytes"
                  }
                ],
                "internalType": "struct UserOperation",
                "name": "userOp",
                "type": "tuple"
              },
              {
                "internalType": "bytes32",
                "name": "userOpHash",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "missingAccountFunds",
                "type": "uint256"
              }
            ],
            "name": "validateUserOp",
            "outputs": [
              {
                "internalType": "uint256",
                "name": "validationData",
                "type": "uint256"
              }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
          },
          {
            "stateMutability": "payable",
            "type": "receive"
          }
]
    "#;
    serde_json::from_str::<ethers::abi::Contract>(abi_json).unwrap()
}
