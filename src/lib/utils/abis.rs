
ethers::contract::abigen!(WalletContract, "./src/lib/generated/abi/contract_soulwallet.json");
ethers::contract::abigen!(WalletFactoryContract, "./src/lib/generated/abi/contract_soulwalletfactory.json");
ethers::contract::abigen!(EntryPointContract, "./src/lib/generated/abi/contract_entrypoint.json");
// ethers::contract::abigen!(ERC20Contract, "./src/lib/abi/ERC20.json");
// ethers::contract::abigen!(ERC721Contract, "./src/lib/abi/ERC721.json");