
# Liquidation Bot

Liquidation Bots improve the overall health of the protocol by deleveraging users approaching bankruptcy.

The liquidation in the protocol is a liability transfer mechanism from the selected **liquidatee** (user in liquidation territory) to the **liquidator** (the user running the Liquidation Bot).

Liquidators inherit the liabilities they liquidate and receive an asset at a discount for doing so. In order to participate, liquidators must have even collateral in their account in order to satisfy the initial margin requirements for the transferred positions.


## What are Liquidations?[](https://docs.drift.trade/liquidations/liquidations#what-are-liquidations)

Liquidations are a part of leveraged trading. Traders that elect to use leverage are using the collateral they deposited as margin to borrow money from the protocol. Traders may choose to do this in order to open a larger position and have more exposure to a particular asset, i.e.  _leveraged exposure_.

When this occurs, the protocol must protect itself by ensuring that there is enough margin for the position to settle any losses that occur. There is a prescribed minimum ratio between a position's value and its margin for each asset


[Liquidations – Drift Protocol](https://docs.drift.trade/liquidations/liquidations)

You deposit 500 USDC and you long 200 SOL-PERP at $25

Your position value is $5000 USDC and your margin/collateral is $500 USDC, you have 10x leverage

Note that the Minimum Maintenance Margin for a SOL-PERP is 5% (see Minimum Maintenance Margin for assets here:  [Margin](https://docs.drift.trade/trading/margin))

Your total collateral is  `$500 * 1.00 = $500`  and your margin requirement is  `$5000 * .05 = 250`

You are liquidated if the price of SOL decreases by 5.28% to $23.6825. This is because your perp pnl is  `200 * $25 * -.0527 = -263.5`  and thus your total collateral is  `$500 - 263.5 = $236.5`while your margin requirement is  `$23.6825 * 200 * .05 = $236.825`.




A liquidation event on Lista occurs when BNB, ETH or other collaterals need to be sold, due to various circumstances like loan collateral requirements not being met. The bot helps users by automating the bidding process, aiming to secure assets at favorable prices. Everything that you need to have your liquidation bot set up can be found [here](https://github.com/lista-dao/AuctionBots-go/blob/main/README.md).

[zach030/morpho-liquidator-bot (github.com)](https://github.com/zach030/morpho-liquidator-bot)

