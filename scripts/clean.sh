#!/bin/bash



dfx stop
killall replica dfx icx-proxy
dfx stop

items=(".dfx" "target" "src/declarations" "web_front_end.sh" "src/mahjong_icp_frontend/build")

all_clear=true

for item in "${items[@]}"; do
    if [ -d "$item" ]; then
        rm -rf "$item"
        if [ -d "$item" ]; then
            echo "Directory $item was not deleted"
            all_clear=false
        fi
    elif [ -f "$item" ]; then
        rm -f "$item"
        if [ -f "$item" ]; then
            echo "File $item was not deleted"
            all_clear=false
        fi
    fi
done

if $all_clear; then
    echo "All cleared OK"
fi