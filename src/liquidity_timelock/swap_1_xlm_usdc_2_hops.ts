import { Address, nativeToScVal, scValToNative, scValToBigInt } from "@stellar/stellar-sdk";
import { AddressBook } from "../utils/address_book.js";
import { invokeCustomContract, invokeContract} from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { getCurrentTimePlusOneHour } from "../utils/tx.js";
import BigNumber from "bignumber.js";


export async function liquidityTimelock(
  addressBook: AddressBook,
  contractKey: string
) {
  
  console.log("-------------------------------------------------------");
  console.log("Initial Balances");
  console.log("-------------------------------------------------------");
  let xlmUserBalance = await invokeCustomContract(
    loadedConfig.xlm_address,
    "balance",
    [new Address(loadedConfig.admin.publicKey()).toScVal()],
    loadedConfig.admin,
    true
  );
  let xlmUserBalanceBigInt = scValToBigInt(xlmUserBalance.result.retval);
  console.log("🚀 ~ xlmUserBalanceBigInt.toString():", xlmUserBalanceBigInt.toString())
  
  let xlm100BigIng: bigint = BigInt('1000000000');
  console.log("🚀 ~ xlm100BigIng.toString():", xlm100BigIng.toString())
  
  // create a new bigint xlmuserbalance minus xlm100BigINt
  let xlmUserBalanceMinus100 = xlmUserBalanceBigInt - xlm100BigIng;
  console.log("🚀 ~ xlmUserBalanceMinus100.toString():", xlmUserBalanceMinus100.toString())
  

  let usdcUserBalance = await invokeCustomContract(
    loadedConfig.usdc_address,
    "balance",
    [new Address(loadedConfig.admin.publicKey()).toScVal()],
    loadedConfig.admin,
    true
  );
  let usdcUserBalanceBigInt = scValToBigInt(usdcUserBalance.result.retval);
  console.log("🚀 ~ usdcUserBalanceBigInt.toString():", usdcUserBalanceBigInt.toString())

  let xlm_usdc_lp_address = await invokeCustomContract(
    loadedConfig.soroswap_router,
    "router_pair_for",
    [ new Address(loadedConfig.xlm_address).toScVal(),
      new Address(loadedConfig.usdc_address).toScVal(),
    ],
    loadedConfig.admin,
    true
  );
 
  console.log("-------------------------------------------------------");
  console.log("Swap 1 XLM to hops");
  console.log("-------------------------------------------------------");
  try {
    
  //   fn swap_exact_tokens_for_tokens(
  //     e: Env,
  //     amount_in: i128,
  //     amount_out_min: i128,
  //     path: Vec<Address>,
  //     to: Address,
  //     deadline: u64,
  // ) -> Result<Vec<i128>, CombinedRouterError>;


  const path = [
    new Address("CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"), 
    new Address("CDPU5TPNUMZ5JY3AUSENSINOEB324WI65AHI7PJBUKR3DJP2ULCBWQCS"),
    new Address("CCGCRYUTDRP52NOPS35FL7XIOZKKGQWSP3IYFE6B66KD4YOGJMWVC5PR"),
  ]
  const swapParams = [
    nativeToScVal(10000000, { type: "i128" }),  //     amount_in: i128,
    nativeToScVal(0, { type: "i128" }), //410211972
    nativeToScVal(path, { type: "Vec" }),
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
  ];

    
    const result2 = await invokeCustomContract(
      loadedConfig.soroswap_router,
      "swap_exact_tokens_for_tokens",
      swapParams,
      loadedConfig.admin
    );
    console.log("🚀 Done! :), low lets check new token balances");

    // check new token balances
    let newXlmUserBalance = await invokeCustomContract(
      loadedConfig.xlm_address,
      "balance",
      [new Address(loadedConfig.admin.publicKey()).toScVal()],
      loadedConfig.admin,
      true
    );
    let newXlmUserBalanceBigInt = scValToBigInt(newXlmUserBalance.result.retval);
    console.log("🚀 ~ newXlmUserBalanceBigInt.toString():", newXlmUserBalanceBigInt.toString())
  
    let newUsdcUserBalance = await invokeCustomContract(
      loadedConfig.usdc_address,
      "balance",
      [new Address(loadedConfig.admin.publicKey()).toScVal()],
      loadedConfig.admin,
      true
    );
    let newUsdcUserBalanceBigInt = scValToBigInt(newUsdcUserBalance.result.retval);
    console.log("🚀 ~ newUsdcUserBalanceBigInt.toString():", newUsdcUserBalanceBigInt.toString())


  } catch (error) {
    console.log("🚀 « error:", error);
  }

  // console.log("-------------------------------------------------------");
  // console.log("Testing Claim Liquidity Contract");
  // console.log("-------------------------------------------------------");
  // try {
  //   const claimParams = [
  //     new Address(
  //       "CAEHRFL6HROIYFQK62ATJVRYEMLATWXVUYOBWCR5T7MFJB23BK56QD5J"
  //     ).toScVal(), //THIS IS THE PAIR ADDRESS
  //   ];

  //   const result = await invokeCustomContract(
  //     addressBook.getContractId(contractKey),
  //     "claim",
  //     claimParams,
  //     loadedConfig.admin
  //   );
  //   console.log("🚀 « result:", result);
  //   console.log("🚀 « result:", scValToNative(result.returnValue));
  // } catch (error) {
  //   console.log("🚀 « error:", error);
  // }

}


const network = process.argv[2];
const contractKey = process.argv[3];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);

try {
  await liquidityTimelock(addressBook, contractKey);
} catch (e) {
  console.error(e);
}
