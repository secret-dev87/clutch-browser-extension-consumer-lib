use std::vec;

use clutch_wallet_lib::utils::clutch_wallet_lib::ClutchWalletLib;
use ethers::{
    types::{Address, U256},
    utils::parse_units,
};
use expectest::prelude::*;

mod helpers;
use crate::helpers::{guardian_factory_abi, wallet_factory_abi};

// #[tokio::test]
// async fn test_get_initialize_data() {
//     let singleton_factory_address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//         .parse::<Address>()
//         .unwrap();
//     let wallet_factory_config = wallet_factory_abi();
//     let guardian_proxy = guardian_factory_abi();

//     let wallet_lib = ClutchWalletLib::new(
//         singleton_factory_address,
//         wallet_factory_config,
//         guardian_proxy,
//     );

//     let entry_point_address = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
//         .parse::<Address>()
//         .unwrap();
//     let owner_address = "0xF3C0D700b4D3617d6b963c61074CD206AAaBA9F1"
//         .parse::<Address>()
//         .unwrap();
//     let upgrade_delay = U256::from(30);
//     let guardian_delay = U256::from(30);
//     let guardian_address = "0x77Cd03e8a2e57134c02d23Ef544a8B30A10Ec9A7"
//         .parse::<Address>()
//         .unwrap();

//     let data = wallet_lib
//         .get_initialize_data(
//             entry_point_address,
//             owner_address,
//             upgrade_delay,
//             guardian_delay,
//             guardian_address,
//         )
//         .await
//         .unwrap();

//     let expected_data = "82778654000000000000000000000000cf7ed3acca5a467e9e704c703e8d87f634fb0fc9000000000000000000000000f3c0d700b4d3617d6b963c61074cd206aaaba9f1000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000000000000000000000000000000000000000001e00000000000000000000000077cd03e8a2e57134c02d23ef544a8b30a10ec9a7";
//     expect!(hex::encode(data)).to(be_equal_to(expected_data));
// }

// #[tokio::test]
// async fn test_get_wallet_code() {
//     let singleton_factory_address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//         .parse::<Address>()
//         .unwrap();
//     let wallet_factory_config = wallet_factory_abi();
//     let guardian_proxy = guardian_factory_abi();

//     let wallet_lib = ClutchWalletLib::new(
//         singleton_factory_address,
//         wallet_factory_config,
//         guardian_proxy,
//     );

//     let logic_address = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
//         .parse::<Address>()
//         .unwrap();
//     let entry_point_address = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
//         .parse::<Address>()
//         .unwrap();
//     let owner_address = "0xa5335eE0F04863a99dCD1e54254eF2cEF22c85ef"
//         .parse::<Address>()
//         .unwrap();
//     let upgrade_delay = U256::from(30);
//     let guardian_delay = U256::from(30);
//     let guardian_address = "0xcf7C2B7A8D0cEBDdFae7da2e5167776E36582EbC"
//         .parse::<Address>()
//         .unwrap();

//     let data = wallet_lib
//         .get_wallet_code(
//             logic_address,
//             entry_point_address,
//             owner_address,
//             upgrade_delay,
//             guardian_delay,
//             guardian_address,
//         )
//         .await
//         .unwrap();

