# QUIC in libp2p

## Overview
**QUIC** is a modern transport protocol using UDP to provide low-latency, multiplexed, and secure communication. In libp2p, it enhances peer-to-peer connectivity with better performance than TCP-based transports.

## Key Features
- **Low Latency**: Faster connection establishment with fewer round-trips.
- **Multiplexing**: Supports multiple streams over a single connection.
- **Security**: Built-in TLS 1.3 for encrypted communication.
- **Reliability**: Handles packet loss and reordering efficiently.

## How It Works
1. **Connection Setup**:
   - QUIC runs over UDP, avoiding TCP’s handshake overhead.
   - Uses `0-RTT` for faster reconnections when possible.
2. **Stream Management**:
   - Multiplexes streams for concurrent data exchange.
   - Integrates with libp2p protocols like `mplex` or `yamux`.
3. **Peer Addressing**:
   - Multiaddr format: `/ip4/127.0.0.1/udp/1234/quic`.
   - Peers discovered via libp2p mechanisms (e.g., DHT).

## Benefits
- **Performance**: Reduced latency compared to TCP-based transports.
- **Efficiency**: Multiplexing minimizes connection overhead.
- **Resilience**: Better handles network issues like packet loss.

## Limitations
- **UDP Dependency**: May face issues with firewalls or NATs blocking UDP.
- **Implementation Complexity**: Requires specific QUIC libraries.
- **Adoption**: Not as universally supported as TCP.

## Configuration in libp2p
- **Enable QUIC**:
  ```javascript
  import { quic } from '@libp2p/quic'
  const node = await createLibp2p({
    transports: [quic()]
  })
  ```
- **Multiaddr**:
  - Example: `/ip4/127.0.0.1/udp/1234/quic`.

## Use Cases
- High-performance peer-to-peer applications.
- Real-time data streaming in decentralized networks.
- Replacing TCP transports for faster, more efficient communication.


-------------


### **QUIC**

**QUIC** is a new transport protocol that provides an always-encrypted, stream-multiplexed connection built on top of *UDP*. It began as a Google experiment and was later standardized by the IETF.

***

### **Key challenges with TCP**

A standard TCP connection with TLS encryption requires multiple round trips between the client and the server, which can result in a slow connection start. Additionally, TCP suffers from head-of-line (HoL) blocking, where a single lost packet can block all other streams.

***

### **How does QUIC work?**

QUIC combines the functionality of the transport and security layers, building on UDP to bypass the challenges of TCP.

* **Encryption**: By using encryption, QUIC avoids issues with `ossified middleboxes`.
* **TLS 1.3 handshake**: The TLS 1.3 handshake is performed in the first flight, saving a round-trip time (RTT).
* **Multiple streams**: QUIC exposes multiple streams, so no stream multiplexer is needed at the application layer.
* ***0-RTT***: A client can make use of QUIC's 0-RTT feature for subsequent connections, sending encrypted data even before the handshake is complete.

***

### **QUIC native multiplexing**

A single QUIC packet can carry frames containing stream data from one or more streams. This solves the problem of HoL blocking because if a packet containing stream data for one stream is lost, it only blocks that one stream, while all other streams can still make progress.

***

### **QUIC in libp2p**

> Libp2p only supports bidirectional streams and uses TLS 1.3 by default. Since QUIC already provides an encrypted, stream-multiplexed connection, libp2p directly uses QUIC streams, without any additional framing. To authenticate each others' peer IDs, peers encode their peer ID into a self-signed certificate, which they sign using their host's private key. This is the same way peer IDs are authenticated in the libp2p TLS handshake.

***

### **Distinguishing multiple QUIC versions in libp2p**

Libp2p uses different code points to distinguish between two QUIC versions:

1.  `quic-v1` for RFC 9000
2.  `quic` for draft-29