

**Boot Node**
In a P2P network, new nodes first connect to **boot nodes**, which are **predefined nodes listed** in the app binary or config. Boot nodes help newcomers discover other peers. Once connected to enough peers, the new node no longer depends on the boot nodes.

https://en.wikipedia.org/wiki/Bootstrapping_node

https://ethereum.org/en/developers/docs/nodes-and-clients/bootnodes/

---------------------
**Circuit Relay**
When two peers can’t connect directly—due to firewalls, NAT, or incompatible transports—they can communicate through a **relay peer** that both can reach. The relay acts as a bridge between them.

Libp2p implements this via the **relay spec**, which defines how relayed connections work at the protocol and addressing level.


---------


**DHT (Distributed Hash Table)**
A *DHT* spreads key-value data across a network of peers. Like a regular hash table, values are retrieved by key, but the data is distributed. Each node is responsible for part of the key space, enabling *efficient lookup and routing*.

libp2p includes a **Kademlia-based DHT** in *Go* and *JavaScript*.
It's used for **peer discovery**, **content metadata**, **service advertisements**, and more.

https://en.wikipedia.org/wiki/Kademlia

https://en.wikipedia.org/wiki/Distributed_hash_table

-----------------------------
### DCUtR 

Direct Connection Upgrade through Relay (DCUtR) is a protocol for establishing direct connections between nodes via hole punching, without a [signaling server](https://docs.libp2p.io/concepts/appendix/glossary/#signaling-server). DCUtR synchronizes and opens connections to each peer's predicted external addresses.

### Hole punching
https://en.wikipedia.org/wiki/Hole_punching_(networking)

-----------------------------
### Dial 

The process of opening a libp2p connection to another peer is known as "dialing", and accepting connections is known as ["listening"](https://docs.libp2p.io/concepts/appendix/glossary/#listen). Together, an implementation of dialing and listening forms a [transport](https://docs.libp2p.io/concepts/appendix/glossary/#transport).


-----------------------------

### Listen 

The process of accepting incoming libp2p connections is known as "listening", and it allows other peers to ["dial"](https://docs.libp2p.io/concepts/appendix/glossary/#dial) up and open network connections to your peer.

-----------------------------
### mDNS 

[Multicast DNS](https://en.wikipedia.org/wiki/Multicast_DNS) is a protocol for service discovery on a local network. One of libp2p's [peer routing](https://docs.libp2p.io/concepts/appendix/glossary/#peer-routing) implementations leverages mDNS to discover local peers quickly and efficiently.

https://blog.matrixpost.net/what-is-multicast-dns-mdns-and-how-it-works/



**Multicast DNS (mDNS)** is a protocol used in network environments **to resolve hostnames** to **IP addresses** within **small networks** without the need for **a dedicated DNS server**.

It is particularly useful in **local area networks (LANs)**, such as **home** or **office networks**, where devices need to discover and communicate with each other dynamically.

**mDNS** operates **by using multicast packets over UDP (User Datagram Protocol)** to send **DNS queries** to **all devices on the local network**, allowing them to respond with the **appropriate IP addresses**.

-----------------------------

### multiaddr 

A `multiaddress` (often abbreviated `multiaddr`), is a convention for encoding multiple layers of addressing information into a single "future-proof" path structure.

For example: `/ip4/192.0.2.0/udp/1234` encodes two protocols along with their essential addressing information. The `/ip4/192.0.2.0` informs us that we want the `192.0.2.0` loopback address of the IPv4 protocol, and `/udp/1234` tells us we want to send UDP packets to port `1234`.

Multiaddresses can be composed to describe multiple "layers" of addresses.

For more detail, see [Addressing](https://docs.libp2p.io/concepts/fundamentals/addressing/), or the [multiaddr spec](https://github.com/multiformats/multiaddr), which has links to many implementations.

-----------------------------
Here's a **shorter, clearer, and more concise** version of the explanation using **bold** and *italic* formatting:

---

### **Multihash – A Future-Proof Hashing Format**

**Multihash** is a *compact, self-describing* format for cryptographic hashes. It includes:

* **Hash function type**
* **Hash output length**
* **Raw hash value**

This info is stored as a **2-byte prefix** added to the hash, enabling systems to *identify and validate* the hash algorithm used — now and in the future.



**Why It Matters**

Most systems store only the raw hash (e.g., Git), making it *hard to change* the hash function later. **Multihash solves this** by making the function *explicitly part of the output*.


**Where It’s Used**

* **libp2p:** In `PeerId`, which is a hash of the public key.
* **IPFS:** Uses multihashes in:

  * **Content identifiers (CID)**
  * **Peer identity**
  * *CID v0* = raw multihash
  * *Modern CID* = multihash + metadata (via [IPLD](https://ipld.io/))



 **Example**

A **base58 multihash** like
`QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N`
starts with `Qm`, indicating:

* **SHA-256**
* **256-bit output**

This is common for all SHA-256 base58 multihashes.



https://github.com/multiformats/multihash

-----------------------------

# **Multiplexing**

**Multiplexing** (*"muxing"*) combines multiple communication streams over a single logical medium.

## **How It Works**
- Multiple independent data streams → single TCP connection
- TCP connection → single physical connection (ethernet, wifi)

## **Benefits**
- **Reduces network overhead**
- **Improves NAT traversal** efficiency

## **libp2p Implementation**
**Supported protocols:**
- **mplex** - simple protocol with multi-language support
- **yamux** 
- **spdy**

*Multiple protocols can run over one connection, enabling peers to communicate more efficiently.*

-----------------------------
# **multistream**

**multistream** is a lightweight convention for ***"tagging"*** binary data streams with short headers that identify the stream's content.

## **libp2p Usage**
- **Identifies protocols** used for peer communication
- **multistream-select** handles protocol negotiation

*The header tag tells peers what type of data they're receiving in each stream.*

https://github.com/multiformats/multistream-select/

https://github.com/multiformats/multicodec/

-----------------------------
# **NAT (Network Address Translation)**

**NAT** maps addresses from one address space to another, typically at the boundary between private networks and the global internet.

## **Why NAT Exists**
- **IPv4 address space is limited**
- Private networks use many internal addresses while consuming only **one public IP**

## **The Problem**
- ***Outgoing connections*** (private → public) are **easy**
- ***Incoming connections*** (public → private) are **difficult**
- Machines must explicitly tell the router to **forward traffic** for specific ports

## **Impact on Network Models**
**Client/Server:** Works well - outgoing connections provide routing information for responses

**Peer-to-Peer:** Problematic - peers need to **accept incoming connections**, requiring public reachability

## **libp2p Solution**
Implements multiple **NAT Traversal** approaches to enable P2P connectivity.

https://en.wikipedia.org/wiki/Network_address_translation

-----------------------------
### NAT Traversal

**NAT Traversal** – *The process of establishing connections across NAT boundaries.*

- Private networks use internal IP ranges (e.g., *10.0.1.x*).  
- Routers block **incoming traffic** unless explicitly told where to route it.  
- **Port forwarding** via router admin or **UPnP** (supported by *libp2p*) can enable access.

**When NAT fails**:  
- Multiple NAT layers can block traversal.  
- **Circuit Relay** is used: a *public peer* acts as a relay, forwarding traffic to the private peer.  
- Private peers advertise the relay’s *multiaddr* to receive incoming connections.

https://en.wikipedia.org/wiki/Universal_Plug_and_Play

-----------------------------
### Node

 **Node** has multiple meanings in peer-to-peer networking:

## **Software Instance**
A *node* is a running instance of P2P software. For example: "I'm running an orbit-db node on AWS version 3.2.0"
- Also called a **peer** (terms used interchangeably)
- Refers to the entire program participating in the network

## **Node.js Runtime**
The JavaScript runtime environment that supports js-libp2p (context usually makes this clear)

## **Graph Theory Applications**

### *Network Topology*
Nodes represent connected peers in the network graph
- Efficient graph construction and traversal enables effective **peer routing**

### *Data Structures*
Nodes are key elements in structures like:
- **Linked lists**: nodes contain values + links to next node
- **DAGs** (Directed Acyclic Graphs): IPFS naturally stores these
- **IPLD** (Interplanetary Linked Data): grew from IPFS needs but useful beyond it


-----------------------------
### Overlay [#](https://docs.libp2p.io/concepts/appendix/glossary/#overlay)

An "overlay network" or just "overlay" refers to the logical structure of a peer-to-peer network, which is "overlaid" on top of the underlying [transport mechanisms](https://docs.libp2p.io/concepts/appendix/glossary/#transport) used for lower-level network communication.



An **overlay network** is a *logical P2P structure* built on top of lower-level transport protocols.

It defines:

* *Peer discovery and identification*
* *Message propagation*
* *Network behavior*

**libp2p** uses overlays in:

* **DHT** (Kademlia-based)
* **PubSub** networks


An overlay network is a virtual network built on top of an existing network (the underlay). It allows for the creation of new functionalities or services that the underlying network alone cannot provide.

https://en.wikipedia.org/wiki/Overlay_network

-----------------------------


### **PeerId – Unique Cryptographic Identity**

**PeerId** is a *globally unique, verifiable identifier* for a peer in **libp2p**, making impersonation easily detectable.

* Typically a **multihash** of the peer’s **public key**
* Used to retrieve the full public key (e.g., from a **DHT**) for *encryption* or *signature verification*
* Experimental: small public keys can be *inlined* into the PeerId (not recommended for production yet)

**PeerIds** are:

* *Transport-independent*, enabling identity verification across networks
* *Persistent*, unlike IP addresses, surviving location or address changes

-----------------------------

### Peer store 

A data structure that stores [PeerIds](https://docs.libp2p.io/concepts/appendix/glossary/#peerid) for known peers, along with known [multiaddresses](https://docs.libp2p.io/concepts/appendix/glossary/#multiaddr) that can be used to communicate with them.

-----------------------------

**Peer Routing** – *The process of finding a peer’s network address using its peer ID.*

- Can include **local discovery** via methods like *mDNS* (multicast DNS).  
- **Libp2p's main method** uses a *DHT* with the **Kademlia algorithm** for efficient peer lookup.

-----------------------------
**Peer-to-Peer (P2P) Network**  

A *peer-to-peer (P2P)* network allows participants (*peers* or *nodes*) to communicate **directly** on a relatively equal basis.  

- Peers may have different roles, but no central *servers* are required.  
- Unlike the *client/server* model, there’s no strict division between providers (*servers*) and requesters (*clients*).

-----------------------------
### Pubsub 

In general, refers to "publish / subscribe", a communication pattern in which participants "subscribe" for updates "published" by other participants, often on a named "topic".

libp2p defines a [pubsub spec](https://github.com/libp2p/specs/blob/master/pubsub/README.md), with links to several implementations in supported languages. Pubsub is an area of ongoing research and development, with multiple implementations optimized for different use cases and environments.


-----------------------------



-----------------------------



-----------------------------


-----------------------------



-----------------------------



-----------------------------



-----------------------------



-----------------------------



-----------------------------



-----------------------------



-----------------------------



-----------------------------



-----------------------------



-----------------------------



-----------------------------


-----------------------------



-----------------------------



-----------------------------



-----------------------------