https://stakely.io/

## Validator Operations and Infrastructure

### Hardware Setup and Provider Strategy
Stakely employs a **split-service infrastructure** across multiple providers, including **Dedalism** and **OVH**, plus other specialized providers. This diversification minimizes single-provider risk.

**Node Placement:**
* **Critical Validator Nodes:** Hosted on Dedalism, OVH, and other specialized providers.
* **RPC Nodes:** Hosted on providers like **Hetzner**. Non-critical RPC nodes are separated for financial reasons and to maintain security for the main validator keys.

### Validated Ecosystems and Cosmos
The team validates for approximately **39 to 40 networks**, with roughly half being **Cosmos** chains. Other supported ecosystems include Polkadot, Solana, Near, Avalanche, Ethereum, Fantom, Covalent, Silo, Aptos, and Concordium.

**Cosmos Validation Advantage:**
* **Ease of Use:** Cosmos is generally considered one of the easier ecosystems to validate for.
* **Consistency:** The deployment process for a validator is **almost identical** across different Cosmos SDK-based chains.
* **Ecosystem Support:** The community provides extensive support for users, developers, and validators, including access to numerous **open-source tools**.

### Public Goods and Technical Tools

Stakely focuses on developing tools to solve common community and operational problems.

#### 1. Multi-Coin Faucet
The service provides small amounts of tokens for both **mainnets and testnets**.

**Core Purposes:**
* **New User Onboarding:** Simplifies the process for new users to try a chain without needing to navigate exchanges or acquisition steps.
* **Fee Coverage:** Solves the common problem of users having staked or swapped all their tokens, leaving them with **zero balance to pay transaction fees** (i.e., "out of gas"). This bypasses the lengthy process of using a Centralized Exchange (CEX), converting tokens, and routing via IBC.

#### 2. Load-Balanced RPC Service
A public load balancer feeds endpoints using **public RPCs** contributed by different operators. Any operator can contribute an endpoint by submitting a pull request to the Stakely GitHub repository.

#### 3. Staking Dashboard (Beta)
This tool offers an **account-centric view** for managing and tracking staking rewards across multiple ecosystems, addressing the inconvenience of managing many browser wallet extensions.

**Technical Features:**
* **Multi-Ecosystem Support:** Supports all chains Stakely validates for, regardless of the user's chosen validator.
* **Wallet Integration:** Supports connecting **Kepler** and Ethereum wallets.
* **Address Verification:** Requires signing a transaction to verify ownership of the wallet.
* **Derivation Path:** Displays all related addresses that share the same derivation path.
* **Data Saving:** Historical data is only saved from the user's first access; no historical data is available prior to initial use if the user was not already delegating to Stakely.

### Maximum Extractable Value (MEV)

Stakely supports MEV in various ecosystems.

* **Ethereum:** Runs **MEV-Boost** and supports its relays.
* **Cosmos:** Is experimenting with MEV via **Skip Protocol** on testnets (e.g., Terra testnet).

**MEV Perspective:**
MEV is seen as necessary for the ecosystem, bringing financial value to the chains. Rewards generated from MEV are planned to be distributed between the **operating team members** and the **community**.

### Staking Insurance and Decentralization Challenges

The organization recognizes the need for financial protection against validator misbehavior and slashing events.

**Current Implementation (Centralized):**
* **Mechanism:** A portion of staking rewards is manually saved into an internal fund to cover potential losses.
* **Status:** The fund has never been used.
* **Issue:** This is a **centralized "best effort"** model requiring delegator trust, which is philosophically undesirable for a decentralized ecosystem.

**Decentralization Challenges:**
* **Existing Products:** Decentralized insurance protocols (e.g., Nexus Mutual) are often **prohibitively expensive** or do not support the wide range of Cosmos chains.
* **Cost:** Achieving decentralized coverage requires paying a percentage of the entire Total Value Locked (TVL) being covered. For a TVL of $400 million, the insurance fee could amount to **hundreds of thousands of dollars**, making the service financially non-viable for the validator.
* **Automation:** A decentralized solution requires the payout decision to be **automatic** upon a slashing event, removing the decision from the centralized validator.

### The Future of Cosmos

#### Interchange Security (ICS)
ICS is viewed as an essential development to bring significant use and value to the **Cosmos Hub (ATOM)** by leasing its security to consumer chains.

**Operational Questions and Concerns:**
* **Support Load:** Concerns exist over the number of chains validators will be expected to support and if running consumer chains will be compulsory.
* **Validator Capacity:** The ability of the existing validator set to support 50+ new chains is a major operational question.

**Economic Risk of ICS:**
The model where a consumer chain pays the validator for security, with a portion of inflation going to **ATOM delegators**, creates an economic risk:
* **Sell Pressure:** Delegators who receive consumer chain tokens as airdrops/rewards may immediately **sell those tokens for ATOM** to re-stake and increase their share of future rewards.
* **Consequence:** This will likely cause **cell pressure/dumping** on new consumer chain tokens. Conversely, this forces consumer chain protocols to have **high intrinsic utility** to prevent immediate sell-offs.

#### Sovereign App Chains
The growth of fully **sovereign app chains** is expected to slow down.

**Reasons for Expected Slowdown:**
* **High Workload:** Spinning up a sovereign chain requires extensive and ongoing work, including building a dedicated validator set.
* **Validator Exit:** Validators are already exiting support for lower-value sovereign chains.
* **Alternative Preference:** New projects will increasingly opt for:
    * **ICS** (Interchain Security)
    * **Deployment on established chains** that support smart contracts and Cosmos modules (e.g., Osmosis, Juno).
    * **Modular architecture** and data availability solutions like **Celestia**.



