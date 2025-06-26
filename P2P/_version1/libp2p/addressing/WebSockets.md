WebSocket connections are encapsulated within TCP/IP sockets, and the WebSocket
multiaddr format mirrors this arrangement.

A libp2p WebSocket multiaddr is of the form `<tcp-multiaddr>/ws` or
`<tcp-multiaddr>/wss` (TLS-encrypted), where `<tcp-multiaddr`> is a valid
mulitaddr for the TCP transport, as [described above](#tcp).