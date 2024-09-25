### State Management

While the connection establishment process itself does not require any
persistent state, some state management is useful to assist bootstrapping and
maintain resource limits.

#### Peer Metadata Storage

It's recommended that libp2p implementations provide a persistent metadata
storage interface that contains at minimum the peer id and last known valid
addresses for each peer. This allows you to more easily "catch
back up" and rejoin a dense network between invocations of your libp2p
application without having to rely on a few bootstrap nodes and random DHT walks
to build up a routing table.

Even during a single invocation of an application, you're likely to benefit from
an in-memory metadata storage facility, which will allow you to cache addresses
for connection resumption. Designing a storage interface which can be backed by
memory or persistent storage will let you swap in whichever is appropriate for
your use case and stage of development.

For examples, see [go-libp2p-peerstore][go-libp2p-peerstore] and
[js-peer-book][js-peer-book].

#### Connection Limits

Maintaining a large number of persistent connections can cause issues with some
network environments and can lead to resource exhaustion and erratic behavior.

It's highly recommended that libp2p implementations maintain an upper bound on
the number of open connections. Doing so while still maintaining robust
performance and connectivity will likely require implementing some kind of
priority mechanism for selecting which connections are the most "expendable"
when you're near the limit.

Resource allocation, measurement and enforcement policies are all an active area
of discussion in the libp2p community, and implementations are free to develop
whatever prioritization system makes sense.

#### Supported protocols

A libp2p node SHOULD scope its set of supported protocols to the underlying
physical connection to a peer. It MAY only support a protocol based on properties
of a physical connection to e.g. limit the use of bandwidth-heavy protocols over
a relayed or metered connection. A libp2p node MAY offer different sets of protocols
to different peers. It MAY revoke or add the support for a protocol at any time,
for example to only offer certain services after learning its NAT status on a connection.
Therefore, libp2p nodes SHOULD NOT assume that the set of protocols on a connection
is static.

### Connection Lifecycle Events

The establishment of new connections and streams is likely to be a
"cross-cutting concern" that's of interest to various parts of your application
(or parts of libp2p) besides the protocol handlers that directly deal with the
traffic.

For example, the [persistent metadata component](#peer-metadata-storage) could
automatically add peer ids and addresses to its registry whenever a new peer
connects, or a DHT module could update its routing tables when a connection is
terminated.

To support this, it's recommended that libp2p implementations support a
notification or event delivery system that can inform interested parties about
connection lifecycle events.

The full set of lifecycle events is not currently specified, but a recommended
baseline would be:

| Event        | Description                               |
|--------------|-------------------------------------------|
| Connected    | A new connection has been opened          |
| Disconnected | A connection has closed                   |
| OpenedStream | A new stream has opened over a connection |
| ClosedStream | A stream has closed                       |
| Listen       | We've started listening on a new address  |
| ListenClose  | We've stopped listening on an address     |