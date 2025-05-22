# **Salon Foundation Validator Discussion - December 12, 2024**

---

## **Agenda Overview**
1. **Validator Updates**:
   - Recommended versions for Agave and Fire Dancer.
   - Feature activations and future plans.
2. **Educational Workshops**:
   - Call for topics for 2025.
3. **SIMD for Slashing**:
   - Presentation by Ashwin from Anza.
4. **MEV Pools and Sandwiching**:
   - Community discussion on recent concerns.
5. **Open Discussion**:
   - Other topics from the community.

---

## **Validator Updates**

### **Mainnet Updates**
- **Agave**:
  - Recommended version: **2.0.8**.
  - Fixes:
    - Snapshot size issues.
    - Resource leak in the accounts DB.
  - Performance improvements:
    - Better validator performance.
    - Increased vote credits.
- **Fire Dancer**:
  - Recommended version: **2.2002.2016**.
  - Few operators currently, but expected to grow.

### **Feature Activations**
- **Recent Activations**:
  - **TVC (Transaction Version Control)**.
  - **Partitioned Epoch Rewards**:
    - Epoch rewards calculated over multiple slots, reducing delays.
- **Upcoming Activations**:
  - **SIMD 148**: Expected in early 2025.
  - **Full Prior Fees to Validators**: Expected in early 2025.
  - **Get Epoch Stakes**: Slated for **2.1** release (Q2 2025).

---

## **Testnet Updates**
- **Recommended Versions**:
  - Agave: **21.15**.
  - Fire Dancer: **2.2002.2016**.
- **Testnet Restart**:
  - Purpose: Practice for operators in restarting and building snapshots.
  - Future plans: Swap all validators to Fire Dancer.

---

## **Educational Workshops**
- **Next Workshop**: January 2025 (third week).
- **Potential Topics**:
  - Fire Dancer setup and best practices.
  - Performance metrics and optimization.
  - Token metadata issues (raised by community).

---

## **SIMD for Slashing (Ashwin from Anza)**

### **Overview**
- **Purpose**: Detect and punish protocol violations (e.g., double voting, duplicate blocks).
- **Two-Step Process**:
  1. **On-Chain Reporting**:
     - Violations reported within one epoch.
     - Proof submitted via transaction.
  2. **Economic Slashing**:
     - Stake slashing based on violation severity.

### **SIMD 204: On-Chain Reporting**
- **Features**:
  - On-chain program for reporting violations.
  - Reports stored in PDAs (Program Derived Accounts).
  - Initial focus: Duplicate blocks.
- **Future Plans**:
  - Voting violations (e.g., double voting).
  - Economic slashing (quadratic formula proposed).

### **Economic Slashing Proposal**
- **Quadratic Formula**:
  - Slashing based on the ratio of violating stake to total stake.
  - Comparison: Ethereum uses a linear formula.
- **Thresholds**:
  - **<1% of stake**: No slashing.
  - **>33% of stake**: 100% slashing.
  - **In-between**: Quadratic slashing (e.g., 5% violation → 1% slashing).

### **Timeline**
- **Reporting Program**: Early 2025.
- **Economic Slashing**: Late summer 2025 (after governance process).

---

## **MEV Pools and Sandwiching**

### **Current Concerns**
- **Sandwiching**:
  - Claims of significant profits by centralized entities.
  - Temporal’s article highlighted the issue.
- **Proposed Solutions**:
  1. **Whitelist Validators**:
     - DApps send transactions only to trusted validators.
     - Concerns: Slower inclusion times, permissioned environment.
  2. **Re-enable GTO MEV Pool**:
     - Public MEV pool to decentralize profits.

### **Community Feedback**
- **Whitelist Concerns**:
  - Negative sentiment due to permissioned nature.
  - Users prioritize fast inclusion over MEV protection.
- **MEV Protect**:
  - Existing solution (e.g., Jito) but underutilized.
  - Users (e.g., Telegram bots) prefer speed over protection.

### **Long-Term Solutions**
- **Protocol-Level Changes**:
  - Multi-proposer models.
  - Decentralized MEV solutions.

---

## **Open Discussion**

### **Performance Metrics**
- **Discussion**:
  - Importance of understanding and optimizing metrics (e.g., skip rate, vote latency).
  - Proposal for a deep dive on performance metrics in January 2025.

### **Other Topics**
- **Token Metadata Issues**:
  - Community member raised difficulties with token metadata uploads.
  - Suggestion to address in a separate forum or Discord.

---

## **Next Steps**
- **Upcoming Calls**:
  - Next community call in January 2025.
  - Deep dive on performance metrics planned for January.
- **SIMD Discussions**:
  - GitHub discussion on SIMD 204 and economic slashing.
  - Feedback and governance process to follow.

---

## **Closing Remarks**
- **Thank You**: Ashwin for the slashing presentation, and all participants for the MEV discussion.
- **Next Meeting**: January 2025.