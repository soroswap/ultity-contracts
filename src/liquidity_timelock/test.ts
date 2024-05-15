import { Address, nativeToScVal } from '@stellar/stellar-sdk';
import { AddressBook } from '../../utils/address_book.js';
import { invokeCustomContract } from '../../utils/contract.js';
import { config } from '../../utils/env_config.js';
import { getCurrentTimePlusOneHour } from '../../utils/tx.js';

export async function liquidityTimelock(addressBook: AddressBook, contractKey: string) {
  let account = await loadedConfig.horizonRpc.loadAccount(loadedConfig.admin.publicKey())
  let balance = account.balances[0].balance
  console.log('Current Admin account balance:', balance);
  console.log('-------------------------------------------------------');
  console.log('Initialize Liquidity Timelock Contract');
  console.log('-------------------------------------------------------');
  try {
    const initParams = [
      new Address(loadedConfig.admin.publicKey()).toScVal(),
      new Address("CDGHOS7DDZ7DB24J7TMFDEAIR7LS7GLMT5J5KEZMUF6MSX5BFHCXQIB3").toScVal(),
      nativeToScVal(1715794839, {type: "u64"})
    ]
    const result = await invokeCustomContract(addressBook.getContractId(contractKey), "initialize", initParams, loadedConfig.admin)
    console.log('🚀 « result:', result);
  } catch (error) {
    console.log('🚀 « error:', error);
  }

  console.log('-------------------------------------------------------');
  console.log('Testing Liquidity Contract');
  console.log('-------------------------------------------------------');
  try {
    const addLiquidityParams = [
      new Address("CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC").toScVal(), //TOKEN_0 Address
      new Address("CCC6BBR7WFW72VJULVQUY2QYSBFWFOBMFVZFGPSF4DVMM2LMME63QEPE").toScVal(), //TOKEN_1 Address
      nativeToScVal(1000000000, {type: "i128"}), //Amount_0 
      nativeToScVal(102710000000, {type: "i128"}), //Amount_1 
      new Address(loadedConfig.admin.publicKey()).toScVal(), //from account address
      nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }), //deadline
    ]

    const result = await invokeCustomContract(addressBook.getContractId(contractKey), "add_liquidity", addLiquidityParams, loadedConfig.admin)
    console.log('🚀 « result:', result);
  } catch (error) {
    console.log('🚀 « error:', error);
  }
}

const network = process.argv[2];
const contractKey = process.argv[3];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network,loadedConfig);

try {
  await liquidityTimelock(addressBook, contractKey);
}
catch (e) {
  console.error(e)
}
