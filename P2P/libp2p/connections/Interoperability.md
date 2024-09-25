### Interoperability

Support for connection security protocols and stream multiplexers varies across libp2p implementations. To support the widest variety of peers, implementations should support a baseline "stack" of security and multiplexing protocols.

The recommended baseline **security** protocol is [Noise](https://github.com/libp2p/specs/blob/master/noise/README.md), which is supported in all current libp2p implementations.

The recommended baseline **stream multiplexer** is [yamux](https://github.com/hashicorp/yamux/blob/master/spec.md), which provides a very simple programmatic API and is supported in most libp2p implementations.