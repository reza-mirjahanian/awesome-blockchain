# TLS in libp2p

**Definition**: Transport Layer Security (TLS) is a cryptographic protocol ensuring secure data channels with encryption, authentication, and data integrity.

**Learn More**: [TLS 1.3 RFC 8446](https://tools.ietf.org/html/rfc8446)

## TLS in libp2p

- **Purpose**: Secures transports lacking built-in security (e.g., TCP, WebSocket) via a handshake.
- **TLS 1.3 Usage**:
  - Establishes secure connections between peers.
  - Protocol ID: `/tls/1.0.0`.
  - Peers authenticate each other’s `Peer ID` during the handshake.
- **Authentication**:
  - Public key encoded in the TLS certificate.
  - Supports key types like `secp256k1`, not typically supported by TLS stacks.
  - Nodes derive `Peer ID` from the public key to verify identity.

## Comparison with Noise

- **TLS**:
  - Industry-standard, widely adopted.
  - Complex but robust for authentication and encryption.
- **Noise**:
  - Lightweight, designed for P2P.
  - Offers forward secrecy, simpler implementation.

> **Note**: TLS 1.3 is mandatory; lower versions are not supported.[](https://docs.libp2p.io/concepts/secure-comm/tls/)