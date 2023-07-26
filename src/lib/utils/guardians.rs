use crate::build::abi_and_bytecode_for_contract;
use ethers::{
    abi::{self, Token},
    prelude::ContractFactory,
    providers::{Provider, Map},
    types::{Address, Bytes, U256},
};
use eyre::ContextCompat;
use std::{str::FromStr, collections::HashMap};

pub struct GuardHookInputData {
  pub sender: Address,
  pub input_data: Map<Address, Vec<u8>>
}