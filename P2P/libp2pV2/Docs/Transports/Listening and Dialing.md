Common transport interfaces 
------------------------------------------------------------------------------------------------------------------------

Transports are defined in terms of two core operations, **listening** and **dialing**.

Listening means that you can accept incoming connections from other peers, using whatever facility is provided by the transport implementation. For example, a TCP transport on a unix platform could use the `bind` and `listen` system calls to have the operating system route traffic on a given TCP port to the application.

Dialing is the process of opening an outgoing connection to a listening peer. Like listening, the specifics are determined by the implementation, but every transport in a libp2p implementation will share the same programmatic interface.

---------------

Addresses 
------------------------------------------------------------------------------------

Before you can dial up a peer and open a connection, you need to know how to reach them. Because each transport will likely require its own address scheme, libp2p uses a convention called a "multiaddress" or `multiaddr` to encode many different addressing schemes.

The [addressing doc](https://docs.libp2p.io/concepts/fundamentals/addressing/) goes into more detail, but an overview of how multiaddresses work is helpful for understanding the dial and listen interfaces.

Here's an example of a multiaddr for a TCP/IP transport:

```
/ip4/198.51.100.0/tcp/6543
```

This is equivalent to the more familiar `198.51.100.0:6543` construction, but it has the advantage of being explicit about the protocols that are being described. With the multiaddr, you can see at a glance that the `198.51.100.0` address belongs to the IPv4 protocol, and the `6543` belongs to TCP.

For more complex examples, see [addressing](https://docs.libp2p.io/concepts/fundamentals/addressing/).

Both dial and listen deal with multiaddresses. When listening, you give the transport the address you'd like to listen on, and when dialing you provide the address to dial to.

When dialing a remote peer, the multiaddress should include the [PeerId](https://docs.libp2p.io/concepts/fundamentals/peers/#peer-id) of the peer you're trying to reach. This lets libp2p establish a [secure communication channel](https://docs.libp2p.io/concepts/secure-comm/overview/) and prevents impersonation.

An example multiaddress that includes a `PeerId`:

```
/ip4/192.0.2.0/tcp/4321/p2p/QmcEPrat8ShnCph8WjkREzt5CPXF2RwhYxYBALDcLC1iV6
```

The `/p2p/QmcEPrat8ShnCph8WjkREzt5CPXF2RwhYxYBALDcLC1iV6` component uniquely identifies the remote peer using the hash of its public key. For more, see the [peer identity content](https://docs.libp2p.io/concepts/fundamentals/peers/#peer-id).

