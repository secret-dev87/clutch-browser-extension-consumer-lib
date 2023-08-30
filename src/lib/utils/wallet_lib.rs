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
    type_guard,
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
pub struct WalletInstance;

#[derive(Debug, Clone)]
pub struct PreFund {
    pub deposit: U256,
    pub prefund: U256,
    pub missfund: U256,
}

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub to: Address,
    pub value: Option<U256>,
    pub data: Option<Bytes>,
    pub gas_limit: Option<U256>,
}

impl WalletInstance {
    fn approve(address: Address, amount: U256) -> eyre::Result<Bytes> {
        let erc20_abi = abi_erc20();
        let approve_call_data = encode_function_data(
            erc20_abi.function("approve")?,
            (Token::Address(address), Token::Uint(amount)),
        )?;
        Ok(approve_call_data)
    }

    pub fn execute_batch(token_addresses: Vec<Address>, data: Vec<Bytes>) -> eyre::Result<Bytes> {
        let mut tokens = token_addresses.into_iter().map(|item| Token::Address(item)).collect::<Vec<Token>>();
        let mut batch_data = data.into_iter().map(|d| Token::Bytes(d.to_vec())).collect::<Vec<Token>>();
        let clutch_abi = abi_clutch_wallet();
        let call_data =
            encode_function_data(clutch_abi.function("executeBatch")?, (Token::Array(tokens), Token::Array(batch_data)))?;
        Ok(call_data)
    }
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

        let abi = abi_clutch_wallet();
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

        let abi = abi_clutch_wallet_factory();
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

        println!("user_op_gas_ret, {:?}", user_op_gas_ret);
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
        // println!("{}", Bytes::from(user_op_hash_local.clone()));
        if send_user_op_ret.eq_ignore_ascii_case(&user_op_hash_local) == false {
            return Err(eyre::eyre!("user_op_hash != user_op_hash_local"));
        }
        Ok(true)
    }

    pub async fn get_nonce(
        &self,
        wallet_address: Address,
        nonce_key: Option<Bytes>,
    ) -> eyre::Result<U256> {
        let key = match nonce_key {
            Some(_key) => _key,
            None => Bytes::from_str("").unwrap(),
        };
        let key = type_guard::max_to_uint192(key).unwrap();
        let provider = Provider::<Http>::try_from(self.provider.clone())?;
        let entry_point_contract =
            super::abis::EntryPointContract::new(self.entry_point_address, Arc::new(provider));
        let nonce = entry_point_contract.get_nonce(wallet_address, key).await?;
        Ok(nonce)
    }

    pub async fn from_transaction(
        &self,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: U256,
        from: Address,
        txs: Vec<Transaction>,
        nonce_key: Option<Bytes>,
    ) -> eyre::Result<UserOperationTransport> {
        if txs.len() == 0 {
            return Err(eyre::eyre!("txs.length == 0"));
        }

        if let Ok(ret) = self.wallet_deployed(&from).await {
            if ret == false {
                return Err(eyre::eyre!("{} is not a deployed contract", from));
            }
        }

        let mut call_gas_limit = U256::zero();
        for tx in txs.iter() {
            call_gas_limit = match tx.gas_limit {
                Some(limit) => call_gas_limit.checked_add(limit).unwrap(),
                None => U256::zero(),
            }
        }

        call_gas_limit = match call_gas_limit.div_mod(U256::from(2)) {
            (divide, remain) if remain == U256::from(2) => {
                call_gas_limit.checked_add(U256::from(1)).unwrap()
            }
            _ => call_gas_limit,
        };

        let nonce_ret = self.get_nonce(from, nonce_key).await?;

        let mut call_data: Bytes = Bytes::from(b"");
        let mut to: Vec<Token> = Vec::new();
        let mut values: Vec<Token> = Vec::new();
        let mut data: Vec<Token> = Vec::new();
        let mut has_value = false;

        let abi = abi_clutch_wallet();

        for tx in txs.iter() {
            to.push(Token::Address(tx.to));
            let value = match tx.value {
                Some(val) => {
                    has_value = true;
                    val
                }
                None => U256::zero(),
            };
            values.push(Token::Uint(value));
            let element_data = match tx.data.clone() {
                Some(data) => data,
                None => Bytes::from(b""),
            };
            data.push(Token::Bytes(element_data.to_vec()))
        }

        if txs.len() > 1 {
            if has_value {
                call_data = encode_function_data(
                    abi.function("executeBatchexecuteBatch")?,
                    (Token::Array(to), Token::Array(values), Token::Array(data)),
                )
                .unwrap();
            } else {
                call_data = encode_function_data(
                    abi.function("executeBatch")?,
                    (Token::Array(to), Token::Array(data)),
                )
                .unwrap();
            }
        } else {
            call_data = encode_function_data(
                abi.function("execute")?,
                (to[0].clone(), values[0].clone(), data[0].clone()),
            )
            .unwrap();
        }

        let user_operation = UserOperationTransport {
            sender: from,
            nonce: nonce_ret,
            init_code: Bytes::from(b""),
            call_data: call_data,
            call_gas_limit: call_gas_limit,
            verification_gas_limit: U256::zero(),
            pre_verification_gas: U256::zero(),
            max_fee_per_gas: max_fee_per_gas,
            max_priority_fee_per_gas: max_priority_fee_per_gas,
            paymaster_and_data: Bytes::from(b""),
            signature: Bytes::from(b""),
        };

        Ok(user_operation)
    }

    pub async fn paymaster_and_data(pay_token: Address, paymaster: Address) -> eyre::Result<Bytes> {
        if pay_token == Address::zero() {
            return Ok(Bytes::from(b""));
        }

        let mut ret: Vec<u8> = Vec::new();
        ret.extend_from_slice(paymaster.as_bytes());
        let encode = encode(&[
            Token::Address(pay_token),
            Token::Int(ethers::utils::parse_ether(1000).unwrap()),
        ]);
        ret.extend_from_slice(&encode);
        return Ok(Bytes::from(ret));
    }

    pub fn transfer_erc20_calldata(token: Address, amount: U256) -> eyre::Result<Bytes> {
        let abi = abi_erc20();
        let calldata = encode_function_data(
            abi.function("transfer")?,
            (Token::Address(token), Token::Uint(amount)),
        )
        .unwrap();
        return Ok(calldata);
    }

    async fn wallet_deployed(&self, wallet_addr: &Address) -> eyre::Result<bool> {
        let provider = Provider::<Http>::try_from(self.provider.clone())?;
        let code = provider.get_code(*wallet_addr, None).await?;
        let no_code = Bytes::from_str("").unwrap();

        match code.eq(&no_code) {
            true => Ok(false),
            false => Ok(true),
        }
    }

    pub fn entrypoint(&self) -> Address {
        self.entry_point_address.clone()
    }

    pub fn bundler(&self) -> BundlerClient {
        self.bundler_client.clone()
    }
}

