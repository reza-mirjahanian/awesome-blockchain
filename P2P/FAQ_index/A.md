

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