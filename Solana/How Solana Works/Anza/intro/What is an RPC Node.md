# 🌐 What is an **RPC Node**?  

---

## 🖥 **Definition**  
**RPC** = *Remote Procedure Call*  
An **RPC Node** runs the same core software as a **validator**, but **does not participate in consensus**.

> ⚠ *While you technically can run RPC + validator voting together, it’s strongly discouraged* — performance will suffer for both operations.

---

## 🎯 **Primary Role**  

An **RPC Node** serves a completely different function from validators in the cluster:  

1. **📡 Answer blockchain queries**  
   - Provides data on accounts, transactions, blocks, program states.  

2. **✉ Accept and forward new transactions**  
   - Receives user-signed transactions → forwards them to cluster leaders for block inclusion.  

---

## 🔄 **Typical Workflow**  

**Example Scenario:**  
1. 🖥 Website requests: `Transfer tokens from wallet A → wallet B` (*with wallet A's permission*).  
2. 🔑 Wallet A signs the transaction locally.  
3. 📤 Signed transaction is sent to an **RPC Node**.  
4. 🚀 RPC Node relays it to the **current leader** for block inclusion.  

---

## 🏗 **Thinking Like an Engineer**  

Running an **RPC Node** is akin to:  

> *💡 Providing a blockchain-powered API endpoint*  
> Developers send requests, get blockchain data, or push signed transactions.  

---

## 👨‍💻 **Who Uses RPC Nodes?**  

- **Blockchain Developers** 🛠  
- Analytics dashboards 📊  
- Web3 wallets 💼  
- DeFi & NFT platforms ⛓  

---

## 📜 **RPC API Familiarity**  

To operate or use an RPC Node effectively, you must be comfortable with **RPC calls**, for example:  

```bash
# Get account info
curl https://<RPC_URL> \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"getAccountInfo",
    "params":["<account_pubkey>"]
  }'
```

```bash
# Send transaction
curl https://<RPC_URL> \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"sendTransaction",
    "params":["<signed_tx_base58>"]
  }'
```

---

## 📌 **Key Takeaways**  

- **Not a voting node** → purely handles client requests.  
- **Carries high network load** due to heavy read and write operations.  
- **Requires technical skills** to run, secure, and scale.  