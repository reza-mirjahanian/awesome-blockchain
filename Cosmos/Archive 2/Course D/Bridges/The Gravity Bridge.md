The [Gravity Bridge ](https://www.gravitybridge.net/)is an on-going project currently being built by Althea with the goal to facilitate the transfer of ERC-20 tokens to interchain-based blockchains and back.

![](https://ida.interchain.io/hi-info-icon.svg)

The Gravity Bridge allows for novel applications that combine the power of the interchain with the value of assets from Ethereum.

Developers can use a Cosmos chain for computation that is expensive or impossible to perform with Ethereum smart contracts. Developers could accept Ethereum ERC-20 tokens as payment, or build an entire interchain application around Ethereum tokens.

[#](https://ida.interchain.io/academy/2-cosmos-concepts/15-bridges.html#how-it-works)How it works
-------------------------------------------------------------------------------------------------

The Gravity Bridge consists of several components:

-   **`Gravity.sol`:** an Ethereum smart contract on the Ethereum blockchain.
-   **Gravity module:** a Cosmos module designed to run on the Cosmos Hub.
-   **Orchestrator:** a program that runs on Cosmos validators, which monitors the Ethereum chain and submits events that occur on Ethereum to the interchain as messages.
-   **Relayers:** a network of nodes that compete for the opportunity to earn fees for sending transactions on behalf of the Cosmos validators.

Tokens are locked on the Ethereum side by sending them to the `Gravity.sol` smart contract. This emits an event that is observable to validators running the orchestrator. When a quorum of validators agrees that tokens have been locked on Ethereum, including the requisite confirmation blocks, a relayer is selected to send an instruction to the Gravity module, which issues new tokens. This is non-dilutive - it does not increase the circulating supply because an equal number of tokens is locked on the Ethereum side.

To transfer tokens from the Cosmos Hub to the Ethereum blockchain, tokens on the interchain network are destroyed and an equal number is released (they were previously deposited) from the `Gravity.sol` smart contract.

![](https://ida.interchain.io/hi-info-icon.svg)

The Gravity Bridge is designed to run on the Cosmos Hub. It focuses on maximum design simplicity and efficiency. The bridge can transfer ERC-20 assets originating on Ethereum to a Cosmos-based chain and back to Ethereum. Transactions are batched, with transfers netted out. This creates efficiency at scale and lowers the transaction cost for each transfer.


###
Key design components: trust in integrity

The signing of fraudulent validator set updates and transaction batches meant for the Ethereum smart contract is punished on the Cosmos chain by slashing. If the Cosmos chain is trustworthy, you can trust the Gravity Bridge operated by it as long as it operates within certain parameters.

![](https://ida.interchain.io/hi-info-icon.svg)

**Slashing** is done to penalize validators. When a validator loses a percentage of its staked tokens, the tokens were slashed as a penalty. Thus, penalties for validators can include (but are not limited to):

-   Burning some of the validator's stake.
-   Removing their permission to engage in voting for a determined time period.

Bridges to Cosmos chains derive their trustworthiness from the degree of trust associated with the Cosmos chain to which they bridge. Peg-zone validators must maintain a trusted Ethereum node. This is mandatory. This removes all trust and game theory issues that usually arise when involving independent relayers. This once again dramatically simplifies the design.

![](https://ida.interchain.io/hi-info-icon.svg)

Verifying the votes of the validator set is the most expensive on-chain operation Gravity has to perform. Existing bridges incur more than double the gas costs for signature sets as small as eight signers.