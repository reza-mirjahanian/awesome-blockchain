https://glittr-whitepaper.s3.us-east-2.amazonaws.com/Glittr+Whitepaper.pdf



Glittr is a new protocol built on Bitcoin to enable decentralized finance (DeFi) applications. It provides key features like automated market **makers (AMMs), lending, staking, and token** swapping, making it easier to build these systems on Bitcoin. The protocol is governed by the community, allowing flexibility for future updates and new integrations.

Glittr focuses on being transparent, decentralized, and minimizing the need for trust---meaning you can use it without relying on any central authority. It is based on Bitcoin's **UTXO** model, with transactions embedded in the blockchain using a method called **OP RETURN**. This allows Glittr to offer new DeFi features by creating customizable tokens (called Glyphs) that can be tailored to different use cases. These Glyphs can also be combined to create more complex DeFi applications.

----------
The Challenge of DeFi on Bitcoin The comprehensive DeFi ecosystem that exists on Ethereum does not have a corollary on Bitcoin. There are many reasons for this, including throughput, but the biggest contributor is that the two blockchains (and their respective protocols) simply have significantly different designs. Where the Ethereum protocol could be seen as a collection of bank accounts, Bitcoin should be seen as a series of IOUs (read: UTXOs). These singular, persistent entities inherent in Ethereum make smart contracts as we know them possible. Bitcoin, by comparison, operates using a chain-of-custody model. While a user may choose to re-use a persistent public key1 , UTXOs must still be fully spent, OP codes are limited in complexity, and transactions cannot be triggered from within the chain itself as you would see with an EVM-based smart contract

-----

While validating transactions is a necessary step in Bitcoin mining, validating Bitcoin transactions is very cheap (O(1)). The computational expense of Bitcoin is derived from the mining itself, whereas Ethereum's proof-of-stake model allows significant computational resources to process smart contracts in a distributed fashion. With Bitcoin you pay for block space, not computation


---
Smart Contract on Bitcoin" solutions which seek to replicate the functionalities and structure of smart contracts on EVM are in fundamental tension with the structure of Bitcoin. The introduction of a VM that can process arbitrary smart contracts necessarily removes validation and enforcement of such contracts from the Bitcoin blockchain. Because the computation cannot be deferred to the validation consensus of the mining network these solutions instead rely on centralized, off-chain computation with the results of the contract then written to a Bitcoin transaction. The problem with computation leaving the chain - even if it is multi-party or provable, as some solutions propose - is that trustless computation does not imply trustless transactions. 2 For a contract to be enforced, the enforcing party must have access to the underlying assets. On Ethereum this is acceptable because the contract itself is the enforcer, but with Bitcoin there will always be a third party (i.e. a company) that facilitates the bridge to the VM. At the very least a VM-based solution introduces an oracle problem. For more complex contracts not achievable through DLCs, the third party may become custodial.
-------

Glittr Protocol Instead of attempting to create "Smart Contracts on Bitcoin", Glittr attempts to reframe - do we need Smart ContractsTM or do we simply need a system that enables DeFi? Glittr enables DeFi by creating a structure that works within the UTXO-based framework of Bitcoin, allowing all validation and enforcement to stay directly on Bitcoin Layer 1. Glittr relies on decentralization and consensus as its enforcement mechanism as opposed to provable or self-executing code. After all, no amount of cryptography can save you from a 51% attack.


Glittr is a system that uses templates to create tokens called Glyphs. These Glyphs are used to facilitate specific use cases in DeFi applications. Each Glyph has unique, configurable arguments that serve as validation instructions for Bitcoin nodes.

**Key Features**

-   Glyphs are fungible tokens, but their primary purpose is utility, not speculation.
-   Glittr transactions are mined before validation, ensuring Bitcoin-level finality.
-   The system is reactive, with users issuing transactions that must be consistent with a Glyph's constraints.
-   Validators confirm transaction consistency, and valid transactions become state.
-   
----

**Glyphs**

Glyphs are created using templates that define specific validation rules for transactions. There are three initial templates:

1.  **Pool Equity Glyphs**: Represent fractional equity in a liquidity pool.
2.  **Wrapped Runes**: A wrapped version of Runes.
3.  **Collateralized Glyphs**: Tokens minted against collateral, such as stablecoins.

Glyphs without a template have the same properties as Runes.