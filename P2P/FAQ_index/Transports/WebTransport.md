**WebTransport / QUIC-based transport in libp2p**

---

### 🚀 What is WebTransport?

* It’s a **QUIC-based, bidirectional, multiplexed transport** protocol exposed via a Web API—essentially “WebSocket over QUIC” for browsers [1]).
* Benefits over WebSocket (TCP):

  * Multiple streams in parallel (no head‑of‑line blocking)
  * Faster handshake: 3 RTTs vs WebSocket’s \~6 RTTs [1])
  * Native TLS-hashing support for self-signed certs in browsers

---

### ⚙️ How libp2p uses WebTransport

#### **Multiaddr syntax**

```
/ip4/192.0.2.0/udp/1234/quic/webtransport/certhash/<hash>
```

* Points to a QUIC+WebTransport server
* `certhash` lists SHA‑256 hashes of acceptable self‑signed certs [1])

#### **Connecting workflow**

1. Browser issues an HTTPS CONNECT to `https://<address>/.well-known/libp2p-webtransport?type=noise`
2. QUIC + WebTransport handshake
3. Noise multistream-security negotiation
4. libp2p streams start flowing [1], [docs.libp2p.io][2])

#### **Availability**

* **go-libp2p**: default transport, supported in Kubo
* **js-libp2p**: supported
* **rust-libp2p**: WebTransport via Wasm [1])

---

### 🔐 Why QUIC over TCP?

| Feature                   | QUIC (WebTransport)      | TCP (WebSocket)                                     |
| ------------------------- | ------------------------ | --------------------------------------------------- |
| **Encryption**            | Built-in (TLS and Noise) | Optional, via WebSocket+TLS                         |
| **Multiplexing**          | Native layer support     | Uses mplex/yamux stack                              |
| **Handshake RTTs**        | 3 total                  | \~6 RTTs [1], [blog.libp2p.io][3]) |
| **Head-of-line blocking** | No                       | Yes                                                 |

---

### 🧩 Integration into libp2p

* **Pluggable transport**: integrates seamlessly in modular libp2p stacks
* Negotiates peer’s security and stream capabilities via the switch component [1])
* QUIC/WebTransport/WebRTC provide native multiplexing—no mplex/yamux needed [4])

---

### ✅ Quick Config in JavaScript

```js
import { createLibp2p } from 'libp2p'
import { webTransport } from '@libp2p/webtransport'
import { noise } from '@chainsafe/libp2p-noise'

const node = await createLibp2p({
  transports: [ webTransport() ],
  connectionEncryption: [ noise() ]
})
```

(The JS library currently *only dials* peers—you can’t listen yet—pending QUIC support in Node.js) ([libp2p.github.io][5])

---

### 📌 Key Takeaways (no meta-summary)

* **WebTransport = QUIC + multiplexed streams + fast handshakes**
* Ideal for **browser‑to‑peer** connectivity in p2p apps
* Secure: accepts **self-signed certs via hash**
* Fully implemented in **go/js/rust‑Wasm libp2p stacks**
* Simplifies libp2p transport pipeline—**native mux & security**

---
