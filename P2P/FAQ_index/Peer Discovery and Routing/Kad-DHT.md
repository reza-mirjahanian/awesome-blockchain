https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf

# **Kad-DHT (Kademlia Distributed Hash Table)**

A *distributed hash table* designed for P2P networks. The **libp2p Kad-DHT** is based on the Kademlia whitepaper and enables **node and data discovery** through a structured **routing table**.

---

## 🔁 **Routing Table Structure**

* **Key Concepts**:

  * *Prefix Length*: Measures key similarity (common leading bits).
  * *Distance Metric*: XOR of SHA-256 hashes → lower result = closer keys.

* **Buckets**:

  * Each bucket groups nodes by **shared prefix length**.
  * Maintains `k` closest peers for each prefix length from `0` to `255` (SHA-256 keyspace).

> **Example**: If two SHA-256 hashes share 8 leading bits, they go in the bucket for prefix length 8.

---

## 🧮 **Distance Metric**

* `distance = hash(A) XOR hash(B)`
* Resulting value defines key proximity:

  * `0` → identical keys
  * `1` → one bit differs → very close keys

---

## 🔍 **Peer Routing**

**Purpose**: Find a specific peer.

1. Contact `k` closest nodes to target peer ID.
2. Ask them for *closer* peers.
3. Repeat until:

   * Peer is found, or
   * No closer nodes are returned.

---

## 📦 **Content Provider Routing**

**Purpose**: Find providers for a key.

1. Contact `k` closest nodes to the **key**.
2. Ask for:

   * Providers of the key, and/or
   * Closer nodes.
3. Repeat until:

   * Providers are found, or
   * Search is exhausted.

---

## 🚀 **Bootstrap Process**

**Purpose**: Maintain an up-to-date routing table.

1. Generate a random peer ID.
2. Lookup using **peer routing**.
3. Add discovered peers to the table.
4. Repeat for multiple random IDs.
5. Also lookup *own ID* to stay aware of nearby peers.
