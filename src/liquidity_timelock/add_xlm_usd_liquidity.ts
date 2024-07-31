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
  xlm_usdc_lp_address = scValToNative(xlm_usdc_lp_address.result.retval); 
  console.log("xlm_usdc_lp_address:", xlm_usdc_lp_address);

  
  let lpContractBalance = await invokeCustomContract(
    xlm_usdc_lp_address,
    "balance",
    [new Address(addressBook.getContractId(contractKey)).toScVal()],
    loadedConfig.admin,
    true
  );
  let lpContractBalanceBigInt = scValToBigInt(lpContractBalance.result.retval);
  console.log("🚀 ~ lpContractBalanceBigInt.toString():", lpContractBalanceBigInt.toString())

  console.log("-------------------------------------------------------");
  console.log("Adding XLM-USDC Liquidity Using the Soroswap Liquidity TimelockContract");
  console.log("-------------------------------------------------------");
  try {
    
  //   fn add_liquidity(
  //     e: Env,
  //     token_a: Address,
  //     token_b: Address,
  //     amount_a: i128,
  //     amount_b: i128,
  //     amount_a_min: i128,
  //     amount_b_min: i128,
  //     from: Address,
  //     deadline: u64,
  // ) -> Result<(i128, i128, i128), CombinedLiquidityTimelockError>;
    const addLiquidityParams = [
      new Address(
        loadedConfig.xlm_address
      ).toScVal(), //     token_a: Address,
      new Address(
        loadedConfig.usdc_address
      ).toScVal(),   //     token_b: Address,
      nativeToScVal(xlmUserBalanceMinus100, { type: "i128" }), //     amount_a: i128,
      nativeToScVal(usdcUserBalanceBigInt, { type: "i128" }), //     amount_b: i128,
      nativeToScVal(0, { type: "i128" }), //     amount_a_min: i128,
      nativeToScVal(0, { type: "i128" }), //     amount_b_min: i128,
      new Address(loadedConfig.admin.publicKey()).toScVal(), //from account address
      nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }), //deadline
    ];

    const result = await invokeCustomContract(
      addressBook.getContractId(contractKey),
      "add_liquidity",
      addLiquidityParams,
      loadedConfig.admin
    );
    console.log("🚀 « result:", result);
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

  // console.log("-------------------------------------------------------");
  // console.log("ending Balances");
  // console.log("-------------------------------------------------------");
  // xlmUserBalance = await invokeCustomContract(
  //   "CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO",
  //   "balance",
  //   [new Address(loadedConfig.admin.publicKey()).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log(
  //   "xlm USER BALANCE:",
  //   scValToNative(xlmUserBalance.result.retval)
  // );
  // xlmContractBalance = await invokeCustomContract(
  //   "CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO",
  //   "balance",
  //   [new Address(addressBook.getContractId(contractKey)).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log(
  //   "xlm CONTRACT BALANCE:",
  //   scValToNative(xlmContractBalance.result.retval)
  // );
  // usdcUserBalance = await invokeCustomContract(
  //   "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA",
  //   "balance",
  //   [new Address(loadedConfig.admin.publicKey()).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log(
  //   "USDC USER BALANCE:",
  //   scValToNative(usdcUserBalance.result.retval)
  // );
  // usdcContractBalance = await invokeCustomContract(
  //   "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA",
  //   "balance",
  //   [new Address(addressBook.getContractId(contractKey)).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log(
  //   "USDC CONTRACT BALANCE:",
  //   scValToNative(usdcContractBalance.result.retval)
  // );
  // lpUserBalance = await invokeCustomContract(
  //   "CAEHRFL6HROIYFQK62ATJVRYEMLATWXVUYOBWCR5T7MFJB23BK56QD5J",
  //   "balance",
  //   [new Address(loadedConfig.admin.publicKey()).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log("LP USER BALANCE:", scValToNative(lpUserBalance.result.retval));
  // lpContractBalance = await invokeCustomContract(
  //   "CAEHRFL6HROIYFQK62ATJVRYEMLATWXVUYOBWCR5T7MFJB23BK56QD5J",
  //   "balance",
  //   [new Address(addressBook.getContractId(contractKey)).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log(
  //   "LP CONTRACT BALANCE:",
  //   scValToNative(lpContractBalance.result.retval)
  // );
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
