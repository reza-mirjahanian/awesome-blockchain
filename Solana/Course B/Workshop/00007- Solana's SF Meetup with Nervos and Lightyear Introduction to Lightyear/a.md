### 🏢 Lightyear (Lier)

* **Role**: *A company that develops software, apps, and infrastructure for the Stellar ecosystem.*
* **Contributions**: *Contributes to Stellar's open-source infrastructure, helps companies integrate with the Stellar network, and scales the network.*
* **Product**: *Developed **StellarX**, a front-end for the Stellar decentralized exchange.*

---

### ⭐ What is Stellar?

* **Definition**: *A decentralized, permissionless, **multi-currency** public blockchain network designed to exchange value using tokens.*
* **Core Concept**: *Currencies and assets are a central, first-class citizen of the network.*

---

### ⛓️ Blockchain Infrastructure Challenges

1.  **Decentralized Operations**:
    * *The difficulty of coordinating network-wide upgrades across many independent nodes, requiring extensive community communication.*
2.  **Scaling the Network**: 📈
    * *Anticipating and meeting the future needs of the community for scale.* The Stellar consensus protocol is designed to be low on computational cost.
    * *Stellar is implementing **Lightning Channels** to handle heavy usage off-chain as the network grows.*
3.  **Scaling Data Access**: 💾
    * *The challenge of getting large amounts of data in and out of the network efficiently and inexpensively.*

---

### 🛠️ Stellar's Architecture & Solutions

* **Horizon API**:
    * *A web server that acts as an API gateway to the Stellar network.*
    * *It ingests historical data from **Stellar Core** and stores it in queryable SQL databases, allowing Stellar Core to delete old data and remain lightweight.* * **Stellar Core vs. Horizon**:
    * **Stellar Core**: *Optimized for decentralized communication between nodes.*
    * **Horizon**: *Optimized for data transfer and storage.*

---

### 💡 Why Use Stellar?

* **Low Cost** 💰: *Extremely cheap transactions; a single dollar can fund approximately **450,000** transactions, making it ideal for micropayments.*
* **Fast Speed** ⚡: *Transactions are confirmed quickly, with ledgers closing in **less than 5 seconds**.*
* **Flexible Token Creation**: *You can create tokens for anything imaginable, such as stablecoins, ICO tokens, carbon credits, or even coffee tokens.*
* **Decentralized Exchange (DEX)** 💹: *A built-in feature that allows for fast and secure trading of any token on the network.*
* **Path Payments**:
    * *An innovative feature allowing a user to send one currency and the recipient to receive another, all in a single, atomic transaction.*
    * *Example: An employee could pay for coffee using their company stock. The DEX automatically converts a small amount of the stock into the coffee vendor's preferred token at the moment of payment.*

    ---

    # Blockchain Infrastructure & Stellar Meetup – Interview Q&A

---

## 1. **Company & Ecosystem Knowledge**

### Q1. Who is speaking, what company do they represent, and what is that company’s primary mission?
**A.**  
- **Speaker:** Nick Hill  
- **Company:** Lio (pronounced “Lee-oh”)  
- **Mission:** Develop software, applications, and infrastructure **for the Stellar ecosystem**; maintain open-source contributions that **scale the network**, help **enterprise integrations**, and ship **consumer-facing products** (e.g., **StellarX** – a DEX front-end).

---

## 2. **Stellar 101 – Definition & Core Concepts**

### Q2. Give a concise, one-sentence definition of Stellar.
**A.**  
> Stellar is a **decentralized, permissionless, multi-currency public blockchain network built to exchange value using tokens**, where **assets/currencies are first-class citizens**.

---

### Q3. Why does the speaker emphasize “multi-currency” as a first-class concept?
**A.**  
Because every **asset/token** (fiat, stable-coin, loyalty point, carbon credit, etc.) is **native** to the protocol; issuance, trustlines, and atomic path-payments are **core features**, not smart-contract overlays.

---

## 3. **Infrastructure Challenges & Solutions**

