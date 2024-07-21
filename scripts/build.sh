#! /usr/bin/env bash
# Build and Run Modes:
# 1. local: Deploys to the local replica. Ideal for development and testing.
# 2. playground: Deploys to the playground (which is on the mainnet). This is a free service but is limited to 20-minute sessions.
# 3. mainnet: Deploys to the mainnet. This is the production environment and consumes cycles.


export ROOT_DIRECTORY=~/mydes2/mahjong/mahjong_icp

set -e







mode=${1:-local}

echo "mode=$mode"

if [ -z "$mode" ]
then
    echo "Please provide the mode as the first parameter: local, playground or mainnet"
    exit 1
fi

if [ "$mode" == "playground" ]
then
    echo "Deploying to the playground"
    deploy_param="--playground"
elif [ "$mode" == "local" ]
then
    echo "Deploying to local"
    deploy_param=""
elif [ "$mode" == "mainnet" ]
then
    echo "Deploying to mainnet"
    deploy_param="--network=ic"
    echo "not supported yet"
    exit 1
else
    echo "Invalid mode. Please provide the mode as the first parameter: local, playground or mainnet"
    exit 1
fi

dart format $ROOT_DIRECTORY/src/mahjong_icp_frontend/lib/*.dart
# dart format --line-length 120 src/mahjong_icp_frontend/lib/*.dart

# dfx stop
dfx start --clean --background &
# dfx start --background &
# flutter clean
# flutter pub get

# echo "Running canister create with parameter: $deploy_param"
# dfx canister create d_backend $deploy_param
# dfx canister create d_frontend $deploy_param

echo "Running dart generate_config.dart with parameter: $mode"
dart $ROOT_DIRECTORY/scripts/generate_config.dart $mode

pushd $ROOT_DIRECTORY/src/mahjong_icp_frontend
flutter build web
sed -i 's|<base href="/ED-Mahjong/">|<base href="">|g' build/web/index.html
popd
dfx deploy
