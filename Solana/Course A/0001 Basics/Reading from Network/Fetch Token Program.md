This example fetches the Token Program to demonstrate the difference between wallet and program accounts.

The Token Program is an executable program account on Solana. Like wallet accounts, programs have the same underlying [Account](https://github.com/anza-xyz/agave/blob/v2.1.11/sdk/account/src/lib.rs#L48-L60) data structure, but with key differences in its fields:

| Field | Description |
| --- |  --- |
| `executable` | Set to `true`, indicating that this account contains executable program code. |
| --- |  --- |
| `data` | For program accounts, this field stores the program's executable code. In contrast, wallet accounts have an empty data field. |
| `owner` | The account is owned by a Loader program, which is a category of built-in programs that own executable program accounts on Solana. (`BPFLoader2111111111111111111111111111111111`) |

The program account stores the compiled bytecode for the Token Program's [source code](https://github.com/solana-program/token/tree/main/program). You can view this program account on the [Solana Explorer](https://explorer.solana.com/address/TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA).

![alt text](image-1.png)

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