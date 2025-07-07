## 🔐 Noise Protocol Framework

> **Noise** is a modular encryption framework that combines cryptographic primitives into secure, verifiable patterns for private communication.
> ➡️ *Learn more at* [noiseprotocol.org](https://noiseprotocol.org)

---

## 📡 Noise in **libp2p**

libp2p uses **Noise** to:

* 🔒 Encrypt data between peers
* 🔄 Enable *forward secrecy*

### 🔧 `noise-libp2p` Implementation

* Establishes secure channels between nodes
* Performs key exchange + traffic encryption during the **libp2p handshake**
* Produces shared keys used to:

  * Encrypt/decrypt ciphertext messages
  * Secure all future communication

### 📄 Specification Details

* Message **wire format** and **encryption primitives** are defined in the `libp2p-noise` spec.
* Protocol ID:

  * Current: `/noise`
  * Future versions: `/noise/2`, `/noise/3`, etc.


