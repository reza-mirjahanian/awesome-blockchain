The goal is to allow peers to discover each other when on the same local network with zero configuration. mDNS uses a multicast system of DNS records; this allows all peers on the local network to see all query responses.

Conceptually, it is very simple. When a peer starts (or detects a network change), it sends a query for all peers. As responses come in, the peer adds the other peers' information into its local database of peers.

Definitions
-----------

-   `service-name` is the DNS Service Discovery (DNS-SD) service name for all peers. It is defined as `_p2p._udp.local`.

-   `host-name` is the fully qualified name of the peer. It is derived from the peer's name and `p2p.local`.

-   `peer-name` is the case-insensitive unique identifier of the peer, and is less than 64 characters.

    As the this field doesn't carry any meaning, it is sufficient to ensure the uniqueness of this identifier. Peers SHOULD generate a random, lower-case alphanumeric string of least 32 characters in length when booting up their node. Peers SHOULD NOT use their Peer ID here because a future Peer ID could exceed the DNS label limit of 63 characters.

If a [private network](https://github.com/libp2p/specs/blob/master/pnet/Private-Networks-PSK-V1.md) is in use, then the `service-name` contains the base-16 encoding of the network's fingerprint as in `_p2p-X._udp.local`. This prevents public and private networks from discovering each other's peers.

Peer Discovery
--------------

### Request

To find all peers, a DNS message is sent with the question `_p2p._udp.local PTR`. Peers will then start responding with their details.

Note that a peer must respond to its own query. This allows other peers to passively discover it.