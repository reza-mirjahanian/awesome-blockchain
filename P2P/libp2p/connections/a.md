
A **connection** is a reliable, bidirectional communication channel between two
libp2p peers that provides **security** and the ability to open multiple
logically independent **streams**.

**Security** in this context means that all communications (after an initial
handshake) are encrypted, and that the identity of each peer is cryptographically
verifiable by the other peer.

**Streams** are reliable, bidirectional channels that are multiplexed over a
libp2p connection. They must support backpressure, which prevents receivers from
being flooded by data from eager senders. They can also be "half closed",
meaning that a stream can be closed for writing data but still open to receiving
data and vice versa.

Support for multiple streams ensures that a single connection between peers can
support a wide variety of interactions, each with their own protocol. This is
especially helpful if connections are difficult to establish due to NAT
traversal issues or other connectivity barriers.

Connections take place over an underlying **transport**, for example TCP
sockets, websockets, or various protocols layered over UDP.

While some transport protocols like [QUIC][quic-spec] have "built in" security
and stream multiplexing, others such as TCP need to have those capabilities
layered on top of the "raw" transport connection.

When the base capabilities of security and stream multiplexing are not natively
supported by the underlying transport protocol, a **connection upgrade** process
occurs to augment the raw transport connection with the required features.

libp2p peers can both initiate connections to other peers and accept incoming
connections. We use the term **dial** to refer to initiating outbound
connections, and **listen** to refer to accepting inbound connections.