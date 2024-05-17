# Liquidity Timelock contract

using soroswap router

## How to test? 
```bash
bash scripts/quickstart.sh standalone
```

in another terminal
```bash
bash scripts/run.sh
```

first build the contract
```bash
cd liquidity_timelock
make build

# to test run 
make test
```

to test on testnet using the soroswap deployment do this
```bash 
# install dependencies
yarn 

# to deploy the contract
yarn deploy testnet liquidity_timelock

# to test the contract
yarn test testnet liquidity_timelock
```