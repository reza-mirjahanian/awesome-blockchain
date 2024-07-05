### 1. How do AMMs work?

AMMs use liquidity pools, where users can deposit [cryptocurrencies](https://www.gemini.com/cryptopedia/what-are-cryptocurrency-pairs) to provide liquidity. These pools then use algorithms to set token prices based on the ratio of assets in the pool. When a user wants to trade, they swap one token for another directly through the AMM, with prices determined by the pool's algorithm.

### 2. What are the advantages of using AMMs?

-   Liquidity: AMMs can provide continuous liquidity for a wide range of assets, making it easier to trade less popular cryptocurrencies.
    
-   Accessibility: Anyone can provide liquidity to AMMs and participate in trading, often with lower fees compared to traditional exchanges.
    
-   Decentralization: AMMs often operate without centralized intermediaries, offering greater autonomy and control to users.
    

### 3. Which are the popular AMMs?

Some of the well-known AMMs include [Uniswap](https://www.gemini.com/cryptopedia/uniswap-decentralized-exchange-crypto-defi), [SushiSwap](https://www.gemini.com/cryptopedia/sushiswap-sushi-coin-sushibar-chef-nomi), [PancakeSwap](https://www.gemini.com/cryptopedia/pancakeswap-exchange-cake-crypto-pancake-swap), and [Balancer](https://www.gemini.com/cryptopedia/balancer-crypto-automated-pools). Each has its unique features and supported assets.

### 4. Do AMMs support fiat-to-crypto trading?

AMMs primarily facilitate cryptocurrency-to-cryptocurrency trading. To trade with fiat currency, users usually need to go through a [centralized exchange](https://www.gemini.com/exchange) or other on/off-ramp services to convert fiat to cryptocurrency before interacting with AMMs.


## Automated Market Makers Explained

[Automated market makers](https://www.gemini.com/cryptopedia/glossary#automated-market-maker-amm) (AMMs) allow digital assets to be traded without permission and automatically by using [liquidity pools](https://www.gemini.com/cryptopedia/glossary#liquidity-pool) instead of a traditional market of buyers and sellers. On a [traditional exchange platform](https://www.gemini.com/cryptopedia/centralized-exchanges-crypto), buyers and sellers offer up different prices for an asset. When other users find a listed price to be acceptable, they execute a trade and that price becomes the asset’s market price. Stocks, gold, real estate, and most other assets rely on this traditional market structure for trading. However, AMMs have a different approach to trading assets.

AMMs are a financial tool unique to [Ethereum](https://www.gemini.com/cryptopedia/ethereum-blockchain-smart-contracts-dapps) and decentralized finance (DeFi). This new technology is decentralized, always available for trading, and does not rely on the traditional interaction between buyers and sellers. This new method of exchanging assets embodies the ideals of Ethereum, crypto, and blockchain technology in general: no one entity controls the system, and anyone can build new solutions and participate.

## Liquidity Pools and Liquidity Providers

[Liquidity](https://www.gemini.com/cryptopedia/glossary#liquidity) refers to how easily one asset can be converted into another asset, often a fiat currency, without affecting its market price. Before AMMs came into play, liquidity was a challenge for [decentralized exchanges](https://www.gemini.com/cryptopedia/decentralized-exchange-crypto-dex) (DEXs) on Ethereum. As a new technology with a complicated interface, the number of buyers and sellers was small, which meant it was difficult to find enough people willing to trade on a regular basis. AMMs fix this problem of limited liquidity by creating liquidity pools and offering [liquidity providers](https://www.gemini.com/cryptopedia/glossary#liquidity-provider-lp) the incentive to supply these pools with assets. The more assets in a pool and the more liquidity the pool has, the easier trading becomes on decentralized exchanges.

On AMM platforms, instead of trading between buyers and sellers, users trade against a pool of tokens — a liquidity pool. At its core, a liquidity pool is a shared pot of tokens. Users supply liquidity pools with tokens and the price of the tokens in the pool is determined by a mathematical formula. By tweaking the formula, liquidity pools can be optimized for different purposes.

Anyone with an internet connection and in possession of any type of [ERC-20 tokens](https://www.gemini.com/cryptopedia/erc20-token-standard-ethereum) can become a liquidity provider by supplying tokens to an AMM’s liquidity pool. Liquidity providers normally earn a fee for providing tokens to the pool. This fee is paid by traders who interact with the liquidity pool. Recently, liquidity providers have also been able to earn yield in the form of project tokens through what is known as “[yield farming](https://www.gemini.com/cryptopedia/glossary#yield-farming).”


## Constant Product Formula

AMMs have become a primary way to trade assets in the DeFi ecosystem, and it all began with a blog post about “on-chain market makers” by Ethereum founder Vitalik Buterin. The secret ingredient of AMMs is a simple mathematical formula that can take many forms. The most common one was proposed by Vitalik as:

tokenA_balance(p) * tokenB_balance(p) = k

and popularized by Uniswap as:

x * y = k

The constant, represented by “k” means there is a constant balance of assets that determines the price of tokens in a liquidity pool. For example, if an AMM has ether (ETH) and bitcoin (BTC), two volatile assets, every time ETH is bought, the price of ETH goes up as there is less ETH in the pool than before the purchase. Conversely, the price of BTC goes down as there is more BTC in the pool. The pool stays in constant balance, where the total value of ETH in the pool will always equal the total value of BTC in the pool. Only when new liquidity providers join in will the pool expand in size. Visually, the prices of tokens in an AMM pool follow a curve determined by the formula.

In this constant state of balance, buying one ETH brings the price of ETH up slightly along the curve, and selling one ETH brings the price of ETH down slightly along the curve. The opposite happens to the price of BTC in an ETH-BTC pool. It doesn’t matter how volatile the price gets, there will eventually be a return to a state of balance that reflects a relatively accurate market price. If the AMM price ventures too far from market prices on other exchanges, the model incentivizes traders to take advantage of the price differences between the AMM and outside crypto exchanges until it is balanced once again.

The [constant formula](https://www.gemini.com/cryptopedia/glossary#constant-product-formula) is a unique component of AMMs — it determines how the different AMMs function.

## Automated Market Maker Variations

In Vitalik Buterin’s original post calling for automated or [on-chain](https://www.gemini.com/cryptopedia/glossary#on-chain) money markets, he emphasized that AMMs should not be the only available option for decentralized trading. Instead, there needed to be many ways to trade tokens, since non-AMM exchanges were vital to keeping AMM prices accurate. What he didn’t foresee, however, was the development of various approaches to AMMs.

The [DeFi ecosystem](https://www.gemini.com/cryptopedia/what-is-defi-crypto-decentralized-finance-projects) evolves quickly, but three dominant AMM models have emerged: [Uniswap](https://www.gemini.com/cryptopedia/uniswap-decentralized-exchange-crypto-defi), [Curve](https://www.gemini.com/cryptopedia/curve-crypto-automated-market-maker), and [Balancer](https://www.gemini.com/cryptopedia/balancer-crypto-automated-pools).

-   Uniswap’s pioneering technology allows users to create a liquidity pool with any pair of [ERC-20](https://www.gemini.com/cryptopedia/erc20-token-standard-ethereum) tokens with a 50/50 ratio, and has become the most enduring AMM model on Ethereum.
    
-   Curve specializes in creating liquidity pools of similar assets such as stablecoins, and as a result, offers some of the lowest rates and most efficient trades in the industry while solving the problem of limited liquidity.
    
-   Balancer stretches the limits of Uniswap by allowing users to create dynamic liquidity pools of up to eight different assets in any ratio, thus expanding AMMs’ flexibility.
    

Although Automated Market Makers harness a new technology, iterations of it have already proven an essential financial instrument in the fast-evolving DeFi ecosystem and a sign of a maturing industry.