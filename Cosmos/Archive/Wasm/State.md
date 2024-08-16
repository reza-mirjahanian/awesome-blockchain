
###  State

# Simple State

State is where the smart contract works with saving and retrieving data. You can think of it much like a database interaction layer in a traditional application.

The most simple way of writing state is by writing a single item.

For example, in the  `cw20-base`  contract,  `TokenInfo`  is written when the contract is instantiated.

# Complex State and Maps

Of course, for most non-trivial examples, additional data will need to be stored. You can serialise larger JSON data structures, and use key-value lookups to access this data.

In CW20, the mapping of addresses to their CW20 balance is achieved through just such a map:

```
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
```

The code for this can be found  [here](https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/state.rs#L35).


More sophisticated contracts, such as CW1155, allow for the creation and management of multiple coins.

For more advanced usage, indexing and more, check out:

-   [Indexes in CosmWasm](https://docs.cosmwasm.com/tutorials/storage/indexes)
-   [Advanced State Modeling in CosmWasm](https://docs.cosmwasm.com/tutorials/storage/state-modeling)
-   [How CW Storage Works](https://docs.cosmwasm.com/tutorials/storage/key-value-store)