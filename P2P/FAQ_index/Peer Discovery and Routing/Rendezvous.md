### 🤝 What is **Rendezvous**?

**Rendezvous** is a *routing protocol* in P2P networks that helps nodes **discover each other** using a common meeting point.

> 📍 A **rendezvous point** is typically a stable, well-connected node acting as a **hub** for peer discovery.

---

### ⚠️ Rendezvous is *Not Decentralized*

* **Federated**, not fully decentralized → introduces **single point of failure**.
* Alternatives:

  * **DHT (Distributed Hash Table)**:

    * Key-value storage across peers
    * Fast lookups, fully decentralized
  * **Gossipsub**:

    * Gossip-based **pub-sub** protocol
    * Message propagation without central coordination

---

### 🚀 Rendezvous in **libp2p**

> Replaces older `ws-star-rendezvous` with **rendezvous daemons** + **p2p-circuit relays**

#### 🔧 Use Cases:

* **Bootstrap**: Discover relays for browser nodes
* **Service Discovery**: Find peers offering specific services
* **Content Sharding**: Discover peers hosting specific shards
* **Query Routing**: App-specific peer discovery

---

### 📝 **Registration Process**

1. Peer connects to a **rendezvous point**
2. Sends a `REGISTER` message with:

   * One or more **namespaces** (e.g. `test-app`)
   * Optional **TTL** (max: `72h`, default: `2h`)
3. Other peers query the rendezvous point to discover them

> 📘 Query supports:
>
> * `max_peers` (limit results)
> * `cookie` (pagination by skipping known entries)

---

### 🧵 Protocol Info

* Runs over **libp2p streams**
* Uses protocol ID:

  ```text
  /rendezvous/1.0.0
  ```

---

### 🔄 Integration with **PubSub**

* **Bootstraps** pubsub: Discover topic subscribers
* **Federated rendezvous points** can use **pubsub** internally
* Allows:

  * Publishing packets
  * Subscribing/unsubscribing to topics
  * Simplified client interface
