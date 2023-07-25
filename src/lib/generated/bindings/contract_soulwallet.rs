pub use soul_wallet::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod soul_wallet {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract IEntryPoint\",\"name\":\"_EntryPoint\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"ADDRESS_ALREADY_EXISTS\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ADDRESS_NOT_EXISTS\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"CALLER_MUST_BE_ENTRYPOINT\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"CALLER_MUST_BE_MODULE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"CALLER_MUST_BE_SELF_OR_MODULE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"HASH_ALREADY_APPROVED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"HASH_ALREADY_REJECTED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_ADDRESS\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_GUARD_HOOK_DATA\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_LOGIC_ADDRESS\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_SELECTOR\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_SIGNTYPE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"MODULE_ADDRESS_EMPTY\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"MODULE_EXECUTE_FROM_MODULE_RECURSIVE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"MODULE_NOT_SUPPORT_INTERFACE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"MODULE_SELECTORS_EMPTY\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NOT_IMPLEMENTED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NO_OWNER\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"PLUGIN_ADDRESS_EMPTY\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"PLUGIN_HOOK_TYPE_ERROR\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"PLUGIN_INIT_FAILED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"PLUGIN_NOT_REGISTERED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"PLUGIN_NOT_SUPPORT_INTERFACE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"PLUGIN_POST_HOOK_FAILED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"PLUGIN_PRE_HOOK_FAILED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"SAME_LOGIC_ADDRESS\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"SELECTOR_ALREADY_EXISTS\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UNSUPPORTED_SIGNTYPE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UPGRADE_FAILED\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"}],\"name\":\"ApproveHash\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"fallbackContract\",\"type\":\"address\"}],\"name\":\"FallbackChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\"}],\"name\":\"Initialized\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\"}],\"name\":\"ModuleAdded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\"}],\"name\":\"ModuleRemoved\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\"}],\"name\":\"ModuleRemovedWithError\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"OwnerAdded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"OwnerCleared\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"OwnerRemoved\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"plugin\",\"type\":\"address\"}],\"name\":\"PluginAdded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"plugin\",\"type\":\"address\"}],\"name\":\"PluginRemoved\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"plugin\",\"type\":\"address\"}],\"name\":\"PluginRemovedWithError\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"}],\"name\":\"RejectHash\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oldImplementation\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\"}],\"name\":\"Upgraded\",\"type\":\"event\"},{\"stateMutability\":\"payable\",\"type\":\"fallback\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"moduleAndData\",\"type\":\"bytes\"}],\"name\":\"addModule\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"addOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"owners\",\"type\":\"address[]\"}],\"name\":\"addOwners\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"pluginAndData\",\"type\":\"bytes\"}],\"name\":\"addPlugin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"}],\"name\":\"approveHash\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"entryPoint\",\"outputs\":[{\"internalType\":\"contract IEntryPoint\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dest\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"func\",\"type\":\"bytes\"}],\"name\":\"execute\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"dest\",\"type\":\"address[]\"},{\"internalType\":\"bytes[]\",\"name\":\"func\",\"type\":\"bytes[]\"}],\"name\":\"executeBatch\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"dest\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"value\",\"type\":\"uint256[]\"},{\"internalType\":\"bytes[]\",\"name\":\"func\",\"type\":\"bytes[]\"}],\"name\":\"executeBatch\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"executeFromModule\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"anOwner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"defalutCallbackHandler\",\"type\":\"address\"},{\"internalType\":\"bytes[]\",\"name\":\"modules\",\"type\":\"bytes[]\"},{\"internalType\":\"bytes[]\",\"name\":\"plugins\",\"type\":\"bytes[]\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\"}],\"name\":\"isAuthorizedModule\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"plugin\",\"type\":\"address\"}],\"name\":\"isAuthorizedPlugin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\"}],\"name\":\"isOwner\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"name\":\"isValidSignature\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"magicValue\",\"type\":\"bytes4\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"listModule\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"modules\",\"type\":\"address[]\"},{\"internalType\":\"bytes4[][]\",\"name\":\"selectors\",\"type\":\"bytes4[][]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"listOwner\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"owners\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"hookType\",\"type\":\"uint8\"}],\"name\":\"listPlugin\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"plugins\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"plugin\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"key\",\"type\":\"bytes32\"}],\"name\":\"pluginDataLoad\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"key\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"value\",\"type\":\"bytes\"}],\"name\":\"pluginDataStore\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"}],\"name\":\"rejectHash\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\"}],\"name\":\"removeModule\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"removeOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"plugin\",\"type\":\"address\"}],\"name\":\"removePlugin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"resetOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"newOwners\",\"type\":\"address[]\"}],\"name\":\"resetOwners\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fallbackContract\",\"type\":\"address\"}],\"name\":\"setFallbackHandler\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldImplementation\",\"type\":\"address\"}],\"name\":\"upgradeFrom\",\"outputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\"}],\"name\":\"upgradeTo\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct UserOperation\",\"name\":\"userOp\",\"type\":\"tuple\"},{\"internalType\":\"bytes32\",\"name\":\"userOpHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"missingAccountFunds\",\"type\":\"uint256\"}],\"name\":\"validateUserOp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"validationData\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]";
    ///The parsed JSON ABI of the contract.
    pub static SOULWALLET_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct SoulWallet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SoulWallet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SoulWallet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SoulWallet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SoulWallet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SoulWallet)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SoulWallet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SOULWALLET_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addModule` (0xd3bdf4b5) function
        pub fn add_module(
            &self,
            module_and_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 189, 244, 181], module_and_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOwner` (0x7065cb48) function
        pub fn add_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 101, 203, 72], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOwners` (0x6c46a2c5) function
        pub fn add_owners(
            &self,
            owners: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 70, 162, 197], owners)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPlugin` (0x47361356) function
        pub fn add_plugin(
            &self,
            plugin_and_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 54, 19, 86], plugin_and_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveHash` (0xd4d9bdcd) function
        pub fn approve_hash(
            &self,
            hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 217, 189, 205], hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `entryPoint` (0xb0d691fe) function
        pub fn entry_point(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([176, 214, 145, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0xb61d27f6) function
        pub fn execute(
            &self,
            dest: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            func: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 29, 39, 246], (dest, value, func))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch` (0x18dfb3c7) function
        pub fn execute_batch(
            &self,
            dest: ::std::vec::Vec<::ethers::core::types::Address>,
            func: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 223, 179, 199], (dest, func))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch` (0x47e1da2a) function
        pub fn execute_batch_with_dest_and_value(
            &self,
            dest: ::std::vec::Vec<::ethers::core::types::Address>,
            value: ::std::vec::Vec<::ethers::core::types::U256>,
            func: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 225, 218, 42], (dest, value, func))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeFromModule` (0xae9411ab) function
        pub fn execute_from_module(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 148, 17, 171], (to, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0xd087d288) function
        pub fn get_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([208, 135, 210, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xa6c87ecb) function
        pub fn initialize(
            &self,
            an_owner: ::ethers::core::types::Address,
            defalut_callback_handler: ::ethers::core::types::Address,
            modules: ::std::vec::Vec<::ethers::core::types::Bytes>,
            plugins: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [166, 200, 126, 203],
                    (an_owner, defalut_callback_handler, modules, plugins),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAuthorizedModule` (0x399dd463) function
        pub fn is_authorized_module(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 157, 212, 99], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAuthorizedPlugin` (0x48d7b1d4) function
        pub fn is_authorized_plugin(
            &self,
            plugin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([72, 215, 177, 212], plugin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOwner` (0x2f54bf6e) function
        pub fn is_owner(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 84, 191, 110], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x1626ba7e) function
        pub fn is_valid_signature(
            &self,
            hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([22, 38, 186, 126], (hash, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `listModule` (0x6bbd74d3) function
        pub fn list_module(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::std::vec::Vec<[u8; 4]>>,
            ),
        > {
            self.0
                .method_hash([107, 189, 116, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `listOwner` (0xa52afc20) function
        pub fn list_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([165, 42, 252, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `listPlugin` (0x7799da64) function
        pub fn list_plugin(
            &self,
            hook_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([119, 153, 218, 100], hook_type)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pluginDataLoad` (0x3900c3ba) function
        pub fn plugin_data_load(
            &self,
            plugin: ::ethers::core::types::Address,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([57, 0, 195, 186], (plugin, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pluginDataStore` (0x14ee68ad) function
        pub fn plugin_data_store(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 238, 104, 173], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rejectHash` (0xe3b57475) function
        pub fn reject_hash(
            &self,
            hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 181, 116, 117], hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeModule` (0xa0632461) function
        pub fn remove_module(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 99, 36, 97], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeOwner` (0x173825d9) function
        pub fn remove_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 56, 37, 217], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePlugin` (0xa4d95b64) function
        pub fn remove_plugin(
            &self,
            plugin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 217, 91, 100], plugin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetOwner` (0x73cc802a) function
        pub fn reset_owner(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 204, 128, 42], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetOwners` (0xf093ac9c) function
        pub fn reset_owners(
            &self,
            new_owners: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 147, 172, 156], new_owners)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFallbackHandler` (0xf08a0323) function
        pub fn set_fallback_handler(
            &self,
            fallback_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 138, 3, 35], fallback_contract)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeFrom` (0xf0e06327) function
        pub fn upgrade_from(
            &self,
            old_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 224, 99, 39], old_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeTo` (0x3659cfe6) function
        pub fn upgrade_to(
            &self,
            new_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateUserOp` (0x3a871cdd) function
        pub fn validate_user_op(
            &self,
            user_op: UserOperation,
            user_op_hash: [u8; 32],
            missing_account_funds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [58, 135, 28, 221],
                    (user_op, user_op_hash, missing_account_funds),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ApproveHash` event
        pub fn approve_hash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApproveHashFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FallbackChanged` event
        pub fn fallback_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FallbackChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ModuleAdded` event
        pub fn module_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ModuleAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ModuleRemoved` event
        pub fn module_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ModuleRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ModuleRemovedWithError` event
        pub fn module_removed_with_error_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ModuleRemovedWithErrorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnerAdded` event
        pub fn owner_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnerCleared` event
        pub fn owner_cleared_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerClearedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnerRemoved` event
        pub fn owner_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PluginAdded` event
        pub fn plugin_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PluginAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PluginRemoved` event
        pub fn plugin_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PluginRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PluginRemovedWithError` event
        pub fn plugin_removed_with_error_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PluginRemovedWithErrorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RejectHash` event
        pub fn reject_hash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RejectHashFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SoulWalletEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SoulWallet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ADDRESS_ALREADY_EXISTS` with signature `ADDRESS_ALREADY_EXISTS()` and selector `0xf2d4d191`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ADDRESS_ALREADY_EXISTS", abi = "ADDRESS_ALREADY_EXISTS()")]
    pub struct ADDRESS_ALREADY_EXISTS;
    ///Custom Error type `ADDRESS_NOT_EXISTS` with signature `ADDRESS_NOT_EXISTS()` and selector `0xad6ab975`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ADDRESS_NOT_EXISTS", abi = "ADDRESS_NOT_EXISTS()")]
    pub struct ADDRESS_NOT_EXISTS;
    ///Custom Error type `CALLER_MUST_BE_ENTRYPOINT` with signature `CALLER_MUST_BE_ENTRYPOINT()` and selector `0x550c9fa2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CALLER_MUST_BE_ENTRYPOINT", abi = "CALLER_MUST_BE_ENTRYPOINT()")]
    pub struct CALLER_MUST_BE_ENTRYPOINT;
    ///Custom Error type `CALLER_MUST_BE_MODULE` with signature `CALLER_MUST_BE_MODULE()` and selector `0xf4dcf772`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CALLER_MUST_BE_MODULE", abi = "CALLER_MUST_BE_MODULE()")]
    pub struct CALLER_MUST_BE_MODULE;
    ///Custom Error type `CALLER_MUST_BE_SELF_OR_MODULE` with signature `CALLER_MUST_BE_SELF_OR_MODULE()` and selector `0x3b246e72`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "CALLER_MUST_BE_SELF_OR_MODULE",
        abi = "CALLER_MUST_BE_SELF_OR_MODULE()"
    )]
    pub struct CALLER_MUST_BE_SELF_OR_MODULE;
    ///Custom Error type `HASH_ALREADY_APPROVED` with signature `HASH_ALREADY_APPROVED()` and selector `0xf934c05f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "HASH_ALREADY_APPROVED", abi = "HASH_ALREADY_APPROVED()")]
    pub struct HASH_ALREADY_APPROVED;
    ///Custom Error type `HASH_ALREADY_REJECTED` with signature `HASH_ALREADY_REJECTED()` and selector `0xf93f5f93`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "HASH_ALREADY_REJECTED", abi = "HASH_ALREADY_REJECTED()")]
    pub struct HASH_ALREADY_REJECTED;
    ///Custom Error type `INVALID_ADDRESS` with signature `INVALID_ADDRESS()` and selector `0x5963709b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "INVALID_ADDRESS", abi = "INVALID_ADDRESS()")]
    pub struct INVALID_ADDRESS;
    ///Custom Error type `INVALID_GUARD_HOOK_DATA` with signature `INVALID_GUARD_HOOK_DATA()` and selector `0x7969849a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "INVALID_GUARD_HOOK_DATA", abi = "INVALID_GUARD_HOOK_DATA()")]
    pub struct INVALID_GUARD_HOOK_DATA;
    ///Custom Error type `INVALID_LOGIC_ADDRESS` with signature `INVALID_LOGIC_ADDRESS()` and selector `0x40e01415`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "INVALID_LOGIC_ADDRESS", abi = "INVALID_LOGIC_ADDRESS()")]
    pub struct INVALID_LOGIC_ADDRESS;
    ///Custom Error type `INVALID_SELECTOR` with signature `INVALID_SELECTOR()` and selector `0xd858d3d3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "INVALID_SELECTOR", abi = "INVALID_SELECTOR()")]
    pub struct INVALID_SELECTOR;
    ///Custom Error type `INVALID_SIGNTYPE` with signature `INVALID_SIGNTYPE()` and selector `0x8ba6972a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "INVALID_SIGNTYPE", abi = "INVALID_SIGNTYPE()")]
    pub struct INVALID_SIGNTYPE;
    ///Custom Error type `MODULE_ADDRESS_EMPTY` with signature `MODULE_ADDRESS_EMPTY()` and selector `0x17335375`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "MODULE_ADDRESS_EMPTY", abi = "MODULE_ADDRESS_EMPTY()")]
    pub struct MODULE_ADDRESS_EMPTY;
    ///Custom Error type `MODULE_EXECUTE_FROM_MODULE_RECURSIVE` with signature `MODULE_EXECUTE_FROM_MODULE_RECURSIVE()` and selector `0xa1fb703b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "MODULE_EXECUTE_FROM_MODULE_RECURSIVE",
        abi = "MODULE_EXECUTE_FROM_MODULE_RECURSIVE()"
    )]
    pub struct MODULE_EXECUTE_FROM_MODULE_RECURSIVE;
    ///Custom Error type `MODULE_NOT_SUPPORT_INTERFACE` with signature `MODULE_NOT_SUPPORT_INTERFACE()` and selector `0x295f9680`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "MODULE_NOT_SUPPORT_INTERFACE",
        abi = "MODULE_NOT_SUPPORT_INTERFACE()"
    )]
    pub struct MODULE_NOT_SUPPORT_INTERFACE;
    ///Custom Error type `MODULE_SELECTORS_EMPTY` with signature `MODULE_SELECTORS_EMPTY()` and selector `0xca4ed850`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "MODULE_SELECTORS_EMPTY", abi = "MODULE_SELECTORS_EMPTY()")]
    pub struct MODULE_SELECTORS_EMPTY;
    ///Custom Error type `NOT_IMPLEMENTED` with signature `NOT_IMPLEMENTED()` and selector `0x43f6e4ab`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NOT_IMPLEMENTED", abi = "NOT_IMPLEMENTED()")]
    pub struct NOT_IMPLEMENTED;
    ///Custom Error type `NO_OWNER` with signature `NO_OWNER()` and selector `0xbec3801d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NO_OWNER", abi = "NO_OWNER()")]
    pub struct NO_OWNER;
    ///Custom Error type `PLUGIN_ADDRESS_EMPTY` with signature `PLUGIN_ADDRESS_EMPTY()` and selector `0x1a26809e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PLUGIN_ADDRESS_EMPTY", abi = "PLUGIN_ADDRESS_EMPTY()")]
    pub struct PLUGIN_ADDRESS_EMPTY;
    ///Custom Error type `PLUGIN_HOOK_TYPE_ERROR` with signature `PLUGIN_HOOK_TYPE_ERROR()` and selector `0x7e66834b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PLUGIN_HOOK_TYPE_ERROR", abi = "PLUGIN_HOOK_TYPE_ERROR()")]
    pub struct PLUGIN_HOOK_TYPE_ERROR;
    ///Custom Error type `PLUGIN_INIT_FAILED` with signature `PLUGIN_INIT_FAILED()` and selector `0xdadd4ffc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PLUGIN_INIT_FAILED", abi = "PLUGIN_INIT_FAILED()")]
    pub struct PLUGIN_INIT_FAILED;
    ///Custom Error type `PLUGIN_NOT_REGISTERED` with signature `PLUGIN_NOT_REGISTERED()` and selector `0x27631396`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PLUGIN_NOT_REGISTERED", abi = "PLUGIN_NOT_REGISTERED()")]
    pub struct PLUGIN_NOT_REGISTERED;
    ///Custom Error type `PLUGIN_NOT_SUPPORT_INTERFACE` with signature `PLUGIN_NOT_SUPPORT_INTERFACE()` and selector `0x304a2c6e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "PLUGIN_NOT_SUPPORT_INTERFACE",
        abi = "PLUGIN_NOT_SUPPORT_INTERFACE()"
    )]
    pub struct PLUGIN_NOT_SUPPORT_INTERFACE;
    ///Custom Error type `PLUGIN_POST_HOOK_FAILED` with signature `PLUGIN_POST_HOOK_FAILED()` and selector `0x0c2b0a60`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PLUGIN_POST_HOOK_FAILED", abi = "PLUGIN_POST_HOOK_FAILED()")]
    pub struct PLUGIN_POST_HOOK_FAILED;
    ///Custom Error type `PLUGIN_PRE_HOOK_FAILED` with signature `PLUGIN_PRE_HOOK_FAILED()` and selector `0x5fa8a13d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PLUGIN_PRE_HOOK_FAILED", abi = "PLUGIN_PRE_HOOK_FAILED()")]
    pub struct PLUGIN_PRE_HOOK_FAILED;
    ///Custom Error type `SAME_LOGIC_ADDRESS` with signature `SAME_LOGIC_ADDRESS()` and selector `0x8fa7dbf4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SAME_LOGIC_ADDRESS", abi = "SAME_LOGIC_ADDRESS()")]
    pub struct SAME_LOGIC_ADDRESS;
    ///Custom Error type `SELECTOR_ALREADY_EXISTS` with signature `SELECTOR_ALREADY_EXISTS()` and selector `0xc8c17d42`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SELECTOR_ALREADY_EXISTS", abi = "SELECTOR_ALREADY_EXISTS()")]
    pub struct SELECTOR_ALREADY_EXISTS;
    ///Custom Error type `UNSUPPORTED_SIGNTYPE` with signature `UNSUPPORTED_SIGNTYPE()` and selector `0x4571255b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UNSUPPORTED_SIGNTYPE", abi = "UNSUPPORTED_SIGNTYPE()")]
    pub struct UNSUPPORTED_SIGNTYPE;
    ///Custom Error type `UPGRADE_FAILED` with signature `UPGRADE_FAILED()` and selector `0x81f7e244`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UPGRADE_FAILED", abi = "UPGRADE_FAILED()")]
    pub struct UPGRADE_FAILED;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SoulWalletErrors {
        ADDRESS_ALREADY_EXISTS(ADDRESS_ALREADY_EXISTS),
        ADDRESS_NOT_EXISTS(ADDRESS_NOT_EXISTS),
        CALLER_MUST_BE_ENTRYPOINT(CALLER_MUST_BE_ENTRYPOINT),
        CALLER_MUST_BE_MODULE(CALLER_MUST_BE_MODULE),
        CALLER_MUST_BE_SELF_OR_MODULE(CALLER_MUST_BE_SELF_OR_MODULE),
        HASH_ALREADY_APPROVED(HASH_ALREADY_APPROVED),
        HASH_ALREADY_REJECTED(HASH_ALREADY_REJECTED),
        INVALID_ADDRESS(INVALID_ADDRESS),
        INVALID_GUARD_HOOK_DATA(INVALID_GUARD_HOOK_DATA),
        INVALID_LOGIC_ADDRESS(INVALID_LOGIC_ADDRESS),
        INVALID_SELECTOR(INVALID_SELECTOR),
        INVALID_SIGNTYPE(INVALID_SIGNTYPE),
        MODULE_ADDRESS_EMPTY(MODULE_ADDRESS_EMPTY),
        MODULE_EXECUTE_FROM_MODULE_RECURSIVE(MODULE_EXECUTE_FROM_MODULE_RECURSIVE),
        MODULE_NOT_SUPPORT_INTERFACE(MODULE_NOT_SUPPORT_INTERFACE),
        MODULE_SELECTORS_EMPTY(MODULE_SELECTORS_EMPTY),
        NOT_IMPLEMENTED(NOT_IMPLEMENTED),
        NO_OWNER(NO_OWNER),
        PLUGIN_ADDRESS_EMPTY(PLUGIN_ADDRESS_EMPTY),
        PLUGIN_HOOK_TYPE_ERROR(PLUGIN_HOOK_TYPE_ERROR),
        PLUGIN_INIT_FAILED(PLUGIN_INIT_FAILED),
        PLUGIN_NOT_REGISTERED(PLUGIN_NOT_REGISTERED),
        PLUGIN_NOT_SUPPORT_INTERFACE(PLUGIN_NOT_SUPPORT_INTERFACE),
        PLUGIN_POST_HOOK_FAILED(PLUGIN_POST_HOOK_FAILED),
        PLUGIN_PRE_HOOK_FAILED(PLUGIN_PRE_HOOK_FAILED),
        SAME_LOGIC_ADDRESS(SAME_LOGIC_ADDRESS),
        SELECTOR_ALREADY_EXISTS(SELECTOR_ALREADY_EXISTS),
        UNSUPPORTED_SIGNTYPE(UNSUPPORTED_SIGNTYPE),
        UPGRADE_FAILED(UPGRADE_FAILED),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SoulWalletErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <ADDRESS_ALREADY_EXISTS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ADDRESS_ALREADY_EXISTS(decoded));
            }
            if let Ok(decoded)
                = <ADDRESS_NOT_EXISTS as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ADDRESS_NOT_EXISTS(decoded));
            }
            if let Ok(decoded)
                = <CALLER_MUST_BE_ENTRYPOINT as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CALLER_MUST_BE_ENTRYPOINT(decoded));
            }
            if let Ok(decoded)
                = <CALLER_MUST_BE_MODULE as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CALLER_MUST_BE_MODULE(decoded));
            }
            if let Ok(decoded)
                = <CALLER_MUST_BE_SELF_OR_MODULE as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CALLER_MUST_BE_SELF_OR_MODULE(decoded));
            }
            if let Ok(decoded)
                = <HASH_ALREADY_APPROVED as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HASH_ALREADY_APPROVED(decoded));
            }
            if let Ok(decoded)
                = <HASH_ALREADY_REJECTED as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HASH_ALREADY_REJECTED(decoded));
            }
            if let Ok(decoded)
                = <INVALID_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::INVALID_ADDRESS(decoded));
            }
            if let Ok(decoded)
                = <INVALID_GUARD_HOOK_DATA as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::INVALID_GUARD_HOOK_DATA(decoded));
            }
            if let Ok(decoded)
                = <INVALID_LOGIC_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::INVALID_LOGIC_ADDRESS(decoded));
            }
            if let Ok(decoded)
                = <INVALID_SELECTOR as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::INVALID_SELECTOR(decoded));
            }
            if let Ok(decoded)
                = <INVALID_SIGNTYPE as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::INVALID_SIGNTYPE(decoded));
            }
            if let Ok(decoded)
                = <MODULE_ADDRESS_EMPTY as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MODULE_ADDRESS_EMPTY(decoded));
            }
            if let Ok(decoded)
                = <MODULE_EXECUTE_FROM_MODULE_RECURSIVE as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MODULE_EXECUTE_FROM_MODULE_RECURSIVE(decoded));
            }
            if let Ok(decoded)
                = <MODULE_NOT_SUPPORT_INTERFACE as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MODULE_NOT_SUPPORT_INTERFACE(decoded));
            }
            if let Ok(decoded)
                = <MODULE_SELECTORS_EMPTY as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MODULE_SELECTORS_EMPTY(decoded));
            }
            if let Ok(decoded)
                = <NOT_IMPLEMENTED as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NOT_IMPLEMENTED(decoded));
            }
            if let Ok(decoded)
                = <NO_OWNER as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NO_OWNER(decoded));
            }
            if let Ok(decoded)
                = <PLUGIN_ADDRESS_EMPTY as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PLUGIN_ADDRESS_EMPTY(decoded));
            }
            if let Ok(decoded)
                = <PLUGIN_HOOK_TYPE_ERROR as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PLUGIN_HOOK_TYPE_ERROR(decoded));
            }
            if let Ok(decoded)
                = <PLUGIN_INIT_FAILED as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PLUGIN_INIT_FAILED(decoded));
            }
            if let Ok(decoded)
                = <PLUGIN_NOT_REGISTERED as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PLUGIN_NOT_REGISTERED(decoded));
            }
            if let Ok(decoded)
                = <PLUGIN_NOT_SUPPORT_INTERFACE as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PLUGIN_NOT_SUPPORT_INTERFACE(decoded));
            }
            if let Ok(decoded)
                = <PLUGIN_POST_HOOK_FAILED as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PLUGIN_POST_HOOK_FAILED(decoded));
            }
            if let Ok(decoded)
                = <PLUGIN_PRE_HOOK_FAILED as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PLUGIN_PRE_HOOK_FAILED(decoded));
            }
            if let Ok(decoded)
                = <SAME_LOGIC_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SAME_LOGIC_ADDRESS(decoded));
            }
            if let Ok(decoded)
                = <SELECTOR_ALREADY_EXISTS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SELECTOR_ALREADY_EXISTS(decoded));
            }
            if let Ok(decoded)
                = <UNSUPPORTED_SIGNTYPE as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UNSUPPORTED_SIGNTYPE(decoded));
            }
            if let Ok(decoded)
                = <UPGRADE_FAILED as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UPGRADE_FAILED(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SoulWalletErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ADDRESS_ALREADY_EXISTS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ADDRESS_NOT_EXISTS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CALLER_MUST_BE_ENTRYPOINT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CALLER_MUST_BE_MODULE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CALLER_MUST_BE_SELF_OR_MODULE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HASH_ALREADY_APPROVED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HASH_ALREADY_REJECTED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_GUARD_HOOK_DATA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_LOGIC_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_SELECTOR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_SIGNTYPE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MODULE_ADDRESS_EMPTY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MODULE_EXECUTE_FROM_MODULE_RECURSIVE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MODULE_NOT_SUPPORT_INTERFACE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MODULE_SELECTORS_EMPTY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NOT_IMPLEMENTED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NO_OWNER(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PLUGIN_ADDRESS_EMPTY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PLUGIN_HOOK_TYPE_ERROR(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PLUGIN_INIT_FAILED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PLUGIN_NOT_REGISTERED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PLUGIN_NOT_SUPPORT_INTERFACE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PLUGIN_POST_HOOK_FAILED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PLUGIN_PRE_HOOK_FAILED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SAME_LOGIC_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SELECTOR_ALREADY_EXISTS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UNSUPPORTED_SIGNTYPE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UPGRADE_FAILED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SoulWalletErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ADDRESS_ALREADY_EXISTS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ADDRESS_NOT_EXISTS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CALLER_MUST_BE_ENTRYPOINT as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CALLER_MUST_BE_MODULE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CALLER_MUST_BE_SELF_OR_MODULE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HASH_ALREADY_APPROVED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HASH_ALREADY_REJECTED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <INVALID_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <INVALID_GUARD_HOOK_DATA as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <INVALID_LOGIC_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <INVALID_SELECTOR as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <INVALID_SIGNTYPE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MODULE_ADDRESS_EMPTY as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MODULE_EXECUTE_FROM_MODULE_RECURSIVE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MODULE_NOT_SUPPORT_INTERFACE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MODULE_SELECTORS_EMPTY as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NOT_IMPLEMENTED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NO_OWNER as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PLUGIN_ADDRESS_EMPTY as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PLUGIN_HOOK_TYPE_ERROR as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PLUGIN_INIT_FAILED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PLUGIN_NOT_REGISTERED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PLUGIN_NOT_SUPPORT_INTERFACE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PLUGIN_POST_HOOK_FAILED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PLUGIN_PRE_HOOK_FAILED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SAME_LOGIC_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SELECTOR_ALREADY_EXISTS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UNSUPPORTED_SIGNTYPE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UPGRADE_FAILED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SoulWalletErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ADDRESS_ALREADY_EXISTS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ADDRESS_NOT_EXISTS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CALLER_MUST_BE_ENTRYPOINT(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CALLER_MUST_BE_MODULE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CALLER_MUST_BE_SELF_OR_MODULE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HASH_ALREADY_APPROVED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HASH_ALREADY_REJECTED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::INVALID_ADDRESS(element) => ::core::fmt::Display::fmt(element, f),
                Self::INVALID_GUARD_HOOK_DATA(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::INVALID_LOGIC_ADDRESS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::INVALID_SELECTOR(element) => ::core::fmt::Display::fmt(element, f),
                Self::INVALID_SIGNTYPE(element) => ::core::fmt::Display::fmt(element, f),
                Self::MODULE_ADDRESS_EMPTY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MODULE_EXECUTE_FROM_MODULE_RECURSIVE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MODULE_NOT_SUPPORT_INTERFACE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MODULE_SELECTORS_EMPTY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NOT_IMPLEMENTED(element) => ::core::fmt::Display::fmt(element, f),
                Self::NO_OWNER(element) => ::core::fmt::Display::fmt(element, f),
                Self::PLUGIN_ADDRESS_EMPTY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PLUGIN_HOOK_TYPE_ERROR(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PLUGIN_INIT_FAILED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PLUGIN_NOT_REGISTERED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PLUGIN_NOT_SUPPORT_INTERFACE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PLUGIN_POST_HOOK_FAILED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PLUGIN_PRE_HOOK_FAILED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SAME_LOGIC_ADDRESS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SELECTOR_ALREADY_EXISTS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UNSUPPORTED_SIGNTYPE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UPGRADE_FAILED(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SoulWalletErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ADDRESS_ALREADY_EXISTS> for SoulWalletErrors {
        fn from(value: ADDRESS_ALREADY_EXISTS) -> Self {
            Self::ADDRESS_ALREADY_EXISTS(value)
        }
    }
    impl ::core::convert::From<ADDRESS_NOT_EXISTS> for SoulWalletErrors {
        fn from(value: ADDRESS_NOT_EXISTS) -> Self {
            Self::ADDRESS_NOT_EXISTS(value)
        }
    }
    impl ::core::convert::From<CALLER_MUST_BE_ENTRYPOINT> for SoulWalletErrors {
        fn from(value: CALLER_MUST_BE_ENTRYPOINT) -> Self {
            Self::CALLER_MUST_BE_ENTRYPOINT(value)
        }
    }
    impl ::core::convert::From<CALLER_MUST_BE_MODULE> for SoulWalletErrors {
        fn from(value: CALLER_MUST_BE_MODULE) -> Self {
            Self::CALLER_MUST_BE_MODULE(value)
        }
    }
    impl ::core::convert::From<CALLER_MUST_BE_SELF_OR_MODULE> for SoulWalletErrors {
        fn from(value: CALLER_MUST_BE_SELF_OR_MODULE) -> Self {
            Self::CALLER_MUST_BE_SELF_OR_MODULE(value)
        }
    }
    impl ::core::convert::From<HASH_ALREADY_APPROVED> for SoulWalletErrors {
        fn from(value: HASH_ALREADY_APPROVED) -> Self {
            Self::HASH_ALREADY_APPROVED(value)
        }
    }
    impl ::core::convert::From<HASH_ALREADY_REJECTED> for SoulWalletErrors {
        fn from(value: HASH_ALREADY_REJECTED) -> Self {
            Self::HASH_ALREADY_REJECTED(value)
        }
    }
    impl ::core::convert::From<INVALID_ADDRESS> for SoulWalletErrors {
        fn from(value: INVALID_ADDRESS) -> Self {
            Self::INVALID_ADDRESS(value)
        }
    }
    impl ::core::convert::From<INVALID_GUARD_HOOK_DATA> for SoulWalletErrors {
        fn from(value: INVALID_GUARD_HOOK_DATA) -> Self {
            Self::INVALID_GUARD_HOOK_DATA(value)
        }
    }
    impl ::core::convert::From<INVALID_LOGIC_ADDRESS> for SoulWalletErrors {
        fn from(value: INVALID_LOGIC_ADDRESS) -> Self {
            Self::INVALID_LOGIC_ADDRESS(value)
        }
    }
    impl ::core::convert::From<INVALID_SELECTOR> for SoulWalletErrors {
        fn from(value: INVALID_SELECTOR) -> Self {
            Self::INVALID_SELECTOR(value)
        }
    }
    impl ::core::convert::From<INVALID_SIGNTYPE> for SoulWalletErrors {
        fn from(value: INVALID_SIGNTYPE) -> Self {
            Self::INVALID_SIGNTYPE(value)
        }
    }
    impl ::core::convert::From<MODULE_ADDRESS_EMPTY> for SoulWalletErrors {
        fn from(value: MODULE_ADDRESS_EMPTY) -> Self {
            Self::MODULE_ADDRESS_EMPTY(value)
        }
    }
    impl ::core::convert::From<MODULE_EXECUTE_FROM_MODULE_RECURSIVE>
    for SoulWalletErrors {
        fn from(value: MODULE_EXECUTE_FROM_MODULE_RECURSIVE) -> Self {
            Self::MODULE_EXECUTE_FROM_MODULE_RECURSIVE(value)
        }
    }
    impl ::core::convert::From<MODULE_NOT_SUPPORT_INTERFACE> for SoulWalletErrors {
        fn from(value: MODULE_NOT_SUPPORT_INTERFACE) -> Self {
            Self::MODULE_NOT_SUPPORT_INTERFACE(value)
        }
    }
    impl ::core::convert::From<MODULE_SELECTORS_EMPTY> for SoulWalletErrors {
        fn from(value: MODULE_SELECTORS_EMPTY) -> Self {
            Self::MODULE_SELECTORS_EMPTY(value)
        }
    }
    impl ::core::convert::From<NOT_IMPLEMENTED> for SoulWalletErrors {
        fn from(value: NOT_IMPLEMENTED) -> Self {
            Self::NOT_IMPLEMENTED(value)
        }
    }
    impl ::core::convert::From<NO_OWNER> for SoulWalletErrors {
        fn from(value: NO_OWNER) -> Self {
            Self::NO_OWNER(value)
        }
    }
    impl ::core::convert::From<PLUGIN_ADDRESS_EMPTY> for SoulWalletErrors {
        fn from(value: PLUGIN_ADDRESS_EMPTY) -> Self {
            Self::PLUGIN_ADDRESS_EMPTY(value)
        }
    }
    impl ::core::convert::From<PLUGIN_HOOK_TYPE_ERROR> for SoulWalletErrors {
        fn from(value: PLUGIN_HOOK_TYPE_ERROR) -> Self {
            Self::PLUGIN_HOOK_TYPE_ERROR(value)
        }
    }
    impl ::core::convert::From<PLUGIN_INIT_FAILED> for SoulWalletErrors {
        fn from(value: PLUGIN_INIT_FAILED) -> Self {
            Self::PLUGIN_INIT_FAILED(value)
        }
    }
    impl ::core::convert::From<PLUGIN_NOT_REGISTERED> for SoulWalletErrors {
        fn from(value: PLUGIN_NOT_REGISTERED) -> Self {
            Self::PLUGIN_NOT_REGISTERED(value)
        }
    }
    impl ::core::convert::From<PLUGIN_NOT_SUPPORT_INTERFACE> for SoulWalletErrors {
        fn from(value: PLUGIN_NOT_SUPPORT_INTERFACE) -> Self {
            Self::PLUGIN_NOT_SUPPORT_INTERFACE(value)
        }
    }
    impl ::core::convert::From<PLUGIN_POST_HOOK_FAILED> for SoulWalletErrors {
        fn from(value: PLUGIN_POST_HOOK_FAILED) -> Self {
            Self::PLUGIN_POST_HOOK_FAILED(value)
        }
    }
    impl ::core::convert::From<PLUGIN_PRE_HOOK_FAILED> for SoulWalletErrors {
        fn from(value: PLUGIN_PRE_HOOK_FAILED) -> Self {
            Self::PLUGIN_PRE_HOOK_FAILED(value)
        }
    }
    impl ::core::convert::From<SAME_LOGIC_ADDRESS> for SoulWalletErrors {
        fn from(value: SAME_LOGIC_ADDRESS) -> Self {
            Self::SAME_LOGIC_ADDRESS(value)
        }
    }
    impl ::core::convert::From<SELECTOR_ALREADY_EXISTS> for SoulWalletErrors {
        fn from(value: SELECTOR_ALREADY_EXISTS) -> Self {
            Self::SELECTOR_ALREADY_EXISTS(value)
        }
    }
    impl ::core::convert::From<UNSUPPORTED_SIGNTYPE> for SoulWalletErrors {
        fn from(value: UNSUPPORTED_SIGNTYPE) -> Self {
            Self::UNSUPPORTED_SIGNTYPE(value)
        }
    }
    impl ::core::convert::From<UPGRADE_FAILED> for SoulWalletErrors {
        fn from(value: UPGRADE_FAILED) -> Self {
            Self::UPGRADE_FAILED(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ApproveHash", abi = "ApproveHash(bytes32)")]
    pub struct ApproveHashFilter {
        #[ethevent(indexed)]
        pub hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "FallbackChanged", abi = "FallbackChanged(address)")]
    pub struct FallbackChangedFilter {
        #[ethevent(indexed)]
        pub fallback_contract: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ModuleAdded", abi = "ModuleAdded(address)")]
    pub struct ModuleAddedFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ModuleRemoved", abi = "ModuleRemoved(address)")]
    pub struct ModuleRemovedFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ModuleRemovedWithError", abi = "ModuleRemovedWithError(address)")]
    pub struct ModuleRemovedWithErrorFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OwnerAdded", abi = "OwnerAdded(address)")]
    pub struct OwnerAddedFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OwnerCleared", abi = "OwnerCleared()")]
    pub struct OwnerClearedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OwnerRemoved", abi = "OwnerRemoved(address)")]
    pub struct OwnerRemovedFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PluginAdded", abi = "PluginAdded(address)")]
    pub struct PluginAddedFilter {
        #[ethevent(indexed)]
        pub plugin: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PluginRemoved", abi = "PluginRemoved(address)")]
    pub struct PluginRemovedFilter {
        #[ethevent(indexed)]
        pub plugin: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PluginRemovedWithError", abi = "PluginRemovedWithError(address)")]
    pub struct PluginRemovedWithErrorFilter {
        #[ethevent(indexed)]
        pub plugin: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RejectHash", abi = "RejectHash(bytes32)")]
    pub struct RejectHashFilter {
        #[ethevent(indexed)]
        pub hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Upgraded", abi = "Upgraded(address,address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub old_implementation: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SoulWalletEvents {
        ApproveHashFilter(ApproveHashFilter),
        FallbackChangedFilter(FallbackChangedFilter),
        InitializedFilter(InitializedFilter),
        ModuleAddedFilter(ModuleAddedFilter),
        ModuleRemovedFilter(ModuleRemovedFilter),
        ModuleRemovedWithErrorFilter(ModuleRemovedWithErrorFilter),
        OwnerAddedFilter(OwnerAddedFilter),
        OwnerClearedFilter(OwnerClearedFilter),
        OwnerRemovedFilter(OwnerRemovedFilter),
        PluginAddedFilter(PluginAddedFilter),
        PluginRemovedFilter(PluginRemovedFilter),
        PluginRemovedWithErrorFilter(PluginRemovedWithErrorFilter),
        RejectHashFilter(RejectHashFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for SoulWalletEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApproveHashFilter::decode_log(log) {
                return Ok(SoulWalletEvents::ApproveHashFilter(decoded));
            }
            if let Ok(decoded) = FallbackChangedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::FallbackChangedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = ModuleAddedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::ModuleAddedFilter(decoded));
            }
            if let Ok(decoded) = ModuleRemovedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::ModuleRemovedFilter(decoded));
            }
            if let Ok(decoded) = ModuleRemovedWithErrorFilter::decode_log(log) {
                return Ok(SoulWalletEvents::ModuleRemovedWithErrorFilter(decoded));
            }
            if let Ok(decoded) = OwnerAddedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::OwnerAddedFilter(decoded));
            }
            if let Ok(decoded) = OwnerClearedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::OwnerClearedFilter(decoded));
            }
            if let Ok(decoded) = OwnerRemovedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::OwnerRemovedFilter(decoded));
            }
            if let Ok(decoded) = PluginAddedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::PluginAddedFilter(decoded));
            }
            if let Ok(decoded) = PluginRemovedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::PluginRemovedFilter(decoded));
            }
            if let Ok(decoded) = PluginRemovedWithErrorFilter::decode_log(log) {
                return Ok(SoulWalletEvents::PluginRemovedWithErrorFilter(decoded));
            }
            if let Ok(decoded) = RejectHashFilter::decode_log(log) {
                return Ok(SoulWalletEvents::RejectHashFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(SoulWalletEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SoulWalletEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveHashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FallbackChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModuleAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModuleRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ModuleRemovedWithErrorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnerAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerClearedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnerRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PluginAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PluginRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PluginRemovedWithErrorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RejectHashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveHashFilter> for SoulWalletEvents {
        fn from(value: ApproveHashFilter) -> Self {
            Self::ApproveHashFilter(value)
        }
    }
    impl ::core::convert::From<FallbackChangedFilter> for SoulWalletEvents {
        fn from(value: FallbackChangedFilter) -> Self {
            Self::FallbackChangedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for SoulWalletEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<ModuleAddedFilter> for SoulWalletEvents {
        fn from(value: ModuleAddedFilter) -> Self {
            Self::ModuleAddedFilter(value)
        }
    }
    impl ::core::convert::From<ModuleRemovedFilter> for SoulWalletEvents {
        fn from(value: ModuleRemovedFilter) -> Self {
            Self::ModuleRemovedFilter(value)
        }
    }
    impl ::core::convert::From<ModuleRemovedWithErrorFilter> for SoulWalletEvents {
        fn from(value: ModuleRemovedWithErrorFilter) -> Self {
            Self::ModuleRemovedWithErrorFilter(value)
        }
    }
    impl ::core::convert::From<OwnerAddedFilter> for SoulWalletEvents {
        fn from(value: OwnerAddedFilter) -> Self {
            Self::OwnerAddedFilter(value)
        }
    }
    impl ::core::convert::From<OwnerClearedFilter> for SoulWalletEvents {
        fn from(value: OwnerClearedFilter) -> Self {
            Self::OwnerClearedFilter(value)
        }
    }
    impl ::core::convert::From<OwnerRemovedFilter> for SoulWalletEvents {
        fn from(value: OwnerRemovedFilter) -> Self {
            Self::OwnerRemovedFilter(value)
        }
    }
    impl ::core::convert::From<PluginAddedFilter> for SoulWalletEvents {
        fn from(value: PluginAddedFilter) -> Self {
            Self::PluginAddedFilter(value)
        }
    }
    impl ::core::convert::From<PluginRemovedFilter> for SoulWalletEvents {
        fn from(value: PluginRemovedFilter) -> Self {
            Self::PluginRemovedFilter(value)
        }
    }
    impl ::core::convert::From<PluginRemovedWithErrorFilter> for SoulWalletEvents {
        fn from(value: PluginRemovedWithErrorFilter) -> Self {
            Self::PluginRemovedWithErrorFilter(value)
        }
    }
    impl ::core::convert::From<RejectHashFilter> for SoulWalletEvents {
        fn from(value: RejectHashFilter) -> Self {
            Self::RejectHashFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for SoulWalletEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addModule` function with signature `addModule(bytes)` and selector `0xd3bdf4b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addModule", abi = "addModule(bytes)")]
    pub struct AddModuleCall {
        pub module_and_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `addOwner` function with signature `addOwner(address)` and selector `0x7065cb48`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addOwner", abi = "addOwner(address)")]
    pub struct AddOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addOwners` function with signature `addOwners(address[])` and selector `0x6c46a2c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addOwners", abi = "addOwners(address[])")]
    pub struct AddOwnersCall {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `addPlugin` function with signature `addPlugin(bytes)` and selector `0x47361356`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addPlugin", abi = "addPlugin(bytes)")]
    pub struct AddPluginCall {
        pub plugin_and_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `approveHash` function with signature `approveHash(bytes32)` and selector `0xd4d9bdcd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approveHash", abi = "approveHash(bytes32)")]
    pub struct ApproveHashCall {
        pub hash: [u8; 32],
    }
    ///Container type for all input parameters for the `entryPoint` function with signature `entryPoint()` and selector `0xb0d691fe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "entryPoint", abi = "entryPoint()")]
    pub struct EntryPointCall;
    ///Container type for all input parameters for the `execute` function with signature `execute(address,uint256,bytes)` and selector `0xb61d27f6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "execute", abi = "execute(address,uint256,bytes)")]
    pub struct ExecuteCall {
        pub dest: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub func: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeBatch` function with signature `executeBatch(address[],bytes[])` and selector `0x18dfb3c7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeBatch", abi = "executeBatch(address[],bytes[])")]
    pub struct ExecuteBatchCall {
        pub dest: ::std::vec::Vec<::ethers::core::types::Address>,
        pub func: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `executeBatch` function with signature `executeBatch(address[],uint256[],bytes[])` and selector `0x47e1da2a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeBatch", abi = "executeBatch(address[],uint256[],bytes[])")]
    pub struct ExecuteBatchWithDestAndValueCall {
        pub dest: ::std::vec::Vec<::ethers::core::types::Address>,
        pub value: ::std::vec::Vec<::ethers::core::types::U256>,
        pub func: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `executeFromModule` function with signature `executeFromModule(address,uint256,bytes)` and selector `0xae9411ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeFromModule",
        abi = "executeFromModule(address,uint256,bytes)"
    )]
    pub struct ExecuteFromModuleCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce()` and selector `0xd087d288`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getNonce", abi = "getNonce()")]
    pub struct GetNonceCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,bytes[],bytes[])` and selector `0xa6c87ecb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address,bytes[],bytes[])")]
    pub struct InitializeCall {
        pub an_owner: ::ethers::core::types::Address,
        pub defalut_callback_handler: ::ethers::core::types::Address,
        pub modules: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub plugins: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `isAuthorizedModule` function with signature `isAuthorizedModule(address)` and selector `0x399dd463`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isAuthorizedModule", abi = "isAuthorizedModule(address)")]
    pub struct IsAuthorizedModuleCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isAuthorizedPlugin` function with signature `isAuthorizedPlugin(address)` and selector `0x48d7b1d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isAuthorizedPlugin", abi = "isAuthorizedPlugin(address)")]
    pub struct IsAuthorizedPluginCall {
        pub plugin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOwner` function with signature `isOwner(address)` and selector `0x2f54bf6e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isOwner", abi = "isOwner(address)")]
    pub struct IsOwnerCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes32,bytes)")]
    pub struct IsValidSignatureCall {
        pub hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `listModule` function with signature `listModule()` and selector `0x6bbd74d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "listModule", abi = "listModule()")]
    pub struct ListModuleCall;
    ///Container type for all input parameters for the `listOwner` function with signature `listOwner()` and selector `0xa52afc20`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "listOwner", abi = "listOwner()")]
    pub struct ListOwnerCall;
    ///Container type for all input parameters for the `listPlugin` function with signature `listPlugin(uint8)` and selector `0x7799da64`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "listPlugin", abi = "listPlugin(uint8)")]
    pub struct ListPluginCall {
        pub hook_type: u8,
    }
    ///Container type for all input parameters for the `pluginDataLoad` function with signature `pluginDataLoad(address,bytes32)` and selector `0x3900c3ba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pluginDataLoad", abi = "pluginDataLoad(address,bytes32)")]
    pub struct PluginDataLoadCall {
        pub plugin: ::ethers::core::types::Address,
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `pluginDataStore` function with signature `pluginDataStore(bytes32,bytes)` and selector `0x14ee68ad`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pluginDataStore", abi = "pluginDataStore(bytes32,bytes)")]
    pub struct PluginDataStoreCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `rejectHash` function with signature `rejectHash(bytes32)` and selector `0xe3b57475`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rejectHash", abi = "rejectHash(bytes32)")]
    pub struct RejectHashCall {
        pub hash: [u8; 32],
    }
    ///Container type for all input parameters for the `removeModule` function with signature `removeModule(address)` and selector `0xa0632461`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeModule", abi = "removeModule(address)")]
    pub struct RemoveModuleCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeOwner` function with signature `removeOwner(address)` and selector `0x173825d9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeOwner", abi = "removeOwner(address)")]
    pub struct RemoveOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removePlugin` function with signature `removePlugin(address)` and selector `0xa4d95b64`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removePlugin", abi = "removePlugin(address)")]
    pub struct RemovePluginCall {
        pub plugin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `resetOwner` function with signature `resetOwner(address)` and selector `0x73cc802a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "resetOwner", abi = "resetOwner(address)")]
    pub struct ResetOwnerCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `resetOwners` function with signature `resetOwners(address[])` and selector `0xf093ac9c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "resetOwners", abi = "resetOwners(address[])")]
    pub struct ResetOwnersCall {
        pub new_owners: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `setFallbackHandler` function with signature `setFallbackHandler(address)` and selector `0xf08a0323`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFallbackHandler", abi = "setFallbackHandler(address)")]
    pub struct SetFallbackHandlerCall {
        pub fallback_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeFrom` function with signature `upgradeFrom(address)` and selector `0xf0e06327`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "upgradeFrom", abi = "upgradeFrom(address)")]
    pub struct UpgradeFromCall {
        pub old_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `0x3659cfe6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `validateUserOp` function with signature `validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0x3a871cdd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "validateUserOp",
        abi = "validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)"
    )]
    pub struct ValidateUserOpCall {
        pub user_op: UserOperation,
        pub user_op_hash: [u8; 32],
        pub missing_account_funds: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SoulWalletCalls {
        AddModule(AddModuleCall),
        AddOwner(AddOwnerCall),
        AddOwners(AddOwnersCall),
        AddPlugin(AddPluginCall),
        ApproveHash(ApproveHashCall),
        EntryPoint(EntryPointCall),
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        ExecuteBatchWithDestAndValue(ExecuteBatchWithDestAndValueCall),
        ExecuteFromModule(ExecuteFromModuleCall),
        GetNonce(GetNonceCall),
        Initialize(InitializeCall),
        IsAuthorizedModule(IsAuthorizedModuleCall),
        IsAuthorizedPlugin(IsAuthorizedPluginCall),
        IsOwner(IsOwnerCall),
        IsValidSignature(IsValidSignatureCall),
        ListModule(ListModuleCall),
        ListOwner(ListOwnerCall),
        ListPlugin(ListPluginCall),
        PluginDataLoad(PluginDataLoadCall),
        PluginDataStore(PluginDataStoreCall),
        RejectHash(RejectHashCall),
        RemoveModule(RemoveModuleCall),
        RemoveOwner(RemoveOwnerCall),
        RemovePlugin(RemovePluginCall),
        ResetOwner(ResetOwnerCall),
        ResetOwners(ResetOwnersCall),
        SetFallbackHandler(SetFallbackHandlerCall),
        UpgradeFrom(UpgradeFromCall),
        UpgradeTo(UpgradeToCall),
        ValidateUserOp(ValidateUserOpCall),
    }
    impl ::ethers::core::abi::AbiDecode for SoulWalletCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddModule(decoded));
            }
            if let Ok(decoded)
                = <AddOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddOwner(decoded));
            }
            if let Ok(decoded)
                = <AddOwnersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddOwners(decoded));
            }
            if let Ok(decoded)
                = <AddPluginCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddPlugin(decoded));
            }
            if let Ok(decoded)
                = <ApproveHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApproveHash(decoded));
            }
            if let Ok(decoded)
                = <EntryPointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EntryPoint(decoded));
            }
            if let Ok(decoded)
                = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded)
                = <ExecuteBatchCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteBatch(decoded));
            }
            if let Ok(decoded)
                = <ExecuteBatchWithDestAndValueCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteBatchWithDestAndValue(decoded));
            }
            if let Ok(decoded)
                = <ExecuteFromModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteFromModule(decoded));
            }
            if let Ok(decoded)
                = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <IsAuthorizedModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAuthorizedModule(decoded));
            }
            if let Ok(decoded)
                = <IsAuthorizedPluginCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAuthorizedPlugin(decoded));
            }
            if let Ok(decoded)
                = <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOwner(decoded));
            }
            if let Ok(decoded)
                = <IsValidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsValidSignature(decoded));
            }
            if let Ok(decoded)
                = <ListModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ListModule(decoded));
            }
            if let Ok(decoded)
                = <ListOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ListOwner(decoded));
            }
            if let Ok(decoded)
                = <ListPluginCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ListPlugin(decoded));
            }
            if let Ok(decoded)
                = <PluginDataLoadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PluginDataLoad(decoded));
            }
            if let Ok(decoded)
                = <PluginDataStoreCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PluginDataStore(decoded));
            }
            if let Ok(decoded)
                = <RejectHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RejectHash(decoded));
            }
            if let Ok(decoded)
                = <RemoveModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveModule(decoded));
            }
            if let Ok(decoded)
                = <RemoveOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveOwner(decoded));
            }
            if let Ok(decoded)
                = <RemovePluginCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemovePlugin(decoded));
            }
            if let Ok(decoded)
                = <ResetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ResetOwner(decoded));
            }
            if let Ok(decoded)
                = <ResetOwnersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ResetOwners(decoded));
            }
            if let Ok(decoded)
                = <SetFallbackHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFallbackHandler(decoded));
            }
            if let Ok(decoded)
                = <UpgradeFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpgradeFrom(decoded));
            }
            if let Ok(decoded)
                = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded)
                = <ValidateUserOpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateUserOp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SoulWalletCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddOwners(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPlugin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EntryPoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteBatchWithDestAndValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteFromModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAuthorizedModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAuthorizedPlugin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsValidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ListModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ListOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ListPlugin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PluginDataLoad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PluginDataStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RejectHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePlugin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResetOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResetOwners(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFallbackHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateUserOp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SoulWalletCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOwners(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddPlugin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntryPoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatchWithDestAndValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteFromModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAuthorizedModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsAuthorizedPlugin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ListModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::ListOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ListPlugin(element) => ::core::fmt::Display::fmt(element, f),
                Self::PluginDataLoad(element) => ::core::fmt::Display::fmt(element, f),
                Self::PluginDataStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::RejectHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePlugin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetOwners(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFallbackHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradeFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateUserOp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddModuleCall> for SoulWalletCalls {
        fn from(value: AddModuleCall) -> Self {
            Self::AddModule(value)
        }
    }
    impl ::core::convert::From<AddOwnerCall> for SoulWalletCalls {
        fn from(value: AddOwnerCall) -> Self {
            Self::AddOwner(value)
        }
    }
    impl ::core::convert::From<AddOwnersCall> for SoulWalletCalls {
        fn from(value: AddOwnersCall) -> Self {
            Self::AddOwners(value)
        }
    }
    impl ::core::convert::From<AddPluginCall> for SoulWalletCalls {
        fn from(value: AddPluginCall) -> Self {
            Self::AddPlugin(value)
        }
    }
    impl ::core::convert::From<ApproveHashCall> for SoulWalletCalls {
        fn from(value: ApproveHashCall) -> Self {
            Self::ApproveHash(value)
        }
    }
    impl ::core::convert::From<EntryPointCall> for SoulWalletCalls {
        fn from(value: EntryPointCall) -> Self {
            Self::EntryPoint(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for SoulWalletCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for SoulWalletCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchWithDestAndValueCall> for SoulWalletCalls {
        fn from(value: ExecuteBatchWithDestAndValueCall) -> Self {
            Self::ExecuteBatchWithDestAndValue(value)
        }
    }
    impl ::core::convert::From<ExecuteFromModuleCall> for SoulWalletCalls {
        fn from(value: ExecuteFromModuleCall) -> Self {
            Self::ExecuteFromModule(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for SoulWalletCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SoulWalletCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsAuthorizedModuleCall> for SoulWalletCalls {
        fn from(value: IsAuthorizedModuleCall) -> Self {
            Self::IsAuthorizedModule(value)
        }
    }
    impl ::core::convert::From<IsAuthorizedPluginCall> for SoulWalletCalls {
        fn from(value: IsAuthorizedPluginCall) -> Self {
            Self::IsAuthorizedPlugin(value)
        }
    }
    impl ::core::convert::From<IsOwnerCall> for SoulWalletCalls {
        fn from(value: IsOwnerCall) -> Self {
            Self::IsOwner(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureCall> for SoulWalletCalls {
        fn from(value: IsValidSignatureCall) -> Self {
            Self::IsValidSignature(value)
        }
    }
    impl ::core::convert::From<ListModuleCall> for SoulWalletCalls {
        fn from(value: ListModuleCall) -> Self {
            Self::ListModule(value)
        }
    }
    impl ::core::convert::From<ListOwnerCall> for SoulWalletCalls {
        fn from(value: ListOwnerCall) -> Self {
            Self::ListOwner(value)
        }
    }
    impl ::core::convert::From<ListPluginCall> for SoulWalletCalls {
        fn from(value: ListPluginCall) -> Self {
            Self::ListPlugin(value)
        }
    }
    impl ::core::convert::From<PluginDataLoadCall> for SoulWalletCalls {
        fn from(value: PluginDataLoadCall) -> Self {
            Self::PluginDataLoad(value)
        }
    }
    impl ::core::convert::From<PluginDataStoreCall> for SoulWalletCalls {
        fn from(value: PluginDataStoreCall) -> Self {
            Self::PluginDataStore(value)
        }
    }
    impl ::core::convert::From<RejectHashCall> for SoulWalletCalls {
        fn from(value: RejectHashCall) -> Self {
            Self::RejectHash(value)
        }
    }
    impl ::core::convert::From<RemoveModuleCall> for SoulWalletCalls {
        fn from(value: RemoveModuleCall) -> Self {
            Self::RemoveModule(value)
        }
    }
    impl ::core::convert::From<RemoveOwnerCall> for SoulWalletCalls {
        fn from(value: RemoveOwnerCall) -> Self {
            Self::RemoveOwner(value)
        }
    }
    impl ::core::convert::From<RemovePluginCall> for SoulWalletCalls {
        fn from(value: RemovePluginCall) -> Self {
            Self::RemovePlugin(value)
        }
    }
    impl ::core::convert::From<ResetOwnerCall> for SoulWalletCalls {
        fn from(value: ResetOwnerCall) -> Self {
            Self::ResetOwner(value)
        }
    }
    impl ::core::convert::From<ResetOwnersCall> for SoulWalletCalls {
        fn from(value: ResetOwnersCall) -> Self {
            Self::ResetOwners(value)
        }
    }
    impl ::core::convert::From<SetFallbackHandlerCall> for SoulWalletCalls {
        fn from(value: SetFallbackHandlerCall) -> Self {
            Self::SetFallbackHandler(value)
        }
    }
    impl ::core::convert::From<UpgradeFromCall> for SoulWalletCalls {
        fn from(value: UpgradeFromCall) -> Self {
            Self::UpgradeFrom(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for SoulWalletCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<ValidateUserOpCall> for SoulWalletCalls {
        fn from(value: ValidateUserOpCall) -> Self {
            Self::ValidateUserOp(value)
        }
    }
    ///Container type for all return fields from the `entryPoint` function with signature `entryPoint()` and selector `0xb0d691fe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EntryPointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getNonce` function with signature `getNonce()` and selector `0xd087d288`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isAuthorizedModule` function with signature `isAuthorizedModule(address)` and selector `0x399dd463`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsAuthorizedModuleReturn(pub bool);
    ///Container type for all return fields from the `isAuthorizedPlugin` function with signature `isAuthorizedPlugin(address)` and selector `0x48d7b1d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsAuthorizedPluginReturn(pub bool);
    ///Container type for all return fields from the `isOwner` function with signature `isOwner(address)` and selector `0x2f54bf6e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsOwnerReturn(pub bool);
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsValidSignatureReturn {
        pub magic_value: [u8; 4],
    }
    ///Container type for all return fields from the `listModule` function with signature `listModule()` and selector `0x6bbd74d3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ListModuleReturn {
        pub modules: ::std::vec::Vec<::ethers::core::types::Address>,
        pub selectors: ::std::vec::Vec<::std::vec::Vec<[u8; 4]>>,
    }
    ///Container type for all return fields from the `listOwner` function with signature `listOwner()` and selector `0xa52afc20`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ListOwnerReturn {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `listPlugin` function with signature `listPlugin(uint8)` and selector `0x7799da64`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ListPluginReturn {
        pub plugins: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `pluginDataLoad` function with signature `pluginDataLoad(address,bytes32)` and selector `0x3900c3ba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PluginDataLoadReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `validateUserOp` function with signature `validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0x3a871cdd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ValidateUserOpReturn {
        pub validation_data: ::ethers::core::types::U256,
    }
    ///`UserOperation(address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UserOperation {
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
}
