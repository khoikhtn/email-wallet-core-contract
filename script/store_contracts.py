import json
import os.path
import subprocess
from time import sleep
from typing import Any


def run_command(command: str) -> str:
    print(command)
    print()
    res = subprocess.run(command, capture_output=True, text=True, shell=True)
    if res.stdout == "":
        raise Exception(res.stderr)
    return res.stdout


with open("config.json", "r") as r:
    config = json.loads(r.read())


def save(contract_name: str, file_name: str, text: str):
    path = os.path.join(config['output-path'], contract_name)
    if not os.path.exists(path):
        os.makedirs(path)
    with open(os.path.join(path, file_name), "w") as w:
        w.write(text)


wallet_address = config['wallet-address']
rpc = "https://rpc.xion-testnet-2.burnt.com:443"

def deploy_contract(contract: str, data_init: dict[str, Any]) -> str:
    store_response = run_command(f"xiond tx wasm store ../artifacts/{contract}.wasm \
    --from {wallet_address} \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices 0.001uxion \
    --chain-id xion-testnet-2 \
    --node {rpc} \
    --output json \
    -y")
    save(contract, "store.json", store_response)

    store_response = json.loads(store_response)
    sleep(10)

    tx_hash = store_response['txhash']
    query_tx_cmd = f"xiond q tx {tx_hash} --node {rpc} --output json"
    q_tx = run_command(query_tx_cmd)
    save(contract, "txhash.json", q_tx)

    q_tx = json.loads(q_tx)
    code_id = q_tx['events'][-1]['attributes'][1]['value']
    txhash_init = json.loads(run_command(f"xiond tx wasm instantiate {code_id} \
        '{json.dumps(data_init)}' \
        --admin {wallet_address} \
        --label {contract} \
        --from {wallet_address} \
        --gas auto \
        --gas-adjustment 1.3 \
        --gas-prices 0.001uxion \
        --chain-id xion-testnet-2 \
        --node {rpc} \
        --output json\
        -y"))['txhash']
    sleep(10)

    contract_address = \
    json.loads(run_command(f"xiond query wasm list-contract-by-code {code_id} --output json --node {rpc}"))["contracts"][0]
    print({
        "code_id": code_id,
        "txhash_init": txhash_init,
        "contract_address": contract_address
    })
    save(contract, "env.json", json.dumps({
        "code_id": code_id,
        "txhash_init": txhash_init,
        "contract_address": contract_address
    }, indent=4))
    return contract_address


verifier_addr = deploy_contract("verifier", {})
relayer_handler_addr = deploy_contract("relayer_handler", {})
account_handler_addr = deploy_contract("account_handler", {
    "relayer_handler_addr": relayer_handler_addr,
    "verifier_addr": verifier_addr
})
email_wallet_core_addr = deploy_contract("email_wallet_core", {
    "relayer_handler_addr": relayer_handler_addr,
    "verifier_addr": verifier_addr,
    "account_handler_addr": account_handler_addr
})
