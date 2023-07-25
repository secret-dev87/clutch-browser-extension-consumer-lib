use clutch_wallet_lib::{
    build::get_compiled_contracts,
    utils::{deploy_wallet_factory::WalletFactoryConfig, guardians::GuardianProxyConfig},
};
use ethers::{
    abi::Token,
    prelude::{rand::thread_rng, *},
    types::{Address, BlockNumber},
    utils::GanacheInstance,
};
use ethers_solc::{Artifact, ProjectCompileOutput};
use tokio::sync::{Mutex, OnceCell};

use std::{str::FromStr, time::Duration};

use ethers::{
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    utils::Ganache,
};

use eyre::ContextCompat;
use httpmock::prelude::*;

pub async fn get_ganache_instance() -> &'static Mutex<GanacheInstance> {
    static ONCE: OnceCell<Mutex<GanacheInstance>> = OnceCell::const_new();
    ONCE.get_or_init(|| async {
        // Spawn a ganache instance
        let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
        let ganache = Ganache::new().mnemonic(mnemonic).spawn();
        Mutex::new(ganache)
    })
    .await
}

#[allow(dead_code)]
pub async fn deploy_token_paymaster(
    project: ProjectCompileOutput,
    wallet: LocalWallet,
    provider: Provider<ethers::providers::Http>,
    entry_point_address: Address,
    wallet_factory_address: Address,
) -> Address {
    let (abi, bytecode) = get_abi_and_bytecode(String::from("TokenPaymaster"), project);

    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let wallet = wallet.clone().with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone()).into();

    let factory = ContractFactory::new(abi.clone(), bytecode, client);

    let constructor_args = Token::Tuple(vec![
        Token::Address(entry_point_address),
        Token::Address(wallet.address()),
        Token::Address(wallet_factory_address),
    ]);

    let mut deployer = factory.deploy(constructor_args).unwrap();

    let block = provider
        .clone()
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .context("Failed to get latest block")
        .unwrap();

    // Set gas limit
    let gas_limit = block.gas_limit;
    deployer.tx.set_gas::<U256>(gas_limit);

    // Create transaction and send
    let contract = deployer.clone().legacy().send().await.unwrap();

    contract.address()
}

#[allow(dead_code)]
pub async fn deploy_price_oracle(
    project: ProjectCompileOutput,
    wallet: LocalWallet,
    provider: Provider<ethers::providers::Http>,
    mock_oracle_address: Address,
) -> Address {
    let (abi, bytecode) = get_abi_and_bytecode(String::from("PriceOracle"), project);

    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let wallet = wallet.clone().with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone()).into();

    let factory = ContractFactory::new(abi.clone(), bytecode, client);

    let mut deployer = factory.deploy(mock_oracle_address).unwrap();

    let block = provider
        .clone()
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .context("Failed to get latest block")
        .unwrap();

    // Set gas limit
    let gas_limit = block.gas_limit;
    deployer.tx.set_gas::<U256>(gas_limit);

    // Create transaction and send
    let contract = deployer.clone().legacy().send().await.unwrap();

    contract.address()
}

pub async fn deploy_contract_no_args(
    contract_name: &str,
    project: ProjectCompileOutput,
    wallet: LocalWallet,
    provider: Provider<ethers::providers::Http>,
) -> Address {
    let (abi, bytecode) = get_abi_and_bytecode(String::from(contract_name), project);

    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let wallet = wallet.clone().with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone()).into();

    let factory = ContractFactory::new(abi.clone(), bytecode, client);

    let mut deployer = factory.deploy(()).unwrap();

    let block = provider
        .clone()
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .context("Failed to get latest block")
        .unwrap();

    // Set gas limit
    let gas_limit = block.gas_limit;
    deployer.tx.set_gas::<U256>(gas_limit);

    // Create transaction and send
    let contract = deployer.clone().legacy().send().await.unwrap();

    contract.address()
}

