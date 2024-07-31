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

  // now show user LP balance
  let lpUserBalance = await invokeCustomContract(
    xlm_usdc_lp_address,
    "balance",
    [new Address(loadedConfig.admin.publicKey()).toScVal()],
    loadedConfig.admin,
    true
  );
  let lpUserBalanceBigInt = scValToBigInt(lpUserBalance.result.retval);
  console.log("🚀 ~ lpUserBalanceBigInt.toString():", lpUserBalanceBigInt.toString())

  console.log("-------------------------------------------------------");
  console.log("Removing XLM-USDC Liquidity Using the Soroswap Liquidity TimelockContract");
  console.log("-------------------------------------------------------");
  try {
    
    console.log("First we will claim the LP tokens from the contract");
    // fn claim(e: Env, pair_address: Address) -> Result<(), CombinedLiquidityTimelockError> {
  
    const claimParams = [new Address(xlm_usdc_lp_address).toScVal()];

    const result = await invokeCustomContract(
      addressBook.getContractId(contractKey),
      "claim",
      claimParams,
      loadedConfig.admin
    );
    console.log("🚀 Done claiming! :)");

    let newLPContractBalance = await invokeCustomContract(
      xlm_usdc_lp_address,
      "balance",
      [new Address(addressBook.getContractId(contractKey)).toScVal()],
      loadedConfig.admin,
      true
    );
    let newLPContractBalanceBigInt = scValToBigInt(newLPContractBalance.result.retval);
    console.log("🚀 ~ newLPContractBalanceBigInt.toString():", newLPContractBalanceBigInt.toString())

    // new user LP balance
    let newLPUserBalance = await invokeCustomContract(
      xlm_usdc_lp_address,
      "balance",
      [new Address(loadedConfig.admin.publicKey()).toScVal()],
      loadedConfig.admin,
      true
    );
    let newLPUserBalanceBigInt = scValToBigInt(newLPUserBalance.result.retval);
    console.log("🚀 ~ newLPUserBalanceBigInt.toString():", newLPUserBalanceBigInt.toString())


    console.log("Now we will remove the liquidity from the contract");
   
  //   fn remove_liquidity(
  //     e: Env,
  //     token_a: Address,
  //     token_b: Address,
  //     liquidity: i128,
  //     amount_a_min: i128,
  //     amount_b_min: i128,
  //     to: Address,
  //     deadline: u64,
  // ) -> Result<(i128, i128), CombinedRouterError>;

    const removeLiquidityParams = [
      new Address(
        loadedConfig.xlm_address
      ).toScVal(), //     token_a: Address,
      new Address(
        loadedConfig.usdc_address
      ).toScVal(),   //     token_b: Address,
      nativeToScVal(newLPUserBalanceBigInt, { type: "i128" }), //     liquidity: i128,
      nativeToScVal(0, { type: "i128" }), //     amount_a_min: i128,
      nativeToScVal(0, { type: "i128" }), //     amount_b_min: i128,
      new Address(loadedConfig.admin.publicKey()).toScVal(), //to account address
      nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }), //deadline
    ];

    const result2 = await invokeCustomContract(
      loadedConfig.soroswap_router,
      "remove_liquidity",
      removeLiquidityParams,
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