//     let expected_data = "60806040526103b480380380610014816100bd565b92833981016040828203126100a1578151916001600160a01b03831683036100a1576020810151906001600160401b0382116100a157019181601f840112156100a15782519261006b610066856100ef565b6100bd565b92848452602085830101116100a1576100939361008e9160208086019101610119565b61013c565b60405160bb90816102f98239f35b600080fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f191682016001600160401b038111838210176100e257604052565b6100ea6100a6565b604052565b6020906001600160401b03811161010c575b601f01601f19160190565b6101146100a6565b610101565b60005b83811061012c5750506000910152565b818101518382015260200161011c565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc81905561021f91906000906001600160a01b0381167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b8380a26040519282908190606086016001600160401b0381118782101761022b575b604052602786527f416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c6020870152660819985a5b195960ca1b6040870152602081519101845af4903d15610222573d61020f610066826100ef565b908152809360203d92013e610284565b50565b60609250610284565b6102336100a6565b6101b5565b1561023f57565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b919290156102a75750815115610298575090565b6102a4903b1515610238565b90565b8251909150156102ba5750805190602001fd5b6044604051809262461bcd60e51b8252602060048301526102ea8151809281602486015260208686019101610119565b601f01601f19168101030190fdfe608060405236156049577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546000808092368280378136915af43d82803e156045573d90f35b3d90fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546000808092368280378136915af43d82803e156045573d90f3fea26469706673582212208090d0af820f5b59d1da85476d8d46f49d4a89597f48aaa3b421c213ab65b52d64736f6c634300081100330000000000000000000000009fe46736679d2d9a65f0992f2272de9f3c7fa6e0000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a482778654000000000000000000000000cf7ed3acca5a467e9e704c703e8d87f634fb0fc9000000000000000000000000a5335ee0f04863a99dcd1e54254ef2cef22c85ef000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000cf7c2b7a8d0cebddfae7da2e5167776e36582ebc00000000000000000000000000000000000000000000000000000000";
//     expect!(hex::encode(data)).to(be_equal_to(expected_data));
// }

// #[tokio::test]
// async fn test_calculate_wallet_address_by_code_hash() {
//     let singleton_factory_address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//         .parse::<Address>()
//         .unwrap();
//     let wallet_factory_config = wallet_factory_abi();
//     let guardian_proxy = guardian_factory_abi();

//     let wallet_lib = ClutchWalletLib::new(
//         singleton_factory_address,
//         wallet_factory_config,
//         guardian_proxy,
//     );

//     let init_code_hash =
//         hex::decode("ae95c2cdc8cf35c22e8b78985c008b0d71f724727c81d10f9bcf94bec725511f").unwrap();
//     let salt =
//         hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
//     let singleton_factory_address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//         .parse::<Address>()
//         .unwrap();

//     let data = wallet_lib
//         .calculate_wallet_address_by_code_hash(init_code_hash, salt)
//         .await
//         .unwrap();

//     let expected_data = "3fcc127e83655e1eef90697fecf6910ebc48001e";
//     expect!(hex::encode(data)).to(be_equal_to(expected_data));
// }

// #[tokio::test]
// async fn test_calculate_wallet_address() {
//     let singleton_factory_address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//         .parse::<Address>()
//         .unwrap();
//     let wallet_factory_config = wallet_factory_abi();
//     let guardian_proxy = guardian_factory_abi();

//     let wallet_lib = ClutchWalletLib::new(
//         singleton_factory_address,
//         wallet_factory_config,
//         guardian_proxy,
//     );

//     let wallet_logic_address = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
//         .parse::<Address>()
//         .unwrap();
//     let entrypoint_address = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
//         .parse::<Address>()
//         .unwrap();
//     let owner_address = "0xD98ba10b4B333798604A0d3db7EA210F487C04BA"
//         .parse::<Address>()
//         .unwrap();
//     let upgrade_delay = U256::from(30);
//     let guardian_delay = U256::from(30);
//     let guardian_address = "0xC5F4B97f914cad5aedeB077432fEC957d4150a16"
//         .parse::<Address>()
//         .unwrap();
//     let salt =
//         hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
//     let singleton_factory_address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//         .parse::<Address>()
//         .unwrap();

//     let data = wallet_lib
//         .calculate_wallet_address(
//             wallet_logic_address,
//             entrypoint_address,
//             owner_address,
//             upgrade_delay,
//             guardian_delay,
//             guardian_address,
//             salt,
//         )
//         .await
//         .unwrap();

//     let expected_address = "0xb7a65338fd24965cf6c35ac274782c7fe7223343"
//         .parse::<Address>()
//         .unwrap();

//     expect!(data).to(be_equal_to(expected_address));
// }

// #[tokio::test]
// async fn test_get_packed_init_code_using_wallet_factory() {
//     let singleton_factory_address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//         .parse::<Address>()
//         .unwrap();
//     let wallet_factory_config = wallet_factory_abi();
//     let guardian_proxy = guardian_factory_abi();

