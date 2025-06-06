The value of Yield Tokens and Volatile Tokens are dynamically linked to the price of the collateral asset, ensuring that the total value of all Yield Tokens and Volatile Tokens equals the overall reserve value of the collateral asset. This connection is expressed through the equation:

**The value of Collateral Asset in USD = (Total Supply of Yield Token × NAV of Yield Token) + (Total Supply of Volatile Token × NAV of Volatile Token)**, where the net asset value (NAV) is the current mint/redemption value of Yield Token or Volatile Token determined by the protocol.


Here, the Net Asset Value (NAV) represents the current mint/redemption value of Yield Token or Volatile Token as determined by the protocol. The protocol ensures that Yield Token's Δ=0, while it accrues the yield of the collateral asset. Simultaneously, it adjusts the NAV of the Volatility Token more significantly than the return of the collateral asset to uphold this invariant. This strategy enables Volatility Token to provide leveraged returns on the collateral asset without incurring funding costs

**In other words, the total value of the collateral asset in the protocol = Total market cap of Yield Token + Total market cap of Volatile Token**

**Example 1**

-   User A deposits $1000 worth of collateral assets and mints 1000 Yield Token

-   User B deposits $1000 worth of collateral assets and mints 1000 Volatile Token

**Total Pool: $2000 worth of the collateral asset**

The price of the collateral asset increases by 10%

**New Total Pool: $2200 worth of collateral asset**

-   **Yield Token Worth:** $1 (no price exposure)

-   **Volatile Token Worth:** $1.2 (all price exposure)

User A redeems 1000 Yield Tokens and receives $1000 worth of collateral assets

User B redeems 1000 Volatile Tokens and receives $1200 worth of collateral assets

**Note 1:** The price of the Volatile Token is initially $1, subject to change based on the price action of the collateral asset.

**Note 2:** Even though the price of the collateral asset increased by 10%, Yield Token has no price exposure (Δ=0), while Volatile Token increased by 20% (Δ>1).

**Example 2**

-   User A deposits $1000 worth of collateral assets and mints 1000 Yield Token

-   User B deposits $1000 worth of collateral assets and mints 1000 Volatile Token

**Total Pool: $2000 worth of the collateral asset**

The price of the collateral asset decreases by 10%

New Total Pool: $1800 worth of the collateral asset

-   **Yield Token Worth:** $1 (no price exposure)

-   **Volatile Token Worth:** $0.8 (all price exposure)

User A redeems 1000 Yield Token and receives $1000 worth of collateral assets

User B redeems 1000 Volatile Token and receives $800 worth of collateral assets

**Note 1:** The price of the Volatile Token is taken as 1$ because, at the genesis of the protocol, the price of the volatile Token will be $1, but then it will change based on the price action of the collateral asset.

**Note 2:** Even though the price of the collateral asset decreased by 10%, Yield Token has no price exposure as its Δ=0 while Volatile Token has decreased by 20% as its Δ>1.