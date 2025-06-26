### `p2p-circuit` Relay Addresses

The libp2p [circuit relay protocol][relay-spec] allows a libp2p peer A to
communicate with another peer B via a third party C. This is useful for
circumstances where A and B would be unable to communicate directly.

Once a connection to the relay is established, peers can accept incoming
connections through the relay, using a `p2p-circuit` address.

Like the `ws` WebSocket multiaddr protocol the `p2p-circuit` multiaddr does not
carry any additional address information. Instead it is composed with two other
multiaddrs to describe a relay circuit.

A full `p2p-circuit` address that describes a relay circuit is of the form:
`<relay-multiaddr>/p2p-circuit/<destination-multiaddr>`.

`<relay-multiaddr>` is the full address for the peer relaying the traffic (the
"relay node").

The details of the transport connection between the relay node and the
destination peer are usually not relevant to other peers in the network, so
`<destination-multiaddr>` generally only contains the `p2p` address of the
destination peer.

A full example would be:

```
/ip4/192.0.2.0/tcp/5002/p2p/QmdPU7PfRyKehdrP5A3WqmjyD6bhVpU1mLGKppa2FjGDjZ/p2p-circuit/p2p/QmVT6GYwjeeAF5TR485Yc58S3xRF5EFsZ5YAF4VcP3URHt
```

Here, the destination peer has the peer id
`QmVT6GYwjeeAF5TR485Yc58S3xRF5EFsZ5YAF4VcP3URHt` and is reachable through a
relay node with peer id `QmdPU7PfRyKehdrP5A3WqmjyD6bhVpU1mLGKppa2FjGDjZ` running
on TCP port 5002 of the IPv4 loopback interface.

[peer-id-spec]: ../peer-ids/peer-ids.md
[identify-spec]: ../identify/README.md
[multiaddr-repo]: https://github.com/multiformats/multiaddr
[multiaddr-proto-table]: https://github.com/multiformats/multiaddr/blob/master/protocols.csv
[relay-spec]: ../relay/README.md