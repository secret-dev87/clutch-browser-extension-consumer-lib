# Clutch browser extension blockchain lib

[![BackendTests](https://github.com/clutch-wallet/clutch-wallet-lib/actions/workflows/linux-ci.yml/badge.svg)](https://github.com/clutch-wallet/clutch-wallet-lib/actions/workflows/linux-ci.yml)


## Install dependencies

Dependencies for working with solidity:

```
pip3 install solc-select

solc-select install 0.8.19 && solc-select use 0.8.19

npm install @openzeppelin/contracts
npm install @openzeppelin/contracts-upgradeable
npm install @chainlink/contracts
```

### Run tests

```
cargo test -- --show-output
```

### Check before pushing

```
./check.sh
```

### Entrypoint contracts

An Entrypoint contract has been deployed on several chains with the same address: (ref: [StackOverflow](https://ethereum.stackexchange.com/questions/146974/how-to-start-with-erc-4337))

* 0x0576a174D229E3cFA37253523E645A78A0C91B57

Polygon mumbai testnet: [EntryPoint Contract](https://mumbai.polygonscan.com/address/0x0576a174D229E3cFA37253523E645A78A0C91B57#code)