### Q4. List the three main challenges of building on Stellar that Nick highlights and the corresponding solutions.
| **Challenge** | **Solution** |
|---------------|--------------|
| **1. Decentralized Operations / Upgrades** | Communicate early & often with community; publish change logs, timelines, and upgrade guides. |
| **2. Scaling Transaction Throughput** | - Low-computation **Stellar Consensus Protocol (SCP)**<br>- **Lightning channels** (in development) to move heavy usage off-chain. |
| **3. Scaling Data I/O & Querying** | **Horizon** – an **API gateway** that indexes on-chain data into **SQL tables** and exposes **JSON REST endpoints**. |

---

## 4. **Horizon Architecture Deep-dive**

### Q5. Explain Horizon’s role in the Stellar stack with an analogy to Ethereum.
**A.**  
- **Stellar Core** ≈ **Geth / Parity** (validates, reaches consensus, stores canonical ledger).  
- **Horizon** ≈ **Infura + The Graph** – provides **state & historical data** in **queryable JSON** so apps don’t need to run full archival nodes.  
Stellar Core can **prune** history because Horizon nodes **replicate & serve** it.

---

### Q6. Draw (describe) the application architecture diagram Nick showed.
**A.**  
```
┌────────────┐        ┌──────────────┐        ┌────────────┐
│  Your App  │◄──────►│   Horizon    │◄──────►│Stellar Core│
│  (Wallet,  │ JSON   │  API Server  │        │   Node     │
│  Exchange) │ REST   │  + SQL DB    │        │            │
└────────────┘        └──────────────┘        └────────────┘
```
- **Red box** = Horizon (middle layer)  
- **Green** = Stellar Core (consensus)  
- **Blue** = External apps, SDKs, web front-ends.

---

## 5. **Stellar Advantages – “Why Stellar?”**

### Q7. Recite five concrete advantages of using Stellar, with the numeric claim for cost.
**A.**  
1. **Cost:** $1 funds **≈ 450,000 transactions** – ideal for **micro-payments**.  
2. **Speed:** Ledger closes in **< 5 seconds** – daily retail use-case viable.  
3. **Tokenization:** **Unlimited custom tokens** (stable-coins, ICOs, carbon credits, coffee tokens).  
4. **Built-in DEX:** On-chain order books allow **instant, secure trading** of any token pair.  
5. **Path Payments:** **Atomic, cross-asset transfers** – sender can pay in any token, receiver gets desired token **without trust**.

---

## 6. **Path Payment Scenario**

### Q8. Walk through Nick’s Google-employee-coffee example step-by-step.
**A.**  
1. **Employee holds:** Google stock tokens (GOOG).  
2. **Wants:** Coffee priced in **CoffeeTokens**.  
3. **Path Payment Engine finds:** GOOG → USD → CoffeeTokens (best rate via DEX order books).  
4. **Atomic tx:**  
   - **Locks** GOOG → **Converts** to CoffeeTokens in-flight → **Delivers** CoffeeTokens to café.  
5. **Café may discount** because CoffeeTokens are **low-fee & fast settlement**.  
6. **Employee never holds** intermediate tokens; **single UX** – “Pay with GOOG”.

---

## 7. **Lightning on Stellar**

### Q9. What is the status of Lightning-style channels on Stellar?
**A.**  
- **In development / early implementation**; not live on main-net yet.  
- Goal: offload **high-frequency or micro-payment flows** to **state channels**, reducing on-chain load and enabling **future scalability**.

---

## 8. **Enterprise Integration**

### Q10. How does Lio help enterprises adopt Stellar?
**A.**  
- **Technical consulting:** Map business requirements to Stellar features (tokens, compliance, anchors).  
- **API & SDK guidance:** Onboard devs with **Horizon endpoints**, **Stellar SDKs**, key management.  
- **Custom integrations:** Connect legacy banking rails to **Stellar anchors** for **fiat on/off ramps**.  
- **Product co-build:** Example – StellarX front-end delivered by Lio.

---

## 9. **Open-Source & Community**

