Peer ID 
--------

A Peer Identity (often written `PeerID`) is a unique reference to a specific peer within the overall peer-to-peer network.

As well as serving as a unique identifier for each peer, a Peer ID is a verifiable link between a peer and its public cryptographic key.

Each libp2p peer controls a private key, which it keeps secret from all other peers. Every private key has a corresponding public key, which is shared with other peers.

Together, the public and private key (or "key pair") allow peers to establish [secure communication](https://docs.libp2p.io/concepts/secure-comm/overview/) channels with each other.

Conceptually, a Peer ID is a [cryptographic hash](https://en.wikipedia.org/wiki/Cryptographic_hash_function) of a peer's public key. When peers establish a secure channel, the hash can be used to verify that the public key used to secure the channel is the same one used to identify the peer.

The [Peer ID spec](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md) goes into detail about the byte formats used for libp2p public keys and how to hash the key to produce a valid Peer ID.