[# Detailed Breakdown of the Text](https://github.com/anza-xyz)
https://x.com/HypoNyms/status/1866584138054242683
https://tempxyz.substack.com/p/solanas-largest-existential-risk


## **Meeting Information**
- **Event**: Solana Foundation Validator Discussion  
- **Date**: December 12, 2024  
- **Agenda Overview**:
  - Validator updates
  - Educational workshops for 2025
  - Presentation by Ashwin from Anza on slashing (SIMD 204)
  - Discussion on MEV (Miner Extractable Value) pools and sandwiching
  - Open discussion for additional topics

---

## **Validator Updates**
### **Agave Validator**
- **Recommended Version**: `2.0.8`
  - Fixes a bug related to snapshot sizes.
  - Addresses a **resource leak** in the accounts DB.
  - **Performance Improvements**:
    - Dramatic improvements over previous versions.
    - Impacts vote credits positively.
  - **Recommendation**: Upgrade to `2.0.8` or later for better performance and vote credits.

### **Firedancer Validator**
- **Recommended Version**: `202.2016`
  - Not many operators are on Firedancer yet, but updates are expected to increase adoption.
  - Future announcements will include recommendations for both Agave and Firedancer versions.

---

## **Feature Activations**
### **Recently Activated Features**
- **TVC (Transaction Version Control)**:
  - Pushed by Netu and others in the community.
- **Partition Epoch Rewards**:
  - Calculates epoch rewards over multiple slots instead of the first slot.
  - Reduces delays at the start of epochs.

### **Upcoming Activations**
- **SIMD 148**:
  - Expected activation: Early 2025.
  - Currently 5th in the activation queue.
- **Full Prior Fees to Validators**:
  - Expected activation: Early 2025 (following SIMD 148).
  - Timeline: Approximately 2 weeks between feature activations.
- **Get Epoch Stakes**:
  - Helps governance by tracking validator stakes.
  - Slated for Solana version `2.1`.
  - Expected timeline: Q2 2025 (around April).

### **Activation Resources**
- Feature activation schedules are available online.
- Link to the schedule was shared during the meeting.

---

## **Testnet Updates**
- **Recommended Versions**:
  - **Agave**: `21.15`
  - **Firedancer**: `202.2016`
- **Status**:
  - Testnet was recently down but has been restored.
  - Operators were encouraged to practice restarting and building snapshots locally instead of relying on external sources.
- **Future Plans**:
  - Planned testnet swap to have all validators running on Firedancer.
  - Announcements regarding this will follow soon.

---

## **Educational Workshops**
- **Next Workshop**: January 2025 (third week).
- **Call for Topics**:
  - Suggestions for topics are welcome.
  - Ideas include:
    - Firedancer-related topics.
    - Best practices for validators.
    - Performance metrics (vote performance, replay performance, etc.).
- **Proposed Topics**:
  - Deep dive into performance metrics:
    - Understanding skip rates, vote latency, and vote credits.
    - Identifying key metrics for optimization.
    - Zen Performance Pool metrics.

---

## **Slashing Proposal Presentation (Ashwin from Anza)**
### **Overview**
- **Purpose**: Address protocol violations that are difficult to detect in real time.
- **Violations Covered**:
  - Submitting two blocks for the same leader slot (duplicate blocks).
  - Double voting or violating the voting protocol.

### **SIMD 204: Observability**
- **Key Features**:
  - On-chain program to record violations.
  - Validators or observers can submit proof of violations via transactions.
  - Reports are validated and stored in report accounts on-chain.
  - Useful for:
    - Validator dashboards.
    - Stake pools to evaluate validator behavior.
- **Initial Focus**:
  - **Duplicate Blocks**:
    - Malicious validators submitting multiple versions of the same block.
    - Already detected by the protocol but will now be recorded on-chain.
  - **Voting Violations**:
    - Rules for voting on forks and switching votes will be enforced.
    - Observed violations will be reported on-chain.

### **Implementation and Rollout**
- **Reporting Program**:
  - Tools will be provided to attach to validators or run as standalone clients.
  - Validators can choose whether to participate in reporting violations.
- **Timeline**:
  - Reporting program implementation: Early 2025.
  - Economic slashing discussions: To begin shortly via GitHub.
  - Full economic slashing rollout: Late summer 2025 (earliest).

### **Slashing Economics**
- **Principles**:
  - Avoid slashing for one-off operator errors.
  - Take extreme action (e.g., 100% slashing) if consensus is at risk.
  - Address intermediate cases with a **quadratic formula**:
    - Example: If 5% of stake commits violations, 1% of their delegations will be slashed.
    - If one-third or more of stake violates, 100% slashing applies.
- **Buffer Consideration**:
  - Use the Nakamoto coefficient line (~1%) as a buffer to avoid slashing minor violations.

### **Open Questions and Feedback**
- **Rewards for Reporting**:
  - No rewards planned due to potential front-running risks by leaders.
- **Violation Reporting**:
  - Reports can be submitted up to one epoch (432,000 slots) after the violation.
- **Unstaking Before Reporting**:
  - A follow-up SIMD will address validators unstaking before violations are reported.
- **Dashboard for Violations**:
  - Planned dashboards will display violations using on-chain data.

---

## **Discussion on MEV Pools and Sandwiching**
### **Overview**
- **Issue**: Concerns about sandwiching bots profiting from MEV.
- **Claims**:
  - Some bots are making millions per day by exploiting MEV.
  - Centralization of MEV profits is a concern for the ecosystem.

### **Proposed Solutions**
1. **Whitelist Validators**:
   - Only allow transactions to be sent to trusted validators.
   - **Concerns**:
     - Introduces permissioned elements to the network.
     - Slower transaction inclusion for users.
2. **Re-enable Public Mempool**:
   - Makes MEV opportunities more competitive.
   - **Concerns**:
     - Opens up sandwiching to everyone.
     - May not resolve centralization issues.

### **Community Feedback**
- **Against Whitelists**:
  - Permissionless principles of blockchain should be preserved.
  - Users prioritize fast inclusion over MEV protection.
- **Protocol-Level Solutions**:
  - Long-term fixes should focus on protocol changes rather than temporary solutions.
  - Examples: Multi-proposer models or other systemic changes.
- **Data-Driven Approach**:
  - Decisions should be based on accurate data rather than reactive measures.

---

## **Closing Notes**
- **Next Meeting**: January 2025.
- **Topics for Follow-Up**:
  - Slashing discussions on GitHub.
  - MEV and sandwiching concerns.