
# What to do with a stuck/pending transaction?
Sometimes when submitting a transaction, the gas fee is set too low, and it gets stuck in the [transaction pool](https://help.optimism.io/hc/en-us/articles/16135808401947) (a.k.a. mempool). If you want to push that transaction through, [you can cancel it by submitting another transaction with the same nonce](https://info.etherscan.com/how-to-cancel-ethereum-pending-transactions/). This method increases the fee and the sequencer will process it from the mempool quicker.

# How do transaction gas fees on OP Mainnet work?
Fees on OP Mainnet (also known as L2) are, for the most part, significantly lower than on the L1 Ethereum network.

Every OP Mainnet transaction has two costs: An L2 (execution) fee and an L1 (data) fee. At a high level, the L2 fee is the cost to execute your transaction in L2 and the L1 fee is the estimated cost to publish your transaction on L1 (in a rollup batch).

[For additional details, see the developer documentation.](https://docs.optimism.io/builders/dapp-developers/transactions/fees)

---
# Managing the gas fees that make up the L2 execution fee

[Every OP Mainnet transaction has two costs](https://help.optimism.io/hc/en-us/articles/4411895794715): An L2 (execution) fee and an L1 (security) fee. At a high level, the L2 fee is the cost to execute your transaction in L2 and the L1 fee is the estimated cost to publish your transaction on L1 (in a rollup batch). In contrast to the L1 data price, which is deducted automatically, you can control the parameters for the L2 execution price in the wallet when you submit a transaction.

This price is calculated as  **(base fee + priority fee)*gas used**.

-   **Base fee**. This fee is set to a specific value for all the transactions on a block, so there is no cost of specifying a higher value as acceptable. Transactions with a base fee that is lower than the block base fee and not included in the block.
-   **Priority fee**. This fee is specified by each transaction in the block. Usually, transactions are arranged by priority fee, with the higher priority fee ones being first in the block and the lower priority fee ones being later. If the block gas limit is exceeded, transactions with a lower priority fee don't get included in the block.
-   **Gas used**. This is just  [the gas](https://ethereum.org/en/developers/docs/gas/)  used by the transaction.

[You can see the current values of these fees in the gas tracker](https://optimism.io/gas-tracker).

## What fee level to set?

OP Mainnet uses  [EIP 1559](https://eips.ethereum.org/EIPS/eip-1559), but with  [different parameter values](https://community.optimism.io/docs/developers/build/differences/#blocks). As a result, in every block the base fee can be between 98% and 110% of the previous value. As blocks are produced every two seconds, the base fee can be between 54% and 1,745% of the value a minute earlier.

The base fee you specify in the wallet is not necessarily the base fee you pay, it is an upper limit to the base fee you are willing to pay. It does not hurt you to specify a significantly higher value than the current one, and it might make it possible to include your transaction.

The priority fee, on the other hand, depends on users' choices and you pay all of it. So it makes sense to look  [in the dashboard](https://public-grafana.optimism.io/d/9hkhMxn7z/public-dashboard?orgId=1&refresh=5m&viewPanel=29)  and specify a value based on what was in recent transactions.


# The Transaction Pool (a.k.a. Mempool)

Optimism has a transaction pool and supports EIP 1559. The transaction pool is private, so your transactions are not shared with anybody until they are actually included in a block.

The transaction pool affects users in two ways:

1.  You can specify a high  [priority fee](https://ethereum.org/en/developers/docs/gas/)  to make it more likely that your transaction is included early in the next block.
2.  If a transaction's L2 gas price is so low, it can stay stuck. In that case,  [you can cancel it by submitting another transaction with the same nonce](https://info.etherscan.com/how-to-cancel-ethereum-pending-transactions/). You can also submit the exact same transaction, with the same nonce, and only a higher gas price.

[Current values for both the priority fee and the base fee are available in the gas price dashboard](https://optimism.io/gas-tracker).

---
# How soon can you see transactions?
You can see transactions on [https://explorer.optimism.io/](https://explorer.optimism.io/) when they are included in a block. New blocks are produced every two seconds, so usually this is a very fast process.

# Will you be able to reduce transaction costs in the future? To what?

We're a rollup. This means **every** transaction gets written to L1 (the Ethereum mainnet) to have the availability and censorship resistance of Ethereum. Each transaction byte costs approximately 16 gas, which at writing means 0.6 cents per byte. A transaction has to have a destination (20 bytes), a signature (65 bytes), and a few other fields. This forms the minimum possible price for us.


# Can I withdraw directly from OP Mainnet to a centralized exchange?
A variety of popular exchanges like Binance, Crypto.com, Kucoin and more support direct withdraws from OP Mainnet.

If the centralized exchange supports OP Mainnet then you will be able to transfer directly from OP Mainnet to the respective exchange. If your favorite exchange does not support OP Mainnet yet, then you have to bridge your assets into a network they do support, such as the Ethereum mainnet.

# Why do I need to wait a week when moving assets out of OP Mainnet?
Users are required to wait for a period of one week when moving assets out of OP Mainnet into the Ethereum mainnet. This period of time is called the Challenge Period and serves to help secure the assets stored on OP Mainnet. You can find more information about the Challenge Period  [here](https://community.optimism.io/docs/developers/bridge/messaging.html#understanding-the-challenge-period). Please note that the test network has a Challenge Period of only 60 seconds to simplify the development process.

There are  [third party bridges](https://www.optimism.io/apps/bridges)  that allow you to withdraw your assets faster, typically by fronting you the ETH or tokens and then performing the a large withdrawal when they run short.

# Can I cancel a withdrawal after it has been submitted?
**No, withdrawals currently cannot be cancelled once submitted.** We are exploring this as a future system feature.

# Do I have to wait 7 days to get my funds out?

If you use the OP Mainnet Gateway then yes. But you might be able to use [a third party bridge](https://www.optimism.io/apps/bridges). They provide faster bridging services (but charge a small extra fee). They also usually support multiple networks. Note that their token selection may be more limited.


# Why are bridge withdrawals so expensive?

Our bridge is highly secure, and that is what we where aiming to do with it. It is expensive because security requires a lot of processing.

We expect our bridge to be the "wholesale" bridge used by other bridges as a secure channel to transfer large amounts. For most user L1<->L2 transactions it is sufficient to use one of the  [user facing "retail" bridges.](https://www.optimism.io/apps)