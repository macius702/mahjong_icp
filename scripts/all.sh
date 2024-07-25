#!/usr/bin/env bash

set -e

scripts/clean.sh
dfx start --clean --background &
sleep 5



# scripts/build.sh || true
scripts/build.sh
# scripts/build.sh playground
# scripts/build.sh mainnet



