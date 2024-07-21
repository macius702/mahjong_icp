#!/usr/bin/env bash

set -ex
CANISTERS="$1"

function generate_did() {
  local canister=$1
  canister_root="src/$canister"

  echo "Building the $canister canister..."
  cargo build --manifest-path="$canister_root/Cargo.toml" \
      --target wasm32-unknown-unknown \
      --release --package "$canister" 

  echo "Extracting the Candid interface description from the $canister canister..."
  candid-extractor "target/wasm32-unknown-unknown/release/$canister.wasm" > "$canister_root/$canister.did" 

  echo "Adding the Candid interface description to the $canister canister..."
  ic-wasm "target/wasm32-unknown-unknown/release/$canister.wasm" \
      -o "target/wasm32-unknown-unknown/release/$canister.wasm" \
      metadata candid:service -v public -f "$canister_root/$canister.did"
}

# The list of canisters of your project
# Those shoudl use ic_cdk >= v0.11.0
#

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    echo "Generating the Candid interface description for the $canister canister..."
    generate_did "$canister"
done

echo "Generating the dfx.json file..."
dfx generate