fn generated_contract_path(contract: &str) -> PathBuf {
    let current_file = file!();
    let path = Path::new(current_file).parent().unwrap();
    let contract_path = format!("../generated/abi/contract_{contract}.json");
    path.join(contract_path)
}

fn standard_contract_path(contract: &str) -> PathBuf {
    let current_file = file!();
    let path = Path::new(current_file).parent().unwrap();
    let contract_path = format!("../abi/{contract}.json");
    path.join(contract_path)
}

pub fn abi_clutch_wallet() -> abi::Abi {
    let abi_path = generated_contract_path("soulwallet");
    let mut file = File::open(abi_path).unwrap();
    let mut abi_json = String::new();
    let _ = file.read_to_string(&mut abi_json).unwrap();
    serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap()
}

pub fn abi_clutch_wallet_factory() -> abi::Abi {
    let abi_path = generated_contract_path("soulwalletfactory");
    let mut file = File::open(abi_path).unwrap();
    let mut abi_json = String::new();
    let _ = file.read_to_string(&mut abi_json).unwrap();
    serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap()
}

pub fn abi_entry_point() -> abi::Abi {
    let abi_path = generated_contract_path("entrypoint");
    let mut file = File::open(abi_path).unwrap();
    let mut abi_json = String::new();
    let _ = file.read_to_string(&mut abi_json).unwrap();
    serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap()
}

pub fn abi_erc20() -> abi::Abi {
    let abi_path = standard_contract_path("ERC20");
    let mut file = File::open(abi_path).unwrap();
    let mut abi_json = String::new();
    let _ = file.read_to_string(&mut abi_json).unwrap();
    serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap()
}

pub fn abi_erc721() -> abi::Abi {
    let abi_path = standard_contract_path("ERC721");
    let mut file = File::open(abi_path).unwrap();
    let mut abi_json = String::new();
    let _ = file.read_to_string(&mut abi_json).unwrap();
    serde_json::from_str::<ethers::abi::Contract>(&abi_json).unwrap()
}
