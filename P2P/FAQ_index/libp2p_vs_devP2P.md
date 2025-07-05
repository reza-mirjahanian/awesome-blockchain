# devp2p vs libp2p

## **What is devp2p?**

**devp2p** is a peer-to-peer (P2P) protocol suite originally developed for Ethereum. It defines:

* Node discovery
* Transport and multiplexing
* Application-level protocols (e.g. for Ethereum)

> It’s a tightly-coupled stack tailored for Ethereum’s needs.

---

## **Key Features of devp2p**

1. **Node Discovery**

   * Based on Kademlia DHT
   * Uses UDP for peer discovery
   * Ethereum's ENR (Ethereum Node Record) is an evolution of devp2p discovery.

2. **RLPx Protocol Stack**

   * Handles encryption, framing, and multiplexing over TCP
   * Protocols layered on top using **RLP** (Recursive Length Prefix) encoding

3. **Tight Integration**

   * All protocols (like ETH, LES, Whisper) are defined within the same stack.

---

## **Differences from libp2p**

| Feature                | **devp2p**                | **libp2p**                                 |
| ---------------------- | ------------------------- | ------------------------------------------ |
| **Modularity**         | Monolithic                | Modular & composable                       |
| **Use cases**          | Ethereum-specific         | General-purpose P2P applications           |
| **Protocol Evolution** | Harder to extend          | Easy to add/replace components             |
| **Transport support**  | TCP, UDP                  | TCP, UDP, QUIC, WebRTC, etc.               |
| **Multiplexing**       | Custom via RLPx           | Multiple options: `yamux`, `mplex`, etc.   |
| **Discovery**          | Kademlia-based, UDP       | Several options: Kademlia, mDNS, DHT, etc. |
| **Encryption**         | Built-in (via RLPx)       | Swappable modules like `noise`, `tls`      |
| **Adoption**           | Ethereum-specific clients | IPFS, Filecoin, Ethereum 2.0, etc.         |

---

## **Summary of Approach**

> **devp2p** = Ethereum-focused, static stack
> **libp2p** = General-purpose, flexible, modular framework

---

## **Example: devp2p Protocol Stack**

```
[Application Protocols]
        ↑
     RLPx (Multiplexing + Encryption)
        ↑
         TCP (Transport)
```

## **libp2p Stack (Flexible Layers)**

```
[Custom App Protocols]
        ↑
    Multiplexing (e.g., mplex)
        ↑
     Security (e.g., noise)
        ↑
    Transport (e.g., TCP, QUIC)
        ↑
   Peer Discovery (e.g., mDNS, DHT)
```
