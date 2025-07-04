What is QUIC? 
--------------

QUIC is a new transport protocol that provides an always-encrypted, stream-multiplexed connection built on top of UDP. It started as an experiment by Google between Google services and Chrome in 2014, and was later standardized by the IETF in [RFC 9000](https://datatracker.ietf.org/doc/html/rfc9000), [RFC 9001](https://datatracker.ietf.org/doc/html/rfc9001), and [RFC 9002](https://datatracker.ietf.org/doc/html/rfc9002).

### Key challenges with TCP 

1.  Head-of-line blocking (HoL blocking): TCP is a single byte stream exposed by the kernel, so streams layered on top of TCP experience head-of-line (HoL) blocking.

    💡
    In TCP, head-of-line blocking occurs when a single packet is lost, and packets delivered after that need to wait in the kernel buffer until a retransmission for the lost packet is received.

2.  Ossification: Because the header of TCP packet is not encrypted, middleboxes can inspect and modify TCP header fields and may break unexpectedly when they encounter anything they don't understand. This makes it practically impossible to deploy any changes to the TCP protocol that change the wire format.

3.  Handshake inefficiency: TCP spends one network round-trip (RTT) on verifying the client's address. Only after this can TLS start the cryptographic handshake, consuming another RTT. Setting up an encrypted connection therefore always takes 2 RTTs.

QUIC was designed with the following goals in mind:

-   Making the transport layer aware of streams, so that packet loss doesn't cause HoL blocking between streams.
-   Reducing the latency of connection establishment to a single RTT for new connections, and to allow sending of 0 RTT application data for resumed connections.
-   Encrypting as much as possible. This eliminates the ossification risk, as middleboxes aren't able to read any encrypted fields. This allows future evolution of the protocol.