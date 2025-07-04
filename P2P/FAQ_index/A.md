

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



-----------------------------



-----------------------------