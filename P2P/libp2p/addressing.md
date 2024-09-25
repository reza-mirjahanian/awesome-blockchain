libp2p makes a distinction between a peer's **identity** and its **location**. A peer's identity is stable, verifiable, and valid for the entire lifetime of the peer (whatever that may be for a given application). Peer identities are derived from public keys as described in the [peer id spec](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md).

On a particular network, at a specific point in time, a peer may have one or more locations, which can be represented using addresses. For example, I may be reachable via the global IPv4 address of 198.51.100 on TCP port 1234.

In a system that only supported TCP/IP or UDP over IP, we could easily write our addresses with the familiar `<ip>:<port>` notation and store them as tuples of address and port. However, libp2p was designed to be transport agnostic, which means that we can't assume that we'll even be using an IP-backed network at all.

To support a growing set of transport protocols without special-casing each addressing scheme, libp2p uses [multiaddr](https://github.com/multiformats/multiaddr) to encode network addresses for all supported transport protocols, in a self-describing manner.

This document does not cover the address format ([multiaddr](https://github.com/multiformats/multiaddr)), but rather [how multiaddr is used in libp2p](https://github.com/libp2p/specs/blob/master/addressing/README.md#multiaddr-in-libp2p). For details on the former, visit linked spec. For more information on other use cases, or to find links to multiaddr implementations in various languages, see the [mulitaddr repository](https://github.com/multiformats/multiaddr).

--------

multiaddr in libp2p
-------------------

multiaddrs are used throughout libp2p for encoding network addresses. When addresses need to be shared or exchanged between processes, they are encoded in the binary representation of multiaddr.

When exchanging addresses, peers send a multiaddr containing both their network address and peer id, as described in [the section on the `p2p` multiaddr](https://github.com/libp2p/specs/blob/master/addressing/README.md#the-p2p-multiaddr).