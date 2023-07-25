pub use soul_wallet_factory::*;
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
pub mod soul_wallet_factory {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_walletImpl\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"VERSION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_initializer\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_salt\",\"type\":\"bytes32\"}],\"name\":\"createWallet\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"proxy\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_initializer\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_salt\",\"type\":\"bytes32\"}],\"name\":\"getWalletAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"proxy\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proxyCode\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"walletImpl\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static SOULWALLETFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct SoulWalletFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SoulWalletFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SoulWalletFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SoulWalletFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SoulWalletFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SoulWalletFactory)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SoulWalletFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SOULWALLETFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createWallet` (0xa1aafc9e) function
        pub fn create_wallet(
            &self,
            initializer: ::ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([161, 170, 252, 158], (initializer, salt))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWalletAddress` (0x41d90058) function
        pub fn get_wallet_address(
            &self,
            initializer: ::ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([65, 217, 0, 88], (initializer, salt))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyCode` (0x6fa59bbc) function
        pub fn proxy_code(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([111, 165, 155, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `walletImpl` (0x3943c030) function
        pub fn wallet_impl(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([57, 67, 192, 48], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SoulWalletFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `createWallet` function with signature `createWallet(bytes,bytes32)` and selector `0xa1aafc9e`
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
    #[ethcall(name = "createWallet", abi = "createWallet(bytes,bytes32)")]
    pub struct CreateWalletCall {
        pub initializer: ::ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `getWalletAddress` function with signature `getWalletAddress(bytes,bytes32)` and selector `0x41d90058`
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
    #[ethcall(name = "getWalletAddress", abi = "getWalletAddress(bytes,bytes32)")]
    pub struct GetWalletAddressCall {
        pub initializer: ::ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `proxyCode` function with signature `proxyCode()` and selector `0x6fa59bbc`
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
    #[ethcall(name = "proxyCode", abi = "proxyCode()")]
    pub struct ProxyCodeCall;
    ///Container type for all input parameters for the `walletImpl` function with signature `walletImpl()` and selector `0x3943c030`
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
    #[ethcall(name = "walletImpl", abi = "walletImpl()")]
    pub struct WalletImplCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SoulWalletFactoryCalls {
        Version(VersionCall),
        CreateWallet(CreateWalletCall),
        GetWalletAddress(GetWalletAddressCall),
        ProxyCode(ProxyCodeCall),
        WalletImpl(WalletImplCall),
    }
    impl ::ethers::core::abi::AbiDecode for SoulWalletFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <CreateWalletCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateWallet(decoded));
            }
            if let Ok(decoded)
                = <GetWalletAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetWalletAddress(decoded));
            }
            if let Ok(decoded)
                = <ProxyCodeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProxyCode(decoded));
            }
            if let Ok(decoded)
                = <WalletImplCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WalletImpl(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SoulWalletFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateWallet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWalletAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WalletImpl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SoulWalletFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWalletAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::WalletImpl(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<VersionCall> for SoulWalletFactoryCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<CreateWalletCall> for SoulWalletFactoryCalls {
        fn from(value: CreateWalletCall) -> Self {
            Self::CreateWallet(value)
        }
    }
    impl ::core::convert::From<GetWalletAddressCall> for SoulWalletFactoryCalls {
        fn from(value: GetWalletAddressCall) -> Self {
            Self::GetWalletAddress(value)
        }
    }
    impl ::core::convert::From<ProxyCodeCall> for SoulWalletFactoryCalls {
        fn from(value: ProxyCodeCall) -> Self {
            Self::ProxyCode(value)
        }
    }
    impl ::core::convert::From<WalletImplCall> for SoulWalletFactoryCalls {
        fn from(value: WalletImplCall) -> Self {
            Self::WalletImpl(value)
        }
    }
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `createWallet` function with signature `createWallet(bytes,bytes32)` and selector `0xa1aafc9e`
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
    pub struct CreateWalletReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getWalletAddress` function with signature `getWalletAddress(bytes,bytes32)` and selector `0x41d90058`
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
    pub struct GetWalletAddressReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `proxyCode` function with signature `proxyCode()` and selector `0x6fa59bbc`
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
    pub struct ProxyCodeReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `walletImpl` function with signature `walletImpl()` and selector `0x3943c030`
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
    pub struct WalletImplReturn(pub ::ethers::core::types::Address);
}
