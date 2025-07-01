In addition to the protocols that you write when developing a libp2p application, libp2p itself defines several foundational protocols that are used for core features.

### Common patterns 

The protocols described below all use [protocol buffers](https://developers.google.com/protocol-buffers/) (aka protobuf) to define message schemas.

Messages are exchanged over the wire using a very simple convention which prefixes binary message payloads with an integer that represents the length of the payload in bytes. The length is encoded as a [protobuf varint](https://developers.google.com/protocol-buffers/docs/encoding#varints) (variable-length integer).

### Ping 

| **Protocol id** | spec |  |  | implementations |
| --- |  --- |  --- |  --- |  --- |
| `/ipfs/ping/1.0.0` | N/A | [go](https://github.com/libp2p/go-libp2p/tree/master/p2p/protocol/ping) | [js](https://github.com/libp2p/js-libp2p-ping) | [rust](https://github.com/libp2p/rust-libp2p/blob/master/protocols/ping/src/lib.rs) |


The libp2p ping protocol is a simple liveness check that peers can use to test the connectivity and performance between two peers. The libp2p ping protocol is different from the ping command line utility ([ICMP ping](https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol)), as it requires an already established libp2p connection.

> ICMP Ping is a network utility that uses ICMP packets to check the connectivity and latency between two networked devices. It is typically used to check the reachability of a host on an IP network and to measure the round-trip time for messages sent from the originating host to a destination host.

A peer opens a new stream on an existing libp2p connection and sends a ping request with a random 32 byte payload. The receiver echoes these 32 bytes back on the same stream. By measuring the time between the request and response, the initiator can calculate the round-trip time of the underlying libp2p connection. The stream can be reused for future pings from the initiator.

#### Example [#](https://docs.libp2p.io/concepts/fundamentals/protocols/#example)

[Kubo](https://github.com/ipfs/kubo) exposes a command line interface to ping other peers, which uses the libp2p ping protocol.

```
`ipfs ping /ipfs/QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG/ping

PING /ipfs/QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG/ping (QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG)
32 bytes from QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG: time=11.34ms
`
```
💡
See the ping [technical specification](https://github.com/libp2p/specs/blob/master/ping/ping.md) for more details.


---

## Identify 

| **Protocol id** | spec |  |  | implementations |
| --- |  --- |  --- |  --- |  --- |
| `/ipfs/id/1.0.0` | [identify spec](https://github.com/libp2p/specs/pull/97/files) | [go](https://github.com/libp2p/go-libp2p/tree/master/p2p/protocol/identify) | [js](https://github.com/libp2p/js-libp2p-identify) | [rust](https://github.com/libp2p/rust-libp2p/tree/master/protocols/identify/src) |


The `identify` protocol allows peers to exchange information about each other, most notably their public keys and known network addresses.

The basic identify protocol works by establishing a new stream to a peer using the identify protocol id shown in the table above.

When the remote peer opens the new stream, they will fill out an [`Identify` protobuf message](https://github.com/libp2p/go-libp2p/blob/master/p2p/protocol/identify/pb/identify.proto) containing information about themselves, such as their public key, which is used to derive their [`PeerId`](https://docs.libp2p.io/concepts/fundamentals/peers/).

Importantly, the `Identify` message includes an `observedAddr` field that contains the [multiaddr](https://docs.libp2p.io/concepts/appendix/glossary#multiaddr) that the peer observed the request coming in on. This helps peers determine their NAT status, since it allows them to see what other peers observe as their public address and compare it to their own view of the network.

#### identify/push 

| **Protocol id** | spec & implementations |
| --- |  --- |
| `/ipfs/id/push/1.0.0` | same as [identify above](https://docs.libp2p.io/concepts/fundamentals/protocols/#identify) |


A slight variation on `identify`, the `identify/push` protocol sends the same `Identify` message, but it does so proactively instead of in response to a request.

**This is useful** if a peer starts listening on a new address, establishes a new [relay circuit](https://docs.libp2p.io/concepts/nat/circuit-relay/), or learns of its public address from other peers using the standard `identify` protocol. Upon creating or learning of a new address, the peer can push the new address to all peers it's currently aware of. This keeps everyone's routing tables up to date and makes it more likely that other peers will discover the new address.


### kad-dht 

`kad-dht` is a [Distributed Hash Table](https://en.wikipedia.org/wiki/Distributed_hash_table) based on the [Kademlia](https://en.wikipedia.org/wiki/Kademlia) routing algorithm, with some modifications.

libp2p uses the DHT as the foundation of its [routing](https://docs.libp2p.io/concepts/discovery-routing/overview/) functionality. To learn more about DHT and the Kademlia algorithm, check out the [Distributed Hash Tables guide](https://docs.ipfs.tech/concepts/dht/) on the IPFS documentation site. In addition, check out the [libp2p implementations page](https://libp2p.io/implementations/) for updates on all the kad-libp2p implementations.


### Circuit Relay [#](https://docs.libp2p.io/concepts/fundamentals/protocols/#circuit-relay)

| **Protocol id** | spec |  | implementations |
| --- |  --- |  --- |  --- |
| `/libp2p/circuit/relay/0.1.0` | [circuit relay spec](https://github.com/libp2p/specs/tree/master/relay) | [go](https://github.com/libp2p/go-libp2p-circuit) | [js](https://github.com/libp2p/js-libp2p-circuit) |
| --- |  --- |  --- |  --- |

As described in the [Circuit Relay article](https://docs.libp2p.io/concepts/nat/circuit-relay/), libp2p provides a protocol for tunneling traffic through relay peers when two peers are unable to connect to each other directly. See the article for more information on working with relays