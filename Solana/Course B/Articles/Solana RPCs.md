# 🔗 How Solana RPCs Work

## 🌐 **What are RPCs in Crypto?**

### Traditional Web2 vs Crypto Data Access
- 🏢 **Web2 APIs**: Data controlled by centralized entities (Stripe, Twilio, Bloomberg, Plaid)
- ⛓️ **Crypto**: All data is *on-chain*, completely *permissionless* and *open*

### 📡 **RPC Definition**
- **RPC** = *Remote Procedure Calls* (technical term)
- 💡 Often used as shorthand for **RPC Nodes**
- 🖥️ **Function**: Nodes that participate in blockchain network and expose methods developers can call

## ⚙️ **How RPCs Work on Solana**

### Node Types in Solana Network
1. **🗳️ Validators**
   - *Vote on block validity*
   - *Participate in consensus*
   - *Store latest blockchain state*

2. **📊 RPCs**
   - *Don't vote*
   - *Handle data requests*
   - *Store latest blockchain state*

### 🔄 **Common Ground**
Both node types:
- ✅ Continuously watch the blockchain
- ✅ Store latest data/state

### 🎯 **How RPC Calls Work**
When making an RPC call:
- 📞 You invoke a procedure/function on a network node
- 🔍 Node knows latest data because it participates in the network
- 📤 Exposes data upon request

## 💻 **Working with Solana RPCs**

### 📝 **JSON-RPC Request Structure**
```bash
curl http://localhost:8899 -X POST -H "Content-Type: application/json" -d '
  {
    "jsonrpc": "2.0",
    "id":1,
    "method":"getBlock",
    "params": [430, {
      "encoding": "json",
      "maxSupportedTransactionVersion":0,
      "transactionDetails":"full",
      "rewards":false
    }]
  }'
```

### 🛠️ **Key Components**
- **Endpoint**: `localhost:8899`
- **Method**: `getBlock`
- **Parameters**: Block number and configuration options

### 📚 **Development Tools**
- 🚀 **Direct JSON-RPC**: Possible but rarely used
- ⭐ **RPC Clients**: Most developers prefer these
- 🔧 **Popular Choice**: *Solana Web3 JS library*

## 🚀 **Getting Started with RPCs**

### 💡 **Importance of RPCs**
- ⚠️ *Without RPCs, you cannot work with the blockchain*
- 🏭 Specialized companies focus on perfecting RPC experience
- 🔥 Machines are heavily utilized and must perform under high loads

### ✅ **Good RPC Provider Characteristics**
- 🛡️ **Reliable** performance
- 📊 **Consistent** response times
- 📈 **Metrics exposure** for usage pattern monitoring
- 💪 **High load handling** capabilities