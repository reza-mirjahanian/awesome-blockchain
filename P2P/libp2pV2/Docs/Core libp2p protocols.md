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



