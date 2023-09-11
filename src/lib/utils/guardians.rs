use super::abis::{l1_key_store_contract, L1KeyStoreContract};
use chrono::format::Fixed;
use ethers::{
    abi::{encode, FixedBytes, Token, Uint},
    prelude::ContractFactory,
    providers::{Http, Provider},
    types::{Address, Bytes, H256, U256},
    utils::keccak256,
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Add,
    str::FromStr,
    sync::Arc,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GuardHookInputData {
    pub sender: Address,
    pub input_data: HashMap<Address, Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct HookInputData {
    pub guard_hooks: Vec<Address>,
    pub input_data: HashMap<Address, Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyStoreInfo {
    key: Bytes,
    nonce: U256,
    guardian_hash: Bytes,
    pending_guardian_hash: Bytes,
    guardian_activate_at: U256,
    guardian_safe_period: U256,
    pending_guardian_safe_period: U256,
    guardian_safe_period_activate_at: U256,
}

pub struct L1KeyStore {
    key_store_contract_address: Address,
    provider: Provider<Http>,
    key_store_contract: L1KeyStoreContract<Provider<Http>>,
    days: U256,
}

impl L1KeyStore {
    pub fn new(_provider: String, _contract_addr: Address) -> Self {
        let provider = Provider::<Http>::try_from(_provider.clone()).expect("Wrong provider url");
        let key_store_contract =
            super::abis::L1KeyStoreContract::new(_contract_addr, Arc::new(provider.clone()));
        Self {
            key_store_contract_address: _contract_addr,
            provider,
            key_store_contract,
            days: U256::from(86400),
        }
    }

    pub fn guardian_safe_period(&self, guardian_safe_period: U256) -> eyre::Result<bool> {
        match guardian_safe_period {
            val if val.as_u32() < self.days.as_u32() * 2 => {
                Err(eyre::eyre!("initialGuardianSafePeriod is too small"))
            }
            val if val.as_u32() > self.days.as_u32() * 30 => {
                Err(eyre::eyre!("initialGuardianSafePeriod is too large"))
            }
            _ => Ok(true),
        }
    }

    pub fn get_slot(
        &self,
        initial_key: H256,
        initial_guardian_hash: H256,
        initial_guardian_safe_period: Option<i32>,
    ) -> Bytes {
        let safe_period = initial_guardian_safe_period.unwrap_or(self.days.as_u32() as i32 * 2);

        let initial_guardian_safe_period: H256 = format!("{:064x}", safe_period).parse().unwrap();
        let abi_encoded = encode(&[
            Token::FixedBytes(FixedBytes::from(initial_key.to_fixed_bytes())),
            Token::FixedBytes(FixedBytes::from(initial_guardian_hash.to_fixed_bytes())),
            Token::FixedBytes(FixedBytes::from(
                initial_guardian_safe_period.to_fixed_bytes(),
            )),
        ]);

        let keccak256 = keccak256(abi_encoded);
        Bytes::from(keccak256)
    }

    pub fn has_duplicate<T: Ord>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        let unique = iter.into_iter().all(move |item| uniq.insert(item));
        !unique
    }

    pub fn get_guardian_bytes(
        &self,
        guardians: Vec<Address>,
        threshold: i32,
        salt: H256,
    ) -> eyre::Result<Bytes> {
        if guardians.len() == 0 {
            return Ok(Bytes::from(b""));
        }

        match L1KeyStore::has_duplicate(guardians.clone()) {
            true => Err(eyre::eyre!("Guardian address is duplicated")),
            false => {
                let token_addresses = guardians
                    .into_iter()
                    .map(|addr| Token::Address(addr))
                    .collect::<Vec<Token>>();
                let abi_encoded = encode(&[
                    Token::Array(token_addresses),
                    Token::Uint(U256::from(threshold)),
                    Token::Uint(U256::from(salt.0)),
                ]);
                Ok(Bytes::from(abi_encoded))
            }
        }
    }

    pub async fn get_key(&self, slot: Bytes) -> eyre::Result<Bytes> {
        let data: [u8; 32] = self
            .key_store_contract
            .get_key(slot.to_vec().try_into().unwrap())
            .await?;
        Ok(Bytes::from(data))
    }

    pub async fn get_keystore_info(&self, slot: Bytes) -> eyre::Result<KeyStoreInfo> {
        let data: l1_key_store_contract::KeyStoreInfo = self
            .key_store_contract
            .get_key_store_info(slot.to_vec().try_into().unwrap())
            .await?;        

        let key_store_info = KeyStoreInfo {
            key: Bytes::from(data.0),
            nonce: U256::from(data.1),
            guardian_hash: Bytes::from(data.2),
            pending_guardian_hash: Bytes::from(data.3),
            guardian_activate_at: U256::from(data.4),
            guardian_safe_period: U256::from(data.5),
            pending_guardian_safe_period: U256::from(data.6),
            guardian_safe_period_activate_at: U256::from(data.7),
        };
        Ok(key_store_info)
    }

    pub fn get_sig_hash(
        l1_keystore_contract_addr: Address,
        slot: Bytes,
        nonce: U256,
        data: H256,
    ) -> Bytes {
        let abi_encoded = encode(&[
            Token::Address(l1_keystore_contract_addr),
            Token::FixedBytes(FixedBytes::from(slot.to_vec())),
            Token::Uint(nonce),
            Token::FixedBytes(FixedBytes::from(data.0))
        ]);

        let keccak256 = keccak256(abi_encoded);
        Bytes::from(keccak256)
    }

    pub async fn get_set_key_sig_hash(&self, slot: Bytes, bytes32: H256)-> eyre::Result<Bytes> {
        let key_store_info = self.get_keystore_info(slot.clone()).await?;
        
        let ret = L1KeyStore::get_sig_hash(
            self.key_store_contract_address, 
            slot, 
            key_store_info.nonce, 
            bytes32);
        Ok(ret)
    }

    pub async fn get_set_guardian_sig_hash(&self, slot: Bytes, guardian_hash: H256) -> eyre::Result<Bytes> {
        let key_store_info = self.get_keystore_info(slot.clone()).await?;
        let ret = L1KeyStore::get_sig_hash(
            self.key_store_contract_address, 
            slot, 
            key_store_info.nonce, 
            guardian_hash);
        Ok(ret)
    }

    pub fn calc_guardian_hash(&self, guardians: Vec<Address>, threshold: i32, salt: Option<H256>) -> Bytes {
        if guardians.len() == 0 {
            return Bytes::from(H256::zero().to_fixed_bytes());
        }
        let salt = salt.unwrap_or(H256::zero());
        let abi_encoded = self.get_guardian_bytes(guardians, threshold, salt).unwrap();
        let keccak256 = keccak256(abi_encoded);
        Bytes::from(keccak256)
    }
}
