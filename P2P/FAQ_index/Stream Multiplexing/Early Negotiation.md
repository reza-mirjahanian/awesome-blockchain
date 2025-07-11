---

# 🔄 **Early Muxer Negotiation in libp2p**

## 🔐 **Why Optimize?**

* Traditional handshake:

  1. Negotiate **security** (Noise/TLS)
  2. Do cryptographic handshake
  3. Run **multistream-select** to pick multiplexer (adds 1 RTT) ([docs.libp2p.io][1])

* **Early negotiation** *inlines* muxer selection into the security handshake—saving a round-trip and reducing time to first byte (TTFB) ([docs.libp2p.io][1])

---

## 🤝 **How It Works**

### ✔️ TLS + ALPN

* ClientHello includes ordered list of supported muxers (`/yamux/1.0.0`, `/mplex/6.7.0`, `libp2p`)
* Server replies with selected muxer in ServerHello using ALPN ([github.com][2])

### ✔️ Noise + Extension Registry

* Noise XX handshake message 2: Responder sends its muxer list
* Noise message 3: Initiator sends its list (or pick) → common muxer is chosen ([github.com][2])

---

## ⚙️ **Supported Environments**

* Only for **TCP/WebSocket** transports without native encryption or multiplexing
* Available in **go-libp2p** (others in progress) ([docs.libp2p.io][1])

---

## ✅ **Benefits**

* Cuts **one round-trip**
* Faster connection setup, especially useful for insecure raw transports

---

## 🧭 **Negotiation Flow (Simplified)**

```
Client: Hello (Noise/TLS w/ [muxers list])
Server: Hello (select one muxer) + encrypted channel
-- Channel is now secured and multiplexed in one go!
```

---

[1]: https://docs.libp2p.io/concepts/multiplex/early-negotiation/?utm_source=chatgpt.com "Early Multiplexer Negotiation - libp2p"
[2]: https://github.com/libp2p/specs/blob/master/connections/inlined-muxer-negotiation.md?utm_source=chatgpt.com "specs/connections/inlined-muxer-negotiation.md at master · libp2p/specs ..."
