#### **Current State of Cross-Chain Messaging**

-   **Massive Scale**:

    -   **Wormhole** has processed over **1 billion messages** and facilitated over **$40 billion** in cross-chain token transfers.
    -   **Interoperability Protocols**:
        -   In total, these protocols have processed over **$65 billion** in cross-chain value.
-   **Security Leadership**:

    -   Interoperability protocols run the **largest bug bounty programs** and are the **most audited** in the crypto space.
    -   **Bug Bounties**:
        -   Over **$20 million** in bug bounties offered.
    -   **Audits**:
        -   More than **60 audits** have been conducted on interoperability protocols.
    -   **Standardized Security Frameworks**:
        -   Example: **Uniswap Bridge Assessment Report** found Wormhole to be the most secure protocol.
-   **Cross-Chain Messaging Use Cases**:

    -   **Explosive Growth**:
        -   Almost all crypto protocols are now considering going **multi-chain**.

* * * *

#### **Wormhole Native Token Transfers (NTT)**

-   **Introduction**:

    -   **NTT** is an open framework for **seamlessly taking tokens multi-chain**.
    -   **Key Features**:
        -   **Ownership, upgradability, and customizability** of tokens are preserved.
        -   Integrates with any kind of token, including **governance tokens**, **rebasing tokens**, **stablecoins**, **NFTs**, etc.
-   **Not a Token Standard**:

    -   **NTT** is not a token standard but a **flexible, open-source framework**.
    -   **Composability**:
        -   Can integrate into other decentralized finance (DeFi) protocols and be built upon for specific use cases.

* * * *

#### **NTT Token Transfer Models**

-   **New Tokens**:

    -   Uses a **burn-and-mint model** where tokens are burned on the source chain and minted on the destination chain.
-   **Existing Tokens**:

    -   For tokens that cannot be burned or upgraded, a **hub-and-spoke model** is used, where tokens are locked on the source chain and minted on the destination chain.
-   **Flexibility**:

    -   NTT allows for **custom configurations** based on specific token needs.

* * * *

#### **Long-Term Flexibility and Governance**

-   **Configurable Framework**:

    -   **Ownable and upgradeable contracts** give protocols full control over their deployment.
    -   **Governance Integration**:
        -   Protocols can configure security features, set **access roles**, **rate limits**, and **pause contracts** for security.
-   **Global Accounting**:

    -   Ensures **token balance integrity** across chains, enforced by **Wormhole Guardians**.
-   **Additional Verifiers**:

    -   Allows protocols to configure **custom verifiers** or third-party verifiers with **threshold attestation requirements**.

* * * *

#### **Security Features of NTT**

-   **Defense in Depth**:

    -   Comprehensive security features, including **inbound and outbound rate limits**.
-   **Queuing Mechanism**:

    -   Built-in queuing for users who hit rate limits, ensuring their transfers will go through once capacity opens up.
-   **Global Accounting**:

    -   Prevents more tokens from being transferred out of an ecosystem than have been transferred in.
-   **Custom Verifiers**:

    -   Protocols can add custom verifiers and set **M-of-N threshold requirements** for attestations.

* * * *

#### **Wormhole Stack Integration**

-   **Wormhole Products**:
    -   NTT integrates with the full **Wormhole stack** to simplify and secure multi-chain processes:
        -   **Wormhole Guardians**:
            -   19 validators attest to cross-chain messages.
        -   **ZK Verification**:
            -   Wormhole is working on **zero-knowledge (ZK) verification** with partners like AMD and ZK teams (e.g., **Succinct** and **Zoken**).
        -   **Wormhole Connect**:
            -   A UI widget that allows seamless token transfers across chains.
        -   **Wormhole Relay**:
            -   A network of delivery providers for cross-chain messages.
        -   **Wormhole Scan**:
            -   An explorer for tracking NTT transfers.

* * * *

#### **Protocols Using NTT**

-   **Examples of Protocols**:
    -   **Lido**:
        -   Will use NTT to transfer **WstETH** from Ethereum to BNB.
    -   **EtherFi**:
        -   Will use NTT to take **LRT** multi-chain.
    -   **Pike**:
        -   Will use NTT for their governance and utility tokens.
    -   **Puffer**:
        -   Will use NTT to take their **LST (Puffer)** multi-chain.
    -   **Wormhole's WToken**:
        -   NTT will power cross-chain transfers for Wormhole's native **WToken**.