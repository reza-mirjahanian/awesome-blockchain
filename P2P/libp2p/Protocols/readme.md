These specs define wire protocols that are used by libp2p for connectivity, security, multiplexing, and other purposes.

The protocols described below all use [protocol buffers](https://developers.google.com/protocol-buffers/docs/proto?hl=en) (aka protobuf) to define message schemas.

Existing protocols may use `proto2`, and continue to use them. `proto3` is recommended for new protocols. `proto3` is a simplification of `proto2` and removes some footguns. For context and a discussion around `proto3` vs `proto2`, 465).

-   [ping](https://github.com/libp2p/specs/blob/master/ping/ping.md) \- Ping protocol
-   [autonat](https://github.com/libp2p/specs/blob/master/autonat/README.md) \- NAT detection
-   [identify](https://github.com/libp2p/specs/blob/master/identify/README.md) \- Exchange keys and addresses with other peers
-   [kademlia](https://github.com/libp2p/specs/blob/master/kad-dht/README.md) \- The Kademlia Distributed Hash Table (DHT) subsystem
-   [mdns](https://github.com/libp2p/specs/blob/master/discovery/mdns.md) \- Local peer discovery with zero configuration using multicast DNS
-   [mplex](https://github.com/libp2p/specs/blob/master/mplex/README.md) \- The friendly stream multiplexer
-   [yamux](https://github.com/libp2p/specs/blob/master/yamux/README.md) \- Yet Another Multiplexer
-   [noise](https://github.com/libp2p/specs/blob/master/noise/README.md) \- The libp2p Noise handshake
-   [plaintext](https://github.com/libp2p/specs/blob/master/plaintext/README.md) \- An insecure transport for non-production usage
-   [pnet](https://github.com/libp2p/specs/blob/master/pnet/Private-Networks-PSK-V1.md) \- Private networking in libp2p using pre-shared keys
-   [pubsub](https://github.com/libp2p/specs/blob/master/pubsub/README.md) \- PubSub interface for libp2p
    -   [gossipsub](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/README.md) \- An extensible baseline PubSub protocol
        -   [episub](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/episub.md) \- Proximity Aware Epidemic PubSub for libp2p
-   [relay](https://github.com/libp2p/specs/blob/master/relay/README.md) \- Circuit Switching for libp2p (similar to TURN)
    -   [dcutr](https://github.com/libp2p/specs/blob/master/relay/DCUtR.md) \- Direct Connection Upgrade through Relay protocol
-   [rendezvous](https://github.com/libp2p/specs/blob/master/rendezvous/README.md) \- Rendezvous Protocol for generalized peer discovery
-   [secio](https://github.com/libp2p/specs/blob/master/secio/README.md) \- SECIO, a transport security protocol for libp2p
-   [tls](https://github.com/libp2p/specs/blob/master/tls/tls.md) \- The libp2p TLS Handshake (TLS 1.3+)
-   [quic](https://github.com/libp2p/specs/blob/master/quic/README.md) \- The libp2p QUIC Handshake
-   [webrtc](https://github.com/libp2p/specs/blob/master/webrtc/README.md) \- The libp2p WebRTC transports
-   [WebTransport](https://github.com/libp2p/specs/blob/master/webtransport/README.md) \- Using WebTransport in libp2p