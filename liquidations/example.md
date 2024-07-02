
# build a liquidator bot

Since MakerDAO released the first version of the DAI stablecoin on the Ethereum blockchain at the end of 2017, there have been profit opportunities available to the community, that help maintain market equilibrium. Shortly after a few other protocols went live - Compound and dYdX.

Although all these protocols are different in terms of functionality, security and user experience among other things, one can think about them as systems that provide collaterized loans on the blockchain - users deposit tokens, such as ETH, in a smart contract, and are then able to take out loans that are backed by their collateral, under the condition that their supplied balance is higher than their borrowed balance, plus some health threshold defined by the system. If the value of the account goes below a health threshold, i.e. the value of the loan taken from the system is dangerously close to the provided collateral (i.e. the supplied value), the account/position can be liquidated by anyone.

Liquidators are incentivized to perform that function by receiving a reward - a percentage of the collateral that they liquidate.


# How to build a liquidator bot

In order to build a liquidator bot, you need to do two things:

-   You need to monitor the blockchain for accounts that can be liquidated and you need to make sure that you broadcast transactions to the blockchain as soon as an account can be liquidated (provided you believe this might be profitable for you).
    
-   You need to implement smart contracts that interact with the protocols you monitor and perform liquidations on, such as dYdX or Compound or MakerDAO or Aave.

# Monitoring the blockchain and triggering liquidations

To monitor the Ethereum blockchain, you need to run an Ethereum node, such as Geth, and subscribe to the latest mined block as well as to all pending transactions in the mempool. You could also use a hosted Ethereum node from a provider, such as Infura.

In a nutshell accounts that can be liquidated, become available for liquidation due to some underlying condition, such as a change in price of a supplied or borrowed asset. For example the price of ETH/USD:

-   ETH going up could trigger liquidations on accounts that have taken a short position in ETH, whereas
    
-   ETH going down could trigger liquidations for accounts that have taken a long position in ETH
    

You need to monitor for that condition and detect when it happens. Most platforms use the  [MakerDAO price oracles](https://makerdao.com/en/feeds/)  to determine prices of assets, and you could monitor for pending transactions that are about to change the price of an asset, such as ETH, on the blockchain. Here is an example of such transaction:


1.  **Executing Liquidations:**
    
    -   If the simulated liquidation is profitable, the bot interacts with the deployed On-Chain Liquidator smart contract.
    -   Two key actions take place during liquidation:
        -   **`absorb`:**  The On-Chain Liquidator takes over the undercollateralized loan.
        -   **`buyCollateral`:**  The On-Chain Liquidator purchases the collateral at a discount.
    -   **Flash Loans/Swaps:**  The bot may leverage flash loans (from protocols like Balancer) or flash swaps (from Uniswap or SushiSwap) to obtain the necessary funds for the liquidation.

-   **Front Running:**  Liquidation bots often compete with each other; minimizing latency and optimizing transaction parameters is crucial to successful liquidations.

Once you detect that a transaction is going to affect the health of a given account, and that the account will be liquidatable when the price of an asset changes, you could:

-   Broadcast a liquidation transaction and try to queue it in the same block as the oracle price feed transaction. For example if the oracle transaction has a gas price of X gwei, the gas price of the liquidation transaction you send should be less than or equal to X gwei when queuing.
    
-   Broadcast a liquidation transaction and try to get it mined in the next block, as soon as the oracle price feed transaction has already been mined and the target account has not been liquidated. Monitoring the  [event logs](https://powpark.com/blog/defi/2020/06/19/how-to-build-a-liquidator-bot-for-dydx-compound-makerdao#ref_5)  from the latest mined block is a good way to detect if an account was liquidated. Your aim is for your transaction to be the first one to liquidate a given account. In this case your liquidation transaction could feature gas price as high as  [this one](https://etherscan.io/tx/0xc215b9356db58ce05412439f49a842f8a3abe6c1792ff8f2c3ee425c3501023c).


# Smart contracts for liquidating accounts

Once you have a system in place to monitor the blockchain for accounts that are liquidatable and to broadcast liquidation transactions, you need to implement a smart contract to handle the liquidation transactions and interact with the smart contracts of the given platform - for example dYdX or Compound.

You will also need to create an account on the platform and deposit initial capital. When liquidating an account, you assume the debt of the account as well as its collateral, which according to the index price at the time of the liquidation should be enough to cover the debt, plus the liquidation reward.

Depending on the platform you liquidate on, you might be able to leverage  [flash loans](https://medium.com/aave/flash-loans-one-month-in-73bde954a239)  in order to atomically liquidate positions and repay back the assumed debt within the same transaction, and pocket the reward. In this scenario, your only upfront capital needed is some ETH to pay for the gas fee for the liquidation transaction. This strategy is currently possible on dYdX and on Compound with mostly all supported assets.


# Advanced techniques for efficient liquidations

## GasToken

Generally liquidation transactions use very high gas prices, ranging from 50 gwei up to 10k gwei and above. In these cases it is advisable to use tools such as GasToken. GasToken allows you to mint gas prior to liquidations and get rebates as soon as your liquidation transaction is mined with a high gas price. GasToken works by taking advantage of the storage refund in Ethereum.

## Front running

There are many active liquidators competing for liquidations on the various DeFi platforms. Many of them are monitoring the mempool containing pending transactions during liquidation periods and trying to out-bid each other, in order to win liquidations and the associated rewards. This results in very high transaction costs, sometimes leading to liquidators actually losing money, due to the liquidation reward not covering even the transaction cost, essentially forwarding the reward from the platform to the blockchain miners.

## Canceling of transactions

If you detect that another competitor has broadcasted a liquidation transaction featuring very high gas price, which could and probably would beat your liquidation transaction, you could attempt to reduce your losses. There is no way you could cancel an already broadcasted transaction, but you could overwrite it with a higher gas price and different parameters, ultimately resulting in much lower gas fees.

This strategy is also applicable if you detect that an account has been liquidated, but your transaction is still pending.