This example fetches the Token Program to demonstrate the difference between wallet and program accounts.


```ts
import { Connection, PublicKey } from "@solana/web3.js";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com",
  "confirmed"
);
const address = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
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
      127,
      "...truncated, total bytes: 134080...",
      0
    ]
  },
  "executable": true,
  "lamports": 934087680,
  "owner": "BPFLoader2111111111111111111111111111111111",
  "rentEpoch": 18446744073709552000,
  "space": 134080
}
```