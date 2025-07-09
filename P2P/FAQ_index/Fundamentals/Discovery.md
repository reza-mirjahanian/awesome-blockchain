### 🌐 **libp2p Addressing Fundamentals**

---

#### 1. **Peer Identity**

* Every node has a unique **PeerId**
* Generated from the node’s public key
* Used to **authenticate** and **route** connections

---

#### 2. **Multiaddr: Flexible Address Format**

* Encodes protocol stack and transport layers

  ```
  /ip4/203.0.113.5/tcp/4001
  ```
* Can nest multiple protocols like TCP, WebRTC, QUIC:

  ```
  /ip4/198.51.100.1/udp/4002/quic
  ```
* Includes **PeerId**:

  ```
  /ip4/203.0.113.5/tcp/4001/p2p/QmPeerID...
  ```

---

#### 3. **Address Encoding & Normalization**

* Ensures consistent address representation
* Prevents duplicate addresses due to formatting differences

---

#### 4. **Peer Routing 🔍**

* Enables locating a Peer’s current addresses
* Common mechanisms:

  1. **DHT** (Distributed Hash Table)
  2. **Peer exchange** protocols

---

#### 5. **Address Book**

* Each node keeps a local mapping:

  ```
  PeerId ↔ [multiaddr1, multiaddr2, …]
  ```
* Updated via:

  * Successful dials
  * Peer discovery mechanisms

---

#### 6. **Address Filtering**

* Enables selecting which addresses to use, e.g.:

  * prefer external IPv4 over internal
  * prefer QUIC over TCP
* Helps adapt to network environment
* Example filter layers:

  ```js
  { allowIPv4: true, allowUDP: false }
  ```

---

#### 7. **Practical Use-Cases**

* **Dialing a peer**:

  1. Look up PeerId in address book
  2. Pick the best multiaddr (based on filter)
  3. Dial using chosen transport
* **Updating addresses**:

  * After a successful dial, store or replace old address
  * Prune unreachable or stale entries

---

#### 🔁 **Flow Summary (no meta commentary):**

1. **Identity**: PeerId ← public key
2. **Addressing**: Multiaddr encodes transport + PeerId
3. **Discovery**: Peer routing locates addresses
4. **Storage**: Address book maintains known addresses
5. **Filtering**: Apply rules to choose best address
6. **Connection**: Dial selected multiaddr

---

Code snippets and formatted lists capture the key logic steps for addressing in libp2p.
