---

# 🔀 **Stream Multiplexing in libp2p**

## 🧩 **What Is It?**

**Stream multiplexing** allows multiple independent communication **streams** over a single network **connection** (e.g. TCP, QUIC).

> 📦 Think of it like lanes on a highway: multiple messages (lanes) travel over one connection (highway).

---

## 💡 **Why It Matters**

* Reduces overhead from creating multiple connections.
* Enables:

  * Concurrent protocols (e.g., ping, identify, file transfer).
  * Efficient use of resources.
  * Simplified NAT traversal.

---

## 🔌 **How It Works**

1. A **single connection** is established between peers.
2. Multiple **logical streams** are opened on top.
3. Each stream can carry a different protocol or task.
4. **Multiplexer protocol** tags and manages each stream's data.

---

## 🔧 **Multiplexer Protocols in libp2p**

Supported protocols:

* **mplex** (lightweight, simple)
* **yamux** (reliable, widely used)
* **QUIC** (built-in multiplexing)

> ⚙️ libp2p negotiates the multiplexer during the connection handshake.

---

## 📡 **Usage Example**

```go
stream, err := host.NewStream(ctx, peerID, protocol.ID("/my-protocol/1.0.0"))
```

* Opens a new stream to `peerID` using the specified protocol.
* Sends/receives data **independent** of other streams.

---

## 🧠 **Key Benefits**

* ✅ Efficient resource use
* ✅ Better concurrency
* ✅ Seamless protocol upgrades
* ✅ NAT-friendly by reducing connection churn

---
