### 🔍 What is **mDNS**?

**mDNS** (*multicast DNS*, [RFC 6762](https://datatracker.ietf.org/doc/html/rfc6762)) enables devices on a **local network** to discover each other by:

* **Publishing/receiving DNS records** via **IP multicast**
* **Broadcasting topics** of interest

> 📶 Common in home networks — used by **computers, printers, smart TVs**, etc., for auto-discovery and connectivity.

---

### 🧩 **mDNS in libp2p**

libp2p uses **mDNS** for **zero-config peer discovery** on the same local network.

#### 🔄 Basic Flow:

1. A node **broadcasts a discovery request**
2. Nearby peers **respond with their `multiaddresses`**

---

➡️ For details on fields and mechanics, see the [mDNS libp2p spec](https://github.com/libp2p/specs/blob/master/discovery/mdns.md).
