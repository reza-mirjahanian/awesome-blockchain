QUIC sessions are encapsulated within UDP datagrams, and the libp2p QUIC
multiaddr format mirrors this arrangement.

A libp2p QUIC multiaddr is of the form `<ip-multiaddr>/udp/<udp-port>/quic`,
where `<ip-multiaddr>` is a multiaddr that resolves to an IP address, as
described in the [IP and Name Resolution section](#ip-and-name-resolution).
The `<udp-port>` argument must be a 16-bit unsigned integer in network byte order.