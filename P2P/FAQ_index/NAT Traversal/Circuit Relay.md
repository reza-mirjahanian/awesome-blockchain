---

# 🛣️ **Circuit Relay in libp2p**

## **What Is It?**

* A **transport protocol** (`p2p-circuit`) that lets two peers communicate via a third-party **relay node** 
* Useful when both peers are **behind NATs/firewalls** or lack shared transport mechanisms .

> 🔐 All traffic is **end-to-end encrypted**—the relay cannot inspect or alter the contents .

---

## 🧭 **Protocol Versions**

* **v1** and **v2** exist.
* **v2** is recommended; docs default to v2 

---

## 📫 **Relay Addresses**

Multiaddr structure:

```
[/ip4/.../tcp/.../p2p/QmRelay]/p2p-circuit/p2p/QmAlice
```

* Left portion: **relay transport address + Relay Peer ID**
* Right portion: `/p2p-circuit/` + **target Node ID**
* Helps dialing peers connect directly via a known relay 

💡 *Tip:* advertise **full relay addresses** (with transport info) for faster, reliable connections .

---

## 🔄 **How It Works (v2 Simplified)**

1. **Node A** (behind NAT) dials Relay R and sends a `HOP` request with:

   * `srcPeer`: A’s ID
   * `dstPeer`: B’s ID
2. **Relay R** receives it, then dials **Node B**, sending `STOP` to bind A → B .
3. If both sides are ready, **Relay sends `SUCCESS`**, then **pipes streams**:

   ```
   A ↔ R ↔ B
   ```

   * B also replies with `SUCCESS` .
4. **Application** at Node B sees a new incoming connection via relay.

✨ Once established, the **virtual connection** behaves like a normal libp2p link, fully encrypted 
---

## 🔁 **Multi-Hop Relay**

* Nest relay paths like:

  ```
  /p2p-circuit/p2p/QmR1/p2p-circuit/p2p/QmR2/p2p-circuit/p2p/Target
  ```
* Enables chaining across multiple relay nodes .

---

## 📚 **Origins & Use Cases**

* Inspired by **TURN** (part of ICE protocol suite) 
* Ideal for NAT traversal scenarios and network barriers. Supports use cases like:

  * Browser-based peers
  * Nodes unable to expose public addresses 

---

## 🔧 **Developer Tools & Examples**

* **js-libp2p** and **go-libp2p** implementations available.
* **Example code** demonstrates relay server setup, listener, dialer flows

---

## 💡 **Why Use Relay?**

* 🔄 Enables **P2P connectivity** when direct paths aren’t possible.
* 🔐 Maintains **privacy** via encryption.
* 🌐 Allows **topology flexibility** (multi-hop).
* ✅ Acts as a **fallback** when NAT traversal (e.g., hole punching) fails.

---



# Circuit Relay in libp2p

## Overview
**Circuit Relay** is a libp2p transport protocol that routes traffic between two peers via a third-party *relay peer*, enabling communication when direct connections are blocked by NATs, firewalls, or incompatible transport protocols. It’s inspired by the TURN protocol in ICE (Interactive Connectivity Establishment).

## Key Features
- **Purpose**: Allows peers behind NATs or with mismatched protocols to connect via a relay peer.
- **Protocol**: Uses the `/p2p-circuit` multiaddr to identify relayed connections.
- **Versions**: 
  - *Circuit Relay v1* (deprecated)
  - *Circuit Relay v2* (recommended, with enhanced features like resource reservations).

## How It Works
1. **Relay Setup**:
   - A peer (e.g., QmAlice) behind a NAT dials a relay peer (e.g., QmRelay) to establish a long-lived connection.
   - The relay peer listens for incoming connections on behalf of the NAT’d peer.
2. **Connection Process**:
   - Another peer (e.g., QmBob) dials the relay peer using a multiaddr like `/p2p/QmRelay/p2p-circuit/p2p/QmAlice`.
   - The relay forwards traffic between QmAlice and QmBob, creating a *virtualized connection*.
3. **Circuit Relay v2 Enhancements**:
   - **Hop Protocol**: Clients send commands to reserve resources (`/libp2p/circuit/relay/0.2.0/hop`).
   - **Stop Protocol**: Manages connection termination (`/libp2p/circuit/relay/0.2.0/stop`).
   - **Resource Limits**: Relays enforce limits on duration and data for relayed connections.

## Address Format
- Basic: `/p2p-circuit/p2p/QmAlice`
- Preferred: Includes relay’s transport address, e.g., `/ip4/198.51.100.0/tcp/55555/p2p/QmRelay/p2p-circuit/p2p/QmAlice`

## Security
- Relayed connections are **end-to-end encrypted**, similar to direct libp2p connections.
- AutoNAT ensures peers don’t advertise unreachable addresses, reducing failed connection attempts.

## Use Case
> Enables peers like browser nodes or IoT devices, which lack socket support or public IPs, to participate in P2P networks by routing traffic through relay peers.[](https://docs.libp2p.io/concepts/nat/circuit-relay/)[](https://docs.libp2p.io/concepts/nat/dcutr/)[](https://docs.libp2p.io/concepts/nat/hole-punching/)

## Resources
- [Circuit Relay v2 Specification](https://github.com/libp2p/specs/blob/master/relay/circuit-v2.md) for detailed comparison of v1 and v2.
- [libp2p Examples](https://github.com/libp2p/js-libp2p-example-circuit-relay) for practical implementation.[](https://github.com/libp2p/specs/blob/master/relay/circuit-v2.md)[](https://github.com/libp2p/js-libp2p-example-circuit-relay)