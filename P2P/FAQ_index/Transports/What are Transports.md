# What are Transports

## Overview

When connecting from your computer to an internet machine, you're likely using **TCP/IP** - the successful combination of:

- **Internet Protocol (IP)**: Handles addressing and delivery of data packets
- **Transmission Control Protocol (TCP)**: Ensures data is received completely and in correct order

### Protocol Options

**TCP/IP** is the default choice for networked applications due to its ubiquity and support.

**UDP** serves as an alternative when TCP adds too much overhead - it's a simpler protocol with *no guarantees* about reliability or ordering.

### Beyond TCP and UDP

While TCP and UDP (with IP) are the most common protocols today, alternatives exist at:

- **Lower levels**: 
  - Raw ethernet packets
  - Bluetooth frames
  
- **Higher levels**:
  - QUIC (layered over UDP)

## Transports and libp2p

**Transports** are the foundational protocols that move bits around.

### Core Requirements

libp2p's fundamental requirement is to be **transport agnostic**, meaning:

- Developers choose which transport protocol to use
- Applications can support *multiple different transports simultaneously*
- The system provides flexibility in transport selection

> The decision of what transport protocol to use is up to the developer, and an application can support many different transports at the same time.