### **Staking Basics**

-   **Staking Accounts**: HYPE tokens can be transferred between **spot accounts** (used for trading) and **staking accounts** (used for staking).
-   **Delegated Proof of Stake (DPoS)**: The Hyperliquid L1 blockchain operates on a **delegated proof of stake** system, where users delegate their tokens to validators to participate in block production and earn rewards.
    -   *Note*: The terms **delegate** and **stake** are used interchangeably.

* * * *

### **Validator Requirements**

-   **Self-Delegation Requirement**: Validators must stake **10,000 HYPE** themselves to become active.
-   **Block Production and Rewards**: Active validators produce blocks and earn rewards proportional to the total amount of HYPE staked to them (including delegations).
-   **Commission Fees**: Validators can charge a commission on rewards earned by their delegators. However:
    -   The commission rate **cannot be increased** unless the new rate is **≤ 1%**. This prevents validators from exploiting stakers by drastically increasing fees after attracting a large delegation.

* * * *

### **Delegation and Undelegation Rules**

-   **Lockup Period**: Delegations to a validator are locked for **1 day**. After this period:
    -   Delegators can partially or fully **undelegate** their tokens at any time.
    -   Undelegated tokens are immediately available in the staking account.
-   **Transfers Between Accounts**:
    -   **Spot to Staking Account**: Transfers are **instant**.
    -   **Staking to Spot Account**: Transfers require a **7-day unstaking queue**. This delay is a security measure to discourage large-scale consensus attacks.
-   **No Automatic Slashing**: The Hyperliquid L1 does not currently implement automatic slashing (penalties for validator misbehavior). Instead, it relies on social mechanisms for accountability.

* * * *

### **Staking Rewards**

-   **Reward Rate Formula**: Inspired by Ethereum, the staking reward rate is **inversely proportional to the square root of total HYPE staked**.
    -   Example: At **400M HYPE staked**, the yearly reward rate is approximately **2.37%**.
-   **Source of Rewards**: Staking rewards are funded by the **future emissions reserve**.
-   **Distribution and Compounding**:
    -   Rewards are **accrued every minute** and distributed to stakers **daily**.
    -   Rewards are **automatically redelegated** (compounded) to the same validator.



    # HyperBFT Consensus Protocol Analysis

## 1. Quorum Fundamentals
### Definition
- A quorum represents a set of validators controlling **more than ⅔ of total network stake**
- Critical for maintaining consensus integrity

### Key Requirements
- **Operating Requirement**: Quorum must be honest (non-Byzantine)
- **Staker Responsibility**: Must delegate only to trusted validators
- *Important*: This creates a security model based on stake-weighted trust

## 2. Round Structure
### Basic Components
- Rounds are fundamental discrete units containing:
  1. Transactions
  2. Validator signatures (from quorum)

### Round Processing
1. **Commitment Phase**
   - Specific conditions must be met
   - Once committed → sent to execution state
2. **Key Property**: All honest nodes maintain agreement on committed round order

## 3. State Management
### Execution Blocks
- Generated from consensus rounds
- Tracked by "height" counter
- **Height Increment Rules**:
  - Only increases when round contains ≥1 transaction
  - Maintains strictly increasing sequence

## 4. Epoch System
### Specifications
- Length: 100,000 rounds
- Duration: ~30 minutes (mainnet)
- **Static Parameters per Epoch**:
  - Validator set
  - Consensus stakes

## 5. Validator Governance
### Jailing Mechanism
#### Trigger Conditions
- Inadequate response latency
- Insufficient response frequency
- Requires quorum of jail votes

#### Effects of Jailing
- Validator excluded from consensus
- Delegator rewards suspended
- *Note*: Different from slashing

### Unjailing Process
1. Validator must:
   - Diagnose issues
   - Implement fixes
2. Subject to onchain rate limits

### Slashing vs. Jailing
| Aspect | Jailing | Slashing |
|--------|---------|----------|
| Cause | Performance issues | Malicious behavior |
| Example | Poor response time | Double-signing blocks |
| Severity | Temporary | More severe |
| Recovery | Self-unjail possible | Punitive measures |

## 6. Technical Implications
### Security Considerations
- Quorum requirement ensures Byzantine fault tolerance
- Stake-weighted voting provides economic security
- Jailing mechanism maintains network performance

### Performance Aspects
- Epoch-based validator set updates
- Regular consensus rounds
- Height-based state tracking

## 7. Best Practices
### For Validators
- Maintain high uptime
- Respond promptly to consensus messages
- Monitor for potential jailing triggers

### For Delegators
- Choose validators carefully
- Monitor validator performance
- Understand jailing/slashing risks

