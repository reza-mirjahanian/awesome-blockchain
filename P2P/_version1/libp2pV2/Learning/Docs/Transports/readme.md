![alt text](image.png)

Because TCP/IP is so ubiquitous and well-supported, it's often the default choice for networked applications. In some cases, TCP adds too much overhead, so applications might use [UDP](https://en.wikipedia.org/wiki/User_Datagram_Protocol), a much simpler protocol with no guarantees about reliability or ordering.

While TCP and UDP (together with IP) are the most common protocols in use today, they are by no means the only options. Alternatives exist at lower levels (e.g. sending raw ethernet packets or bluetooth frames), and higher levels (e.g. QUIC, which is layered over UDP).

The foundational **protocols that move bits around are called transports**, and one of libp2p's core requirements is to be transport agnostic. This means that the decision of what transport protocol to use is up to the developer, and an application can support many different transports at the same time. Learn about the fundamentals of transport protocols in libp2p.

Transports and libp2p 
----------------------

To better visualize the types of libp2p connections, check out the [connectivity site](https://connectivity.libp2p.io/) which outlines the different types of connectivity in libp2p.

[Edit this page on GitHub](https://github.com/libp2p/docs/blob/master/content/concepts/transports/overview.md)


