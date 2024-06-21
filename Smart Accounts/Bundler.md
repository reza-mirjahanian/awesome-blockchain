
### What is Bundler?

Bundler is a utility designed to aggregate multiple transactions into a single transaction bundle. This aggregation allows for more efficient processing, reduced transaction costs, and improved scalability on blockchain networks.

### Why Bundler?

As blockchain technology evolves, the need for efficiency and scalability becomes more critical. Bundler addresses these needs by optimizing how transactions are processed, ensuring that multiple smaller transactions can be grouped together to save on computational resources and transaction fees.


1.  **What is a Bundler?**
    
    -   A  **bundler**  is a crucial piece of infrastructure that enables the use of smart contract accounts (also known as  **smart wallets**) instead of Externally Owned Accounts (EOAs) in Ethereum.
    -   EOAs are the default type of accounts used by Ethereum wallets, but bundlers allow us to leverage the benefits of smart contract accounts.
2.  **How Does a Bundler Work?**
    
    -   Bundlers monitor an alternative mempool where user operations (encoded as ABI structs) are waiting to be bundled into a single transaction.
    -   When multiple users initiate transactions, the bundler combines their operations into a single transaction.
    -   The bundler then uses its own Externally Owned Account (EOA) to execute this bundled transaction on behalf of the users.
    -   Importantly, bundlers cover the gas fees upfront, which simplifies the user experience.
3.  **Why Do We Need Bundlers?**
    
    -   Smart contract accounts cannot directly initiate transactions on blockchains that lack native account abstraction support.
    -   Bundlers act as intermediaries, allowing smart contract accounts to participate in transactions.
    -   They facilitate the transition from EOAs to smart contract accounts, enhancing scalability and flexibility.
4.  **Who Can Operate a Bundler?**
    
    -   According to the ERC-4337 proposal, bundlers can be run by specialized actors:
        -   Block builders with custom code.
        -   Users who relay transactions to block builders.
    -   Essentially, any node that bundles user operations can serve as a bundler.
5.  **Gas Fees and Repayment:**
    
    -   Bundlers pay the initial gas fees for executing the bundled transaction.
    -   After validation and execution, bundlers are repaid by the sender’s wallet or a paymaster contract (if the transaction is sponsored).