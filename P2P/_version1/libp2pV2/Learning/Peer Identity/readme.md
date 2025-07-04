# **Complete Reference: Peer Identity in P2P Networks**

---

## **1. What is Peer Identity?**

In P2P (peer-to-peer) networks, a **peer identity** is a **unique cryptographic identifier** used to distinguish each node. It ensures secure communication, verification, and routing.

---

## **2. Core Components of a Peer Identity**

| **Component**   | **Description**                                                                |
| --------------- | ------------------------------------------------------------------------------ |
| **Private Key** | Generates signatures for authentication                                        |
| **Public Key**  | Shared with peers to verify identity                                           |
| **Peer ID**     | A hash (e.g., SHA-256, multihash) of the public key, uniquely identifying peer |
| **Protocols**   | Identity often tied to supported protocols or capabilities                     |

---

## **3. Peer Identity in libp2p**

libp2p is the de facto standard P2P stack used in IPFS, Filecoin, Ethereum, and others.

### Peer ID Formats:

* **v1**: Uses multihash of the public key (`PeerId = multihash(PubKey)`)
* **v2**: Encodes full public key directly

### Code Example (Go / libp2p):

```go
import (
    "github.com/libp2p/go-libp2p-core/peer"
    crypto "github.com/libp2p/go-libp2p-core/crypto"
)

privKey, pubKey, _ := crypto.GenerateKeyPair(crypto.Ed25519, -1)
pid, _ := peer.IDFromPublicKey(pubKey)

fmt.Println("Peer ID:", pid.Pretty())
```

---

## **4. Algorithms Supported**

| **Key Type** | **Size** | **Security**            | **libp2p Support** | **Peer ID Format Impact** |
| ------------ | -------- | ----------------------- | ------------------ | ------------------------- |
| Ed25519      | 256-bit  | High                    | ✅                  | Deterministic hash        |
| RSA          | 2048+    | Moderate-High           | ✅                  | Large Peer ID             |
| Secp256k1    | 256-bit  | High (used in BTC, ETH) | ✅                  | Compatible with ECDSA     |
| ECDSA (P256) | 256-bit  | High                    | ✅                  | Smaller than RSA          |

---

## **5. Peer ID vs Public Key**

| **Aspect**             | **Peer ID**         | **Public Key**             |
| ---------------------- | ------------------- | -------------------------- |
| **Purpose**            | Identifier          | Cryptographic verification |
| **Size**               | Short (20-46 bytes) | Larger (32-512+ bytes)     |
| **Stored in DHT**      | ✅                   | Sometimes                  |
| **Used in Connection** | ✅ (lookup)          | ✅ (handshake, signing)     |

---

## **6. Peer Identity in Connection Lifecycle**

1. **Bootstrap**:

   * Uses known Peer IDs and addresses

2. **Dialing**:

   * Dial peer via `multiaddr` + `peerID`

3. **Handshake**:

   * Exchange public keys
   * Validate `PeerID == hash(pubkey)`

4. **Auth**:

   * Sign payloads using private key

### Go/libp2p connection example:

```go
host, _ := libp2p.New()
targetID, _ := peer.Decode("Qm...")
addrInfo := peer.AddrInfo{ID: targetID, Addrs: []multiaddr.Multiaddr{...}}

host.Connect(ctx, addrInfo)
```

---

## **7. Edge Cases & Tricky Parts**

| **Case**                   | **Challenge**                               | **Resolution**                              |
| -------------------------- | ------------------------------------------- | ------------------------------------------- |
| RSA Peer IDs too long      | DHT storage inefficient                     | Use Ed25519 or ECDSA                        |
| Public Key missing in ID   | Need to retrieve pubkey out-of-band         | Store pubkey in DHT or use ID v2            |
| Collision attacks          | Weak hash function for PeerID               | Use SHA-256 or multihash formats            |
| Malicious peer spoofing ID | Peer pretends to be someone else            | Always verify pubkey hash against ID        |
| Identity rotation          | Changing keys breaks old Peer ID references | Use a stable key store, or pubkey roll-over |

