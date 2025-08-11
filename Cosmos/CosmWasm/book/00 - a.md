Wasm rust compiler backend installed to build Wasm binaries. To install it, run:

```
`rustup target add wasm32-unknown-unknown`
```


if you want to try out your contracts on a testnet, you will need a [wasmd](https://github.com/CosmWasm/wasmd) binary.



[cosmwasm-check utility]
----------------------------------------------------------------------------------------------

An additional helpful tool for building smart contracts is the `cosmwasm-check`[utility](https://github.com/CosmWasm/cosmwasm/tree/main/packages/check). It allows you to check if the wasm binary is a proper smart contract ready to upload into the blockchain. You can install it using cargo:

```

`$ cargo install cosmwasm-check
`
```

If the installation succeeds, you should be able to execute the utility from your command line.

```

`$ cosmwasm-check --version
Contract checking 1.2.3`
```

[Verifying the installation]
------------------------------------------------------------------------------------------------------

To guarantee you are ready to build your smart contracts, you need to make sure you can build examples. Checkout the [cw-plus](https://github.com/CosmWasm/cw-plus) repository and run the testing command in its folder:

```

`$ git clone git@github.com:CosmWasm/cw-plus.git
$ cd ./cw-plus
cw-plus $ cargo test`
```