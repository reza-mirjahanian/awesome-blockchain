### 📡 **Listening & Dialing in libp2p Transports**

---

#### 1. **Core Operations**

* **Listening**

  * Accept incoming connections.
  * Example: A TCP transport uses `bind` + `listen` system calls.
* **Dialing**

  * Initiate outbound connections to another peer.
  * All transports use a common dialing interface.

---

#### 2. **Multiaddrs: Universal Addressing**

* A **multiaddr** encodes protocol layers and addressing information:

  ```
  /ip4/198.51.100.0/tcp/6543
  ```

* For dialing, a multiaddr typically includes a **PeerId**:

  ```
  /ip4/192.0.2.0/tcp/4321/p2p/QmcEPrat...
  ```

* In some cases, you can dial using just a PeerId; libp2p uses peer routing to find its address.

---

#### 3. **Multiple Transports Support**

* libp2p uses a **switch** (previously called "swarm") to manage:

  1. Listening and dialing
  2. Protocol negotiation
  3. Stream multiplexing
  4. Secure connection upgrades

* Developers interact with the switch abstraction, not the low-level transports directly.

---

### ✅ **Quick Reference Table**

| Concept          | Description                                                                 |
| ---------------- | --------------------------------------------------------------------------- |
| **Listen**       | Accept incoming connections                                                 |
| **Dial**         | Connect to a remote peer using multiaddr + PeerId                           |
| **Multiaddr**    | Protocol-aware address format                                               |
| **Switch/Swarm** | Manages transport logic, protocol handling, and secure communication layers |