pub fn get_abi_and_bytecode(
    contract_name: String,
    project: ProjectCompileOutput,
) -> (abi::Abi, Bytes) {
    let factory = project
        .find_first(contract_name.clone())
        .context(format!("Failed to find contract: {:?}", contract_name))
        .unwrap()
        .clone();

    let (abi, bytecode, _) = factory.into_parts();

    let abi: abi::Abi = abi
        .context(format!(
            "Failed to get ABI for contract: {:?}",
            contract_name
        ))
        .unwrap();

    let bytecode = bytecode
        .context(format!(
            "Failed to get bytecode for contract: {:?}",
            contract_name
        ))
        .unwrap();

    (abi, bytecode)
}

#[allow(dead_code)]
pub async fn bundler_test_setup() -> (MockServer, String, Provider<Http>, Address) {
    let mock_bundler_server = MockServer::start();
    let mock_bundler_server_url = mock_bundler_server.base_url();

    let ganache = get_ganache_instance().await.lock().await;

    // Get the first wallet managed by ganache to deploy things
    let wallet: LocalWallet = ganache.keys()[0].clone().into();

    // A provider is an Ethereum JsonRPC client
    let provider: Provider<ethers::providers::Http> = Provider::try_from(ganache.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10));

    // Compile solidity project
    let project = get_compiled_contracts().await.lock().await;

    // Deploy Entrypoint contract
    let entrypoint_address = deploy_contract_no_args(
        "EntryPoint",
        project.clone(),
        wallet.clone(),
        provider.clone(),
    )
    .await;

    (
        mock_bundler_server,
        mock_bundler_server_url,
        provider,
        entrypoint_address,
    )
}

#[allow(dead_code)]
pub async fn create_guardian_addresses(count: u8) -> (Vec<Address>, Vec<LocalWallet>) {
    // create guardians
    let guardian_wallets = (1..count)
        .map(|_| LocalWallet::new(&mut thread_rng()))
        .collect::<Vec<_>>();

    // guardian addresses
    let guardian_addresses = guardian_wallets
        .iter()
        .map(|w| w.address())
        .collect::<Vec<_>>();

    (guardian_addresses, guardian_wallets)
}

#[allow(dead_code)]
pub async fn fixed_guardian_addresses() -> Vec<Address> {
    let fixed_addresses = vec![
        "0x3b4ddbf2b2ef3dfc8785c7d8351c3c0abd1a61b7",
        "0xea4f937c229c694f0fcde534d1836e62ae273e36",
        "0x67539b8a601b6d33e986d766d9e7b5c6ff2de245",
        "0x373d30d5eab8762cea9df73ff8214f145104c0d3",
        "0x4dd885eaae44f9359fa6f8f4b01ed57c0858bec5",
        "0xdc871505eaf43757371a731e4f667b36f82f8795",
        "0xe6b6e025622c86fa0db694b69b54692a4fbec9a3",
        "0x4c269cf0e9e6fef8ea6e48fb162a12ffc02f3ace",
        "0xbe02dc2a5aa224941d904dc4bb65142dd7f9201d",
    ];

    fixed_addresses
        .iter()
        .map(|x| x.parse::<Address>().unwrap())
        .collect::<Vec<Address>>()
}

#[allow(dead_code)]
pub async fn to_addresses(addresses: Vec<&str>) -> Vec<Address> {
    addresses
        .iter()
        .map(|x| x.parse::<Address>().unwrap())
        .collect::<Vec<Address>>()
}

