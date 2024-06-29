## Active set[​](https://docs.osmosis.zone/overview/educate/terminology#active-set "Direct link to heading")

The top 150 validators that participate in consensus and receive rewards.

## Air drops[​](https://docs.osmosis.zone/overview/educate/terminology#air-drops "Direct link to heading")

A transfer of free cryptocurrency from a crypto project into users’ wallets in order to increase interest and incentivize the use of a new token.

## Arbitrage[​](https://docs.osmosis.zone/overview/educate/terminology#arbitrage "Direct link to heading")

To profit from price differences across different markets. Arbitrageurs buy coins in one market and sell them on another market for a higher price.

## Blockchain[​](https://docs.osmosis.zone/overview/educate/terminology#blockchain "Direct link to heading")

An unchangeable ledger of transactions copied among a network of independent computer systems.

## Blocks[​](https://docs.osmosis.zone/overview/educate/terminology#blocks "Direct link to heading")

Groups of information stored on a blockchain. Each block contains transactions that are grouped, verified, and signed by validators.

## Bonded validator[​](https://docs.osmosis.zone/overview/educate/terminology#bonded-validator "Direct link to heading")

A validator in the  [active set](https://docs.osmosis.zone/overview/overview/terminology.md#active-set)  participating in consensus. Bonded validators earn rewards.

## Bonding[​](https://docs.osmosis.zone/overview/educate/terminology#bonding "Direct link to heading")

When a user delegates OSMO to a validator to receive staking rewards and in turn obtain voting power. Validators never have ownership of a delegator's OSMO. Delegating, bonding, and staking generally refer to the same process.

## Burn[​](https://docs.osmosis.zone/overview/educate/terminology#burn "Direct link to heading")

The permanent destruction of coins from the total supply.

## Commission[​](https://docs.osmosis.zone/overview/educate/terminology#commission "Direct link to heading")

The percentage of staking rewards a validator will keep before distributing the rest of the rewards to delegators. Commission is a validator’s income. Validators set their own commission rates. As of this writing, commission must be greater than or equal to 5%

## Community pool[​](https://docs.osmosis.zone/overview/educate/terminology#community-pool "Direct link to heading")

A special fund designated for funding community projects. Any community member can create a governance proposal to spend the tokens in the community pool. If the proposal passes, the funds are spent as specified in the proposal.

## Consensus[​](https://docs.osmosis.zone/overview/educate/terminology#consensus "Direct link to heading")

A system used by validators or miners to agree that each block of transactions in a blockchain is correct. The Osmosis blockchain uses Tendermint consensus engine. Validators earn rewards for participating in consensus. Visit the  [Tendermint official documentation site](https://docs.tendermint.com/)  for more information.

## Cosmos-SDK[​](https://docs.osmosis.zone/overview/educate/terminology#cosmos-sdk "Direct link to heading")

The open-source framework the Osmosis blockchain is built on. For more information, check out the  [Cosmos SDK Documentation](https://docs.cosmos.network/).

## dApp[​](https://docs.osmosis.zone/overview/educate/terminology#dapp "Direct link to heading")

An application built on a decentralized platform (short for decentralized application).

## DDoS[​](https://docs.osmosis.zone/overview/educate/terminology#ddos "Direct link to heading")

Distributed Denial of Service attack. When an attacker floods a network with traffic or requests in order to disrupt service.

## DeFi[​](https://docs.osmosis.zone/overview/educate/terminology#defi "Direct link to heading")

Decentralized finance. A movement away from traditional finance and toward systems that do not require financial intermediaries.

## Delegate[​](https://docs.osmosis.zone/overview/educate/terminology#delegate "Direct link to heading")

When a user bonds OSMO to a validator to receive staking rewards and in turn obtain voting power. Validators never have ownership of the bonded OSMO. Delegating, bonding, and staking generally refer to the same process.

## Delegator[​](https://docs.osmosis.zone/overview/educate/terminology#delegator "Direct link to heading")

A user who delegates, bonds, or stakes OSMO to a validator to earn rewards.

## Fees[​](https://docs.osmosis.zone/overview/educate/terminology#fees "Direct link to heading")

-   **Gas**: Computed fees added on to all transactions to avoid spamming. Validators set minimum gas prices and reject transactions that have implied gas prices below this threshold.

## Full node[​](https://docs.osmosis.zone/overview/educate/terminology#full-node "Direct link to heading")

A computer connected to the Osmosis mainnet able to validate transactions and interact with the Osmosis blockchain. All active validators run full nodes.

## Generalized Solidly Stableswap[​](https://docs.osmosis.zone/overview/educate/terminology#generalized-solidly-stableswap "Direct link to heading")

Generalized Solidly Stableswap: A type of Stableswap that offers low slippage for multiple assets that are expected to maintain a tight correlation. It uses a mathematical formula to maintain a stable price ratio and reduce slippage, but there is still some price impact for each trade. If the liquidity becomes imbalanced, the slippage can increase. This Stableswap is implemented using the Solidly Stableswap curve and is generalized to support multiple assets.

## Governance[​](https://docs.osmosis.zone/overview/educate/terminology#governance "Direct link to heading")

Governance is the democratic process that allows users and validators to make changes to the Osmosis protocol. Community members submit, vote, and implement proposals.

## Governance proposal[​](https://docs.osmosis.zone/overview/educate/terminology#governance-proposal "Direct link to heading")

A written submission for a change or addition to the Osmosis protocol. Topics of proposals can vary from community pool spending, software changes, parameter changes, or any idea pertaining to the Osmosis protocol.

## Inactive set[​](https://docs.osmosis.zone/overview/educate/terminology#inactive-set "Direct link to heading")

Validators that are not in the  [active set](https://docs.osmosis.zone/overview/overview/terminology.md#active-set). These validators do not participate in consensus and do not earn rewards.

## IBC[​](https://docs.osmosis.zone/overview/educate/terminology#ibc "Direct link to heading")

The inter-blockchain communication protocol (IBC) creates communication between independent blockchains. IBC achieves this by specifying a set of structures that can be implemented by any distributed ledger that satisfies a small number of requirements. IBC facilitates cross-chain applications for token transfers, swaps, multi-chain contracts, and data sharding. At launch, Osmosis utilizes IBC for token transfers. Over time, Osmosis will add new features that are made possible through IBC.

## Impermanent Loss[​](https://docs.osmosis.zone/overview/educate/terminology#impermanent-loss "Direct link to heading")

Liquidity providers earn through fees and special pool rewards. However, they are also risking a scenario in which they would have been better off holding the assets rather than supplying them. This outcome is called impermanent loss. Impermanent loss is the net difference between holding the asset verses providing liquidity. Liquidity provider (LP) rewards helps to offset impermanent loss for LPs. When the price of the assets in the pool change at different rates, LPs end up owning larger amounts of the asset that increased less in price (or decreased more in price). For example, if the price of OSMO goes up relative to ATOM, LPs in the OSMO-ATOM pool end up with larger portions of the less valuable asset (ATOM).

Impermanent loss is mitigated in part by the transaction fees earned by LPs. When the profits made from swap fees outweigh an LP’s impermanent loss, the pool is self-sustainable.

To further offset impermanent loss, particularly in the early stages of a protocol when volatility is high, AMMs utilize liquidity mining rewards. Liquidity rewards bootstrap the ecosystem as usage and fee revenues are still ramping up.

Osmosis has many new features and innovations in development to decrease impermanent loss.

## Jailed[​](https://docs.osmosis.zone/overview/educate/terminology#jailed "Direct link to heading")

Validators who misbehave are jailed or excluded from the validator set for a period amount of time.

## Liquidity Mining[​](https://docs.osmosis.zone/overview/educate/terminology#liquidity-mining "Direct link to heading")

Liquidity mining (also called yield farming) is when users earn tokens for providing liquidity to a DeFi protocol. This mechanism is used to offset the impermanent loss experienced by LPs. Liquidity mining rewards create an additional incentive for LPs besides transaction fees. These rewards are particularly useful for nascent protocols. Liquidity mining helps to bootstrap initial liquidity, facilitating increased usage and more fees for LPs. Information on Osmosis' incentive mining program can be found in this

## LP Tokens[​](https://docs.osmosis.zone/overview/educate/terminology#lp-tokens "Direct link to heading")

When users deposit assets into a liquidity pool, they receive LP tokens. These tokens represent their share of the total pool. For example, if Pool #1 is the OSMO< >ATOM pool, users can deposit OSMO and ATOM tokens into the pool and receive back Pool1 share tokens. These tokens do not correspond to an exact quantity of tokens, but rather the proportional ownership of the pool. When users remove their liquidity from the pool, they get back the percentage of liquidity that their LP tokens represent. Since buying and selling from the pool changes the quantities of assets within a pool, users are highly unlikely to withdraw the same amount of each token that they initially deposited. They usually receive more of one and less of another, based on the trades executed from the pool.

## Long-Term Liquidity[​](https://docs.osmosis.zone/overview/educate/terminology#long-term-liquidity "Direct link to heading")

Liquidity mining rewards tend to attract short-term “mercenary farmers” who quickly deposit and withdraw their liquidity after harvesting the yield. These farmers are only interested in the speculative value of the governance tokens that they are earning. They usually bounce between protocols in search of the best yield. Mercenary farmers often create the mirage of protocol adoption, but when these farmers leave, it results in significant liquidity volatility. Users of the AMM have difficulty executing trades without encountering slippage. Therefore, long-term liquidity is crucial to the success of an AMM. Osmosis’ design includes two mechanisms to incentivize long-term liquidity:  [exit fees](https://docs.osmosis.zone/overview/overview/getting-started.md#swap-fees)  and  [bonded liquidity gauges](https://docs.osmosis.zone/overview/overview/getting-started.md#bonded-liquidity-gauges).

## Market swap[​](https://docs.osmosis.zone/overview/educate/terminology#market-swap "Direct link to heading")

A swap in Osmosis that uses the Osmosis protocol's market function.

## Module[​](https://docs.osmosis.zone/overview/educate/terminology#module "Direct link to heading")

A section of the Osmosis core that represents a particular function of the Osmosis protocol.

## Pools[​](https://docs.osmosis.zone/overview/educate/terminology#pools "Direct link to heading")

Groups of tokens. Supply pools represent the total supply of tokens in a market.

## Proof of Stake[​](https://docs.osmosis.zone/overview/educate/terminology#proof-of-stake "Direct link to heading")

Proof of Stake. A style of blockchain where validators are chosen to propose blocks according to the number of coins they hold.

## Rewards[​](https://docs.osmosis.zone/overview/educate/terminology#rewards "Direct link to heading")

Revenue generated from fees given to validators and delegators.

## Scaling Factor[​](https://docs.osmosis.zone/overview/educate/terminology#scaling-factor "Direct link to heading")

Scaling factors are introduced to set the expected price ratio for Stableswaps. Precision of pegged coins, differences in staking derivatives, and other issues make it difficult to simply rely on x_reserves ~= y_reserves. Scaling factors are used to map from raw coin units to AMM math units by dividing the raw units by the scaling factor. This mapping is done by rounding the result based on the rounding mode which has three options: RoundUp, RoundDown, and RoundBankers. All token inputs are scaled and the AMM equations must ensure that rounding is done correctly. The details of scaling and rounding are described in the relevant sections of the specification.  [here](https://github.com/osmosis-labs/osmosis/blob/main/x/gamm/pool-models/stableswap/README.md)

## Self-delegation[​](https://docs.osmosis.zone/overview/educate/terminology#self-delegation "Direct link to heading")

The amount of Osmo a validator bonds to themselves. Also referred to as self-bond.

## Slashing[​](https://docs.osmosis.zone/overview/educate/terminology#slashing "Direct link to heading")

Punishment for validators that misbehave.

## Slippage[​](https://docs.osmosis.zone/overview/educate/terminology#slippage "Direct link to heading")

The difference in a coin's price between the start and end of a transaction.

## Stake[​](https://docs.osmosis.zone/overview/educate/terminology#stake "Direct link to heading")

The amount of Osmo bonded to a validator.

## Staking[​](https://docs.osmosis.zone/overview/educate/terminology#staking "Direct link to heading")

When a user or delegator delegates and bonds Osmo to an active validator in order to receive rewards. Bonded Osmo adds to a validator's stake. Validators provide their stakes as collateral to participate in the consensus process. Validators with larger stakes are chosen to participate more often. Validators receive staking rewards for their participation. A validator's stake can be slashed if the validator misbehaves. Validators never have ownership of a delegator's Osmo, even when staking.

## Tendermint consensus[​](https://docs.osmosis.zone/overview/educate/terminology#tendermint-consensus "Direct link to heading")

The consensus procedure used by the Osmosis protocol. First, a validator proposes a new block. Other validators vote on the block in two rounds. If a block receives a two-thirds majority or greater of yes votes in both rounds, it gets added to the blockchain. Validators get rewarded with the block's transaction fees. Proposers get rewarded extra. Each validator is chosen to propose based on their weight. Checkout the  [Tendermint official documentation](https://docs.tendermint.com/)  for more information.

## Osmosis[​](https://docs.osmosis.zone/overview/educate/terminology#osmosis "Direct link to heading")

The official source code for the Osmosis protocol.

## Osmosis mainnet[​](https://docs.osmosis.zone/overview/educate/terminology#osmosis-mainnet "Direct link to heading")

The Osmosis protocol's blockchain network where all transactions take place.

## Osmosisd[​](https://docs.osmosis.zone/overview/educate/terminology#osmosisd "Direct link to heading")

A command line interface for connecting to a Osmosis node.

## Superfluid Staking[​](https://docs.osmosis.zone/overview/educate/terminology#superfluid-staking "Direct link to heading")

A process that yields additional APR based on the value of your share of the pool you have bonded liquidity for.

## Testnet[​](https://docs.osmosis.zone/overview/educate/terminology#testnet "Direct link to heading")

A version of the mainnet just for testing. The testnet does not use real coins. You can use the testnet to get familiar with transactions.

## Total stake[​](https://docs.osmosis.zone/overview/educate/terminology#total-stake "Direct link to heading")

The total amount of Osmo bonded to a delegator, including self-bonded Osmo.

## Unbonded validator[​](https://docs.osmosis.zone/overview/educate/terminology#unbonded-validator "Direct link to heading")

A validator that is not in the active set and does not participate in consensus or receive rewards. Some unbonded validators may be jailed.

## Unbonding validator[​](https://docs.osmosis.zone/overview/educate/terminology#unbonding-validator "Direct link to heading")

A validator transitioning from the active set to the inactive set. An unbonding validator does not participate in consensus or earn rewards. The unbonding process takes 14 days.

## Unbonded Osmo[​](https://docs.osmosis.zone/overview/educate/terminology#unbonded-osmo "Direct link to heading")

Osmo that can be freely traded and is not staked to a validator.

## Unbonding[​](https://docs.osmosis.zone/overview/educate/terminology#unbonding "Direct link to heading")

When a delegator decides to undelegate their Osmo from a validator. This process takes 14 days. No rewards accrue during this period. This action cannot be stopped once executed.

## Unbonding Osmo[​](https://docs.osmosis.zone/overview/educate/terminology#unbonding-osmo "Direct link to heading")

Osmo that is transitioning from bonded to unbonded. Osmo that is unbonding cannot be traded freely. The unbonding process takes 14 days. No rewards accrue during this period. This action cannot be stopped once executed.

## Undelegate[​](https://docs.osmosis.zone/overview/educate/terminology#undelegate "Direct link to heading")

When a delegator no longer wishes to have their Osmo bonded to a validator. This process takes 14 days. No rewards accrue during this period. This action cannot be stopped once executed.

## Uptime[​](https://docs.osmosis.zone/overview/educate/terminology#uptime "Direct link to heading")

The amount of time a validator has been active in a given timeframe. Validators with low up time may be slashed.

## Validator[​](https://docs.osmosis.zone/overview/educate/terminology#validator "Direct link to heading")

A Osmosis blockchain miner responsible for verifying transactions on the blockchain. Validators run programs called full nodes that allow them to participate in consensus, verify blocks, participate in governance, and receive rewards. The top 130 validators with the highest total stake can participate in consensus.

## Weight[​](https://docs.osmosis.zone/overview/educate/terminology#weight "Direct link to heading")

The measure of a validator's total stake. Validators with higher weights get selected more often to propose blocks. A validator's weight is also a measure of their voting power in governance.