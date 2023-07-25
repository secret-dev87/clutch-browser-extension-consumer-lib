use crate::build::abi_and_bytecode_for_contract;
use ethers::{
    abi::{self, Token},
    prelude::ContractFactory,
    providers::Provider,
    types::{Address, Bytes, U256},
};
use eyre::ContextCompat;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct GuardianProxyConfig {
    pub abi: abi::Abi,
    pub bytecode: Bytes,
}

pub struct Guardian {
    singleton_factory_address: Address,
}

impl Guardian {
    pub fn new(singleton_factory_address: Address) -> Self {
        Self {
            singleton_factory_address,
        }
    }

    pub async fn calculate_guardian_and_init_code(
        &self,
        guardian_logic_address: Address,
        guardian_addresses: Vec<Address>,
        threshold: U256,
        salt: Vec<u8>,
        guardian_proxy_config: GuardianProxyConfig,
    ) -> eyre::Result<(Address, Vec<u8>)> {
        let init_code_with_args = self
            .get_guardian_code(
                guardian_logic_address,
                guardian_addresses,
                threshold,
                guardian_proxy_config,
            )
            .await?;

        let data = init_code_with_args[2..].to_string();
        let init_code_with_args = hex::decode(data)?;
        let init_code_hash = ethers::utils::keccak256(init_code_with_args.clone());

        let address = ethers::utils::get_create2_address_from_hash(
            self.singleton_factory_address,
            salt.clone(),
            init_code_hash,
        );

        let init_code = self
            .get_packed_init_code(self.singleton_factory_address, init_code_with_args, salt)
            .await?;

        Ok((address, init_code))
    }

    pub async fn get_packed_init_code(
        &self,
        singleton_factory_address: Address,
        init_code: Vec<u8>,
        salt: Vec<u8>,
    ) -> eyre::Result<Vec<u8>> {
        let abi_json = r#"[{ "inputs": [{ "internalType": "bytes", "name": "_initCode", "type": "bytes" }, { "internalType": "bytes32", "name": "_salt", "type": "bytes32" }], "name": "deploy", "outputs": [{ "internalType": "address payable", "name": "createdContract", "type": "address" }], "stateMutability": "nonpayable", "type": "function" }]"#;
        let abi = serde_json::from_str::<ethers::abi::Contract>(abi_json)?;
        let encoded = abi
            .function("deploy")?
            .encode_input(&[Token::Bytes(init_code), Token::FixedBytes(salt)])?;

        let result = [singleton_factory_address.as_bytes().to_vec(), encoded].concat();
        Ok(result)
    }

    /// Gets the guardian code for the GuardianMultiSigWallet
    ///
    /// # Arguments
    ///
    /// * `guardian_logic_address` - The guardian logic address
    /// * `guardian_addresses` - The guardian addresses
    /// * `threshold` - The threshold
    /// * `guardian_proxy_config` - The guardian proxy config (abi and bytecode of the GuardianMultiSigWallet contract)
    /// * `client` - The client
    ///
    /// # Returns
    ///
    /// * `Result<String, eyre::Error>` - The result with the guardian code
    pub async fn get_guardian_code(
        &self,
        guardian_logic_address: Address,
        guardian_addresses: Vec<Address>,
        threshold: U256,
        guardian_proxy_config: GuardianProxyConfig,
    ) -> eyre::Result<String> {
        let initialize_data = self
            .get_initialize_data(guardian_addresses, threshold)
            .await?;

        let abi = guardian_proxy_config.abi;
        let bytecode = guardian_proxy_config.bytecode;

        let (provider, _) = Provider::mocked();
        let factory = ContractFactory::new(abi, bytecode, provider.into());

        let constructor_args = Token::Tuple(vec![
            Token::Address(guardian_logic_address),
            Token::Bytes(initialize_data),
        ]);

        let deployer = factory.deploy(constructor_args)?;

        let wallet_bytecode = deployer
            .tx
            .data()
            .context("Failed to get transaction data")?;

        Ok(wallet_bytecode.to_string())
    }

