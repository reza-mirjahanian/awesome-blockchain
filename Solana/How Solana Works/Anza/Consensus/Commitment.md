# 🔍 Solana Commitment Status

> A **commitment metric** is the network’s standardized yard-stick for *how sure* a block (and the transactions inside it) is final.

---

## 🏷️ Three Commitment Levels

| Level | Icon | Short Tag | Meaning |
|-------|------|-----------|---------|
| **Processed** | ⚡ | `processed` | Block *received*, on the *majority fork*, contains the tx |
| **Confirmed** | ✅ | `confirmed` | All of **Processed** **plus** ≥ **66 %** of stake voted on the block |
| **Finalized** | 🔒 | `finalized` | All of **Confirmed** **plus** ≥ **31 additional confirmed blocks** built on top |

---

## 📊 Quick-Reference Matrix

| Requirement | Processed | Confirmed | Finalized |
|-------------|-----------|-----------|-----------|
| 🧾 Block **received** | ✔ | ✔ | ✔ |
| 🌿 On **majority fork** | ✔ | ✔ | ✔ |
| 🎯 Contains **target transaction** | ✔ | ✔ | ✔ |
| 🗳️ **66 %+ stake voted** | ✖ | ✔ | ✔ |
| 🧱 **31+ confirmed blocks** stacked on top | ✖ | ✖ | ✔ |

---

## 🛠️ Client Usage

Query a block or transaction with the desired commitment in the RPC call:

```bash
"commitment": "processed"   # fastest, least safe
"commitment": "confirmed"   # balanced
"commitment": "finalized"   # safest, slowest
```