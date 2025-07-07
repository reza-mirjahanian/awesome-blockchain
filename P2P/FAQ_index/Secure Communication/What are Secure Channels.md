## 🔐 Secure Channels in libp2p

---

### 📦 **Overview**

Before data exchange, peers must establish a **secure channel**.

* libp2p supports multiple transports:

  * 🔐 *Encrypted by default*: `QUIC`, `WebTransport`
  * ❗ *Not encrypted by default*: `TCP`, `WebSocket` → need **security handshake**

---

### 🛡️ **Security Protocols in libp2p**

libp2p defines two options for securing connections:

1. **TLS 1.3**
2. **Noise Protocol**

After a successful **security handshake**, libp2p proceeds to:

➡️ **Negotiate a stream multiplexer** (e.g., `mplex`, `yamux`) for managing multiple streams over a single connection.
