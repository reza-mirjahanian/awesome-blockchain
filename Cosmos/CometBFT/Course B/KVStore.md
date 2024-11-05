The kvstore app is a  [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree)  that just stores all transactions. If the transaction contains an  `=`, e.g.  `key=value`, then the  `value`  is stored under the  `key`  in the Merkle tree. Otherwise, the full transaction bytes are stored as the key and the value.

Let’s start a kvstore application.

```
abci-cli kvstore

```

In another terminal, we can start CometBFT. You should already have the CometBFT binary installed. If not, follow the steps from  [here](https://docs.cometbft.com/v0.38/guides/install). If you have never run CometBFT before, use:

```
cometbft init
cometbft node

```

If you have used CometBFT, you may want to reset the data for a new blockchain by running  `cometbft unsafe-reset-all`. Then you can run  `cometbft node`  to start CometBFT, and connect to the app. For more details, see  [the guide on using CometBFT](https://docs.cometbft.com/v0.38/core/using-cometbft).

You should see CometBFT making blocks! We can get the status of our CometBFT node as follows:

```
curl -s localhost:26657/status

```

The  `-s`  just silences  `curl`. For nicer output, pipe the result into a tool like  [jq](https://stedolan.github.io/jq/)  or  `json_pp`.

Now let’s send some transactions to the kvstore.

```
curl -s 'localhost:26657/broadcast_tx_commit?tx="abcd"'

```

Note the single quote (`'`) around the url, which ensures that the double quotes (`"`) are not escaped by bash. This command sent a transaction with bytes  `abcd`, so  `abcd`  will be stored as both the key and the value in the Merkle tree. The response should look something like:

```
{
  "jsonrpc": "2.0",
  "id": "",
  "result": {
    "check_tx": {},
    "deliver_tx": {
      "tags": [
        {
          "key": "YXBwLmNyZWF0b3I=",
          "value": "amFl"
        },
        {
          "key": "YXBwLmtleQ==",
          "value": "YWJjZA=="
        }
      ]
    },
    "hash": "9DF66553F98DE3C26E3C3317A3E4CED54F714E39",
    "height": 14
  }
}

```

We can confirm that our transaction worked and the value got stored by querying the app:

```
curl -s 'localhost:26657/abci_query?data="abcd"'

```

The result should look like:

```
{
  "jsonrpc": "2.0",
  "id": "",
  "result": {
    "response": {
      "log": "exists",
      "index": "-1",
      "key": "YWJjZA==",
      "value": "YWJjZA=="
    }
  }
}

```

Note the  `value`  in the result (`YWJjZA==`); this is the base64-encoding of the ASCII of  `abcd`. You can verify this in a python 2 shell by running  `"YWJjZA==".decode('base64')`  or in python 3 shell by running  `import codecs; codecs.decode(b"YWJjZA==", 'base64').decode('ascii')`. Stay tuned for a future release that  [makes this output more human-readable](https://github.com/tendermint/tendermint/issues/1794).

Now let’s try setting a different key and value:

```
curl -s 'localhost:26657/broadcast_tx_commit?tx="name=satoshi"'

```

Now if we query for  `name`, we should get  `satoshi`, or  `c2F0b3NoaQ==`  in base64:

```
curl -s 'localhost:26657/abci_query?data="name"'
```