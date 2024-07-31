import { AddressBook } from './utils/address_book.js';
import { airdropAccount, deployContract, installContract, bumpContractCode} from './utils/contract.js';
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


  //   // pub struct Adapter {
  //   //   pub protocol_id: String,
  //   //   pub address: Address,
  //   //   pub paused: bool,
  // // }

  // const adaptersVec = [
  //   {
  //     protocol_id: "soroswap",
  //     address: new Address(addressBook.getContractId('soroswap_adapter')),
  //     paused: false
  //   },
  // ];

  // const adaptersVecScVal = xdr.ScVal.scvVec(adaptersVec.map((adapter) => {
  //   return xdr.ScVal.scvMap([
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('address'),
  //       val: adapter.address.toScVal(),
  //     }),
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('paused'),
  //       val: nativeToScVal(adapter.paused),
  //     }),
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('protocol_id'),
  //       val: xdr.ScVal.scvString(adapter.protocol_id),
  //     }),
  //   ]);
  // }));


  // // fn initialize(e: Env, admin: Address, adapter_vec: Vec<Adapter>)
  // const aggregatorInitParams: xdr.ScVal[] = [ 
  //   new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
  //   adaptersVecScVal, // adapter_vec: Vec<Adapter>,
  // ];

  // console.log("Initializing Aggregator")
  
  // await invokeContract(
  //   'aggregator',
  //   addressBook,
  //   'initialize',
  //   aggregatorInitParams,
  //   loadedConfig.admin
  // );
  // console.log("Aggregator initialized")
    // await bumpContractCode(contract_name, addressBook, loadedConfig.admin);
    // let contractId = await deployContract(contract_name,contract_name, addressBook, loadedConfig.admin)
    // // await bumpContractInstance(contract_name, addressBook, loadedConfig.admin);
    // console.log(`Contract ID of ${contract_name} is ${contractId}\n\n`)
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
