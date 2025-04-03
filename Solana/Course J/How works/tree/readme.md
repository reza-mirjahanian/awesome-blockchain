**Why Build on Solana?**
========================

Solana is a leading blockchain platform that offers developers **high speed** (currently capable of processing [2000--3000 TPS](https://explorer.solana.com/)), **low fees** (which can be as low as **$0.00025 per transaction**), and flexible tools for building decentralized applications (dApps). This exceptional performance stems from Solana's innovative architecture, which utilizes a **Solana Virtual MachineSVM**** *that is built upon* ***Sealevel*** and ***Proof of History (PoH)*** algorithm*.*


Sealevel:
---------------------------------------------------------------------------

**Sealevel** Multi-threaded runtime maximizes performance by efficiently utilizing hardware through parallel processing across validator cores. This enables better scalability as hardware advances. Unlike EVM chains with global fees, Sealevel facilitates localized fee markets, allowing fees to be tailored per smart contract, preventing unrelated transactions (like NFT minting) from impacting swap or DeFi fees.
![alt text](image.png)

**Gulfstream:**
---------------

A mempool-less transaction forwarding protocol that optimizes network load and allows validators to begin executing transactions ahead of time (*Since every validator knows the order of upcoming leaders*), further boosting speed and efficiency.

[**Turbine:**]
--------------------------------------------------------------------------------------------------------------------------------------

Think of Turbine as a block propagation mechanism. It efficiently broadcasts ledger entries to all nodes by breaking down transaction data into smaller packets. These packets are then propagated across the network by nodes, with each node in a layer responsible for forwarding data to a small set of nodes in the subsequent downstream layer. This approach minimizes communication overhead and facilitates faster transaction processing and network scalability within the Solana ecosystem.

![alt text](image-1.png)


**Proof of History (PoH):**
---------------------------

This cryptographic clock embedded within the SVM timestamps events securely, ordering transactions efficiently and contributing to **fast block times** of around 400 milliseconds.

The diagram provides a simplified overview of the Solana Consensus Algorithm:
![alt text](image-2.png)

Program Derived Addresses
=========================

Let's explore how Program Derived Addresses (PDAs) are generated on Solana:

**Key Principles**

-   **Deterministic.** PDAs are not randomly generated. They are calculated based on specific inputs, ensuring a consistent outcome.
-   **Seeds.** The core input for PDA generation is an array of seeds. Seeds can be practically anything: **strings**, **numbers**, **public keys**, etc.
-   `programId.` Every PDA is linked to the ID of the program that controls it.
-   **Bump Seed.** A special 'bump seed' is used to prevent collisions. Solana searches for a bump seed that, when combined with the other seeds and `programId`creates a valid PDA lying off the `ed25519` curve (meaning it doesn't have a corresponding private key).
-   `findProgramAddress.`will return both the address and the bump used to kick the address off of the elliptic curve.


![alt text](image-3.png)