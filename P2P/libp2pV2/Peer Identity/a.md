**1. Definition & Cryptography**  
- **Peer ID**  
  - A *Peer Identity* (PeerID) uniquely identifies a node in a P2P network and binds it to its public key.  
  - It’s the multihash of the peer’s public key; peers prove identity by signing messages with the corresponding private key.  
- **Key Pair**  
  - *Private Key*: kept secret; used to sign and decrypt.  
  - *Public Key*: shared; used to verify signatures and encrypt for the peer.  
- **Security Guarantees**  
  - **Authentication**: you know who you’re talking to.  
  - **Integrity**: messages can’t be tampered without detection.  
  - **Confidentiality**: optional if using hybrid encryption channels (e.g., Noise).  

**2. Representation Formats**  
| Format            | Description                                                   | Encoding/Notes                                                                             |
|-------------------|---------------------------------------------------------------|--------------------------------------------------------------------------------------------|
| Binary multihash  | Raw bytes: `[hash-func][length][digest]`                      | Compact, used internally.                                                                  |
| Base58 string     | Base58-BTC encoding of multihash                              | Default textual form, no multibase prefix.                                            |
| Base16/Base64     | Alternative, but uncommon for Peer IDs                         | Supported by multibase; libraries may or may not support.                                  |

- **Time Complexity**:  
  - Hashing public key → `O(n)` in key length.  
  - Encoding (Base58) → `O(n·log m)` where *m* is alphabet size overhead.  

**3. PeerID in Multiaddr**  
- Embed as `/p2p/<PeerID>` in a multiaddr:  
  ```text
  /ip4/203.0.113.1/tcp/4001/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N
  ```  
- **Dialing**: multiaddr tells dialer both transport (IP/TCP/etc.) and remote identity. If IP/port hijacked, the dial fails crypto handshake.  

**4. PeerInfo & PeerStore**  
- **PeerInfo**: `{ id: PeerID, addrs: Multiaddr[] }` – bundling identity with reachable addresses.  
- **PeerStore**: local database of known `PeerInfo`; allows caching, reputation, backoff timers, metadata.  

**5. Generating & Managing PeerIDs**  
- **JavaScript (libp2p-js)**  
  ```js
  import { createLibp2p } from 'libp2p'
  import { PeerId } from '@libp2p/peer-id'

  // Generate new peer identity
  const peerId = await PeerId.create({ keyType: 'Ed25519' })
  console.log(peerId.toString())  // base58 PeerID
  ```

- **Go (libp2p-go)**  
  ```go
  import (
    libp2p "github.com/libp2p/go-libp2p"
    peer "github.com/libp2p/go-libp2p/core/peer"
  )

  func newHost() (peer.ID, error) {
    h, err := libp2p.New()
    if err != nil { return "", err }
    return h.ID(), nil
  }
  ```

- **Rust (rust-libp2p)**  
  ```rust
  use libp2p::identity;

  let keypair = identity::Keypair::generate_ed25519();
  let peer_id = keypair.public().to_peer_id();
  println!("{}", peer_id.to_base58());
  ```

**Edge Cases**  
- Regenerating keys invalidates old PeerID—connections break.  
- Exporting/importing between languages requires matching multihash formats.  
- Long-lived peers must persist private keys securely (e.g., hardware ORAM).  

**6. Use Cases & Real-World Projects**  
- **IPFS**: NodeID = PeerID; used in routing (DHT lookups), pubsub topics.  
- **Ethereum devp2p**: similar NodeID concept; uses secp256k1 keys.  
- **Filecoin/Textile**: leverage libp2p PeerID for secure swarm communication.  

**7. Tricky Parts & Security Considerations**  
- **Key Rotation**: no built-in revocation—must announce new PeerID and migrate state.  
- **Sybil Attacks**: easy to spin new PeerIDs—combine with stake or reputation to mitigate.  
- **Collision Resistance**: multihash uses strong hashes (SHA-256/Blake2b) to avoid collisions.  
- **Privacy**: static PeerIDs enable tracking; ephemeral IDs or DHT-hidden addressing can help.  

**8. Comparison with Similar Concepts**  
| Feature              | PeerID (libp2p)           | TLS Certificate      | Ethereum NodeID        | SSH Fingerprint      |
|----------------------|---------------------------|----------------------|------------------------|----------------------|
| Binding to pub key   | ✔︎                        | ✔︎ (via CA sig)      | ✔︎                     | ✔︎                   |
| Central authority    | ✘                         | ✔︎                   | ✘                      | ✘                    |
| Human-readable form  | Base58 multihash          | PEM DN names         | Hex of public key      | Hex SHA256          |
| Revocation           | Manual/distributed scheme | CRL/OCSP             | Manual                 | Manual               |

**9. Performance & Scalability**  
- **Peer lookup (DHT)**: `O(log N)` hops in Kademlia-style networks, keyed on PeerID hash.  
- **Handshake**: crypto ops `O(n²)` on key size, but keys (e.g., Ed25519) are small constant.  

**10. Pros & Cons**  
| Pros                                        | Cons                                                       |
|---------------------------------------------|------------------------------------------------------------|
| Decentralized, no CA dependency             | No built-in revocation or expiry                           |
| Strong cryptographic binding                | Key management complexity (rotation, storage)              |
| Compact identifier in routing & addressing | Static ID harms privacy without additional measures        |

**Next Steps Suggestion**  
Explore **Peer Discovery & NAT Traversal in P2P Networks** to learn how peers find and connect to each other behind firewalls and routers.