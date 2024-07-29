# Project Description

This project was created for the Internet Computer RUSH Bootcamp. The main focus of this project is on integration rather than innovation. The frontend of the Mahjong game is implemented in Dart/Flutter, while the backend is developed in Rust.

## Why Mahjong?

The motivation behind choosing Mahjong was the lack of its implementation in the ICP ecosystem. A pretty mature, MIT licensed implementation in Dart/Flutter was found and utilized.

## Why Flutter?

Flutter allows for the development of native applications for Android, iOS, Web, Linux, MacOS, and Windows from a single codebase, which is nice.

## Discoveries While Hacking

- **agent_dart**: This was used in the project, but it didn't work for the web due to a bug in dart2js. A tweak was made in the agent_dart fork to address this. Dart has recently released a wasm target, but it had the same bug as dart2js. However, it looks promising.
- **Internet Identity**: An attempt was made to use Internet Identity, but it seems like it's not fully implemented in agent_dart as it didn't work as expected.

## Live Demo

You can check out the live demo of the project [here](https://h6wxg-niaaa-aaaam-ac4dq-cai.icp0.io/).

For Android users, an APK file is available for download. Please note that this file is hosted on an external server, and as such, you should only download and install it if you trust the source. 

[Download APK](https://h6wxg-niaaa-aaaam-ac4dq-cai.icp0.io/assets/build/app/outputs/flutter-apk/app-debug.apk)

## To build

```
git clone git@github.com:macius702/mahjong_icp.git
cd mahjong_icp
git submodule update --init --recursive
scripts/build.sh
scripts/build.sh mainnet
```

