In the context of blockchain, **inflight messages** refer to **cross-chain messages or transactions that have been initiated but not yet fully processed or confirmed** on the destination chain. These messages are in transit between different blockchains or layers within a blockchain ecosystem, such as Layer 1 and Layer 2 networks or between different blockchains using a cross-chain communication protocol (e.g., LayerZero, Cosmos IBC, Polkadot).

#### Key Characteristics of Inflight Messages:

1.  **Unconfirmed State**: The message or transaction has been sent from the source chain but has not yet been finalized or acknowledged on the destination chain.
2.  **Cross-Chain Communication**: Inflight messages are common in cross-chain communication protocols where data, assets, or instructions are being transferred between distinct blockchain environments.
3.  **Pending Verification**: The message may be waiting for validation, such as proof submission (via oracles or relayers) or consensus on the destination chain.
4.  **Time Sensitivity**: Depending on the network conditions (e.g., block times, congestion), inflight messages may experience delays, which can affect the overall user experience or dApp functionality.

#### Example Scenario:

Imagine a user wants to transfer tokens from **Ethereum** to **Avalanche** using a cross-chain bridge. The process involves the following steps:

1.  The user initiates the transaction on Ethereum, locking the tokens in a smart contract.
2.  A message is sent to Avalanche (via a relayer or oracle) to mint the equivalent tokens on Avalanche.
3.  While this message is in transit and waiting to be confirmed on Avalanche, it is considered **inflight**.
4.  Once the transaction is confirmed on Avalanche, the message is no longer inflight, and the tokens are available on the destination chain.