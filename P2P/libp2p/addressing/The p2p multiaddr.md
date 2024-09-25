### The p2p multiaddr

libp2p defines the `p2p` multiaddr protocol, whose address component is the [peer id](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md) of a libp2p peer. The text representation of a `p2p` multiaddr looks like this:

```
/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N

```

Where `QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N` is the string representation of a peer's peer ID derived from its public key.

By itself, a `p2p` address does not give you enough addressing information to locate a peer on the network; it is not a transport address. However, like the `ws` protocol for WebSockets, a `p2p` address can be [encapsulated within](https://github.com/libp2p/specs/blob/master/addressing/README.md#encapsulation) another multiaddr.

For example, the above `p2p` address can be combined with the transport address on which the node is listening:

```
/ip4/198.51.100/tcp/1234/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N

```

This combination of transport address plus `p2p` address is the format in which peers exchange addresses over the wire in the [identify protocol](https://github.com/libp2p/specs/blob/master/identify/README.md) and other core libp2p protocols.


----
#### Historical Note: the `ipfs` multiaddr Protocol

The `p2p` multiaddr protocol was originally named `ipfs`, and we've been eliminating support for the ipfs string representation of this multiaddr component. It may be printed as `/ipfs/<peer-id>` instead of `/p2p/<peer-id>` in its string representation depending on the implementation in use. Both names resolve to the same protocol code, and they are equivalent in the binary form.