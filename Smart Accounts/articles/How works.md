## How Do Smart Wallets Work?

**The key innovation behind smart wallets is  account abstraction**

Traditional crypto wallets are often associated with Externally Owned Accounts (EOAs). These accounts rely on a single private key  for access and transaction signing, where users have to authorize every interaction with the blockchain on their account.

On the flip side, smart wallets are built as dApps that use smart contracts on the blockchain to enable users to manage their digital assets. This fundamental shift provides immense flexibility and potential for new features, with the following key elements:

-   **Storage:**  Stores the user's assets (tokens, NFTs, etc.) and keeps track of the account's nonce (a number used to prevent replay attacks).
    
-   **Logic:**  This is the executable code that governs the wallet's behavior. It defines rules for:
    
   -   **Batching**: Allowing multiple operations from the same user in a single transaction, where the first operation is part of the input to the second operation.
        
    -   **Authorization**: Who can initiate transactions (e.g., multi-sig requirements, time-based restrictions).
        
    -   **Transaction Validation:**  What types of transactions are allowed (e.g., spending limits, whitelisted addresses).
        
    -   **Gas Payment:**  How transaction fees are handled (e.g., paying with alternative tokens, sponsored transactions).
        

Not all smart wallets are the same, and while ERC-4337 – the intended solution for account abstraction – is not live yet, it has already been approved through Ethereum's on-chain governance. ERC-4337 avoids the need for consensus-layer protocol changes, instead using UserOperation objects that package up the user's intent, and these objects are then bundled into a single transaction which will be included into an Ethereum block.

In the meantime, smart wallets are still being built using the following three foundational technologies:

1.  EIP-5792
    
2.  ERC-6492
    
3.  ERC-7677


