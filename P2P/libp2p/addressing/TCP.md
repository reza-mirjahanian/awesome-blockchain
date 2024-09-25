### TCP

The libp2p TCP transport is supported in all implementations and can be used
wherever TCP/IP sockets are accessible.

Addresses for the TCP transport are of the form `<ip-multiaddr>/tcp/<tcp-port>`,
where `<ip-multiaddr>` is a multiaddr that resolves to an IP address, as
described in the [IP and Name Resolution section](#ip-and-name-resolution).
The `<tcp-port>` argument must be a 16-bit unsigned integer.