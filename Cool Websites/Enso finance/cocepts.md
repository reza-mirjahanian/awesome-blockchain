### DeFi tokens 
are tokens that represent a DeFi position. DeFi tokens are obtained by depositing underlying tokens, or other DeFi tokens inside of  [DeFi projects](https://api-docs.enso.finance/concepts/projects).

For example, depositing  [WETH(opens in a new tab)](https://etherscan.io/token/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2)  into  [Yearn(opens in a new tab)](https://yearn.finance/)  you will obtain  [yvWETH(opens in a new tab)](https://etherscan.io/token/0xa258C4606Ca8206D8aA700cE2143D7db854D168c)  and start generating yield on this.

### Base tokens 
are normal tokens that do not represent a DeFi position. Base tokens are underlying token of a [DeFi Position Token](https://api-docs.enso.finance/concepts/tokens/defiTokens) and can be used to enter a DeFi token/positions.


# Tokenized

Particular DeFi projects associate your interactions with a token sent to your wallet. For example, if you lend on Aave you will get your representative loan in the  `a`  version of your  [base token](https://api-docs.enso.finance/concepts/tokens/baseTokens)  into  `aDAI`  for example which is a  [DeFi position token](https://api-docs.enso.finance/concepts/tokens/defiTokens).


# Non-Tokenized

Particular DeFi actions do not give you a token associated to your interaction with their protocol. For example, depositing on  [liquity(opens in a new tab)](https://www.liquity.org/)  does not give you a token representing your cdp, similarly with  [aave(opens in a new tab)](https://aave.com/)  when you borrow you do not get a tokenized position representing your state.

These stateful interactions are associated with the address whether that is an  [EOA Address](https://api-docs.enso.finance/concepts/eoa-smart-wallet/eoa)  or  [Contract Address](https://api-docs.enso.finance/concepts/eoa-smart-wallet/smart-wallet).


# Behind the hood

### Routing, price impact and amounts out[](https://api-docs.enso.finance/concepts/behind-the-hood#routing-price-impact-and-amounts-out)

In DeFi trading, it's common to encounter variable losses due to  [price impact(opens in a new tab)](https://support.uniswap.org/hc/en-us/articles/8671539602317-What-is-Price-Impact-). The Enso Router does the best effort to find the optimal route for your trade. If your position and its underlyings are supported, it will use the position directly. Otherwise, it looks for the best AMM to route the trade through.

Our response also includes information about the price impact when available. If it's not available, Enso always simulates the transaction, providing you with the expected  **amounts out**  and the  **trade path**. With this information, you as the integrator can check that the outcome matches your expectations and decide whether to go ahead with the trade.

It's important for integrators to review this information. While we do our best to optimize each trade, the final responsibility to verify and confirm that the trade meets your expectations rest on you. This ensures you're always in control and making decisions that align with your specific needs and risk assessment.