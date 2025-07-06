# mplex in libp2p

## Overview
**mplex** is a basic *stream multiplexer* for libp2p, designed early in its development. It operates over a reliable, ordered connection (e.g., *TCP*) and supports opening, writing, closing, and resetting streams using a message-based framing layer, similar to *Yamux*.

## Key Features
- **Functionality**: Multiplexes different data streams, including stream-oriented data.
- **Protocol**: Runs on `/mplex/6.7.0`.

## Drawbacks
- **No Flow Control**: Lacks *backpressure* to prevent overwhelming slower peers.
- **Unlimited Streams**: No restrictions on the number of streams a peer can open.
- **Deprecation**: Being phased out (track progress [here](https://github.com/libp2p/specs/issues/)).
- **Recommendation**: Use *Yamux* for flow control and better handling of large data transfers, or prefer *QUIC* over TCP.

## Use Case
> mplex is only recommended for backward compatibility with legacy nodes, as *js-libp2p* now supports *Yamux*.