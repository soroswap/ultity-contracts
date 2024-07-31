import { Keypair, SorobanRpc } from '@stellar/stellar-sdk';
import dotenv from "dotenv";
import * as fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
dotenv.config({ path: path.join(__dirname, "../../.env") });

interface NetworkConfig {
  network: string;
  friendbot_url: string;
  soroban_rpc_url: string;
  soroban_network_passphrase: string;
}

interface Config {
  previewHash: string;
  quickstartHash: string;
  networkConfig: NetworkConfig[];
}

class EnvConfig {
  rpc: SorobanRpc.Server;
  passphrase: string;
  friendbot: string | undefined;
  admin: Keypair;

  constructor(
    rpc: SorobanRpc.Server,
    passphrase: string,
    friendbot: string | undefined,
    admin: Keypair,
  ) {
    this.rpc = rpc;
    this.passphrase = passphrase;
    this.friendbot = friendbot;
    this.admin = admin;
  }

  /**
   * Load the environment config from the .env file
   * @returns Environment config
   */
  static loadFromFile(network: string): EnvConfig {
    const fileContents = fs.readFileSync(
      path.join(__dirname, "../../configs.json"),
      "utf8",
    );
    const configs: Config = JSON.parse(fileContents);

    let rpc_url, horizon_rpc_url, friendbot_url, passphrase;
    
    const networkConfig = configs.networkConfig.find((config) => config.network === network);
    if (!networkConfig) {
      throw new Error(`Network configuration for '${network}' not found`);
    }

    if (network === 'mainnet') {
      passphrase = networkConfig.soroban_network_passphrase;
      rpc_url = process.env.MAINNET_RPC_URL;
      friendbot_url = undefined;
    } else {
      rpc_url = networkConfig.soroban_rpc_url;
      friendbot_url = networkConfig.friendbot_url;
      passphrase = networkConfig.soroban_network_passphrase;
    }

    const admin = process.env.ADMIN_SECRET_KEY;

    if (
      rpc_url === undefined ||
      (network != "mainnet" && friendbot_url === undefined) ||
      passphrase === undefined ||
      admin === undefined
    ) {
      console.log("🚀 ~ EnvConfig ~ loadFromFile ~ admin:", admin)
      console.log("🚀 ~ EnvConfig ~ loadFromFile ~ passphrase:", passphrase)
      console.log("🚀 ~ EnvConfig ~ loadFromFile ~ rpc_url:", rpc_url)
      console.log("🚀 ~ EnvConfig ~ loadFromFile ~ network:", network)
      console.log("🚀 ~ EnvConfig ~ loadFromFile ~ friendbot_url:", friendbot_url)
      
      throw new Error('Error: Configuration is missing required fields, include <network>');
    }

    const allowHttp = network === "standalone";

    return new EnvConfig(
      new SorobanRpc.Server(rpc_url, { allowHttp }),
      passphrase,
      friendbot_url,
      Keypair.fromSecret(admin),
    );
  }

  /**
   * Get the Keypair for a user from the env file
   * @param userKey - The name of the user in the env file
   * @returns Keypair for the user
   */
  getUser(userKey: string): Keypair {
    const userSecretKey = process.env[userKey];
    if (userSecretKey === undefined) {
      throw new Error(`${userKey} secret key not found in .env`);
    }
    try {
      return Keypair.fromSecret(userSecretKey);
    }
    catch (e) {
      throw new Error(`${userKey} secret key
        might not be found in .env. Failed with error ${e}`);
    }
  }
}

export const config = (network: string) => {
  return EnvConfig.loadFromFile(network);
};
