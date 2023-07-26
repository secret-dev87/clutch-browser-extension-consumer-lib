use crate::build::abi_and_bytecode_for_contract;
use ethers::{
    abi::{self, Token},
    prelude::ContractFactory,
    types::{Address, Bytes, U256},
};
use eyre::ContextCompat;
use std::{str::FromStr, collections::HashMap};

#[derive(Debug, Clone)]
pub struct GuardHookInputData {
  pub sender: Address,
  pub input_data: HashMap<Address, Vec<u8>>
}

#[derive(Debug, Clone)]
pub struct HookInputData{
  pub guard_hooks: Vec<Address>,
  pub input_data: HashMap<Address, Vec<u8>>
}