    /// Gets the initialize data for the GuardianMultiSigWallet
    ///
    /// # Arguments
    ///
    /// * `guardian_addresses` - The guardian addresses
    /// * `threshold` - The threshold
    ///
    /// # Returns
    ///
    /// * `String` - The initialize data
    pub async fn get_initialize_data(
        &self,
        guardian_addresses: Vec<Address>,
        threshold: U256,
    ) -> eyre::Result<Vec<u8>> {
        let sorted_guardian_addresses = self.sort_addresses(guardian_addresses).await;

        let token_addresses = sorted_guardian_addresses
            .iter()
            .map(|address| Token::Address(*address))
            .collect::<Vec<Token>>();

        let (abi, _) = abi_and_bytecode_for_contract("GuardianMultiSigWallet").await?;

        let initialize_data = abi
            .function("initialize")?
            .encode_input(&[Token::Array(token_addresses), Token::Uint(threshold)])
            .map_err(|e| {
                eyre::eyre!(
                    "Failed to encode input for GuardianMultiSigWallet initialize function: {}",
                    e
                )
            })?;

        Ok(initialize_data)
    }

    /// Sorts the guardian addresses
    ///
    /// # Arguments
    ///
    /// * `guardian_addresses` - The guardian addresses to sort
    ///
    /// # Returns
    ///
    /// * `Vec<Address>` - The sorted guardian addresses
    pub async fn sort_addresses(&self, mut guardian_addresses: Vec<Address>) -> Vec<Address> {
        guardian_addresses.sort();
        guardian_addresses
    }
}

pub fn guardian_prxy_conf() -> GuardianProxyConfig {
    let bytecode = "0x60a03461008057601f61074438819003918201601f19168301916001600160401b038311848410176100855780849260209460405283398101031261008057516001600160a01b03811690819003610080578015610080576080526040516106a8908161009c823960805181818161016d015281816102bb015261035b0152f35b600080fd5b634e487b7160e01b600052604160045260246000fdfe60806040818152600436101561001457600080fd5b600091823560e01c9081633943c030146103115750806341d900581461024a5780636fa59bbc146101e8578063a1aafc9e146101185763ffa1ad741461005957600080fd5b3461011457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101145780519080820182811067ffffffffffffffff8211176100e7576100e393508152600582527f302e302e310000000000000000000000000000000000000000000000000000006020830152519182916020835260208301906104c8565b0390f35b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b5080fd5b5090346101e157610128366103ef565b92909161012b8251906101a36020966101438884018561037f565b82845287840192610548843961019d86805180966101698c8301978892519283916104a5565b81017f00000000000000000000000000000000000000000000000000000000000000008c820152038a81018752018561037f565b8661050b565b91519083f59273ffffffffffffffffffffffffffffffffffffffff84169384156101e4578183929183888194519301915af1156101e1575051908152f35b80fd5b8280fd5b503461011457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101145780516100e39161012b61022d602082018461037f565b8083526105486020840139519182916020835260208301906104c8565b5034610114576102b69073ffffffffffffffffffffffffffffffffffffffff6055600b610276366103ef565b6102f096919661012b918751986102eb896020809c6102978289018261037f565b87815281810197610548893982519889928301988992519283916104a5565b81018d7f000000000000000000000000000000000000000000000000000000000000000090820152038c81018852018661037f565b61050b565b915190209085519186830152868201523081520160ff815320915191168152f35b83903461011457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101145760209073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176103c057604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc8201126104a05767ffffffffffffffff6004358181116104a057826023820112156104a05780600401359182116103c0576040519261047960207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f860116018561037f565b828452602483830101116104a0578160009260246020930183860137830101529060243590565b600080fd5b60005b8381106104b85750506000910152565b81810151838201526020016104a8565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602093610504815180928187528780880191016104a5565b0116010190565b602081519101209060405190602082019283526040820152604081526060810181811067ffffffffffffffff8211176103c0576040525190209056fe60803461007f57601f61012b38819003918201601f19168301916001600160401b038311848410176100845780849260209460405283398101031261007f57516001600160a01b038116810361007f577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc556040516090908161009b8239f35b600080fd5b634e487b7160e01b600052604160045260246000fdfe608060405273ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc54166000808092368280378136915af43d82803e156056573d90f35b3d90fdfea26469706673582212209b32a2f4a7feb9a7377de80388e0846b98e065aab7dee844dc0210dd36ae47ea64736f6c63430008140033a26469706673582212202edddb01ef5d8ece61c0f9c653198519f7db591f8882d24100ac1ac57017affb64736f6c63430008140033";
    let bytecode = Bytes::from_str(bytecode).unwrap();
    GuardianProxyConfig {
        abi: abi_guardian_proxy(),
        bytecode: bytecode,
    }
}

pub fn abi_guardian_proxy() -> abi::Abi {
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
