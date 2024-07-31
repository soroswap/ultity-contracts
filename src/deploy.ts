import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { airdropAccount, deployContract, installContract, bumpContractCode, invokeContract} from './utils/contract.js';
import { config } from './utils/env_config.js';

export async function deployContracts(addressBook: AddressBook, contracts_to_deploy: Array<string>) {
  // console.log("🚀 ~ deployContracts ~ contracts_to_deploy:", contracts_to_deploy)
  // console.log("🚀 ~ deployContracts ~ addressBook:", addressBook)
  
  console.log("🚀 ~ deployContracts ~ loadedConfig:", loadedConfig)
  
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  // let account = await loadedConfig.horizonRpc.loadAccount(loadedConfig.admin.publicKey()) 
  // let balance = account.balances[0].balance
  // console.log('Current Admin account balance:', balance);
  
  console.log('-------------------------------------------------------');
  console.log('Deploying Contracts');
  console.log('-------------------------------------------------------');
  for (var contract_name of contracts_to_deploy) {
    
    console.log(`Installing ${contract_name}... `)
    await installContract(contract_name, addressBook, loadedConfig.admin);
    
    console.log("  ")
    console.log(`Bumping ${contract_name}.. `)
    await bumpContractCode(contract_name, addressBook, loadedConfig.admin);

    console.log("  ")
    console.log(`Deploying ${contract_name}.. `)
    await deployContract(contract_name, contract_name, addressBook, loadedConfig.admin);

    console.log("  ")
    console.log(`Initializing ${contract_name}... `)

  //   fn initialize(
  //     e: Env,
  //     admin: Address,
  //     router_address: Address,
  //     end_timestamp: u64,
  // ) -> Result<(), CombinedLiquidityTimelockError>;

  
  const timeloclkInitParams: xdr.ScVal[] = [ 
    new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
    new Address(loadedConfig.soroswap_router).toScVal(), // router_address: Address,
    nativeToScVal(loadedConfig.end_timestamp, { type: "u64" }) , // end_timestamp: u64
  ];

  await invokeContract(
    contract_name,
    addressBook,
    'initialize',
    timeloclkInitParams,
    loadedConfig.admin
  );

  addressBook.setContractId('admin', loadedConfig.admin.publicKey());
  addressBook.setContractId('soroswap_router', loadedConfig.soroswap_router);
  addressBook.setContractId('end_timestamp', String(loadedConfig.end_timestamp));
  addressBook.writeToFile();


  console.log("Setup complete")
  }
  
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);
const contracts_to_deploy = process.argv.slice(3)

try {
  await deployContracts(addressBook, contracts_to_deploy);
}
catch (e) {
  console.error(e)
}
addressBook.writeToFile();
