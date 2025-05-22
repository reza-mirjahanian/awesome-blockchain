Development on Solana can be broken down into two main parts:

1.  **Onchain Program Development**: This is where you create and deploy custom programs directly to the blockchain. Once deployed, anyone who knows how to communicate with them can use them. You can write these programs in Rust, C, or C++. Rust has the most support for onchain program development today.
2.  **Client Development**: This is where you write software (called decentralized applications, or dApps) that communicates with onchain programs. Your apps can submit transactions to perform actions onchain. Client development can be written in any programming language.

The "glue" between the client side and the onchain side is the [Solana JSON RPC API](https://solana.com/docs/rpc). The client-side sends RPC requests to the Solana network to interact with onchain programs. This is very similar to normal development between a frontend and backend. The major difference with working on Solana is that the backend is a global permissionless blockchain. This means that anyone can interact with your onchain program without the need of issuing API keys or any other form of permission

--------

Solana development is a bit different from other blockchains because of its highly composable onchain programs. This means you can build on top of any program already deployed, and often you can do so without needing to do any custom onchain program development. For example, if you wanted to work with tokens, you could use the [Token Program](https://solana.com/docs/core/tokens) that is already deployed on the network. All development on your application would be client-side in your language of choice.

Developers looking to build on Solana will find that the development stack is very similar to any other development stack. The main difference is that you'll be working with a blockchain and have to think about how users potentially interact with your application onchain instead of just on the frontend. Developing on Solana still has CI/CD pipelines, testing, debugging tools, a frontend and backend, and anything you'd find in a normal development flow.

---------
o run a local validator after installing the Solana CLI, run the following command:
```
solana-test-validator
```


When building onchain programs, you have a choice to either build with native Rust (ie, without a framework) or use the Anchor framework. Anchor is a framework that makes it easier to build on Solana by providing a higher-level API for developers. 

------------

### Test

You'll need a way to test your program. There are a few different ways to test your program based on your language preference:

-   [solana-program-test](https://docs.rs/solana-program-test/latest/solana_program_test/) \- Testing framework built in Rust
-   [solana-bankrun](https://kevinheavey.github.io/solana-bankrun/) \- Testing framework built for writing Typescript tests
-   [bankrun](https://kevinheavey.github.io/solders/tutorials/bankrun.html) \- Testing framework built for writing Python tests

If you do not want to develop your programs locally, there's also the [online IDE Solana Playground](https://beta.solpg.io/). Solana Playground allows you to write, test, and deploy programs on Solana. You can get started with Solana Playground by [following our quick start guide](https://solana.com/docs/intro/quick-start).