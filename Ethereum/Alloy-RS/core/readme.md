# Alloy

Core libraries at the root of the Rust Ethereum ecosystem.

Alloy is a rewrite of [`ethers-rs`] from the ground up, with exciting new
features, high performance, and excellent docs.

We have a [book](https://alloy.rs) on all things Alloy and many [examples](https://github.com/alloy-rs/examples) to help you get started.

[`ethers-rs`] has been deprecated, and users are recommended to migrate to Alloy.

[`ethers-rs`]: https://github.com/gakonst/ethers-rs



## Overview

This repository contains the following crates:

- [`alloy-core`]: Meta-crate for the entire project
- [`alloy-primitives`] - Primitive integer and byte types
- [`alloy-sol-types`] - Compile-time [ABI] and [EIP-712] implementations
- [`alloy-sol-macro`] - The [`sol!`] procedural macro
- [`alloy-dyn-abi`] - Run-time [ABI] and [EIP-712] implementations
- [`alloy-json-abi`] - Full Ethereum [JSON-ABI] implementation
- [`alloy-sol-type-parser`] - A simple parser for Solidity type strings
- [`syn-solidity`] - [`syn`]-powered Solidity parser

[`alloy-core`]: https://github.com/alloy-rs/core/tree/main/crates/core
[`alloy-primitives`]: https://github.com/alloy-rs/core/tree/main/crates/primitives
[`alloy-sol-types`]: https://github.com/alloy-rs/core/tree/main/crates/sol-types
[`alloy-sol-macro`]: https://github.com/alloy-rs/core/tree/main/crates/sol-macro
[`alloy-dyn-abi`]: https://github.com/alloy-rs/core/tree/main/crates/dyn-abi
[`alloy-json-abi`]: https://github.com/alloy-rs/core/tree/main/crates/json-abi
[`alloy-sol-type-parser`]: https://github.com/alloy-rs/core/tree/main/crates/sol-type-parser
[`syn-solidity`]: https://github.com/alloy-rs/core/tree/main/crates/syn-solidity
[JSON-ABI]: https://docs.soliditylang.org/en/latest/abi-spec.html#json
[ABI]: https://docs.soliditylang.org/en/latest/abi-spec.html
[EIP-712]: https://eips.ethereum.org/EIPS/eip-712
[`sol!`]: https://docs.rs/alloy-sol-macro/latest/alloy_sol_macro/macro.sol.html
[`syn`]: https://github.com/dtolnay/syn

## Supported Rust Versions (MSRV)

<!--
When updating this, also update:
- clippy.toml
- Cargo.toml
- .github/workflows/ci.yml
-->

The current MSRV (minimum supported rust version) is 1.85.



## WASM support

We provide full support for all the `wasm*-*` targets. If a crate does not
build on a WASM target, please [open an issue].

When building for the `wasm32-unknown-unknown` target and the `"getrandom"`
feature is enabled, compilation for the `getrandom` crate will fail. This is
expected: see [their documentation][getrandom] for more details.

To fix this, either disable the `"getrandom"` feature on `alloy-core` or add
`getrandom` to your dependencies with the `"wasm_js"` feature enabled:

```toml
getrandom = { version = "0.3", features = ["wasm_js"] }
```



### A no_std Rust Environment

The term Embedded Programming is used for a wide range of different classes of programming. Ranging from programming 8-Bit MCUs (like the ST72325xx) with just a few KB of RAM and ROM, up to systems like the Raspberry Pi (Model B 3+) which has a 32/64-bit 4-core Cortex-A53 @ 1.4 GHz and 1GB of RAM. Different restrictions/limitations will apply when writing code depending on what kind of target and use case you have.

[There are two general Embedded Programming classifications:](https://docs.rust-embedded.org/book/intro/no-std.html)

## Credits

None of these crates would have been possible without the great work done in:

- [`ethers.js`](https://github.com/ethers-io/ethers.js/)
- [`rust-web3`](https://github.com/tomusdrw/rust-web3/)
- [`ruint`](https://github.com/recmo/uint)
- [`ethabi`](https://github.com/rust-ethereum/ethabi)
- [`ethcontract-rs`](https://github.com/gnosis/ethcontract-rs/)
- [`guac_rs`](https://github.com/althea-net/guac_rs/)