#[allow(dead_code)]
pub fn wallet_factory_abi() -> WalletFactoryConfig {
    let abi_json = r#" [
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "_walletImpl",
                    "type": "address"
                },
                {
                    "internalType": "address",
                    "name": "_singletonFactory",
                    "type": "address"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "constructor"
        },
        {
            "anonymous": false,
            "inputs": [
                {
                    "indexed": true,
                    "internalType": "address",
                    "name": "_proxy",
                    "type": "address"
                },
                {
                    "indexed": true,
                    "internalType": "address",
                    "name": "_owner",
                    "type": "address"
                },
                {
                    "indexed": true,
                    "internalType": "address",
                    "name": "_implementation",
                    "type": "address"
                },
                {
                    "indexed": false,
                    "internalType": "string",
                    "name": "version",
                    "type": "string"
                }
            ],
            "name": "SoulWalletCreated",
            "type": "event"
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
                    "internalType": "address",
                    "name": "_entryPoint",
                    "type": "address"
                },
                {
                    "internalType": "address",
                    "name": "_owner",
                    "type": "address"
                },
                {
                    "internalType": "uint32",
                    "name": "_upgradeDelay",
                    "type": "uint32"
                },
                {
                    "internalType": "uint32",
                    "name": "_guardianDelay",
                    "type": "uint32"
                },
                {
                    "internalType": "address",
                    "name": "_guardian",
                    "type": "address"
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
                    "name": "",
                    "type": "address"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "_entryPoint",
                    "type": "address"
                },
                {
                    "internalType": "address",
                    "name": "_owner",
                    "type": "address"
                },
                {
                    "internalType": "uint32",
                    "name": "_upgradeDelay",
                    "type": "uint32"
                },
                {
                    "internalType": "uint32",
                    "name": "_guardianDelay",
                    "type": "uint32"
                },
                {
                    "internalType": "address",
                    "name": "_guardian",
                    "type": "address"
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
                    "name": "",
                    "type": "address"
                }
            ],
            "name": "isWalletActive",
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
            "name": "singletonFactory",
            "outputs": [
                {
                    "internalType": "address",
                    "name": "",
                    "type": "address"
                }
            ],
            "stateMutability": "view",
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
    ]"#;

    let abi = serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap();
    let bytecode = "0x60c03461012b57601f61121e38819003918201601f19168301916001600160401b0383118484101761013057808492604094855283398101031261012b57610052602061004b83610146565b9201610146565b906001600160a01b0390818116156100f3576080528116156100ae5760a0526040516110c3908161015b8239608051818181610182015281816105580152610663015260a05181818161027d015281816104a801526106e30152f35b60405162461bcd60e51b815260206004820152601660248201527f73696e676c65746f6e466163746f7279206572726f72000000000000000000006044820152606490fd5b60405162461bcd60e51b815260206004820152601060248201526f3bb0b63632ba24b6b8361032b93937b960811b6044820152606490fd5b600080fd5b634e487b7160e01b600052604160045260246000fd5b51906001600160a01b038216820361012b5756fe60406080815260043610156200001457600080fd5b600090813560e01c806330b8d376146200057c5780633943c030146200050c5780636fa59bbc14620004cc578063bc10273e146200045c578063d0ed7b9114620003f0578063f452880814620000cf5763ffa1ad74146200007457600080fd5b34620000cb57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112620000cb57620000c790620000b362000884565b9051918291602083526020830190620007ce565b0390f35b5080fd5b509034620003ed57620000e23662000712565b90969391969592956103b48551976020988993849283850162000106908462000813565b8483528383019462000926863989517f82778654000000000000000000000000000000000000000000000000000000008582015273ffffffffffffffffffffffffffffffffffffffff9b8c166024820152888c16604482015263ffffffff91821660648201529c1660848d0152891660a4808d01919091528b527f00000000000000000000000000000000000000000000000000000000000000009a8b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0620001d160c48462000813565b8a51809286820194620001e59186620008cf565b039081018252620001f7908262000813565b89519485935190818686016200020d92620007a9565b8301905191828583016200022192620007a9565b01038083520162000233908262000813565b8551809381927f4af63f02000000000000000000000000000000000000000000000000000000008352886004840152604483016200027191620007ce565b906024830152038186897f0000000000000000000000000000000000000000000000000000000000000000165af1908115620003e35790859184916200039e575b50169384156200034157847f8f4ebbea6fac3dc9485cb4581e23041567937fa082c34a8504307cd1b7ecc3da879886979862000308620002f162000884565b95808a519485948552169716958c830190620007ce565b0390a48381528085522060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905551908152f35b6064868551907f08c379a00000000000000000000000000000000000000000000000000000000082526004820152600e60248201527f63726561746532206661696c65640000000000000000000000000000000000006044820152fd5b809250878092503d8311620003db575b620003ba818362000813565b81010312620003d757518481168103620003d757849038620002b2565b8280fd5b503d620003ae565b84513d85823e3d90fd5b80fd5b5034620000cb5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112620000cb5760043573ffffffffffffffffffffffffffffffffffffffff8116809103620003d757818360ff926020955280855220541690519015158152f35b5034620000cb57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112620000cb576020905173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b5034620000cb57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112620000cb57620000c790620000b362000901565b5034620000cb57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112620000cb576020905173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b5034620000cb57600b90620006d26055620005973662000712565b93620005a99995979192939962000901565b89517f827786540000000000000000000000000000000000000000000000000000000060208083019190915273ffffffffffffffffffffffffffffffffffffffff9a8b166024830152928a16604482015263ffffffff9b8c16606482015294909a16608485015291871660a480850191909152835290979190887fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe06200065160c48562000813565b620006968a51918262000689858201977f000000000000000000000000000000000000000000000000000000000000000089620008cf565b0390810183528262000813565b8951938491620006c584620006b5818601998a815193849201620007a9565b84019151809386840190620007a9565b0103808452018262000813565b5190209085519186830152868201527f000000000000000000000000000000000000000000000000000000000000000081520160ff815320915191168152f35b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc60c0910112620007a45773ffffffffffffffffffffffffffffffffffffffff6004358181168103620007a457916024358281168103620007a4579163ffffffff916044358381168103620007a457926064359081168103620007a457916084359081168103620007a4579060a43590565b600080fd5b60005b838110620007bd5750506000910152565b8181015183820152602001620007ac565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f6020936200080c81518092818752878088019101620007a9565b0116010190565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176200085557604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051906040820182811067ffffffffffffffff8211176200085557604052600582527f302e302e310000000000000000000000000000000000000000000000000000006020830152565b60409073ffffffffffffffffffffffffffffffffffffffff620008fe94931681528160208201520190620007ce565b90565b6040516103b462000916602082018362000813565b80825262000cda60208301399056fe60806040526103b480380380610014816100bd565b92833981016040828203126100a1578151916001600160a01b03831683036100a1576020810151906001600160401b0382116100a157019181601f840112156100a15782519261006b610066856100ef565b6100bd565b92848452602085830101116100a1576100939361008e9160208086019101610119565b61013c565b60405160bb90816102f98239f35b600080fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f191682016001600160401b038111838210176100e257604052565b6100ea6100a6565b604052565b6020906001600160401b03811161010c575b601f01601f19160190565b6101146100a6565b610101565b60005b83811061012c5750506000910152565b818101518382015260200161011c565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc81905561021f91906000906001600160a01b0381167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b8380a26040519282908190606086016001600160401b0381118782101761022b575b604052602786527f416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c6020870152660819985a5b195960ca1b6040870152602081519101845af4903d15610222573d61020f610066826100ef565b908152809360203d92013e610284565b50565b60609250610284565b6102336100a6565b6101b5565b1561023f57565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b919290156102a75750815115610298575090565b6102a4903b1515610238565b90565b8251909150156102ba5750805190602001fd5b6044604051809262461bcd60e51b8252602060048301526102ea8151809281602486015260208686019101610119565b601f01601f19168101030190fdfe608060405236156049577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546000808092368280378136915af43d82803e156045573d90f35b3d90fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546000808092368280378136915af43d82803e156045573d90f3fea26469706673582212208090d0af820f5b59d1da85476d8d46f49d4a89597f48aaa3b421c213ab65b52d64736f6c6343000811003360806040526103b480380380610014816100bd565b92833981016040828203126100a1578151916001600160a01b03831683036100a1576020810151906001600160401b0382116100a157019181601f840112156100a15782519261006b610066856100ef565b6100bd565b92848452602085830101116100a1576100939361008e9160208086019101610119565b61013c565b60405160bb90816102f98239f35b600080fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f191682016001600160401b038111838210176100e257604052565b6100ea6100a6565b604052565b6020906001600160401b03811161010c575b601f01601f19160190565b6101146100a6565b610101565b60005b83811061012c5750506000910152565b818101518382015260200161011c565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc81905561021f91906000906001600160a01b0381167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b8380a26040519282908190606086016001600160401b0381118782101761022b575b604052602786527f416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c6020870152660819985a5b195960ca1b6040870152602081519101845af4903d15610222573d61020f610066826100ef565b908152809360203d92013e610284565b50565b60609250610284565b6102336100a6565b6101b5565b1561023f57565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b919290156102a75750815115610298575090565b6102a4903b1515610238565b90565b8251909150156102ba5750805190602001fd5b6044604051809262461bcd60e51b8252602060048301526102ea8151809281602486015260208686019101610119565b601f01601f19168101030190fdfe608060405236156049577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546000808092368280378136915af43d82803e156045573d90f35b3d90fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546000808092368280378136915af43d82803e156045573d90f3fea26469706673582212208090d0af820f5b59d1da85476d8d46f49d4a89597f48aaa3b421c213ab65b52d64736f6c63430008110033a2646970667358221220f262623bfb427c26a6dafd37b5b230637927300f4d29e53b3e8507983378f99764736f6c63430008110033";
    let bytecode = Bytes::from_str(bytecode).unwrap();
    WalletFactoryConfig { abi, bytecode }
}

