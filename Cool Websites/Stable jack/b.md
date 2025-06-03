# Stable Jack: Innovating Yield, Volatility, and Points Markets

## Introducing Stable Jack: A New DeFi Primitive

* **Stable Jack** is a novel DeFi protocol offering unique financial products that bridge the gap between traditional yield farming and leveraged trading.
* It introduces a *new primitive* by stripping and repackaging the components of underlying assets.
* **Cesar**, Co-founder of Stable Jack, has a background in structured products in DeFi since 2017, leading to the conceptualization and building of Stable Jack over the past 1.5 years.

## The Genesis of Stable Jack

* The initial idea for Stable Jack emerged after significant market events like the **USDC de-peg** and **Luna crash**.
* This highlighted the demand for stablecoins and yield generation, inspiring a product that could offer *yield-bearing stablecoins* and *yield products*.
* **Stable Jack's Core Proposition**: Building a yield, volatility, and points market for *any asset*.

## Stable Jack's Value Proposition

Stable Jack offers several compelling features:

* **Fixed Yield & Leveraged Yield** on stablecoins and LSTs (Liquid Staking Tokens).
* **Leveraged Airdrop Farming**.
* **Leveraged Exposure to Collateral** *without liquidation risk*.

## Current Status and Future Outlook

* Currently live on **Avalanche** and **Sonic**.
* Achieved **$13 million in TVL** with an **80% retention rate** within a few months of launch.
* Recently launched **Version 2 (V2)**, with ongoing optimizations and bug fixes.
* Future plans include rolling out new assets to expand user options.

## Decoding Stable Jack's Three Tokens

Stable Jack utilizes three distinct tokens, often a point of confusion for newcomers familiar with Pendle's similar nomenclature:

### 1. YT (Yield Token)

