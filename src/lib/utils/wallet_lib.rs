use std::{
    convert::TryInto,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use ethers::{
    abi::{self, encode, FixedBytes, Token},
    contract::encode_function_data,
    prelude::*,
    providers::Provider,
    types::{Address, U256},
};

use crate::utils::{
    abis::*, bundler::UserOpErrCodes, gas_overhead::calc_gas_overhead, guardians::HookInputData,
};
use ethers::core::types::H256;

use super::bundler::{BundlerClient, UserOperationTransport};
use super::guardians::GuardHookInputData;
use super::signatures::pack_user_op_hash;
use super::{account_abstraction::get_user_op_hash, signatures::pack_signature};

#[derive(Debug, Clone)]
pub struct WalletLib {
    provider: String,
    bundler: String,
    wallet_factory_address: Address, //_soulWalletFactoryAddress
    default_callback_handler_address: Address, //_defaultCallbackHandlerAddress
    key_store_module_address: Address, // _keyStoreModuleAddress
    security_control_module_address: Address, // _securityControlModuleAddress
    entry_point_address: Address,
    wallet_logic_address: Address,
    chain_id: u64,
    days: i32,
    default_initial_guardian_safe_period: i32,
    bundler_client: BundlerClient,
}

#[derive(Debug, Clone)]
pub struct PreFund {
    pub deposit: U256,
    pub prefund: U256,
    pub missfund: U256,
}

impl WalletLib {
    pub fn new(
        provider: &str,
        bundler: &str,
        wallet_factory_address: &str,
        default_callback_handler_address: &str,
        key_store_module_address: &str,
        security_control_module_address: &str,
        entry_point_address: &str,
        wallet_logic_address: &str,
        chain_id: u64,
    ) -> Self {
        let days = 86400;
        let http_provider = Provider::<Http>::try_from(provider.clone()).unwrap();
        let bundler_client = BundlerClient::new(
            Address::from_str(entry_point_address).unwrap(),
            http_provider,
            bundler.to_string(),
        );
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
            entry_point_address: entry_point_address.parse().unwrap(),
            wallet_logic_address: wallet_logic_address.parse().unwrap(),
            chain_id,
            days: days,
            default_initial_guardian_safe_period: 2 * days,
            bundler_client,
        }
    }

    pub async fn initialize_data(
        &self,
        initial_key: Address,
        initial_guardian_hash: H256,
        initial_guardian_safeperiod: Option<i32>,
    ) -> eyre::Result<ethers::core::types::Bytes> {
        /*
            function initialize(
                address anOwner,
                address defalutCallbackHandler,
                bytes[] calldata modules,
                bytes[] calldata plugins
            )
        */
        let initial_guardian_safeperiod = match initial_guardian_safeperiod {
            Some(safe_period) => safe_period,
            None => self.default_initial_guardian_safe_period,
        };

        let default_initial_guardian_safe_period: H256 =
            format!("{:064x}", self.default_initial_guardian_safe_period)
                .parse()
                .unwrap();
        let initial_guardian_safeperiod: H256 = format!("{:064x}", initial_guardian_safeperiod)
            .parse()
            .unwrap();
        let mut security_control_module_and_data: Vec<u8> = Vec::new();
        security_control_module_and_data
            .extend_from_slice(self.security_control_module_address.as_bytes());
        security_control_module_and_data
            .extend_from_slice(default_initial_guardian_safe_period.as_bytes());

        // let zero_hash: H256 = [0u8; 32].into();
        let initial_key_h256: H256 = H256::from(initial_key);
        let mut key_store_module_and_data: Vec<u8> = Vec::new();
        key_store_module_and_data.extend_from_slice(self.key_store_module_address.as_bytes());
        let key_store_init_data = encode(&[
            Token::FixedBytes(FixedBytes::from(initial_key_h256.clone().to_fixed_bytes())),
            Token::FixedBytes(FixedBytes::from(initial_guardian_hash.to_fixed_bytes())),
            Token::FixedBytes(FixedBytes::from(
                initial_guardian_safeperiod.to_fixed_bytes(),
            )),
        ]);
        key_store_module_and_data.extend_from_slice(&key_store_init_data);

        let abi = abi_soul_wallet();
        let initialize_data = encode_function_data(
            abi.function("initialize")?,
            (
                Token::Address(initial_key),
                Token::Address(self.default_callback_handler_address),
                Token::Array(vec![
                    Token::Bytes(security_control_module_and_data),
                    Token::Bytes(key_store_module_and_data),
                ]),
                Token::Array(vec![]),
            ),
        )
        .unwrap();

        Ok(initialize_data)
    }

    pub async fn calc_wallet_address(
        &self,
        index: i32,
        initial_key: Address,
        initial_guard_hash: H256,
        initial_guardian_safeperiod: Option<i32>,
    ) -> eyre::Result<Address> {
        // let initial_key = initial_key.parse::<Address>().unwrap();
        let initialize_data = self
            .initialize_data(initial_key, initial_guard_hash, initial_guardian_safeperiod)
            .await?;

        let provider = Provider::<Http>::try_from(self.provider.clone())?;
        let wallet_factory = super::abis::WalletFactoryContract::new(
            self.wallet_factory_address,
            Arc::new(provider),
        );

        let index = U256::from(index);
        let salt: [u8; 32] = index.try_into().unwrap();

        let wallet_addr = wallet_factory
            .get_wallet_address(initialize_data, salt)
            .await?;

        Ok(wallet_addr)
    }

    pub async fn create_unsigned_deploy_wallet_user_op(
        &self,
        index: i32,
        initial_key: Address,
        initial_guard_hash: H256,
        call_data: &str,
        initial_guardian_safeperiod: Option<i32>,
    ) -> eyre::Result<UserOperationTransport> {
        let sender = self
            .calc_wallet_address(
                index,
                initial_key,
                initial_guard_hash.clone(),
                initial_guardian_safeperiod,
            )
            .await?;

        let abi = abi_soul_wallet_factory();
        // let initial_key = initial_key.parse::<Address>().unwrap();
        let initialize_data = self
            .initialize_data(initial_key, initial_guard_hash, initial_guardian_safeperiod)
            .await?;
        let index = U256::from(index);
        let index: [u8; 32] = index.try_into().unwrap();
        let init_code = abi
            .function("createWallet")?
            .encode_input(&[
                Token::Bytes(initialize_data.to_vec()),
                Token::FixedBytes(FixedBytes::from(index)),
            ])
            .map_err(|e| {
                eyre::eyre!(
                    "Failed to encode input for ClutchWallet initialize function: {}",
                    e
                )
            })?;

        let init_code = [self.wallet_factory_address.as_bytes(), init_code.as_ref()].concat();

        let user_operation = UserOperationTransport {
            sender,
            nonce: U256::from(0),
            init_code: ethers::types::Bytes::from(init_code),
            call_data: ethers::types::Bytes::from(b""),
            call_gas_limit: U256::from(0),
            verification_gas_limit: U256::from(0),
            pre_verification_gas: U256::from(10000000),
            max_fee_per_gas: U256::from(0),
            max_priority_fee_per_gas: U256::from(0),
            paymaster_and_data: ethers::types::Bytes::from(b""),
            signature: ethers::types::Bytes::from(b""),
        };

        Ok(user_operation)
    }

    pub async fn pre_fund(&self, user_op: UserOperationTransport) -> eyre::Result<PreFund> {
        /*
        function _getRequiredPrefund(MemoryUserOp memory mUserOp) internal pure returns (uint256 requiredPrefund) {
            unchecked {
                //when using a Paymaster, the verificationGasLimit is used also to as a limit for the postOp call.
                // our security model might call postOp eventually twice
                uint256 mul = mUserOp.paymaster != address(0) ? 3 : 1;
                uint256 requiredGas = mUserOp.callGasLimit + mUserOp.verificationGasLimit * mul + mUserOp.preVerificationGas;

                requiredPrefund = requiredGas * mUserOp.maxFeePerGas;
            }
        }
        */
        let zero = U256::zero();
        let max_fee_per_gas = user_op.max_fee_per_gas;
        let pre_verification_gas = user_op.pre_verification_gas;
        let verification_gas_limit = user_op.verification_gas_limit;
        let call_gas_limit = user_op.call_gas_limit;

        if max_fee_per_gas == zero || pre_verification_gas == zero || verification_gas_limit == zero
        {
            return Err(eyre::eyre!(
                "maxFeePerGas, preVerificationGas, verficiationGasLimit must > 0"
            ));
        }

        let mul = match user_op.paymaster_and_data.ne("".as_bytes()) {
            true => U256::from(3),
            false => U256::from(1),
        };

        let required_gas = call_gas_limit
            .checked_add(verification_gas_limit.checked_mul(mul).unwrap())
            .unwrap()
            .checked_add(pre_verification_gas)
            .unwrap();
        let required_prefund = required_gas.checked_mul(max_fee_per_gas).unwrap();
        let provider = Provider::<Http>::try_from(self.provider.clone())?;

        let entry_point_contract =
            super::abis::EntryPointContract::new(self.entry_point_address, Arc::new(provider));

        let deposit: U256 = entry_point_contract.balance_of(user_op.sender).await?;
        let missfund = match deposit.lt(&required_prefund) {
            true => required_prefund.checked_sub(deposit).unwrap(),
            false => zero,
        };

        let ret = PreFund {
            deposit: deposit,
            prefund: required_prefund,
            missfund: missfund,
        };
        println!("======{:?}", ret.clone());
        Ok(ret)
    }

    pub async fn user_op_hash(&self, user_op: UserOperationTransport) -> eyre::Result<Vec<u8>> {
        let user_op_hash =
            get_user_op_hash(user_op, self.entry_point_address, self.chain_id).unwrap();
        Ok(user_op_hash)
    }

    pub async fn pack_user_op_hash(
        &self,
        user_op: UserOperationTransport,
        valid_after: Option<u64>,
        valid_until: Option<u64>,
    ) -> eyre::Result<(Vec<u8>, U256)> {
        let user_op_hash = self.user_op_hash(user_op).await?;
        let ret = pack_user_op_hash(user_op_hash, valid_after, valid_until).unwrap();
        Ok(ret)
    }

    async fn guard_hook_list(&self, wallet_address: Address) -> eyre::Result<Vec<Address>> {
        // function listPlugin(uint8 hookType) external view returns (address[] memory plugins);
        let provider = Provider::<Http>::try_from(self.provider.clone())?;
        let wallet_contract = WalletContract::new(wallet_address, Arc::new(provider));
        let guard_hook_list: Vec<Address> = wallet_contract.list_plugin(1).await?;
        Ok(guard_hook_list)
    }

    pub async fn pack_user_op_signature(
        &self,
        signature: Vec<u8>,
        validation_data: U256,
        guard_hook_input_data: Option<GuardHookInputData>,
    ) -> eyre::Result<Vec<u8>> {
        let mut hook_input_data: Option<HookInputData> = None;
        if let Some(guard_hook_data) = guard_hook_input_data {
            let guard_hooks = self.guard_hook_list(guard_hook_data.sender).await?;

            hook_input_data = Some(HookInputData {
                guard_hooks: guard_hooks,
                input_data: guard_hook_data.input_data,
            });
        }

        let pack_signature = pack_signature(signature, validation_data, hook_input_data)?;
        Ok(pack_signature)
    }

    pub async fn estimate_user_operation_gas(
        &mut self,
        user_op: &mut UserOperationTransport,
        semi_valid_guard_hook_input_data: Option<GuardHookInputData>,
    ) -> eyre::Result<bool> {
        // let mut user_op = user_op.clone();
        if let Some(semi_valid_guard_input_data) = semi_valid_guard_hook_input_data.clone() {
            if semi_valid_guard_input_data.sender.ne(&user_op.sender) {
                return Err(eyre::eyre!(
                    "error code:{:?}, invalid sender {}",
                    UserOpErrCodes::UnknownError,
                    semi_valid_guard_input_data.sender
                ));
            }

            if user_op.init_code.eq("".as_bytes()) {
                return Err(eyre::eyre!(
                    "error code:{:?}, cannot set semi valid guard hook input data when the contract wallet is not deployed {}",
                    UserOpErrCodes::UnknownError,
                    semi_valid_guard_input_data.sender
                ));
            }
        }

        let semi_valid_signature = user_op.signature.eq_ignore_ascii_case("".as_bytes());
        if semi_valid_signature {
            let signature = ethers::types::Bytes::from_str("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
            let validation_data = (U256::from(68719476735 as u64) << U256::from(160))
                + (U256::from(1599999999) << U256::from(160 + 48));
            let signature_ret = self
                .pack_user_op_signature(
                    signature.to_vec(),
                    validation_data,
                    semi_valid_guard_hook_input_data,
                )
                .await?;

            user_op.signature = ethers::types::Bytes::from(signature_ret);
        }
        self.bundler_client.init().await?;

        let user_op_gas_ret = self
            .bundler_client
            .eth_estimate_user_operation_gas(user_op.clone())
            .await?;
        user_op.pre_verification_gas = user_op_gas_ret.pre_verification_gas;
        user_op.verification_gas_limit = user_op_gas_ret.verification_gas_limit;

        // Value of 'gas': Even number: automatic setting,
        //                 Odd number: manually specified. Do not override!
        let is_even = user_op.call_gas_limit % U256::from(2) == U256::from(0);
        if is_even {
            let mut new_call_gas_limit = user_op_gas_ret.call_gas_limit;
            if new_call_gas_limit % U256::from(2) == U256::from(1) {
                new_call_gas_limit = new_call_gas_limit.checked_add(U256::from(1)).unwrap();
            }

            user_op.call_gas_limit = new_call_gas_limit;
        }

        let _ = calc_gas_overhead(user_op);
        if semi_valid_signature {
            user_op.signature = ethers::types::Bytes::from(b"");
        }
        Ok(true)
    }

    pub async fn send_user_operation(&self, user_op: UserOperationTransport) -> eyre::Result<bool> {
        let send_user_op_ret = self
            .bundler_client
            .eth_send_user_operation(user_op.clone())
            .await?;

        let user_op_hash_local = self.user_op_hash(user_op).await?;

        if send_user_op_ret.eq_ignore_ascii_case(&user_op_hash_local) == false {
            eyre::eyre!("user_op_hash != user_op_hash_local");
        }

        Ok(true)
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