-----------------------
[EIP-5792](https://eips.ethereum.org/EIPS/eip-5792)  aims to standardize the interaction between decentralized applications (dApps) and wallets, particularly focusing on smart contract wallets.

This proposal seeks to improve user experience and developer workflows by simplifying how dApps request and execute complex transactions on the blockchain.

EIP-5792 introduces three new JSON-RPC methods:

1.  **_wallet_sendCalls_:**  Allows dApps to request a wallet to execute a batch of on-chain calls (transactions) in a single request.
    
2.  _**wallet_getCallsStatus:**_  Allows dApps to query the status of a previously submitted batch of calls (using wallet_sendCalls).
    
3.  **_wallet_getCapabilities_:**  Allows dApps to discover the capabilities of a connected wallet. These capabilities might include supported transaction types, fee payment options, or other advanced features.

-----------------------------------------

### ERC-6492

A big downside of smart contract wallets (SCWs) is the inability to sign messages or transactions before they are deployed on the blockchain. This is because the contract's address, which is used to derive the public key for signature verification, isn't known until after deployment.

As a result, UX takes a stab, particularly for account abstraction wallets, where users want to interact with dApps even before their wallet contract is created.

[ERC-6492](https://eips.ethereum.org/EIPS/eip-6492)  introduces a new standard for signatures that are compatible with the widely-used  [ERC-1271](https://eips.ethereum.org/EIPS/eip-1271)  (Standard Signature Validation Method for Contracts).

Here's the process:

1.  **Pre-Deployment Signature:**  Before the SCW is deployed, the user (or the wallet creation mechanism) generates a signature using the deterministic CREATE2 address of the yet-to-be-deployed contract.
    
2.  **Signature Format:**  The signature includes a special suffix (0x64926492...) to signal that it's a pre-deployment signature.
    
3.  **Verification Contract:**  A standard "Universal Validator" contract is deployed on the blockchain. This contract can validate both traditional ECDSA signatures and EIP-6492 signatures.
    
4.  **Verification Process:**  When a dApp or another contract needs to verify a signature from an SCW, it checks the signature format and if it’s an EIP-6492 signature


------------------------------

Paymasters are services that can sponsor transaction fees (gas) on behalf of users, allowing for gasless or reduced-fee transactions.

[ERC-7677](https://eips.ethereum.org/EIPS/eip-7677)  defines a standard interface for paymaster services to interact with wallets, making it easier for wallet developers to integrate paymaster functionality and for users to benefit from gasless transactions.

In simpler words, users can perform transactions without worrying about the associated gas fees, as these fees can be covered by the paymaster. This functionality is particularly beneficial for new users who may find the concept of gas fees confusing and off-putting.

ERC-7677 introduces a set of JSON-RPC methods that paymaster services must implement:

1.  **_pm_getPaymasterStubData_:**  Called by the wallet when a user is constructing a transaction. It returns "stub" values for the paymaster-related fields of the UserOperation (a data structure used in account abstraction transactions). These stub values are used for gas estimation and transaction validation.
    
2.  **_pm_getPaymasterData_:**  Called by the wallet after a user has signed a transaction. It returns the final, valid data for the paymaster-related fields of the UserOperation. This data is included in the final transaction that is submitted to the blockchain.



## Benefits of Smart Wallets

Smart wallets bring numerous benefits to the table, transforming the user experience and making blockchain technology more accessible.

Here are six key benefits of smart wallets.

### No Third-Party Installs

Smart wallets eliminate the need for users to install third-party applications or extensions.

Traditionally, setting up a crypto wallet involves downloading an app or browser extension, which can be a deterrent for less tech-savvy individuals.

With smart wallets, users can create and manage their wallets directly within their web browsers or dApps, making the process seamless and user-friendly. It reduces the friction associated with onboarding new users.

### No Seed Phrases

One of the major pain points in traditional wallets is the management of seed phrases. Users are required to securely store a series of words that serve as a backup for their wallet. Losing this phrase means losing access to their assets, and anyone with knowledge of the seed phrase can access the wallet and its contents.

Smart wallets do away with the need for seed phrases, enhancing security and ease of use. Instead, they utilize more intuitive and secure methods for account recovery, such as passkey compatibility and encrypted backups. Implementing passkeys protect against unauthorized access, ensuring that only authorized individuals can initiate transactions or access sensitive information on the wallet.

### Highly Flexible

Smart wallets are highly flexible, allowing users to manage their balances across different platforms and spending across multiple apps. Based on the user's needs, smart wallets can support a wide range of functionalities, from simple transactions to complex multi-step interactions with decentralized applications (dApps).

This flexibility makes smart wallets suitable for both beginners and advanced users. Developers can also leverage this adaptability to create customized user experiences, enhancing the utility and appeal of their dApps.

### Gas Abstraction

Smart wallets integrate paymasters, which are entities that can sponsor transactions on behalf of the user. This feature is particularly useful for enabling gasless transactions.

Users can perform transactions without worrying about gas fees, as these fees can be covered by the paymaster. This reduces the financial barrier to using blockchain applications and makes the experience more user-friendly, especially for newcomers.

_[Explore the Base Gasless Campaign here.](https://www.smartwallet.dev/base-gasless-campaign)_

### Batched Transactions

Batched transactions, enabled by EIP-5792, allow multiple actions to be bundled into a single transaction. This significantly improves efficiency and reduces costs.

For example, users can approve and execute a trade in one seamless step rather than navigating through multiple transactions.

Batched transactions streamline processes and enhance the overall user experience by making interactions more straightforward and cost-effective.

### Magic Spend

[Magic Spend](https://www.smartwallet.dev/guides/magic-spend)  is a unique feature of the Coinbase Smart Wallet that simplifies the process of spending cryptocurrency. It allows users to spend their funds directly from their Coinbase account without needing to transfer assets to their smart wallet first.

This eliminates the many steps involved in managing balances and ensures users can quickly and easily complete transactions.

## Risks of Smart Wallets

Smart wallets are undoubtedly one of the revolutionary innovations around crypto wallets. However, it is not totally immune to risks.

-   **Smart Contract Bugs:**  The smart wallets' reliance on smart contracts opens the door for possible attacks. For example, the existence of smart contract vulnerability allows attackers to access users' wallets and misuse their crypto assets.
    
-   **Malicious Guardians:**  Smart wallets collaborate with multiple third parties for various purposes like transaction relying, wallet locking or recovery, and multi-signature validating. Therefore, there exists a possibility that malicious guardians can initiate foul plays to negatively influence the smart wallet activities.
    

## Guide to Coinbase's Smart Wallet

[Coinbase](https://www.coinbase.com/), which is one of the top crypto exchanges with an asset holding of over $330 billion, launched its  [smart wallet](https://www.coinbase.com/blog/a-new-era-in-crypto-wallets-smart-wallet-is-here)  on June 5, 2024.

Coinbase's Smart Wallet is a new type of self-custody crypto wallet designed to make it easier for users to interact with DeFi. It aims to combine the simplicity of centralized wallets with the security and control of self-custody.


## Other Top Smart Wallets

1.  **OKX Smart Wallets:**  This  [smart wallet](https://www.okx.com/web3/hot/aawallet), launched by one of the largest exchanges, OKX, is the first web3 wallet to utilize multi-party computation (MPC)  [technology](https://www.okx.com/learn/okx-wallet-integrates-leading-edge-mpc-technology)  supported with 37 blockchains. In addition, its 'Emergency Escape' feature helps users recover their smart wallets using their mobile device, OKX account, or cloud backup.
    
2.  **Fuse:**  This is the first Solana-based smart wallet that allows users with mobile wallet experience with social recovery options.  [Fuse](https://fusewallet.com/)  wallet, which is initially available for the public to test on their iOS devices, uses a dual key system including a device and 2FA key to enhance the wallet security.
    
3.  **Locksmith:**  With wallet recovery mechanisms, multisig security, and automated transfer,  [Locksmith](https://locksmithwallet.com/)'s smart wallet facilitates a smooth wallet onboarding experience. The multi-call transaction feature of this smart wallet allows users to execute numerous transactions at once, helping to minimize transaction fees