### Q11. Which open-source repos should a new developer clone first?
**A.**  
- **stellar-core** – C++ consensus & validation node.  
- **horizon** – Go implementation of the API server.  
- **js-stellar-sdk** / **java-stellar-sdk** – language SDKs.  
- **stellar-protocol** – SEPs & CAPs (standards & proposals).

---

## 10. **Developer Onboarding Resources**

### Q12. List the exact resources Nick flashes on his final slide.
**A.**  
- **GitHub:** github.com/stellar  
- **Stellar API Docs:** developers.stellar.org  
- **StellarX (Product):** stellarx.com  
- **Community Slack:** stellar-public.slack.com

---

## 11. **Hands-On Coding Challenge (Mock Interview)**

### Q13. *Live Coding* – “Write a short Node.js snippet that submits a path-payment from Alice (USD anchor) to Bob (EUR anchor) on testnet.”
**A.**  
```javascript
const StellarSdk = require('stellar-sdk');
const server = new StellarSdk.Server('https://horizon-testnet.stellar.org');

(async () => {
  const sourceKeys  = StellarSdk.Keypair.fromSecret('SALICE_SECRET');
  const destAccount = 'GBOB...';

  const tx = new StellarSdk.TransactionBuilder(
      await server.loadAccount(sourceKeys.publicKey()), 
      { fee: 100, networkPassphrase: StellarSdk.Networks.TESTNET })
    .addOperation(StellarSdk.Operation.pathPaymentStrictSend({
        sendAsset: new StellarSdk.Asset('USD', 'GANCHOR1...'),
        sendAmount: '100',
        destAsset: new StellarSdk.Asset('EUR', 'GANCHOR2...'),
        destMin: '85',
        destination: destAccount
    }))
    .setTimeout(30)
    .build();

  tx.sign(sourceKeys);
  const resp = await server.submitTransaction(tx);
  console.log('Hash:', resp.hash);
})();
```

---

## 12. **Security & Key Management**

### Q14. How should a wallet app handle key storage for non-custodial users?
**A.**  
- **Client-side encryption:** Encrypt secret key with **user passphrase** using **libsodium secretbox**.  
- **Hardware signing:** Integrate **Ledger / Trezor** for offline key storage.  
- **SEP-10 & SEP-24:** Use **web-auth & interactive deposits/withdrawals** so keys never leave user device.  
- **Rate-limit & 2FA:** For cloud recovery options, require **TOTP / WebAuthn**.

---

## 13. **Tokenomics & Compliance**

### Q15. What SEPs are relevant when issuing a regulated stable-coin on Stellar?
**A.**  
- **SEP-1** – stellar.toml (anchor & issuer metadata).  
- **SEP-6** – programmatic deposit/withdrawal for stable-coin.  
- **SEP-24** – hosted deposit/withdrawal UI for end-users.  
- **SEP-8** – regulated asset compliance checks (KYC/AML flags).  
- **CAP-18** – fine-grained authorization to freeze or clawback tokens.

---

## 14. **Performance Benchmarking**

### Q16. How would you stress-test Horizon under load?
**A.**  
1. **Spin up** a private Stellar network with **3 core nodes** + **1 Horizon**.  
2. **Use** **stellar-load-**tool or **Locust** to generate **1,000 TPS** of path-payments.  
3. **Monitor:**  
   - **p99 latency** of `/transactions` endpoint.  
   - **PostgreSQL** CPU & I/O via **pg_stat_statements**.  
   - **Horizon Go** memory profiling (`pprof`).  
4. **Iterate:** Add **read replicas**, **horizontal partitioning**, or **rate-limiting** as needed.

---

## 15. **Future Roadmap**

### Q17. Ask Nick: “What’s next for Lio after StellarX?”
**A.**  
- **Multi-asset yield products** (staking, lending on Stellar).  
- **Mobile SDK** for **non-custodial wallets** with **Lightning channel support**.  
- **Enterprise SaaS** platform for **fiat anchors** (KYC, compliance-as-a-service).