#[allow(dead_code)]
pub fn guardian_factory_abi() -> GuardianProxyConfig {
    let abi_json = r#"[
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "logic",
                    "type": "address"
                },
                {
                    "internalType": "bytes",
                    "name": "data",
                    "type": "bytes"
                }
            ],
            "stateMutability": "payable",
            "type": "constructor"
        },
        {
            "stateMutability": "payable",
            "type": "fallback"
        },
        {
            "stateMutability": "payable",
            "type": "receive"
        }
    ]"#;

    let abi = serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap();
    let bytecode = "0x60806040526103b480380380610014816100bd565b92833981016040828203126100a1578151916001600160a01b03831683036100a1576020810151906001600160401b0382116100a157019181601f840112156100a15782519261006b610066856100ef565b6100bd565b92848452602085830101116100a1576100939361008e9160208086019101610119565b61013c565b60405160bb90816102f98239f35b600080fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f191682016001600160401b038111838210176100e257604052565b6100ea6100a6565b604052565b6020906001600160401b03811161010c575b601f01601f19160190565b6101146100a6565b610101565b60005b83811061012c5750506000910152565b818101518382015260200161011c565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc81905561021f91906000906001600160a01b0381167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b8380a26040519282908190606086016001600160401b0381118782101761022b575b604052602786527f416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c6020870152660819985a5b195960ca1b6040870152602081519101845af4903d15610222573d61020f610066826100ef565b908152809360203d92013e610284565b50565b60609250610284565b6102336100a6565b6101b5565b1561023f57565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b919290156102a75750815115610298575090565b6102a4903b1515610238565b90565b8251909150156102ba5750805190602001fd5b6044604051809262461bcd60e51b8252602060048301526102ea8151809281602486015260208686019101610119565b601f01601f19168101030190fdfe608060405236156049577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546000808092368280378136915af43d82803e156045573d90f35b3d90fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546000808092368280378136915af43d82803e156045573d90f3fea26469706673582212208090d0af820f5b59d1da85476d8d46f49d4a89597f48aaa3b421c213ab65b52d64736f6c63430008110033";
    let bytecode = Bytes::from_str(bytecode).unwrap();
    GuardianProxyConfig { abi, bytecode }
}
