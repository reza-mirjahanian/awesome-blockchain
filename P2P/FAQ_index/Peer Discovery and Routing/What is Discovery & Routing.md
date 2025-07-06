### Discovery & Routing in P2P Networks

In a peer-to-peer (P2P) network, nodes must be able to find and communicate with each other without a central server. This is achieved through two essential processes: **peer discovery** and **peer routing**.

***

### Peer Discovery 🗺️

**Peer discovery** is the process of *finding and announcing* other available peers on the network. This can be done by:

-   Broadcasting a message to the entire network.
-   Contacting a *bootstrap node* to get an initial list of known peers.

***

### Peer Routing 🧭

**Peer routing** is the process of *finding a specific peer's location* or network address. This typically involves maintaining a routing table that tracks the network's structure. Algorithms are used to find the "closest" peers to a given peer ID.

> In practice, the line between discovery and routing is often blurry. A peer might use a routing algorithm to locate one peer and then use that information to discover others nearby. They often happen at the same time.

***

### Discovery and Routing in `libp2p`

The `libp2p` framework provides several modules for discovery and routing. Peers can find each other by exchanging `multiaddresses`, querying a directory, or using a Distributed Hash Table (DHT).

Key mechanisms include:

-   ***Rendezvous***: A protocol allowing peers to securely and privately exchange their `multiaddresses` at a common meeting point.
-   ***mDNS***: A protocol that uses multicast DNS to let peers discover each other automatically on a **local network**.
-   ***DHT (Distributed Hash Table)***: `libp2p` uses a DHT called *Kademlia*. It assigns content a unique ID and stores it on the peer whose own ID is closest to the content's ID, making data retrieval efficient.