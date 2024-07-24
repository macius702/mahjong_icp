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
else
    echo "Invalid mode. Please provide the mode as the first parameter: local, playground or mainnet"
    exit 1
fi

dart format $ROOT_DIRECTORY/src/mahjong_icp_frontend/lib/engine/*.dart
dart format $ROOT_DIRECTORY/src/mahjong_icp_frontend/lib/screens/*.dart
dart format $ROOT_DIRECTORY/src/mahjong_icp_frontend/lib/widgets/*.dart
# dart format --line-length 120 src/mahjong_icp_frontend/lib/*.dart

(cd $ROOT_DIRECTORY/src && cargo fmt)

# dfx stop
dfx start --clean --background &
# dfx start --background &
# flutter clean
# flutter pub get

# Needed for flutter build web
echo "Running canister create with parameter: $deploy_param"
dfx canister create mahjong_icp_backend $deploy_param
dfx canister create mahjong_icp_frontend $deploy_param

echo "Running dart generate_config.dart with parameter: $mode"
dart $ROOT_DIRECTORY/scripts/generate_config.dart $mode

pushd $ROOT_DIRECTORY/src/mahjong_icp_frontend
    flutter build web --profile --dart-define=Dart2jsOptimization=O0 --source-maps
    sed -i 's|<base href="/ED-Mahjong/">|<base href="">|g' build/web/index.html
popd
dfx build || true
dfx build



dfx deploy -v $deploy_param

flutter devices

if [ "$mode" == "playground" ]
then
    source web_front_end.sh
    xdg-open https://$FRONTEND_CANISTER_ID.ic0.app &
    # pushd $ROOT_DIRECTORY/src/mahjong_icp_frontend
    #     flutter run --release -d emulator-5554 &
    # popd
elif [ "$mode" == "local" ]
then
    echo
    # flutter run -d chrome
fi

#  dfx cycles --network ic balance
# 0.659 TC (trillion cycles).
# maciej@:~/mydes2/mahjong/mahjong_icp$ dfx ledger --network=ic balance
# 0.49900000 ICP
# maciej@:~/mydes2/mahjong/mahjong_icp$ dfx cycles convert --amount 0.49 --network ic
# Transfer sent at block height 13041079
# Using transfer at block height 13041079
# Account was topped up with 3_673_040_000_000 cycles! New balance is 4_332_374_440_000 cycles.
# maciej@:~/mydes2/mahjong/mahjong_icp$ dfx cycles --network ic balance
# 4.332 TC (trillion cycles).
