# 🔒 Secure Vote Signing

## 🛡️ Security Challenge
- Validators sign votes confirming ledger entries are valid using *asymmetric keys*
- 🚨 **Critical risk**: Forged votes violating consensus rules → stake slashing
- > "If validator's key signs incorrect data (e.g., voting on multiple forks), node's stake could be compromised."

## 👥 Key Roles

### Validator
- Tracks all possible forks when receiving multiple blocks for same slot
- Selects "best" fork by submitting votes
- At startup:
  - Creates new `vote_account`
  - Registers with cluster via gossip
  - Included in active set by other nodes

### Stakeholder 💰
- Controls staked capital
- *Delegates stake* to vote signer
- Delegation enables vote signer to represent voting weight of all stakes

### Vote Signer ✍️
- Signs votes on behalf of delegated stakes
- Votes generate rewards for all delegated stakes

## ⚙️ Voting Process
- Validators submit "new vote" transactions signed with private key
- Other nodes verify signature using validator's public key
- Validators continuously evaluate forks and vote on the best chain
- Proper key management = essential for security