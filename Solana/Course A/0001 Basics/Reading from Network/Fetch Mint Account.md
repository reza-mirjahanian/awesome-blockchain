

This example fetches the USD Coin (USDC) Mint account to show how programs on Solana store state in separate accounts.

A [Mint](https://github.com/solana-program/token/blob/program%40v8.0.0/program/src/state.rs#L16-L30) account is an account owned by the Token Program. It stores global metadata for a specific token, including the total supply, number of decimals, and the accounts authorized to mint or freeze tokens. The Mint account's address uniquely identifies a token on the Solana network.


These are the key points to note about the Mint account data:

| Field | Description |
| --- |  --- |
| `executable` | This field is `false` because the mint account stores state, not executable code. |
| --- |  --- |
| `data` | This field contains the serialized account state, such as the mint authority, total supply, number of decimals |
| `owner` | The Token program (`TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`) owns the mint account. |

```ts
import { Connection, PublicKey } from "@solana/web3.js";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com",
  "confirmed"
);

const address = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const accountInfo = await connection.getAccountInfo(address);

console.log(
  JSON.stringify(
    accountInfo,
    (key, value) => {
      if (key === "data" && value && value.length > 1) {
        return [
          value[0],
          "...truncated, total bytes: " + value.length + "...",
          value[value.length - 1]
        ];
      }
      return value;
    },
    2
  )
);
```

```
{
  "data": {
    "type": "Buffer",
    "data": [
      1,
      "...truncated, total bytes: 82...",
      103
    ]
  },
  "executable": false,
  "lamports": 390231807122,
  "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
  "rentEpoch": 18446744073709552000,
  "space": 82
}
```