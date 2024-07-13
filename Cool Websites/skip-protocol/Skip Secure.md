
##  Skip Secure



### What is Skip Secure?
Skip Secure is a fully secure RPC and LCD endpoint on all Skip-enabled chains that you can use from any front end such as:
- **Kepler**
- **Terra Station**
- **Minscan**
- **Etc.**

We hope you like it!

### Understanding the Transaction Process

#### Submission and Gossiping
When you submit a transaction:
1. **Initial Submission**: The transaction is gossiped to a node or a validator.
2. **Mempool Distribution**: The transaction is spread to different parts of the mempool.
3. **Block Creation**: It reaches the validator selected to create a block and gets included in a block.

#### The Problem: Untrustworthy Nodes
- **Interception**: Malicious nodes can intercept your transaction.
- **Manipulation**: These nodes can add more transactions around yours and potentially modify the order.
- **Value Extraction**: The goal is to extract value from your transaction. For example:
  - **Sandwiching**: Placing a transaction before yours to give you a worse execution price, then dumping on you afterwards to profit from your slippage.

### How Skip Secure Works

#### Traditional Gossip vs. Skip Secure

**Traditional Gossip:**
- **Public Mempool**: Transactions are gossiped publicly, visible to all nodes.
- **Vulnerability**: Your transaction can be front-run, sandwiched, or even back-run.

**Skip Secure:**
- **Direct Submission**: The transaction is sent to the Sentinel, a trusted infrastructure.
- **Private Link**: It has a direct link to the validator, ensuring privacy.
- **Secure Inclusion**: Your transaction is included in the block without being seen by any public or malicious nodes.

### Before and After Using Skip Secure

**Before Skip Secure:**
- **Public Transactions**: Transactions are gossiped publicly.
- **Risks**: Possibility of being front-run, sandwiched, or back-run.

**After Skip Secure:**
- **Private Transactions**: Transactions go directly to the validator over a private link.
- **No Front-running**: Impossible to front-run.
- **No Sandwiching**: Impossible to sandwich.
- **No Back-running**: Impossible to back-run.

### Future Enhancements
- **MEV Value Return**: In a future iteration, we might offer users the opportunity to return some of the MEV (Miner Extractable Value) back to you instead of just keeping it fully private.

### How to Use Skip Secure
 how to use Skip Secure on:
- **Kepler**
- **Terra Station**

More integrations will be added over time.

