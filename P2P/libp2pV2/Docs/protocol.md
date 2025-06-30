### Protocol IDs [#](https://docs.libp2p.io/concepts/fundamentals/protocols/#protocol-ids)

libp2p protocols have unique string identifiers, which are used in the [protocol negotiation](https://docs.libp2p.io/concepts/fundamentals/protocols/#protocol-negotiation) process when connections are first opened.

By convention, protocol ids have a path-like structure, with a version number as the final component:

```
/my-app/amazing-protocol/1.0.1
```

Breaking changes to your protocol's wire format or semantics should result in a new version number. See the [protocol negotiation section](https://docs.libp2p.io/concepts/fundamentals/protocols/#protocol-negotiation) for more information about how version selection works during the dialing and listening process.

💡While libp2p will technically accept any string as a valid protocol id, using the recommended path structure with a version component is both developer-friendly and enables [easier matching by version](https://docs.libp2p.io/concepts/fundamentals/protocols/#match-using-semver).


#### Handler functions 

To accept connections, a libp2p application will register handler functions for protocols using their protocol id with the [switch](https://docs.libp2p.io/concepts/appendix/glossary#switch) (aka "swarm"), or a higher level interface such as [go's Host interface](https://github.com/libp2p/go-libp2p/blob/master/core/host/host.go).

The handler function will be invoked when an incoming stream is tagged with the registered protocol id. If you register your handler with a [match function](https://docs.libp2p.io/concepts/fundamentals/protocols/#using-a-match-function), you can choose whether to accept non-exact string matches for protocol ids, for example, to match on [semantic major versions](https://docs.libp2p.io/concepts/fundamentals/protocols/#match-using-semver).


#### Binary streams 

The "medium" over which a libp2p protocol transpires is a bi-directional **binary** stream with the following properties:

-   Bidirectional, reliable delivery of binary data
    -   Each side can read and write from the stream at any time
    -   Data is read in the same order as it was written
    -   Can be "half-closed", that is, closed for writing and open for reading, or closed for reading and open for writing
-   Supports backpressure
    -   Readers can't be flooded by eager writers

Behind the scenes, libp2p will also ensure that the stream is [secure](https://docs.libp2p.io/concepts/secure-comm/overview/) and efficiently [multiplexed](https://docs.libp2p.io/concepts/multiplex/overview/). This is transparent to the protocol handler, which reads and writes unencrypted binary data over the stream.

The format of the binary data and the mechanics of what to send when and by whom are all up to the protocol to determine. For inspiration, some [common patterns](https://docs.libp2p.io/concepts/fundamentals/protocols/#common-patterns) that are used in libp2p's internal protocols are outlined below.

> 
> How Backpressure Works
> ----------------------
> 
> When a network component (like a router, switch, or receiving host) becomes congested and can't process incoming data fast enough, it sends signals back toward the source to slow down transmission. This creates a "pressure" that flows backward against the normal data flow direction.
> 
> Common Backpressure Mechanisms
> ------------------------------
> 
> **TCP Flow Control**: Uses sliding window protocols where receivers advertise their available buffer space. When buffers fill up, the receiver reduces the advertised window size, forcing the sender to slow down.
> 
> **Buffer Management**: Network devices drop packets when their queues are full, which signals congestion to upstream devices through packet loss detection and retransmission timeouts.
> 
> **Explicit Congestion Notification (ECN)**: Routers mark packets instead of dropping them when experiencing congestion, allowing endpoints to reduce transmission rates without packet loss.
> 
> **Quality of Service (QoS)**: Traffic shaping and policing mechanisms that limit bandwidth for certain traffic types, creating backpressure for lower-priority flows.


Protocol Negotiation 
---------------------

When dialing out to initiate a new stream, libp2p will send the protocol id of the protocol you want to use. The listening peer on the other end will check the incoming protocol id against the registered protocol handlers.

If the listening peer does not support the requested protocol, it will end the stream, and the dialing peer can try again with a different protocol, or possibly a fallback version of the initially requested protocol.

If the protocol is supported, the listening peer will echo back the protocol id as a signal that future data sent over the stream will use the agreed protocol semantics.

This process of reaching agreement about what protocol to use for a given stream or connection is called **protocol negotiation**.

-----------------

### Matching protocol ids and versions 

When you register a protocol handler, there are two methods you can use.

The first takes two arguments: a protocol id, and a handler function. If an incoming stream request sends an exact match for the protocol id, the handler function will be invoked with the new stream as an argument.


#### Using a match function 

The second kind of protocol registration takes three arguments: the protocol id, a protocol match function, and the handler function.

When a stream request comes in whose protocol id doesn't have any exact matches, the protocol id will be passed through all of the registered match functions. If any returns `true`, the associated handler function will be invoked.

This gives you a lot of flexibility to do your own "fuzzy matching" and define whatever rules for protocol matching make sense for your application.