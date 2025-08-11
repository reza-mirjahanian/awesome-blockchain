Unlike native applications, which have only a single `main` entry point, smart contracts have a couple corresponding to different message types: `instantiate`, `execute`, `query`, `sudo`, `migrate` and more


Go to your `src/lib.rs` file, and start with an `instantiate` entry point:

```

`use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}
`
```

In fact, `instantiate` is the only entry point required for a smart contract to be valid. It is not very useful in this form, but it is a start. Let's take a closer look at the entry point structure.

First, we start with importing couple of types just for more consistent usage. Then we define our entry point. The `instantiate` takes four arguments:

-   [`deps: DepsMut`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.DepsMut.html) is a utility type for communicating with the outer world - it allows querying and updating the contract state, querying other contracts state, and gives access to an `Api` object with a couple of helper functions for dealing with CW addresses.
-   [`env: Env`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Env.html) is an object representing the blockchains state when executing the message - the chain height and id, current timestamp, and the called contract address.
-   [`info: MessageInfo`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.MessageInfo.html) contains metainformation about the message which triggered an execution - an address that sends the message, and chain native tokens sent with the message.
-   [`msg: Empty`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Empty.html) is the message triggering execution itself - for now, it is `Empty` type that represents `{}` JSON, but the type of this argument can be anything that is deserializable, and we will pass more complex types here in the future.