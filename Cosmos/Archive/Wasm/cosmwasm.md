### CosmWasm

 - CosmWasm is a smart contract framework built to make appchain
   development and maintenance a breeze.
   
 - Provides the power and performance of native Cosmos SDK modules with 
   a simpler developer experience

 - Unit tests, integration tests, and full backtraces are first-class   
   citizens in CosmWasm
 - Smart contract upgrades are dead simple with CosmWasm's built-in   
   permissioned, per-contract migrate function

 - Rust language performance and safety guarantees fast and memory-safe 
   dapps

## What is CosmWasm?

CosmWasm is a smart contracting platform built for the Cosmos ecosystem. Simply put, it's the Cosmos (Cosm) way of using WebAssembly (Wasm) hence the name.

CosmWasm is written as a module that can plug into the Cosmos SDK. This means that anyone currently building a blockchain using the Cosmos SDK can quickly and easily add CosmWasm smart contracting support to their chain, without adjusting existing logic.

[Rust](https://www.rust-lang.org/) is currently the most used programming language for CosmWasm, in the future, it is possible to have different programming languages like [AssemblyScript](https://www.assemblyscript.org/)


## How to use CosmWasm

As CosmWasm is another Cosmos SDK module, a binary is enough to start integrating it into your blockchain.

A sample binary of CosmWasm integrated into the  `gaiad`  binary, called  `wasmd`  is provided and can be found  [here](https://github.com/CosmWasm/wasmd). Using wasmd it is possible to launch a new smart-contract enabled blockchain out of the box, using documented and tested tooling and the same security model as the Cosmos Hub.

A running blockchain is needed to host and interact with the contracts. It can be either localhost, testnet, or a mainnet blockchain.

The details on how to  [connect to a testnet](https://docs.cosmwasm.com/docs/getting-started/setting-env#setting-up-environment)  or  [set up a local devnet](https://docs.cosmwasm.com/docs/getting-started/setting-env#run-local-node-optional)  will be explained in the later sections.

[Stewarding The Interchain Ecosystem - Interchain Foundation](https://interchain.io/)



## wasmd

`wasmd`  is the backbone of the CosmWasm platform. It is the implementation of a Cosmos zone with wasm smart contracts enabled.

The code was forked from the  `cosmos/gaia`  repository as a basis, then x/wasm was added and many gaia-specific files were cleaned up. However, the wasmd binary should function just like gaiad except for the addition of the x/wasm module. As such,  `wasmd`  have all the same features (plus WASM smart contracts obviously). If you'd like to learn more about accessing those features take a look at the  [Gaia docs](https://github.com/cosmos/gaia/tree/main/docs/hub-tutorials).


[Setting up Environment | CosmWasm Documentation](https://docs.cosmwasm.com/docs/getting-started/setting-env)

## Setting up the CosmJS CLI client[​](https://docs.cosmwasm.com/docs/getting-started/setting-env#setting-up-the-cosmjs-cli-client "Direct link to heading")

Beyond the standard CLI tooling,  [CosmJS](https://github.com/CosmWasm/cosmjs)  was developed as a flexible TypeScript library, which runs in Node.js as well as in modern browsers. Among other capabilities, the library supports smart contract queries and transactions. Along with this library, the  [@cosmjs/cli](https://www.npmjs.com/package/@cosmjs/cli)  was developed to act as a super-charged Node console. It supports the keyword  `await`, does type checking for helpful error messages, and preloads many CosmJS utilities. If you are comfortable with the Node console, you will probably find CosmJS CLI easier and more powerful than the wasmd Go CLI tooling.



### Source
This is a very simple example for the name service contract we developed, but it should show you what is possible, limited only by the wasm code you upload and the json messages you send. The next step is the  [Hijack Escrow tutorial](https://docs.cosmwasm.com/tutorials/hijack-escrow/intro)  where you will edit a smart contract to put a backdoor that enables a thief to steal funds.

-   [Videos and Workshops](https://docs.cosmwasm.com/tutorials/videos-workshops): We curated some video and workshop resources you can take a look at.
-   [Learn](https://docs.cosmwasm.com/tutorials/simple-option/intro)  will demonstrate developing smart contracts from zero to production with step by step explanations, code snippets, scripts and more.
    -   [Dev Academy](https://docs.cosmwasm.com/dev-academy/intro)  provides structured learning content for CosmWasm smart contracts and clients.
-   [Terra Academy](https://academy.terra.money/courses/cosmwasm-smart-contracts-i): is a great tutorial series apart from here.
-   [cw-awesome](https://github.com/InterWasm/cw-awesome): Curated CosmWasm resources.
-   [cw-template](https://github.com/CosmWasm/cw-template): CosmWasm starter project. Do not clone the repo, but rather follow the  [README](https://github.com/CosmWasm/cosmwasm-template/blob/master/README.md)  on how to use  `cargo-generate`  to generate your skeleton.
-   [cw-plus](https://github.com/CosmWasm/cw-plus): Production grade, ready to secure billions, smart contracts. Maintained and developed actively by  [Confio](https://confio.gmbh/). Community made high quality smart contracts are hosted here.
-   [cw-contracts](https://github.com/InterWasm/cw-contracts): Community made smart contracts. Contributions are welcome.
-   [InterWasm DAO](https://github.com/InterWasm/DAO): DAO for CosmWasm. If you have a good project for the community and require funds or help, create an  [IWP](https://github.com/InterWasm/DAO#interwasm-proposalsiwps).