* **Definition**: Receives the *entire yield* of the collateral with *principal protection*.
* **Usage**: Offers **leverage yield exposure** on a specific asset.
* *For conservative traders* who want yield without price volatility exposure.
* Example: Depositing $100 of sAVAX gives you AUSD (Stable Jack's stablecoin equivalent), which remains $100 and earns the majority of the yield from sAVAX, *without exposure to sAVAX price fluctuations*.
* Historically, AUSD has generated competitive and consistent yields (e.g., over 20% in the first 6 months, around 15% subsequently).

### 2. PT (Points Token)

* **Definition**: Receives the *entire points* of the collateral asset with *principal protection*.
* **Usage**: Offers **leverage points exposure**, ideal for airdrop farming.
* *For users looking to maximize their exposure to potential airdrops* without risking their principal.
* *Maintains principal exposure* and has *no maturity date*, unlike some other protocols.

### 3. WT (Volatility Token)

* **Definition**: Receives the *entire price action* of the collateral asset.
* **Usage**: Offers **leverage exposure to collateral** *without liquidation risk* or *paying any funding fees*.
* *Primarily for leveraged traders* seeking long-term exposure without the typical downsides of perpetual exchanges.
* The leverage can be up to **4x**, but typically hovers around **1.5x**, as determined by market demand.

## Stable Jack's Trading Pairs and Mechanics

Stable Jack offers three primary trading options:

### 1. Yield vs. Volatility Trade

* **Mechanism**: A collateral asset (e.g., an LST like sAVAX) has associated yield (staking rewards) and price action. Stable Jack *strips these into two separate assets*.
* **YT Holders**: Give up price action exposure for *leveraged yield exposure*.
* **WT Holders**: Give up staking yield for *leveraged price action exposure* to the collateral, *without liquidation risk or funding fees*.
* **Example**: If you and another user both deposit 5 sAVAX, you might get AUSD (YT) and they might get an equivalent of 10 sAVAX worth of price action (WT), while you receive 10 sAVAX worth of staking APR.

### 2. Yield vs. Points Trade

* **Mechanism**: For assets that generate both yield and points. Stable Jack *strips these into two separate assets*.
* **YT Holders**: Give up points for *leveraged yield*.
* **PT Holders**: Give up yield for *leveraged points exposure*.
* *Principal protection* and *no maturity date* are key features.

### 3. Only Yield Trade

* **Mechanism**: Similar to Pendle, users can *short or long the yield* based on future projections.
* **Fixed Yield**: Obtain a fixed yield for a period *without a maturity date*.
* **Leveraged Yield**: Long the yield for *leveraged yield exposure*.
* Users can earn more if the yield performs as expected or better, or less if it underperforms.

## Mechanism Design Deep Dive

### Optimal Scenario for the System

* Stable Jack aims for a **balance between YT and VT demand**, or slightly more demand for VT.
* **More YT demand**: Increases collateralization ratio and yield for YT holders, making the product more competitive.
* **More VT demand**: Decreases leverage for VT holders, requiring more YT demand to increase leverage for VT holders.
* *In essence, an equilibrium is ideal, but a slight bias towards VT demand can increase yield for YT holders, subsequently boosting YT demand and then leverage for VT holders.*

### Imbalance Management and Market Behavior

* Stable Jack has not faced major imbalances, but consistently observes *more demand for the volatility token (WT)*.
* **Intriguing Trend**: During bearish market conditions, users tend to *increase their exposure to volatility tokens*, expecting a rebound. This *contrasts typical "buy high, sell low" behavior*, suggesting smart traders.
* **Dynamic Adjustments**:
    * At **160% collateralization ratio**, minting and redemption fees are adjusted to *increase demand for WT* and *decrease demand for YT*.
    * At **147% collateralization ratio**, the protocol *automatically deleverages itself*. This means staked AUSD (YT) is converted into sAVAX (collateral asset). This is *not a loss of principal* but a conversion, effectively "buying the dip" for users and rebalancing the protocol.
    * *Even with significant price drops (e.g., 60-70% in AVX), the protocol has maintained stability and would continue to do so even with further 50% drops.*

## Stability and Collateralization

* The **YT token (AUSD)** is always assumed to be pegged to **$1**.
* **Pegging Mechanism**: If AUSD de-pegs (e.g., to $0.98), users can *redeem it for $1 at the protocol level*.
* **Robust Backing**: Unlike many de-pegged stablecoins, Stable Jack has:
    1.  A clear **pegging mechanism**.
    2.  Maintains **reserves** and does not utilize them elsewhere, ensuring **1:1 redemption** of collateral.
* Stable Jack operates on a **collateralized debt position (CDP) model**, similar to Liquity.

## Revenue Generation

Stable Jack generates revenue through:

* **Minting and Redemption Fees**.
* A percentage of the **LSD yield** (currently **20%**) is directed to the treasury.

## Stable Jack vs. Other Protocols

Stable Jack positions itself uniquely, bridging the gap between yield products like Pendle and perpetual products like GMX:

### Stable Jack vs. Pendle

| Feature          | Pendle                                | Stable Jack                                     |
| :--------------- | :------------------------------------ | :---------------------------------------------- |
| **Maturity Date** | Yes (fixed expiry)                    | No (perpetual)                                  |
| **Principal** | Can be given up for leverage          | *Always principal protected* |
| **Slippage/IL** | Yes (AMM model)                       | No (pool model - 1:1 minting/redemption)        |
| **Management** | Requires rollovers, higher gas fees   | Less management, no rollover fees               |
| **Fungibility** | Can be less fungible due to expiry    | Higher fungibility, potential for looping       |

### Stable Jack vs. GMX (Perpetual Products)

| Feature         | Perpetual Products (e.g., GMX, dYdX) | Stable Jack (VT)                           |
| :-------------- | :----------------------------------- | :----------------------------------------- |
| **Liquidation** | Yes                                  | *No liquidation risk* |
| **Funding Fees**| Yes                                  | *No funding fees* |
| **Leverage** | Higher (e.g., 5x-10x+)               | Lower (e.g., 1.5x average, up to 4x)       |
| **Use Case** | Short-term trading, technical analysis | Long-term exposure, less risk              |

## Future Plans and Roadmap

### Short to Mid-Term (1-3 Months)

* **Optimization & Bug Fixes**: Enhancing protocol stability and efficiency.
* **UI Improvements**: Simplifying the user interface for better accessibility for new users.
* **New Stablecoin Pairs**: Expanding offerings on Sonic and Avalanche.
* **Yield vs. Volatility Trade on Sonic**: Allowing users to long sAVAX without liquidation risk or earn leveraged sAVAX yield.

### Long-Term (Next Couple of Years)

* **Chain Expansion**:
    * Part of the **EthGlobal accelerator program**, aiming for presence in the Ethereum ecosystem.
    * Targeting new chains like **Itina's Convergence**.
* **Institutional Adoption & RWA Integration**:
    * Working with **local banks and exchanges in Turkey** to offer DeFi products to private and retail investors via centralized platforms.
    * Utilizing **tokenized Real World Assets (RWAs)** from banks to offer them to DeFi users.
    * Focusing on **emerging market assets** like Turkish Eurobonds, money market funds, and potentially stocks, due to their higher speculation and upside potential.
* **AI Agent Development**:
    * An AI agent to simplify the entire DeFi process, including transactions and automation.
    * Aimed at *retail traders and Web2 users* who find DeFi complexity challenging.

## Governance and Tokenomics (JACK)

* **Native Token**: **JACK**
* **Token Distribution Model**: **Discount Tickets**
    * Users engage with the protocol and earn "discount tickets" (points).
    * These tickets allow them to buy JACK at a discount to its market price.
    * **Benefits**:
        * Eliminates sybil and mercenary capital.
        * Rewards power users and real users.
        * **Demand-based unlock schedule**, not timing-based, preventing unnecessary supply and sell pressure.
        * Better token distribution to long-term aligned holders.
* **Token Utility**:
    * **Revenue Sharing**: **50% of protocol revenues** are directed to JACK stakers (with plans to increase this).
    * **Staking Mechanism**: Simple staking with a *one-day cooldown period* for unstaking.
    * **Airdrop Eligibility**: JACK stakers will be eligible for airdrops from protocols that list their points tokens on Stable Jack.
* **Governance**: Currently, the Stable Jack team makes listing decisions.
    * Future goal: **Permissionless listing of pairs**, allowing anyone to launch their own.
    * Considering a **curator model** as an optional layer for new asset listings, similar to Morpho, given different risk parameters for various assets.

## Vision for the Future of DeFi

* The DeFi ecosystem is maturing, with fewer new protocols and increased difficulty for new entrants.
* Expect **consolidation** with major players (e.g., Aave, Pendle, Uniswap) becoming even larger.
* Stable Jack aims to become a "powerhouse" in the DeFi ecosystem, a central piece that every new chain looks to integrate.
* Aspires to be the **main distribution channel between DeFi and TradFi**, bridging the gap for Web2 participants and bringing tokenized RWAs to DeFi.

## Call to Action for the Community

* Hyper Nest community members are encouraged to **try Stable Jack**, provide feedback, and share suggestions for product improvement.
* Engage with the Stable Jack team on their **Discord community** and **Twitter**.