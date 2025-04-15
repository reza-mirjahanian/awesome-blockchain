As smart contracts are Rust library crates, we will start with creating one:

```

`$ cargo new --lib ./empty-contract
`
```

You created a simple Rust library, but it is not yet ready to be a smart contract. The first thing to do is to update the `Cargo.toml` file:

```

`[package]
name = "contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
cosmwasm-std = { version = "1.0.0-beta8", features = ["staking"] }`
```