---

## **8. Comparison with Similar Concepts**

### Peer Identity vs IP Address

| **Aspect**     | **Peer Identity**      | **IP Address**        |
| -------------- | ---------------------- | --------------------- |
| **Stability**  | Stable                 | Dynamic (DHCP, NAT)   |
| **Security**   | Cryptographic          | Not secure            |
| **Uniqueness** | Guaranteed (if secure) | Potentially reused    |
| **Use Case**   | Routing, Auth          | Network layer routing |

### Peer Identity vs Node ID (DHTs)

| **Aspect**     | **Peer Identity**        | **Node ID (e.g., Kademlia)** |
| -------------- | ------------------------ | ---------------------------- |
| **Purpose**    | Authenticated identity   | DHT lookup and routing       |
| **Generation** | Derived from public key  | Often random or from hash    |
| **Security**   | Cryptographically secure | Varies; can be sybil-prone   |

---

## **9. Real-World Usage**

| **Project**  | **Peer ID Usage**                                    |
| ------------ | ---------------------------------------------------- |
| **IPFS**     | Uses Peer IDs for routing, content discovery, pubsub |
| **Ethereum** | Uses Peer IDs in devp2p/libp2p for handshake/auth    |
| **Filecoin** | Peer IDs for deal negotiation and proof exchange     |
| **libp2p**   | Foundation for all transport, discovery, routing     |

---

## **10. Performance: Computational Complexity**

| **Operation**           | **Complexity (O)**         |
| ----------------------- | -------------------------- |
| Peer ID generation      | O(1)                       |
| Public Key verification | O(n) (depends on key type) |
| Peer ID comparison      | O(1)                       |
| DHT Lookup via Peer ID  | O(log n)                   |

---

## **11. Security Considerations**

| **Threat**     | **Mitigation**                                 |
| -------------- | ---------------------------------------------- |
| Peer spoofing  | Require public key validation during handshake |
| Sybil attacks  | Require proof of work or stake (in DHT usage)  |
| Replay attacks | Use nonces/timestamps in signed payloads       |
| Key leakage    | Secure key storage and use ephemeral sessions  |

---

## **12. Advanced Features**

* **Peer Store**: Persists peer info including addresses, protocols, public keys
* **Identity Exchange Protocol**: Protocol in libp2p for exchanging IDs securely
* **Persistent Identity**: Store private key in secure keystore to maintain identity

---

## **13. Pros & Cons**

| **Pros**                      | **Cons**                            |
| ----------------------------- | ----------------------------------- |
| Unique and secure identity    | Large key sizes for some algorithms |
| Cryptographic verification    | Key management complexity           |
| No reliance on IP             | Requires DHT or discovery layer     |
| Supports protocol negotiation | Potential for privacy leaks         |

---

## **14. Full Flow Example: ID Exchange and Verification (Go)**

```go
hostA, _ := libp2p.New()
hostB, _ := libp2p.New()

// Extract Peer IDs
fmt.Println("Host A:", hostA.ID())
fmt.Println("Host B:", hostB.ID())

// B connects to A
addrInfo := peer.AddrInfo{
    ID:    hostA.ID(),
    Addrs: hostA.Addrs(),
}

hostB.Connect(context.Background(), addrInfo)

// Verify identity during stream
hostB.SetStreamHandler("/verify", func(s network.Stream) {
    peerID := s.Conn().RemotePeer()
    pubKey := s.Conn().RemotePublicKey()

    // Must match
    derivedID, _ := peer.IDFromPublicKey(pubKey)
    if derivedID != peerID {
        panic("Identity verification failed!")
    }
})
```

---

## **15. Tools and Utilities**

| **Tool**                 | **Function**                      |
| ------------------------ | --------------------------------- |
| `peer.IDFromPublicKey()` | Generate ID from public key       |
| `peer.Decode()`          | Parse Peer ID from string         |
| `peerstore.Peerstore`    | Persist public keys and protocols |
| `multiaddr`              | Address format used with Peer ID  |

---


