pub use key_store::*;
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
pub mod key_store {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"name\":\"GUARDIAN_SIGNATURE_INVALID\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"HASH_ALREADY_APPROVED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"HASH_ALREADY_REJECTED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_DATA\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_KEY\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_SIGNATURE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"INVALID_TIME_RANGE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NOT_INITIALIZED\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UNAUTHORIZED\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"guardian\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"}],\"name\":\"ApproveHash\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"guardianHash\",\"type\":\"bytes32\"}],\"name\":\"CancelSetGuardian\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"guardianSafePeriod\",\"type\":\"uint64\"}],\"name\":\"CancelSetGuardianSafePeriod\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"guardianHash\",\"type\":\"bytes32\"}],\"name\":\"GuardianChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"guardianSafePeriod\",\"type\":\"uint64\"}],\"name\":\"GuardianSafePeriodChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"}],\"name\":\"Initialized\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"key\",\"type\":\"bytes32\"}],\"name\":\"KeyChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"guardian\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"}],\"name\":\"RejectHash\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"guardianHash\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"effectAt\",\"type\":\"uint64\"}],\"name\":\"SetGuardian\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"guardianSafePeriod\",\"type\":\"uint64\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"effectAt\",\"type\":\"uint64\"}],\"name\":\"SetGuardianSafePeriod\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"}],\"name\":\"approveHash\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"keySignature\",\"type\":\"bytes\"}],\"name\":\"cancelSetGuardian\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"keySignature\",\"type\":\"bytes\"}],\"name\":\"cancelSetGuardianSafePeriod\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"rawGuardian\",\"type\":\"bytes\"}],\"name\":\"getGuardianHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"guardianHash\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"}],\"name\":\"getKey\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"key\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"}],\"name\":\"getKeyStoreInfo\",\"outputs\":[{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"key\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"guardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"pendingGuardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint64\",\"name\":\"guardianActivateAt\",\"type\":\"uint64\"},{\"internalType\":\"uint64\",\"name\":\"guardianSafePeriod\",\"type\":\"uint64\"},{\"internalType\":\"uint64\",\"name\":\"pendingGuardianSafePeriod\",\"type\":\"uint64\"},{\"internalType\":\"uint64\",\"name\":\"guardianSafePeriodActivateAt\",\"type\":\"uint64\"}],\"internalType\":\"struct IKeyStore.keyStoreInfo\",\"name\":\"_keyStoreInfo\",\"type\":\"tuple\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"initialKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"initialGuardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint64\",\"name\":\"guardianSafePeriod\",\"type\":\"uint64\"}],\"name\":\"getSlot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"l1Slot\",\"type\":\"bytes32\"}],\"name\":\"keystoreBySlot\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"signingKey\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"}],\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"}],\"name\":\"rejectHash\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"newGuardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"keySignature\",\"type\":\"bytes\"}],\"name\":\"setGuardian\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"initialKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"initialGuardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint64\",\"name\":\"initialGuardianSafePeriod\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"newGuardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"keySignature\",\"type\":\"bytes\"}],\"name\":\"setGuardian\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"initialKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"initialGuardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint64\",\"name\":\"initialGuardianSafePeriod\",\"type\":\"uint64\"},{\"internalType\":\"uint64\",\"name\":\"newGuardianSafePeriod\",\"type\":\"uint64\"},{\"internalType\":\"bytes\",\"name\":\"keySignature\",\"type\":\"bytes\"}],\"name\":\"setGuardianSafePeriod\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"internalType\":\"uint64\",\"name\":\"newGuardianSafePeriod\",\"type\":\"uint64\"},{\"internalType\":\"bytes\",\"name\":\"keySignature\",\"type\":\"bytes\"}],\"name\":\"setGuardianSafePeriod\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"newKey\",\"type\":\"bytes32\"}],\"name\":\"setKey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"initialKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"initialGuardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint64\",\"name\":\"initialGuardianSafePeriod\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"newKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"rawGuardian\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"guardianSignature\",\"type\":\"bytes\"}],\"name\":\"setKey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"initialKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"initialGuardianHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint64\",\"name\":\"initialGuardianSafePeriod\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"newKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"keySignature\",\"type\":\"bytes\"}],\"name\":\"setKey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"newKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"keySignature\",\"type\":\"bytes\"}],\"name\":\"setKey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"slot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"newKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"rawGuardian\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"guardianSignature\",\"type\":\"bytes\"}],\"name\":\"setKey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static KEYSTORE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct KeyStore<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for KeyStore<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for KeyStore<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for KeyStore<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for KeyStore<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(KeyStore)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> KeyStore<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KEYSTORE_ABI.clone(),
                    client,
                ),
            )
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
        ///Calls the contract's `cancelSetGuardian` (0xc8249b83) function
        pub fn cancel_set_guardian(
            &self,
            slot: [u8; 32],
            key_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 36, 155, 131], (slot, key_signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelSetGuardianSafePeriod` (0x99b62a52) function
        pub fn cancel_set_guardian_safe_period(
            &self,
            slot: [u8; 32],
            key_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 182, 42, 82], (slot, key_signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGuardianHash` (0x527689e4) function
        pub fn get_guardian_hash(
            &self,
            raw_guardian: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 118, 137, 228], raw_guardian)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getKey` (0x12aaac70) function
        pub fn get_key(
            &self,
            slot: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([18, 170, 172, 112], slot)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getKeyStoreInfo` (0xc353f4bd) function
        pub fn get_key_store_info(
            &self,
            slot: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, KeyStoreInfo> {
            self.0
                .method_hash([195, 83, 244, 189], slot)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlot` (0x8e0cdf64) function
        pub fn get_slot(
            &self,
            initial_key: [u8; 32],
            initial_guardian_hash: [u8; 32],
            guardian_safe_period: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [142, 12, 223, 100],
                    (initial_key, initial_guardian_hash, guardian_safe_period),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `keystoreBySlot` (0x5e85a9bc) function
        pub fn keystore_by_slot(
            &self,
            l_1_slot: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([94, 133, 169, 188], l_1_slot)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0x905da30f) function
        pub fn nonce(
            &self,
            slot: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 93, 163, 15], slot)
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
        ///Calls the contract's `setGuardian` (0x7104eb0a) function
        pub fn set_guardian(
            &self,
            slot: [u8; 32],
            new_guardian_hash: [u8; 32],
            key_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 4, 235, 10], (slot, new_guardian_hash, key_signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGuardian` (0xbfeebe3a) function
        pub fn set_guardian_with_initial_key_and_initial_guardian_hash_and_initial_guardian_safe_period(
            &self,
            initial_key: [u8; 32],
            initial_guardian_hash: [u8; 32],
            initial_guardian_safe_period: u64,
            new_guardian_hash: [u8; 32],
            key_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [191, 238, 190, 58],
                    (
                        initial_key,
                        initial_guardian_hash,
                        initial_guardian_safe_period,
                        new_guardian_hash,
                        key_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGuardianSafePeriod` (0x61810cf0) function
        pub fn set_guardian_safe_period_with_initial_key_and_initial_guardian_hash_and_initial_guardian_safe_period(
            &self,
            initial_key: [u8; 32],
            initial_guardian_hash: [u8; 32],
            initial_guardian_safe_period: u64,
            new_guardian_safe_period: u64,
            key_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [97, 129, 12, 240],
                    (
                        initial_key,
                        initial_guardian_hash,
                        initial_guardian_safe_period,
                        new_guardian_safe_period,
                        key_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGuardianSafePeriod` (0xf38a935d) function
        pub fn set_guardian_safe_period(
            &self,
            slot: [u8; 32],
            new_guardian_safe_period: u64,
            key_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [243, 138, 147, 93],
                    (slot, new_guardian_safe_period, key_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setKey` (0x07a00330) function
        pub fn set_key_0(
            &self,
            slot: [u8; 32],
            new_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 160, 3, 48], (slot, new_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setKey` (0x259d32d7) function
        pub fn set_key_4(
            &self,
            initial_key: [u8; 32],
            initial_guardian_hash: [u8; 32],
            initial_guardian_safe_period: u64,
            new_key: [u8; 32],
            raw_guardian: ::ethers::core::types::Bytes,
            guardian_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [37, 157, 50, 215],
                    (
                        initial_key,
                        initial_guardian_hash,
                        initial_guardian_safe_period,
                        new_key,
                        raw_guardian,
                        guardian_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setKey` (0x7acd0787) function
        pub fn set_key_3(
            &self,
            initial_key: [u8; 32],
            initial_guardian_hash: [u8; 32],
            initial_guardian_safe_period: u64,
            new_key: [u8; 32],
            key_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [122, 205, 7, 135],
                    (
                        initial_key,
                        initial_guardian_hash,
                        initial_guardian_safe_period,
                        new_key,
                        key_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setKey` (0xcd5acd4d) function
        pub fn set_key_1(
            &self,
            slot: [u8; 32],
            new_key: [u8; 32],
            key_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 90, 205, 77], (slot, new_key, key_signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setKey` (0xe634882e) function
        pub fn set_key_2(
            &self,
            slot: [u8; 32],
            new_key: [u8; 32],
            raw_guardian: ::ethers::core::types::Bytes,
            guardian_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [230, 52, 136, 46],
                    (slot, new_key, raw_guardian, guardian_signature),
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
        ///Gets the contract's `CancelSetGuardian` event
        pub fn cancel_set_guardian_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CancelSetGuardianFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CancelSetGuardianSafePeriod` event
        pub fn cancel_set_guardian_safe_period_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CancelSetGuardianSafePeriodFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GuardianChanged` event
        pub fn guardian_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GuardianChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GuardianSafePeriodChanged` event
        pub fn guardian_safe_period_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GuardianSafePeriodChangedFilter,
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
        ///Gets the contract's `KeyChanged` event
        pub fn key_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            KeyChangedFilter,
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
        ///Gets the contract's `SetGuardian` event
        pub fn set_guardian_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetGuardianFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetGuardianSafePeriod` event
        pub fn set_guardian_safe_period_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetGuardianSafePeriodFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            KeyStoreEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for KeyStore<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `GUARDIAN_SIGNATURE_INVALID` with signature `GUARDIAN_SIGNATURE_INVALID()` and selector `0xc8e9d6fe`
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
        name = "GUARDIAN_SIGNATURE_INVALID",
        abi = "GUARDIAN_SIGNATURE_INVALID()"
    )]
    pub struct GUARDIAN_SIGNATURE_INVALID;
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
    ///Custom Error type `INVALID_DATA` with signature `INVALID_DATA()` and selector `0x1c698bde`
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
    #[etherror(name = "INVALID_DATA", abi = "INVALID_DATA()")]
    pub struct INVALID_DATA;
    ///Custom Error type `INVALID_KEY` with signature `INVALID_KEY()` and selector `0xce7045bd`
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
    #[etherror(name = "INVALID_KEY", abi = "INVALID_KEY()")]
    pub struct INVALID_KEY;
    ///Custom Error type `INVALID_SIGNATURE` with signature `INVALID_SIGNATURE()` and selector `0xa3402a38`
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
    #[etherror(name = "INVALID_SIGNATURE", abi = "INVALID_SIGNATURE()")]
    pub struct INVALID_SIGNATURE;
    ///Custom Error type `INVALID_TIME_RANGE` with signature `INVALID_TIME_RANGE()` and selector `0x99addf25`
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
    #[etherror(name = "INVALID_TIME_RANGE", abi = "INVALID_TIME_RANGE()")]
    pub struct INVALID_TIME_RANGE;
    ///Custom Error type `NOT_INITIALIZED` with signature `NOT_INITIALIZED()` and selector `0x3da3f98c`
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
    #[etherror(name = "NOT_INITIALIZED", abi = "NOT_INITIALIZED()")]
    pub struct NOT_INITIALIZED;
    ///Custom Error type `UNAUTHORIZED` with signature `UNAUTHORIZED()` and selector `0x075fd2b1`
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
    #[etherror(name = "UNAUTHORIZED", abi = "UNAUTHORIZED()")]
    pub struct UNAUTHORIZED;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum KeyStoreErrors {
        GUARDIAN_SIGNATURE_INVALID(GUARDIAN_SIGNATURE_INVALID),
        HASH_ALREADY_APPROVED(HASH_ALREADY_APPROVED),
        HASH_ALREADY_REJECTED(HASH_ALREADY_REJECTED),
        INVALID_DATA(INVALID_DATA),
        INVALID_KEY(INVALID_KEY),
        INVALID_SIGNATURE(INVALID_SIGNATURE),
        INVALID_TIME_RANGE(INVALID_TIME_RANGE),
        NOT_INITIALIZED(NOT_INITIALIZED),
        UNAUTHORIZED(UNAUTHORIZED),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for KeyStoreErrors {
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
                = <GUARDIAN_SIGNATURE_INVALID as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GUARDIAN_SIGNATURE_INVALID(decoded));
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
                = <INVALID_DATA as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::INVALID_DATA(decoded));
            }
            if let Ok(decoded)
                = <INVALID_KEY as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::INVALID_KEY(decoded));
            }
            if let Ok(decoded)
                = <INVALID_SIGNATURE as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::INVALID_SIGNATURE(decoded));
            }
            if let Ok(decoded)
                = <INVALID_TIME_RANGE as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::INVALID_TIME_RANGE(decoded));
            }
            if let Ok(decoded)
                = <NOT_INITIALIZED as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NOT_INITIALIZED(decoded));
            }
            if let Ok(decoded)
                = <UNAUTHORIZED as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UNAUTHORIZED(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KeyStoreErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::GUARDIAN_SIGNATURE_INVALID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HASH_ALREADY_APPROVED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HASH_ALREADY_REJECTED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_DATA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_KEY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_SIGNATURE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::INVALID_TIME_RANGE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NOT_INITIALIZED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UNAUTHORIZED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for KeyStoreErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <GUARDIAN_SIGNATURE_INVALID as ::ethers::contract::EthError>::selector() => {
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
                    == <INVALID_DATA as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <INVALID_KEY as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <INVALID_SIGNATURE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <INVALID_TIME_RANGE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NOT_INITIALIZED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UNAUTHORIZED as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for KeyStoreErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GUARDIAN_SIGNATURE_INVALID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HASH_ALREADY_APPROVED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HASH_ALREADY_REJECTED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::INVALID_DATA(element) => ::core::fmt::Display::fmt(element, f),
                Self::INVALID_KEY(element) => ::core::fmt::Display::fmt(element, f),
                Self::INVALID_SIGNATURE(element) => ::core::fmt::Display::fmt(element, f),
                Self::INVALID_TIME_RANGE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NOT_INITIALIZED(element) => ::core::fmt::Display::fmt(element, f),
                Self::UNAUTHORIZED(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for KeyStoreErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<GUARDIAN_SIGNATURE_INVALID> for KeyStoreErrors {
        fn from(value: GUARDIAN_SIGNATURE_INVALID) -> Self {
            Self::GUARDIAN_SIGNATURE_INVALID(value)
        }
    }
    impl ::core::convert::From<HASH_ALREADY_APPROVED> for KeyStoreErrors {
        fn from(value: HASH_ALREADY_APPROVED) -> Self {
            Self::HASH_ALREADY_APPROVED(value)
        }
    }
    impl ::core::convert::From<HASH_ALREADY_REJECTED> for KeyStoreErrors {
        fn from(value: HASH_ALREADY_REJECTED) -> Self {
            Self::HASH_ALREADY_REJECTED(value)
        }
    }
    impl ::core::convert::From<INVALID_DATA> for KeyStoreErrors {
        fn from(value: INVALID_DATA) -> Self {
            Self::INVALID_DATA(value)
        }
    }
    impl ::core::convert::From<INVALID_KEY> for KeyStoreErrors {
        fn from(value: INVALID_KEY) -> Self {
            Self::INVALID_KEY(value)
        }
    }
    impl ::core::convert::From<INVALID_SIGNATURE> for KeyStoreErrors {
        fn from(value: INVALID_SIGNATURE) -> Self {
            Self::INVALID_SIGNATURE(value)
        }
    }
    impl ::core::convert::From<INVALID_TIME_RANGE> for KeyStoreErrors {
        fn from(value: INVALID_TIME_RANGE) -> Self {
            Self::INVALID_TIME_RANGE(value)
        }
    }
    impl ::core::convert::From<NOT_INITIALIZED> for KeyStoreErrors {
        fn from(value: NOT_INITIALIZED) -> Self {
            Self::NOT_INITIALIZED(value)
        }
    }
    impl ::core::convert::From<UNAUTHORIZED> for KeyStoreErrors {
        fn from(value: UNAUTHORIZED) -> Self {
            Self::UNAUTHORIZED(value)
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
    #[ethevent(name = "ApproveHash", abi = "ApproveHash(address,bytes32)")]
    pub struct ApproveHashFilter {
        #[ethevent(indexed)]
        pub guardian: ::ethers::core::types::Address,
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
    #[ethevent(name = "CancelSetGuardian", abi = "CancelSetGuardian(bytes32,bytes32)")]
    pub struct CancelSetGuardianFilter {
        #[ethevent(indexed)]
        pub slot: [u8; 32],
        pub guardian_hash: [u8; 32],
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
    #[ethevent(
        name = "CancelSetGuardianSafePeriod",
        abi = "CancelSetGuardianSafePeriod(bytes32,uint64)"
    )]
    pub struct CancelSetGuardianSafePeriodFilter {
        #[ethevent(indexed)]
        pub slot: [u8; 32],
        pub guardian_safe_period: u64,
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
    #[ethevent(name = "GuardianChanged", abi = "GuardianChanged(bytes32,bytes32)")]
    pub struct GuardianChangedFilter {
        #[ethevent(indexed)]
        pub slot: [u8; 32],
        pub guardian_hash: [u8; 32],
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
    #[ethevent(
        name = "GuardianSafePeriodChanged",
        abi = "GuardianSafePeriodChanged(bytes32,uint64)"
    )]
    pub struct GuardianSafePeriodChangedFilter {
        #[ethevent(indexed)]
        pub slot: [u8; 32],
        pub guardian_safe_period: u64,
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
    #[ethevent(name = "Initialized", abi = "Initialized(bytes32)")]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub slot: [u8; 32],
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
    #[ethevent(name = "KeyChanged", abi = "KeyChanged(bytes32,bytes32)")]
    pub struct KeyChangedFilter {
        #[ethevent(indexed)]
        pub slot: [u8; 32],
        pub key: [u8; 32],
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
    #[ethevent(name = "RejectHash", abi = "RejectHash(address,bytes32)")]
    pub struct RejectHashFilter {
        #[ethevent(indexed)]
        pub guardian: ::ethers::core::types::Address,
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
    #[ethevent(name = "SetGuardian", abi = "SetGuardian(bytes32,bytes32,uint64)")]
    pub struct SetGuardianFilter {
        #[ethevent(indexed)]
        pub slot: [u8; 32],
        pub guardian_hash: [u8; 32],
        pub effect_at: u64,
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
    #[ethevent(
        name = "SetGuardianSafePeriod",
        abi = "SetGuardianSafePeriod(bytes32,uint64,uint64)"
    )]
    pub struct SetGuardianSafePeriodFilter {
        #[ethevent(indexed)]
        pub slot: [u8; 32],
        pub guardian_safe_period: u64,
        pub effect_at: u64,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum KeyStoreEvents {
        ApproveHashFilter(ApproveHashFilter),
        CancelSetGuardianFilter(CancelSetGuardianFilter),
        CancelSetGuardianSafePeriodFilter(CancelSetGuardianSafePeriodFilter),
        GuardianChangedFilter(GuardianChangedFilter),
        GuardianSafePeriodChangedFilter(GuardianSafePeriodChangedFilter),
        InitializedFilter(InitializedFilter),
        KeyChangedFilter(KeyChangedFilter),
        RejectHashFilter(RejectHashFilter),
        SetGuardianFilter(SetGuardianFilter),
        SetGuardianSafePeriodFilter(SetGuardianSafePeriodFilter),
    }
    impl ::ethers::contract::EthLogDecode for KeyStoreEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApproveHashFilter::decode_log(log) {
                return Ok(KeyStoreEvents::ApproveHashFilter(decoded));
            }
            if let Ok(decoded) = CancelSetGuardianFilter::decode_log(log) {
                return Ok(KeyStoreEvents::CancelSetGuardianFilter(decoded));
            }
            if let Ok(decoded) = CancelSetGuardianSafePeriodFilter::decode_log(log) {
                return Ok(KeyStoreEvents::CancelSetGuardianSafePeriodFilter(decoded));
            }
            if let Ok(decoded) = GuardianChangedFilter::decode_log(log) {
                return Ok(KeyStoreEvents::GuardianChangedFilter(decoded));
            }
            if let Ok(decoded) = GuardianSafePeriodChangedFilter::decode_log(log) {
                return Ok(KeyStoreEvents::GuardianSafePeriodChangedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(KeyStoreEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = KeyChangedFilter::decode_log(log) {
                return Ok(KeyStoreEvents::KeyChangedFilter(decoded));
            }
            if let Ok(decoded) = RejectHashFilter::decode_log(log) {
                return Ok(KeyStoreEvents::RejectHashFilter(decoded));
            }
            if let Ok(decoded) = SetGuardianFilter::decode_log(log) {
                return Ok(KeyStoreEvents::SetGuardianFilter(decoded));
            }
            if let Ok(decoded) = SetGuardianSafePeriodFilter::decode_log(log) {
                return Ok(KeyStoreEvents::SetGuardianSafePeriodFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for KeyStoreEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveHashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelSetGuardianFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CancelSetGuardianSafePeriodFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GuardianChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GuardianSafePeriodChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeyChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RejectHashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGuardianFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGuardianSafePeriodFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApproveHashFilter> for KeyStoreEvents {
        fn from(value: ApproveHashFilter) -> Self {
            Self::ApproveHashFilter(value)
        }
    }
    impl ::core::convert::From<CancelSetGuardianFilter> for KeyStoreEvents {
        fn from(value: CancelSetGuardianFilter) -> Self {
            Self::CancelSetGuardianFilter(value)
        }
    }
    impl ::core::convert::From<CancelSetGuardianSafePeriodFilter> for KeyStoreEvents {
        fn from(value: CancelSetGuardianSafePeriodFilter) -> Self {
            Self::CancelSetGuardianSafePeriodFilter(value)
        }
    }
    impl ::core::convert::From<GuardianChangedFilter> for KeyStoreEvents {
        fn from(value: GuardianChangedFilter) -> Self {
            Self::GuardianChangedFilter(value)
        }
    }
    impl ::core::convert::From<GuardianSafePeriodChangedFilter> for KeyStoreEvents {
        fn from(value: GuardianSafePeriodChangedFilter) -> Self {
            Self::GuardianSafePeriodChangedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for KeyStoreEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<KeyChangedFilter> for KeyStoreEvents {
        fn from(value: KeyChangedFilter) -> Self {
            Self::KeyChangedFilter(value)
        }
    }
    impl ::core::convert::From<RejectHashFilter> for KeyStoreEvents {
        fn from(value: RejectHashFilter) -> Self {
            Self::RejectHashFilter(value)
        }
    }
    impl ::core::convert::From<SetGuardianFilter> for KeyStoreEvents {
        fn from(value: SetGuardianFilter) -> Self {
            Self::SetGuardianFilter(value)
        }
    }
    impl ::core::convert::From<SetGuardianSafePeriodFilter> for KeyStoreEvents {
        fn from(value: SetGuardianSafePeriodFilter) -> Self {
            Self::SetGuardianSafePeriodFilter(value)
        }
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
    ///Container type for all input parameters for the `cancelSetGuardian` function with signature `cancelSetGuardian(bytes32,bytes)` and selector `0xc8249b83`
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
    #[ethcall(name = "cancelSetGuardian", abi = "cancelSetGuardian(bytes32,bytes)")]
    pub struct CancelSetGuardianCall {
        pub slot: [u8; 32],
        pub key_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `cancelSetGuardianSafePeriod` function with signature `cancelSetGuardianSafePeriod(bytes32,bytes)` and selector `0x99b62a52`
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
        name = "cancelSetGuardianSafePeriod",
        abi = "cancelSetGuardianSafePeriod(bytes32,bytes)"
    )]
    pub struct CancelSetGuardianSafePeriodCall {
        pub slot: [u8; 32],
        pub key_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getGuardianHash` function with signature `getGuardianHash(bytes)` and selector `0x527689e4`
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
    #[ethcall(name = "getGuardianHash", abi = "getGuardianHash(bytes)")]
    pub struct GetGuardianHashCall {
        pub raw_guardian: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getKey` function with signature `getKey(bytes32)` and selector `0x12aaac70`
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
    #[ethcall(name = "getKey", abi = "getKey(bytes32)")]
    pub struct GetKeyCall {
        pub slot: [u8; 32],
    }
    ///Container type for all input parameters for the `getKeyStoreInfo` function with signature `getKeyStoreInfo(bytes32)` and selector `0xc353f4bd`
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
    #[ethcall(name = "getKeyStoreInfo", abi = "getKeyStoreInfo(bytes32)")]
    pub struct GetKeyStoreInfoCall {
        pub slot: [u8; 32],
    }
    ///Container type for all input parameters for the `getSlot` function with signature `getSlot(bytes32,bytes32,uint64)` and selector `0x8e0cdf64`
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
    #[ethcall(name = "getSlot", abi = "getSlot(bytes32,bytes32,uint64)")]
    pub struct GetSlotCall {
        pub initial_key: [u8; 32],
        pub initial_guardian_hash: [u8; 32],
        pub guardian_safe_period: u64,
    }
    ///Container type for all input parameters for the `keystoreBySlot` function with signature `keystoreBySlot(bytes32)` and selector `0x5e85a9bc`
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
    #[ethcall(name = "keystoreBySlot", abi = "keystoreBySlot(bytes32)")]
    pub struct KeystoreBySlotCall {
        pub l_1_slot: [u8; 32],
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce(bytes32)` and selector `0x905da30f`
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
    #[ethcall(name = "nonce", abi = "nonce(bytes32)")]
    pub struct NonceCall {
        pub slot: [u8; 32],
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
    ///Container type for all input parameters for the `setGuardian` function with signature `setGuardian(bytes32,bytes32,bytes)` and selector `0x7104eb0a`
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
    #[ethcall(name = "setGuardian", abi = "setGuardian(bytes32,bytes32,bytes)")]
    pub struct SetGuardianCall {
        pub slot: [u8; 32],
        pub new_guardian_hash: [u8; 32],
        pub key_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setGuardian` function with signature `setGuardian(bytes32,bytes32,uint64,bytes32,bytes)` and selector `0xbfeebe3a`
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
        name = "setGuardian",
        abi = "setGuardian(bytes32,bytes32,uint64,bytes32,bytes)"
    )]
    pub struct SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall {
        pub initial_key: [u8; 32],
        pub initial_guardian_hash: [u8; 32],
        pub initial_guardian_safe_period: u64,
        pub new_guardian_hash: [u8; 32],
        pub key_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setGuardianSafePeriod` function with signature `setGuardianSafePeriod(bytes32,bytes32,uint64,uint64,bytes)` and selector `0x61810cf0`
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
        name = "setGuardianSafePeriod",
        abi = "setGuardianSafePeriod(bytes32,bytes32,uint64,uint64,bytes)"
    )]
    pub struct SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall {
        pub initial_key: [u8; 32],
        pub initial_guardian_hash: [u8; 32],
        pub initial_guardian_safe_period: u64,
        pub new_guardian_safe_period: u64,
        pub key_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setGuardianSafePeriod` function with signature `setGuardianSafePeriod(bytes32,uint64,bytes)` and selector `0xf38a935d`
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
        name = "setGuardianSafePeriod",
        abi = "setGuardianSafePeriod(bytes32,uint64,bytes)"
    )]
    pub struct SetGuardianSafePeriodCall {
        pub slot: [u8; 32],
        pub new_guardian_safe_period: u64,
        pub key_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setKey` function with signature `setKey(bytes32,bytes32)` and selector `0x07a00330`
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
    #[ethcall(name = "setKey", abi = "setKey(bytes32,bytes32)")]
    pub struct SetKey0Call {
        pub slot: [u8; 32],
        pub new_key: [u8; 32],
    }
    ///Container type for all input parameters for the `setKey` function with signature `setKey(bytes32,bytes32,uint64,bytes32,bytes,bytes)` and selector `0x259d32d7`
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
        name = "setKey",
        abi = "setKey(bytes32,bytes32,uint64,bytes32,bytes,bytes)"
    )]
    pub struct SetKey4Call {
        pub initial_key: [u8; 32],
        pub initial_guardian_hash: [u8; 32],
        pub initial_guardian_safe_period: u64,
        pub new_key: [u8; 32],
        pub raw_guardian: ::ethers::core::types::Bytes,
        pub guardian_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setKey` function with signature `setKey(bytes32,bytes32,uint64,bytes32,bytes)` and selector `0x7acd0787`
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
    #[ethcall(name = "setKey", abi = "setKey(bytes32,bytes32,uint64,bytes32,bytes)")]
    pub struct SetKey3Call {
        pub initial_key: [u8; 32],
        pub initial_guardian_hash: [u8; 32],
        pub initial_guardian_safe_period: u64,
        pub new_key: [u8; 32],
        pub key_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setKey` function with signature `setKey(bytes32,bytes32,bytes)` and selector `0xcd5acd4d`
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
    #[ethcall(name = "setKey", abi = "setKey(bytes32,bytes32,bytes)")]
    pub struct SetKey1Call {
        pub slot: [u8; 32],
        pub new_key: [u8; 32],
        pub key_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setKey` function with signature `setKey(bytes32,bytes32,bytes,bytes)` and selector `0xe634882e`
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
    #[ethcall(name = "setKey", abi = "setKey(bytes32,bytes32,bytes,bytes)")]
    pub struct SetKey2Call {
        pub slot: [u8; 32],
        pub new_key: [u8; 32],
        pub raw_guardian: ::ethers::core::types::Bytes,
        pub guardian_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum KeyStoreCalls {
        ApproveHash(ApproveHashCall),
        CancelSetGuardian(CancelSetGuardianCall),
        CancelSetGuardianSafePeriod(CancelSetGuardianSafePeriodCall),
        GetGuardianHash(GetGuardianHashCall),
        GetKey(GetKeyCall),
        GetKeyStoreInfo(GetKeyStoreInfoCall),
        GetSlot(GetSlotCall),
        KeystoreBySlot(KeystoreBySlotCall),
        Nonce(NonceCall),
        RejectHash(RejectHashCall),
        SetGuardian(SetGuardianCall),
        SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
            SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall,
        ),
        SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
            SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall,
        ),
        SetGuardianSafePeriod(SetGuardianSafePeriodCall),
        SetKey0(SetKey0Call),
        SetKey4(SetKey4Call),
        SetKey3(SetKey3Call),
        SetKey1(SetKey1Call),
        SetKey2(SetKey2Call),
    }
    impl ::ethers::core::abi::AbiDecode for KeyStoreCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ApproveHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApproveHash(decoded));
            }
            if let Ok(decoded)
                = <CancelSetGuardianCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelSetGuardian(decoded));
            }
            if let Ok(decoded)
                = <CancelSetGuardianSafePeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelSetGuardianSafePeriod(decoded));
            }
            if let Ok(decoded)
                = <GetGuardianHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetGuardianHash(decoded));
            }
            if let Ok(decoded)
                = <GetKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetKey(decoded));
            }
            if let Ok(decoded)
                = <GetKeyStoreInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetKeyStoreInfo(decoded));
            }
            if let Ok(decoded)
                = <GetSlotCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSlot(decoded));
            }
            if let Ok(decoded)
                = <KeystoreBySlotCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::KeystoreBySlot(decoded));
            }
            if let Ok(decoded)
                = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded)
                = <RejectHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RejectHash(decoded));
            }
            if let Ok(decoded)
                = <SetGuardianCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetGuardian(decoded));
            }
            if let Ok(decoded)
                = <SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = <SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = <SetGuardianSafePeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetGuardianSafePeriod(decoded));
            }
            if let Ok(decoded)
                = <SetKey0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetKey0(decoded));
            }
            if let Ok(decoded)
                = <SetKey4Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetKey4(decoded));
            }
            if let Ok(decoded)
                = <SetKey3Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetKey3(decoded));
            }
            if let Ok(decoded)
                = <SetKey1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetKey1(decoded));
            }
            if let Ok(decoded)
                = <SetKey2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetKey2(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KeyStoreCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApproveHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelSetGuardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelSetGuardianSafePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGuardianHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetKeyStoreInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSlot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::KeystoreBySlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RejectHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGuardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetGuardianSafePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetKey0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetKey4(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetKey3(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetKey1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetKey2(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for KeyStoreCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelSetGuardian(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelSetGuardianSafePeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGuardianHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetKeyStoreInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeystoreBySlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::RejectHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGuardian(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::SetGuardianSafePeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetKey0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetKey4(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetKey3(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetKey1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetKey2(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveHashCall> for KeyStoreCalls {
        fn from(value: ApproveHashCall) -> Self {
            Self::ApproveHash(value)
        }
    }
    impl ::core::convert::From<CancelSetGuardianCall> for KeyStoreCalls {
        fn from(value: CancelSetGuardianCall) -> Self {
            Self::CancelSetGuardian(value)
        }
    }
    impl ::core::convert::From<CancelSetGuardianSafePeriodCall> for KeyStoreCalls {
        fn from(value: CancelSetGuardianSafePeriodCall) -> Self {
            Self::CancelSetGuardianSafePeriod(value)
        }
    }
    impl ::core::convert::From<GetGuardianHashCall> for KeyStoreCalls {
        fn from(value: GetGuardianHashCall) -> Self {
            Self::GetGuardianHash(value)
        }
    }
    impl ::core::convert::From<GetKeyCall> for KeyStoreCalls {
        fn from(value: GetKeyCall) -> Self {
            Self::GetKey(value)
        }
    }
    impl ::core::convert::From<GetKeyStoreInfoCall> for KeyStoreCalls {
        fn from(value: GetKeyStoreInfoCall) -> Self {
            Self::GetKeyStoreInfo(value)
        }
    }
    impl ::core::convert::From<GetSlotCall> for KeyStoreCalls {
        fn from(value: GetSlotCall) -> Self {
            Self::GetSlot(value)
        }
    }
    impl ::core::convert::From<KeystoreBySlotCall> for KeyStoreCalls {
        fn from(value: KeystoreBySlotCall) -> Self {
            Self::KeystoreBySlot(value)
        }
    }
    impl ::core::convert::From<NonceCall> for KeyStoreCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<RejectHashCall> for KeyStoreCalls {
        fn from(value: RejectHashCall) -> Self {
            Self::RejectHash(value)
        }
    }
    impl ::core::convert::From<SetGuardianCall> for KeyStoreCalls {
        fn from(value: SetGuardianCall) -> Self {
            Self::SetGuardian(value)
        }
    }
    impl ::core::convert::From<
        SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall,
    > for KeyStoreCalls {
        fn from(
            value: SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall,
        ) -> Self {
            Self::SetGuardianWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
                value,
            )
        }
    }
    impl ::core::convert::From<
        SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall,
    > for KeyStoreCalls {
        fn from(
            value: SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriodCall,
        ) -> Self {
            Self::SetGuardianSafePeriodWithInitialKeyAndInitialGuardianHashAndInitialGuardianSafePeriod(
                value,
            )
        }
    }
    impl ::core::convert::From<SetGuardianSafePeriodCall> for KeyStoreCalls {
        fn from(value: SetGuardianSafePeriodCall) -> Self {
            Self::SetGuardianSafePeriod(value)
        }
    }
    impl ::core::convert::From<SetKey0Call> for KeyStoreCalls {
        fn from(value: SetKey0Call) -> Self {
            Self::SetKey0(value)
        }
    }
    impl ::core::convert::From<SetKey4Call> for KeyStoreCalls {
        fn from(value: SetKey4Call) -> Self {
            Self::SetKey4(value)
        }
    }
    impl ::core::convert::From<SetKey3Call> for KeyStoreCalls {
        fn from(value: SetKey3Call) -> Self {
            Self::SetKey3(value)
        }
    }
    impl ::core::convert::From<SetKey1Call> for KeyStoreCalls {
        fn from(value: SetKey1Call) -> Self {
            Self::SetKey1(value)
        }
    }
    impl ::core::convert::From<SetKey2Call> for KeyStoreCalls {
        fn from(value: SetKey2Call) -> Self {
            Self::SetKey2(value)
        }
    }
    ///Container type for all return fields from the `getGuardianHash` function with signature `getGuardianHash(bytes)` and selector `0x527689e4`
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
    pub struct GetGuardianHashReturn {
        pub guardian_hash: [u8; 32],
    }
    ///Container type for all return fields from the `getKey` function with signature `getKey(bytes32)` and selector `0x12aaac70`
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
    pub struct GetKeyReturn {
        pub key: [u8; 32],
    }
    ///Container type for all return fields from the `getKeyStoreInfo` function with signature `getKeyStoreInfo(bytes32)` and selector `0xc353f4bd`
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
    pub struct GetKeyStoreInfoReturn {
        pub key_store_info: KeyStoreInfo,
    }
    ///Container type for all return fields from the `getSlot` function with signature `getSlot(bytes32,bytes32,uint64)` and selector `0x8e0cdf64`
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
    pub struct GetSlotReturn {
        pub slot: [u8; 32],
    }
    ///Container type for all return fields from the `keystoreBySlot` function with signature `keystoreBySlot(bytes32)` and selector `0x5e85a9bc`
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
    pub struct KeystoreBySlotReturn {
        pub signing_key: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `nonce` function with signature `nonce(bytes32)` and selector `0x905da30f`
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
    pub struct NonceReturn {
        pub nonce: ::ethers::core::types::U256,
    }
    ///`KeyStoreInfo(bytes32,uint256,bytes32,bytes32,uint64,uint64,uint64,uint64)`
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
    pub struct KeyStoreInfo {
        pub key: [u8; 32],
        pub nonce: ::ethers::core::types::U256,
        pub guardian_hash: [u8; 32],
        pub pending_guardian_hash: [u8; 32],
        pub guardian_activate_at: u64,
        pub guardian_safe_period: u64,
        pub pending_guardian_safe_period: u64,
        pub guardian_safe_period_activate_at: u64,
    }
}
