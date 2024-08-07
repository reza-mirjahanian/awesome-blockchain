# Mendi Finance
Mendi Finance is an EVM compatible **lending/borrowing protocol** on **Linea**. Mendi Finance provides peer-to-peer lending solutions that are **fully decentralized**, transparent and non-custodial. Similar to (and based on) existing lending platforms like **Compound Finance** and **AAVE** users will be able to lend any supported assets on our platform, and use their capital to borrow supported assets.

Mendi aims to be the prime lending protocol on Linea by offering highest competitive incentives for money markets, having the deepest liquidity and being native to Linea.

Mendi Finance is going to release further products to enhance and simplify user experience. **Our goal is to simplify the different trading strategies available within a lending protocol** for all of our users and reward those with further incentives who contribute the most to our community.

Mendi Finance also aims to connect retail investors with institutional DeFi through the Mendi money markets. Our aim is to enable our users to generate more yield eFi products. By leveraging this huge potential market there is great upside to the community for the next bull run, and this can enable the community to generate high and secure yield on their assets.


## Liquidations


----------------

Liquidation happen by calling the liquidateBorrow function for the specific user, specific collateral and specific cToken.

Liquidators are responsible to track all user's balances and to make sure the users AccountLiquidity is negative.

There are multiple guides available to build your own liquidation bot, but it is important to consider it is a very competitive space where success is dependent on gas efficiency of the contract and latency.


Live Liquidation Dashboard


------------------------------

All liquidations are tracked live by the Intotheblock RIsk Dashboard. It is available on the following links, and users can check the liquidation history to see if they have been liquidated. There is no notification in the UI about liquidation, but sudden balance changes usually indicate potential liquidations.

[IntoTheBlock Risk Radar - Real-time Economic Risk Analysis for DeFi - Stay One Step Ahead](https://defirisk.intotheblock.com/metrics/linea/mendi)

[Mendi Finance](https://mendi.finance/analytics/)

Users interested can find an example transaction for a liquidation on this Lineascan link: [https://lineascan.build/tx/0x86299c0a359f79dc817574019f1198ff2c459530e71a1b71a6228135d75fb154](https://lineascan.build/tx/0x86299c0a359f79dc817574019f1198ff2c459530e71a1b71a6228135d75fb154)

Liquidation Details


-----------------------

Liquidation is determined by borrow collateral factors (used to determine initial borrowing capacity).

When an account’s borrow balance exceeds the limits set by collateral factors, it is eligible for liquidation. A liquidator (a bot, contract, or user) can call the absorb function, which relinquishes ownership of the accounts collateral, and returns the value of the collateral, minus a penalty (liquidation factor), to the user in the base asset. The liquidated user has no remaining debt, and typically, will have an excess (interest earning) balance of the base asset.

Each absorption is paid for by the protocol’s reserves of the base asset. In return, the protocol receives the collateral assets. If the remaining reserves are less than the target, liquidators are able to buy the collateral at a discount using the base asset, which increases the protocol’s base asset reserves.

Our liquidation process is permissionless, and everyone is eligible to set up a liquidator to decrease the amount of bad debt in the protocol.


Linea Surge is live! To celebrate the launch of LXP-L, we’re introducing MLP-L to match, which can be used to multiply one’s exposure to the potential future rewards. Through the Mendi Loyalty Points system, we have successfully distributed the Pyth airdrop to the community - and there might be more coming in the future! **MLP-L = MLP-Liquidity** MLP-L works the same as the traditional Mendi Loyalty Points, but can be accumulated by _supplying_ assets to Mendi Finance (rather than _borrow_ with traditional MLP). In case of future rewards the protocol receives, we will distribute it proportionally to MLP-L and MLP holders.

Mendi Finance has seen immense growth after the launch of MLP, with TVL and total protocol borrows both surpassing $30 million just a few months after. The success of MLP can be tied to its unique non-inflationary model, securing a user’s share of potential future rewards for their contribution like no other point system out there.

MLP-L adds on to this fair and apparent successful system, further incentivizing Linea Surge participants to take part in the full Mendi ecosystem to take advantage of maximizing rewards.

About MLP-L


---------------

MLP-L has the same value and utility as MLP — sharing the potential future Linea airdrop to holders proportionally.

**Where does MLP-L come from?** The original MLP system was designed in a way that at the final milestone, 50% of potential future rewards will be given to the community while the other 50% goes to protocol reserves.

With the introduction of MLP-L, we’re shifting 10% of the protocol reserve’s over to the community making the new ratio 60–40. Using this methodology, we still ensure that value of MLP does not inflate in the end of the day.

MLP-L Distribution


----------------------

200,000 MLP-L (10% shifted from protocol reserve’s share) will be distributed proportionally among suppliers at the end of each Volt during the Linea Surge. At the end of Volt 6, all MLP-L will be distributed. One Volt will last approximately one month.

Just like MLP, we are using on-chain data to calculate the time-weighted pro-rata for each address’ total supply balance between each Volt. This means that the amount of time your supply assets for will directly impact the amount of MLP-L you earn for that Volt. Your total supply $-value between a given Volt will matter. This prevents gaming the system and rewards those who commit to supplying assets over time. If you have assets supplied already, they will be accounted for at the start of the Surge, no need to withdraw and resupply. MLP-L time-weighted calculations will automatically begin at the start of the Volt.

MLP-L distribution will follow Linea’s early adopter multiplier. The earlier you contribute to supply, the more MLP-L and potential future airdrop you will receive. It’s worth to act on time.