//     let wallet_lib = ClutchWalletLib::new(
//         singleton_factory_address,
//         wallet_factory_config,
//         guardian_proxy,
//     );

//     let wallet_logic_address = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
//         .parse::<Address>()
//         .unwrap();
//     let entrypoint_address = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
//         .parse::<Address>()
//         .unwrap();
//     let owner_address = "0xD98ba10b4B333798604A0d3db7EA210F487C04BA"
//         .parse::<Address>()
//         .unwrap();
//     let upgrade_delay = U256::from(30);
//     let guardian_delay = U256::from(30);
//     let guardian_address = "0xC5F4B97f914cad5aedeB077432fEC957d4150a16"
//         .parse::<Address>()
//         .unwrap();
//     let salt =
//         hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();

//     let data = wallet_lib
//         .get_packed_init_code_using_wallet_factory(
//             wallet_logic_address,
//             entrypoint_address,
//             owner_address,
//             upgrade_delay,
//             guardian_delay,
//             guardian_address,
//             salt,
//         )
//         .await
//         .unwrap();

//     let expected_data = "ef773838631dd2cf038d825120cbffab3482fbecf4528808000000000000000000000000cf7ed3acca5a467e9e704c703e8d87f634fb0fc9000000000000000000000000d98ba10b4b333798604a0d3db7ea210f487c04ba000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000c5f4b97f914cad5aedeb077432fec957d4150a160000000000000000000000000000000000000000000000000000000000000000";
//     expect!(hex::encode(data)).to(be_equal_to(expected_data));
// }

// #[tokio::test]
// async fn test_activate_wallet_op() {
//     let singleton_factory_address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//         .parse::<Address>()
//         .unwrap();
//     let wallet_factory_config = wallet_factory_abi();
//     let guardian_proxy = guardian_factory_abi();

//     let wallet_lib = ClutchWalletLib::new(
//         singleton_factory_address,
//         wallet_factory_config,
//         guardian_proxy,
//     );

//     let wallet_logic_address = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
//         .parse::<Address>()
//         .unwrap();
//     let entrypoint_address = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
//         .parse::<Address>()
//         .unwrap();
//     let owner_address = "0x8bB47403DD540f830D0d913bC843380EeD5779D6"
//         .parse::<Address>()
//         .unwrap();
//     let upgrade_delay = U256::from(30);
//     let guardian_delay = U256::from(30);
//     let guardian_address = "0xA70fB18035E823901a067De21a9591079D648e17"
//         .parse::<Address>()
//         .unwrap();
//     let salt =
//         hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
//     let max_fee_per_gas = parse_units("100000000000", "wei").unwrap().into();
//     let max_priority_fee_per_gas = parse_units("100000000000", "wei").unwrap().into();
//     let paymaster_and_data = vec![];

//     let data = wallet_lib
//         .activate_wallet_op(
//             wallet_logic_address,
//             entrypoint_address,
//             owner_address,
//             upgrade_delay,
//             guardian_delay,
//             guardian_address,
//             paymaster_and_data,
//             max_fee_per_gas,
//             max_priority_fee_per_gas,
//             salt,
//         )
//         .await
//         .unwrap();

//     let json = serde_json::to_string(&data).unwrap();
//     let expected_json = "{\"sender\":\"0xe5db7525a6d376704e9e460781770126f124b5ef\",\"nonce\":\"0x0\",\"init_code\":\"0xef773838631dd2cf038d825120cbffab3482fbecf4528808000000000000000000000000cf7ed3acca5a467e9e704c703e8d87f634fb0fc90000000000000000000000008bb47403dd540f830d0d913bc843380eed5779d6000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000a70fb18035e823901a067de21a9591079d648e170000000000000000000000000000000000000000000000000000000000000000\",\"call_data\":\"0x\",\"call_gas_limit\":\"0x0\",\"verification_gas_limit\":\"0x0\",\"pre_verification_gas\":\"0x0\",\"max_fee_per_gas\":\"0x174876e800\",\"max_priority_fee_per_gas\":\"0x174876e800\",\"paymaster_and_data\":\"0x\",\"signature\":\"0x\"}";

//     expect!(json).to(be_equal_to(expected_json));
// }
