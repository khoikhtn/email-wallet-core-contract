# Xion Email wallet contracts

This project contains the contracts for the [xion-email-wallet](https://github.com/hduoc2003/xion-email-wallet).

## Install Xion CLI

Follow this [guide](https://docs.burnt.com/xion/developers/featured-guides/setup-local-environment/interact-with-xion-chain-setup-xion-daemon)

## Deploy contracts

### Use Pre-Deployed contracts

You can use the pre-deployed contracts provided in this project. Each contract address is listed in the `build/$contract/env.json` file.

### Redeploy Contracts

To redeploy the contracts, follow these steps:

#### Optimize the Wasm Binary
Use the CosmWasm Rust Optimizer to optimize the generated Wasm binary file by running the following command:

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.15.0
```

#### Update Configuration

Modify the [config.json](script/config.json) file with your Xion account address and the output path for the build files:

```json
{
  "wallet-address": "your xion account address",
  "output-path": "../build"
}
```

#### Deploy the Contracts
   
Navigate to the script directory and run the deployment script:
```bash
cd script
python store_contracts.py
```

All deployment data will be saved in the `build` folder.
