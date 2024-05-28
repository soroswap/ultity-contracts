import { Address, nativeToScVal, scValToNative } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { invokeCustomContract } from "../../utils/contract.js";
import { config } from "../../utils/env_config.js";
import { getCurrentTimePlusOneHour } from "../../utils/tx.js";

export async function liquidityTimelock(
  addressBook: AddressBook,
  contractKey: string
) {
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  let balance = account.balances[0].balance;
  console.log("Current Admin account balance:", balance);
  console.log("-------------------------------------------------------");
  console.log("Initialize Liquidity Timelock Contract");
  console.log("-------------------------------------------------------");
  try {
    const initParams = [
      new Address(loadedConfig.admin.publicKey()).toScVal(),
      new Address(
        "CB74KXQXEGKGPU5C5FI22X64AGQ63NANVLRZBS22SSCMLJDXNHED72MO"
      ).toScVal(),
      nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
    ];
    const result = await invokeCustomContract(
      addressBook.getContractId(contractKey),
      "initialize",
      initParams,
      loadedConfig.admin
    );
    console.log("🚀 « result:", result);
  } catch (error) {
    console.log("Already initialized:");
  }

  console.log("-------------------------------------------------------");
  console.log("Starting Balances");
  console.log("-------------------------------------------------------");
  let xtarUserBalance = await invokeCustomContract(
    "CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO",
    "balance",
    [new Address(loadedConfig.admin.publicKey()).toScVal()],
    loadedConfig.admin,
    true
  );
  console.log(
    "XTAR USER BALANCE:",
    scValToNative(xtarUserBalance.result.retval)
  );
  let xtarContractBalance = await invokeCustomContract(
    "CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO",
    "balance",
    [new Address(addressBook.getContractId(contractKey)).toScVal()],
    loadedConfig.admin,
    true
  );
  console.log(
    "XTAR CONTRACT BALANCE:",
    scValToNative(xtarContractBalance.result.retval)
  );
  let usdcUserBalance = await invokeCustomContract(
    "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA",
    "balance",
    [new Address(loadedConfig.admin.publicKey()).toScVal()],
    loadedConfig.admin,
    true
  );
  console.log(
    "USDC USER BALANCE:",
    scValToNative(usdcUserBalance.result.retval)
  );
  let usdcContractBalance = await invokeCustomContract(
    "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA",
    "balance",
    [new Address(addressBook.getContractId(contractKey)).toScVal()],
    loadedConfig.admin,
    true
  );
  console.log(
    "USDC CONTRACT BALANCE:",
    scValToNative(usdcContractBalance.result.retval)
  );
  let lpUserBalance = await invokeCustomContract(
    "CAEHRFL6HROIYFQK62ATJVRYEMLATWXVUYOBWCR5T7MFJB23BK56QD5J",
    "balance",
    [new Address(loadedConfig.admin.publicKey()).toScVal()],
    loadedConfig.admin,
    true
  );
  console.log("LP USER BALANCE:", scValToNative(lpUserBalance.result.retval));
  let lpContractBalance = await invokeCustomContract(
    "CAEHRFL6HROIYFQK62ATJVRYEMLATWXVUYOBWCR5T7MFJB23BK56QD5J",
    "balance",
    [new Address(addressBook.getContractId(contractKey)).toScVal()],
    loadedConfig.admin,
    true
  );
  console.log(
    "LP CONTRACT BALANCE:",
    scValToNative(lpContractBalance.result.retval)
  );

  console.log("-------------------------------------------------------");
  console.log("Testing Liquidity Contract");
  console.log("-------------------------------------------------------");
  try {
    const addLiquidityParams = [
      new Address(
        "CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO"
      ).toScVal(), //TOKEN_0 Address
      new Address(
        "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA"
      ).toScVal(), //TOKEN_1 Address
      nativeToScVal(10000000000, { type: "i128" }), //Amount_0
      nativeToScVal(6243764045, { type: "i128" }), //Amount_1
      nativeToScVal(0, { type: "i128" }), //Amount_0
      nativeToScVal(0, { type: "i128" }), //Amount_1
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

  // console.log('-------------------------------------------------------');
  // console.log('Testing Claim Liquidity Contract');
  // console.log('-------------------------------------------------------');
  // try {
  //   const claimParams = [
  //     new Address("CAEHRFL6HROIYFQK62ATJVRYEMLATWXVUYOBWCR5T7MFJB23BK56QD5J").toScVal(), //THIS IS THE PAIR ADDRESS
  //   ]

  //   const result = await invokeCustomContract(addressBook.getContractId(contractKey), "claim", claimParams, loadedConfig.admin)
  //   console.log('🚀 « result:', result);
  // } catch (error) {
  //   console.log('🚀 « error:', error);
  // }

  // console.log('-------------------------------------------------------');
  // console.log('ending Balances');
  // console.log('-------------------------------------------------------');
  // xtarUserBalance = await invokeCustomContract("CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO", "balance", [new Address(loadedConfig.admin.publicKey()).toScVal()], loadedConfig.admin, true)
  // console.log('XTAR USER BALANCE:', scValToNative(xtarUserBalance.result.retval));
  // xtarContractBalance = await invokeCustomContract("CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO", "balance", [new Address(addressBook.getContractId(contractKey)).toScVal()], loadedConfig.admin, true)
  // console.log('XTAR CONTRACT BALANCE:', scValToNative(xtarContractBalance.result.retval));
  // usdcUserBalance = await invokeCustomContract("CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA", "balance", [new Address(loadedConfig.admin.publicKey()).toScVal()], loadedConfig.admin, true)
  // console.log('USDC USER BALANCE:', scValToNative(usdcUserBalance.result.retval));
  // usdcContractBalance = await invokeCustomContract("CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA", "balance", [new Address(addressBook.getContractId(contractKey)).toScVal()], loadedConfig.admin, true)
  // console.log('USDC CONTRACT BALANCE:', scValToNative(usdcContractBalance.result.retval));
  // lpUserBalance = await invokeCustomContract("CAEHRFL6HROIYFQK62ATJVRYEMLATWXVUYOBWCR5T7MFJB23BK56QD5J", "balance", [new Address(loadedConfig.admin.publicKey()).toScVal()], loadedConfig.admin, true)
  // console.log('LP USER BALANCE:', scValToNative(lpUserBalance.result.retval));
  // lpContractBalance = await invokeCustomContract("CAEHRFL6HROIYFQK62ATJVRYEMLATWXVUYOBWCR5T7MFJB23BK56QD5J", "balance", [new Address(addressBook.getContractId(contractKey)).toScVal()], loadedConfig.admin, true)
  // console.log('LP CONTRACT BALANCE:', scValToNative(lpContractBalance.result.retval));
}

const network = process.argv[2];
const contractKey = process.argv[3];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network, loadedConfig);

try {
  await liquidityTimelock(addressBook, contractKey);
} catch (e) {
  console.error(e);
}
