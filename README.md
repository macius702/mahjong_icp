# Project Description

This project was created for the Internet Computer RUSH Bootcamp. The main focus of this project is on integration rather than innovation. The frontend of the Mahjong game is implemented in Dart/Flutter, while the backend is developed in Rust.

## Why Mahjong?

The motivation behind choosing Mahjong was the lack of its implementation in the ICP ecosystem. A mature, GNU licensed implementation in Dart/Flutter was found and utilized.

## Why Flutter?

Flutter allows for the development of native applications for Android, iOS, Web, Linux, MacOS, and Windows from a single codebase, which is nice.

## Discoveries While Hacking

- **agent_dart**: This was used in the project, but it didn't work for the web due to a bug in dart2js. A tweak was made in the agent_dart fork to address this. Dart has recently released a wasm target, but it had the same bug as dart2js. However, it looks promising.
- **Internet Identity**: An attempt was made to use Internet Identity, but it seems like it's not fully implemented in agent_dart as it didn't work as expected.

## Live Demo

You can check out the live demo of the project [here](https://h6wxg-niaaa-aaaam-ac4dq-cai.icp0.io/).