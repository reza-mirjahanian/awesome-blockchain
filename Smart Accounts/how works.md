


### How Do Smart Accounts Work?

The operation of smart accounts is facilitated by the ERC-4337 standard, which introduces the concept of  `UserOperation`. This mechanism enables users to send blockchain transactions without directly handling private keys. Instead, transactions are bundled by nodes acting as "Bundlers" and submitted to the Ethereum network as a single transaction. This process involves interaction with an entry point contract on the Ethereum blockchain, which manages the creation and operation of smart accounts based on predefined init codes. The entire system operates without the need for private keys, enhancing security and convenience


**Types of Smart Accounts**There are several types of smart accounts, each designed to cater to specific use cases and industries:

1.  **Multi-Signature Accounts**: Require multiple signatures or approvals to execute a transaction, ensuring that assets are protected and transactions are secure.
2.  **Time-Locked Accounts**: Allow users to set specific time-based rules for their assets, such as releasing funds at a future date or time.
3.  **Conditional Accounts**: Enable users to set specific conditions for their assets, such as requiring a specific signature or approval before executing a transaction.
4.  **Escrow Accounts**: Hold assets in a secure, neutral location until specific conditions are met, ensuring that both parties in a transaction are protected.
5.  **Decentralized Finance (DeFi) Accounts**: Designed for decentralized lending, borrowing, and yield farming, these accounts enable users to participate in DeFi protocols and earn interest on their assets.

# What is a Smart Account?

A Smart account is a smart contract wallet that follows the  [ERC-4337 specification](https://eips.ethereum.org/EIPS/eip-4337).

Ethereum has 2 types of accounts to transfer and receive tokens: EOAs (Externally Owned Accounts) and Contract Accounts. A smart account is a wallet managed by a contract account instead of an EOA. A smart account is a wallet type requiring  **no private keys or seed phrases**. Smart accounts rely on code instead of private keys to secure and recover wallet information.

**A smart account is a type of Web3 wallet powered by smart contracts.**

This smart account is unlocked by a 'key' - a personal account. This key can be anything from a **MetaMask wallet** or even a **In-App Walle**t and is used as a way to 'sign in' to the wallet.


## Benefits of Smart Accounts


-   Optimized transaction efficiency & batch transactions
    -   Combine multiple transactions into a single atomic transaction
-   Improved Security Features
-   Social & Multi-Sig Recovery
    -   Recover a wallet without seed phrases/passwords
    -   Nominate a trusted person to recover your wallet in the case of an emergency
-   Programmability
    -   Smart accounts can be programmed to do anything a smart contract can do
    -   Smart accounts can be upgraded to add new features

## Account Factories
`
In order to issue smart accounts for users, an account factory contract must be used. This factory contract is responsible for deploying individual user contracts when required. The SDK provides a global factory ready to use, but you can also deploy your own factory.


### Personal Wallet/Key



This is the default admin on an account or the "key" to an account. It can be any wallet and is used to initialize the account. Only one wallet can only be the "key" to one account per factory contract.

This wallet is the primary way to access and interact with the account.

### Account



The account is the ERC-4337 compatible smart contract which holds all of the assets.

### UserOperations



This is the data structure for the "pseudo-transaction" that the user wants to perform. The transaction is pseudo as Smart Accounts cannot initiate transactions on EVM chains as they are not supported natively. It contains the following fields:

-   `sender`: The account making the operation.
-   `nonce`: Anti-replay parameter; also used as the salt for first-time account creation.
-   `initCode`: The initialization code needed to create the account (needed if and only if the account is not yet onchain).
-   `callData`: The data to pass to the  `sender`  during the operation.
-   `callGasLimit`: The amount of gas to allocate for the operation.
-   `verificationGasLimit`: The amount of gas to allocate for the verification step.
-   `preVerificationGas`: The amount of gas to pay to compensate the bundler for pre-verification execution and calldata.
-   `maxFeePerGas`: Maximum fee per gas (similar to  [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)  `max_fee_per_gas`).
-   `maxPriorityFeePerGas`: Maximum priority fee per gas (similar to  [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)).
-   `paymasterAndData`: Address of the paymaster sponsoring the transaction, followed by extra data to send to the paymaster (empty for self-sponsored transaction).
-   `signature`: Data passed into the account along with the nonce during the verification step. Should depend on the  `chainid`  and  `EntryPoint`  address to prevent replay attacks.

### EntryPoint


The EntryPoint is a singleton contract (a contract that has a predictable address that is the same on every chain). It has two methods that are used as entry points to execute bundles of UserOperations:  `handleOps`  and  `handleAggregatedOps`.

### Bundler (relayer)



A bundler is a node that monitors the alternative mempool of  `UserOperations`  and bundles multiple  `UserOps`  together to forward to the EntryPoint contract as a single transaction. These  `UserOps`  can be sent from different accounts and are bundled and sent to the  `EntryPoint`  contract via a  `handleOps`  call.

The bundler is controlled by its own EOA which initially pays for the gas fees upfront and is then repaid by either the sender or a paymaster if the transaction is sponsored.

The entry point contract then uses the  `validateOp`  and  `executeOp`  functions on the smart account contract to verify and execute the  `UserOps`  on behalf of the users.

It allows you to send transactions with smart accounts.

### Paymaster



A paymaster is a smart contract that relays transactions. It provides a service that enables a third party to pay the transaction fee on behalf of the user by funding the Paymaster contract in advance. The paymaster acts as a gas reserve which then can be used during the call execution via the  `EntryPoint`  contract. 