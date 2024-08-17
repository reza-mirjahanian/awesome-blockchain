Creation
-------------------------------------------------------------------------------------------------------------

### Transaction Creation[​](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#transaction-creation "Direct link to Transaction Creation")

One of the main application interfaces is the command-line interface. The transaction `Tx` can be created by the user inputting a command in the following format from the [command-line](https://docs.cosmos.network/v0.50/learn/advanced/cli), providing the type of transaction in `[command]`, arguments in `[args]`, and configurations such as gas prices in `[flags]`:

```
[appname] tx [command][args][flags]

```

This command automatically **creates** the transaction, **signs** it using the account's private key, and **broadcasts** it to the specified peer node.

There are several required and optional flags for transaction creation. The `--from` flag specifies which [account](https://docs.cosmos.network/v0.50/learn/beginner/accounts) the transaction is originating from. For example, if the transaction is sending coins, the funds are drawn from the specified `from` address.

#### Gas and Fees[​](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#gas-and-fees "Direct link to Gas and Fees")

Additionally, there are several [flags](https://docs.cosmos.network/v0.50/learn/advanced/cli) users can use to indicate how much they are willing to pay in [fees](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees):

-   `--gas` refers to how much [gas](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees), which represents computational resources, `Tx` consumes. Gas is dependent on the transaction and is not precisely calculated until execution, but can be estimated by providing `auto` as the value for `--gas`.
-   `--gas-adjustment` (optional) can be used to scale `gas` up in order to avoid underestimating. For example, users can specify their gas adjustment as 1.5 to use 1.5 times the estimated gas.
-   `--gas-prices` specifies how much the user is willing to pay per unit of gas, which can be one or multiple denominations of tokens. For example, `--gas-prices=0.025uatom, 0.025upho` means the user is willing to pay 0.025uatom AND 0.025upho per unit of gas.
-   `--fees` specifies how much in fees the user is willing to pay in total.
-   `--timeout-height` specifies a block timeout height to prevent the tx from being committed past a certain height.

The ultimate value of the fees paid is equal to the gas multiplied by the gas prices. In other words, `fees = ceil(gas * gasPrices)`. Thus, since fees can be calculated using gas prices and vice versa, the users specify only one of the two.

Later, validators decide whether or not to include the transaction in their block by comparing the given or calculated `gas-prices` to their local `min-gas-prices`. `Tx` is rejected if its `gas-prices` is not high enough, so users are incentivized to pay more.

#### CLI Example[​](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#cli-example "Direct link to CLI Example")

Users of the application `app` can enter the following command into their CLI to generate a transaction to send 1000uatom from a `senderAddress` to a `recipientAddress`. The command specifies how much gas they are willing to pay: an automatic estimate scaled up by 1.5 times, with a gas price of 0.025uatom per unit gas.

```
appd tx send <recipientAddress> 1000uatom --from <senderAddress> --gas auto --gas-adjustment 1.5 --gas-prices 0.025uatom

```

#### Other Transaction Creation Methods[​](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#other-transaction-creation-methods "Direct link to Other Transaction Creation Methods")

The command-line is an easy way to interact with an application, but `Tx` can also be created using a [gRPC or REST interface](https://docs.cosmos.network/v0.50/learn/advanced/grpc_rest) or some other entry point defined by the application developer. From the user's perspective, the interaction depends on the web interface or wallet they are using (e.g. creating `Tx` using [Lunie.io](https://lunie.io/#/) and signing it with a Ledger Nano S).