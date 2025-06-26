# libp2p: From Foundations to Mastery

## **Core Concepts and Architecture**

**libp2p** is a modular network stack for peer-to-peer applications. Unlike traditional client-server architectures, libp2p enables direct communication between nodes without centralized intermediaries.

### **Key Components**

**Transport Layer** - Handles the actual network connections (TCP, UDP, WebSockets, QUIC)
**Connection Security** - Encrypts communications (TLS, Noise Protocol)
**Peer Identity** - Cryptographic identities for nodes
**Peer Discovery** - Finding other peers in the network
**Stream Multiplexing** - Multiple logical streams over single connection
**Addressing** - Multiaddresses for flexible endpoint specification

### **Network Stack Comparison**

| Traditional Networking | libp2p |
|------------------------|---------|
| Fixed protocol stack | Modular, swappable components |
| Client-server model | Peer-to-peer model |
| IP addresses | Multiaddresses |
| Single transport | Multiple transports |
| Centralized discovery | Distributed discovery |

## **Peer Identity and Cryptography**

Every libp2p node has a **PeerID** derived from its public key. This creates a self-sovereign identity system.

```go
// Go example: Creating a peer identity
package main

import (
    "crypto/rand"
    "fmt"
    "github.com/libp2p/go-libp2p/core/crypto"
    "github.com/libp2p/go-libp2p/core/peer"
)

func createPeerIdentity() (crypto.PrivKey, peer.ID, error) {
    // Generate Ed25519 private key
    priv, pub, err := crypto.GenerateEd25519Key(rand.Reader)
    if err != nil {
        return nil, "", err
    }
    
    // Derive PeerID from public key
    peerID, err := peer.IDFromPublicKey(pub)
    if err != nil {
        return nil, "", err
    }
    
    return priv, peerID, nil
}
```

```rust
// Rust example: Peer identity creation
use libp2p::identity::Keypair;
use libp2p::PeerId;

fn create_peer_identity() -> (Keypair, PeerId) {
    // Generate Ed25519 keypair
    let keypair = Keypair::generate_ed25519();
    
    // Derive PeerID
    let peer_id = PeerId::from(keypair.public());
    
    (keypair, peer_id)
}
```

## **Multiaddresses: Flexible Addressing**

**Multiaddresses** compose multiple protocols into a single address format, enabling protocol flexibility and future-proofing.

### **Multiaddress Structure**

```
/ip4/192.168.1.1/tcp/4001/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt
│    │            │    │    │    │
│    │            │    │    │    └── Peer ID
│    │            │    │    └────── Protocol identifier
│    │            │    └─────────── Port
│    │            └──────────────── Transport protocol
│    └───────────────────────────── Network protocol
└────────────────────────────────── Address family
```

```go
// Go: Working with multiaddresses
package main

import (
    "fmt"
    ma "github.com/multiformats/go-multiaddr"
)

func demonstrateMultiaddresses() {
    // Create various multiaddresses
    tcpAddr, _ := ma.NewMultiaddr("/ip4/127.0.0.1/tcp/4001")
    udpAddr, _ := ma.NewMultiaddr("/ip4/127.0.0.1/udp/4001")
    wsAddr, _ := ma.NewMultiaddr("/ip4/127.0.0.1/tcp/8080/ws")
    quicAddr, _ := ma.NewMultiaddr("/ip4/127.0.0.1/udp/4001/quic")
    
    // Combine with peer ID
    peerAddr, _ := ma.NewMultiaddr("/ip4/127.0.0.1/tcp/4001/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt")
    
    fmt.Printf("TCP: %s\n", tcpAddr)
    fmt.Printf("UDP: %s\n", udpAddr)
    fmt.Printf("WebSocket: %s\n", wsAddr)
    fmt.Printf("QUIC: %s\n", quicAddr)
    fmt.Printf("With Peer ID: %s\n", peerAddr)
    
    // Extract components
    protocols := peerAddr.Protocols()
    for _, p := range protocols {
        fmt.Printf("Protocol: %s\n", p.Name)
    }
}
```

## **Transport Layer: Connection Management**

libp2p supports multiple transport protocols simultaneously, allowing peers to communicate over the best available medium.

### **Transport Comparison**

| Transport | Use Case | Pros | Cons |
|-----------|----------|------|------|
| TCP | Reliable data | Ordered, reliable | Higher latency |
| UDP | Low latency | Fast, lightweight | Unreliable |
| WebSockets | Browser compatibility | NAT traversal | HTTP overhead |
| QUIC | Modern alternative | Built-in security, multiplexing | Newer protocol |

```go
// Go: Setting up multiple transports
package main

import (
    "context"
    "fmt"
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/p2p/transport/tcp"
    "github.com/libp2p/go-libp2p/p2p/transport/websocket"
)

func setupTransports() {
    // Create host with multiple transports
    host, err := libp2p.New(
        // Enable TCP transport
        libp2p.Transport(tcp.NewTCPTransport()),
        // Enable WebSocket transport
        libp2p.Transport(websocket.New()),
        // Listen on multiple addresses
        libp2p.ListenAddrStrings(
            "/ip4/0.0.0.0/tcp/4001",
            "/ip4/0.0.0.0/tcp/8080/ws",
        ),
    )
    
    if err != nil {
        panic(err)
    }
    
    fmt.Printf("Host ID: %s\n", host.ID())
    fmt.Printf("Addresses: %v\n", host.Addrs())
}
```

```rust
// Rust: Multi-transport setup
use libp2p::{
    tcp::TcpConfig,
    websocket::WsConfig,
    Transport,
    Swarm,
};

fn setup_transports() -> Result<(), Box<dyn std::error::Error>> {
    // Create TCP transport
    let tcp_transport = TcpConfig::new();
    
    // Create WebSocket transport
    let ws_transport = WsConfig::new(tcp_transport.clone());
    
    // Combine transports
    let transport = tcp_transport
        .or_transport(ws_transport)
        .boxed();
    
    Ok(())
}
```

## **Connection Security: Encryption and Authentication**

libp2p provides pluggable security protocols for encrypting connections and authenticating peers.

### **Security Protocol Comparison**

| Protocol | Key Exchange | Performance | Adoption |
|----------|--------------|-------------|----------|
| TLS 1.3 | ECDH/DHE | High | Industry standard |
| Noise | Various patterns | Very high | Growing |
| SecIO | ECDH | Medium | Legacy |

```go
// Go: Configuring security protocols
package main

import (
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/p2p/security/noise"
    "github.com/libp2p/go-libp2p/p2p/security/tls"
)

func setupSecurity() {
    host, err := libp2p.New(
        // Prefer Noise, fallback to TLS
        libp2p.Security(noise.ID, noise.New),
        libp2p.Security(tls.ID, tls.New),
    )
    
    if err != nil {
        panic(err)
    }
    
    defer host.Close()
}
```

## **Stream Multiplexing: Efficient Connection Usage**

Stream multiplexing allows multiple logical streams over a single physical connection, improving efficiency and reducing connection overhead.

### **Multiplexer Comparison**

| Multiplexer | Flow Control | Reliability | Performance |
|-------------|--------------|-------------|-------------|
| Yamux | Yes | High | Good |
| Mplex | Basic | Medium | Better |
| QUIC | Advanced | High | Best |

```go
// Go: Stream multiplexing example
package main

import (
    "bufio"
    "context"
    "fmt"
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/network"
    "github.com/libp2p/go-libp2p/core/protocol"
)

const protocolID = "/echo/1.0.0"

func setupStreamHandler() {
    host, err := libp2p.New()
    if err != nil {
        panic(err)
    }
    
    // Set stream handler
    host.SetStreamHandler(protocol.ID(protocolID), handleStream)
    
    fmt.Printf("Host ready. Connect to: %s\n", host.Addrs()[0])
}

func handleStream(stream network.Stream) {
    defer stream.Close()
    
    // Read from stream
    reader := bufio.NewReader(stream)
    writer := bufio.NewWriter(stream)
    
    for {
        line, err := reader.ReadString('\n')
        if err != nil {
            break
        }
        
        // Echo back
        writer.WriteString(fmt.Sprintf("Echo: %s", line))
        writer.Flush()
    }
}

func connectAndSend(targetAddr string) {
    host, _ := libp2p.New()
    defer host.Close()
    
    // Parse target address
    targetMultiaddr, _ := ma.NewMultiaddr(targetAddr)
    targetInfo, _ := peer.AddrInfoFromP2pAddr(targetMultiaddr)
    
    // Connect to peer
    ctx := context.Background()
    err := host.Connect(ctx, *targetInfo)
    if err != nil {
        panic(err)
    }
    
    // Open stream
    stream, err := host.NewStream(ctx, targetInfo.ID, protocol.ID(protocolID))
    if err != nil {
        panic(err)
    }
    defer stream.Close()
    
    // Send data
    writer := bufio.NewWriter(stream)
    writer.WriteString("Hello, libp2p!\n")
    writer.Flush()
    
    // Read response
    reader := bufio.NewReader(stream)
    response, _ := reader.ReadString('\n')
    fmt.Printf("Received: %s", response)
}
```

## **Peer Discovery: Finding Network Participants**

Peer discovery mechanisms help nodes find each other in the network without centralized coordination.

### **Discovery Methods**

**Bootstrap Nodes** - Well-known entry points
**mDNS** - Local network discovery
**DHT** - Distributed hash table
**Rendezvous** - Meeting point protocol
**Relay Discovery** - Through relay nodes

```go
// Go: DHT-based discovery
package main

import (
    "context"
    "fmt"
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/peer"
    dht "github.com/libp2p/go-libp2p-kad-dht"
    "github.com/libp2p/go-libp2p/p2p/discovery/routing"
)

func setupDHTDiscovery() {
    ctx := context.Background()
    
    // Create host
    host, err := libp2p.New()
    if err != nil {
        panic(err)
    }
    
    // Create DHT
    kademliaDHT, err := dht.New(ctx, host)
    if err != nil {
        panic(err)
    }
    
    // Bootstrap DHT
    if err = kademliaDHT.Bootstrap(ctx); err != nil {
        panic(err)
    }
    
    // Connect to bootstrap peers
    bootstrapPeers := []string{
        "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
        "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
    }
    
    for _, addr := range bootstrapPeers {
        peerAddr, _ := peer.AddrInfoFromString(addr)
        host.Connect(ctx, *peerAddr)
    }
    
    // Create discovery service
    discovery := routing.NewRoutingDiscovery(kademliaDHT)
    
    // Advertise service
    discovery.Advertise(ctx, "my-service")
    
    // Find peers providing service
    peerChan, err := discovery.FindPeers(ctx, "my-service")
    if err != nil {
        panic(err)
    }
    
    for peer := range peerChan {
        fmt.Printf("Found peer: %s\n", peer.ID)
    }
}
```

```rust
// Rust: mDNS discovery
use libp2p::{
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
struct MyBehaviour {
    mdns: Mdns,
}

#[derive(Debug)]
enum MyBehaviourEvent {
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for MyBehaviourEvent {
    fn from(event: MdnsEvent) -> Self {
        MyBehaviourEvent::Mdns(event)
    }
}

async fn setup_mdns_discovery() -> Result<(), Box<dyn std::error::Error>> {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = libp2p::PeerId::from(local_key.public());
    
    // Create mDNS behaviour
    let mdns = Mdns::new(MdnsConfig::default()).await?;
    let behaviour = MyBehaviour { mdns };
    
    // Create swarm
    let mut swarm = Swarm::new(
        libp2p::development_transport(local_key).await?,
        behaviour,
        local_peer_id,
    );
    
    // Handle events
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    println!("Discovered peer: {} at {}", peer_id, multiaddr);
                }
            },
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Expired(list))) => {
                for (peer_id, multiaddr) in list {
                    println!("Peer expired: {} at {}", peer_id, multiaddr);
                }
            },
            _ => {}
        }
    }
}
```

## **Content Routing: Distributed Hash Tables**

The **Kademlia DHT** provides content routing capabilities, allowing peers to store and retrieve data in a distributed manner.

### **DHT Operations**

**PUT** - Store key-value pair
**GET** - Retrieve value by key
**FIND_NODE** - Locate peers close to a key
**FIND_VALUE** - Locate value or closest peers

```go
// Go: DHT operations
package main

import (
    "context"
    "fmt"
    "github.com/libp2p/go-libp2p"
    dht "github.com/libp2p/go-libp2p-kad-dht"
)

func dhtOperations() {
    ctx := context.Background()
    
    // Create two hosts
    host1, _ := libp2p.New()
    host2, _ := libp2p.New()
    
    // Create DHTs
    dht1, _ := dht.New(ctx, host1)
    dht2, _ := dht.New(ctx, host2)
    
    // Connect hosts
    host1.Connect(ctx, peer.AddrInfo{
        ID:    host2.ID(),
        Addrs: host2.Addrs(),
    })
    
    // Bootstrap DHTs
    dht1.Bootstrap(ctx)
    dht2.Bootstrap(ctx)
    
    // Store value in DHT
    key := "/example/key"
    value := []byte("Hello, DHT!")
    
    err := dht1.PutValue(ctx, key, value)
    if err != nil {
        panic(err)
    }
    
    // Retrieve value from DHT
    retrievedValue, err := dht2.GetValue(ctx, key)
    if err != nil {
        panic(err)
    }
    
    fmt.Printf("Retrieved: %s\n", string(retrievedValue))
}
```

## **NAT Traversal and Relay**

Network Address Translation (NAT) creates challenges for peer-to-peer connectivity. libp2p provides several solutions.

### **NAT Traversal Methods**

**AutoRelay** - Automatic relay discovery and usage
**Hole Punching** - Direct connection through NAT
**Circuit Relay** - Proxy connections through relay nodes
**UPnP** - Universal Plug and Play port mapping

```go
// Go: AutoRelay configuration
package main

import (
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/p2p/host/autorelay"
)

func setupAutoRelay() {
    // Create host with AutoRelay
    host, err := libp2p.New(
        // Enable AutoRelay
        libp2p.EnableAutoRelay(),
        // Limit relay connections
        autorelay.WithMaxCandidates(3),
    )
    
    if err != nil {
        panic(err)
    }
    
    defer host.Close()
    
    fmt.Printf("Host with AutoRelay: %s\n", host.ID())
}
```

```rust
// Rust: Relay configuration
use libp2p::{
    relay::v2::relay::{Relay, RelayConfig},
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
struct Behaviour {
    relay: Relay,
}

fn setup_relay() -> Result<(), Box<dyn std::error::Error>> {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    
    // Create relay behaviour
    let relay = Relay::new(
        libp2p::PeerId::from(local_key.public()),
        RelayConfig::default(),
    );
    
    let behaviour = Behaviour { relay };
    
    Ok(())
}
```

## **Publish-Subscribe: Event Distribution**

**Gossipsub** provides efficient message distribution across the network using epidemic broadcast algorithms.

### **Gossipsub Features**

**Mesh Formation** - Optimized overlay topology
**Scoring** - Peer reputation system
**Flood Publishing** - Redundant message delivery
**Heartbeat** - Topology maintenance

```go
// Go: Gossipsub implementation
package main

import (
    "context"
    "fmt"
    "github.com/libp2p/go-libp2p"
    pubsub "github.com/libp2p/go-libp2p-pubsub"
)

func setupGossipsub() {
    ctx := context.Background()
    
    // Create host
    host, err := libp2p.New()
    if err != nil {
        panic(err)
    }
    
    // Create gossipsub
    ps, err := pubsub.NewGossipSub(ctx, host)
    if err != nil {
        panic(err)
    }
    
    // Join topic
    topic, err := ps.Join("my-topic")
    if err != nil {
        panic(err)
    }
    
    // Subscribe to topic
    sub, err := topic.Subscribe()
    if err != nil {
        panic(err)
    }
    
    // Publisher goroutine
    go func() {
        for i := 0; i < 10; i++ {
            msg := fmt.Sprintf("Message %d", i)
            topic.Publish(ctx, []byte(msg))
        }
    }()
    
    // Subscriber goroutine
    go func() {
        for {
            msg, err := sub.Next(ctx)
            if err != nil {
                break
            }
            fmt.Printf("Received: %s\n", string(msg.Data))
        }
    }()
}
```

```rust
// Rust: Gossipsub with custom configuration
use libp2p::{
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity},
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
struct MyBehaviour {
    gossipsub: Gossipsub,
}

#[derive(Debug)]
enum MyBehaviourEvent {
    Gossipsub(GossipsubEvent),
}

impl From<GossipsubEvent> for MyBehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
        MyBehaviourEvent::Gossipsub(event)
    }
}

fn setup_gossipsub() -> Result<MyBehaviour, Box<dyn std::error::Error>> {
    // Create custom gossipsub config
    let gossipsub_config = GossipsubConfig::builder()
        .heartbeat_interval(std::time::Duration::from_secs(1))
        .validation_mode(libp2p::gossipsub::ValidationMode::Strict)
        .build()
        .expect("Valid config");
    
    // Create gossipsub
    let gossipsub = Gossipsub::new(
        MessageAuthenticity::Signed(libp2p::identity::Keypair::generate_ed25519()),
        gossipsub_config,
    )?;
    
    Ok(MyBehaviour { gossipsub })
}
```

## **Advanced Protocols and Patterns**

### **Request-Response Pattern**

For direct peer-to-peer communication with guaranteed delivery and response.

```go
// Go: Request-Response protocol
package main

import (
    "context"
    "fmt"
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/network"
    "github.com/libp2p/go-libp2p/core/protocol"
    "encoding/json"
)

type Request struct {
    ID   string `json:"id"`
    Data string `json:"data"`
}

type Response struct {
    ID     string `json:"id"`
    Result string `json:"result"`
    Error  string `json:"error,omitempty"`
}

const requestProtocol = "/request-response/1.0.0"

func setupRequestResponse() {
    host, _ := libp2p.New()
    
    // Set handler for incoming requests
    host.SetStreamHandler(protocol.ID(requestProtocol), handleRequest)
    
    fmt.Printf("Request-Response server ready: %s\n", host.ID())
}

func handleRequest(stream network.Stream) {
    defer stream.Close()
    
    // Decode request
    var req Request
    decoder := json.NewDecoder(stream)
    if err := decoder.Decode(&req); err != nil {
        return
    }
    
    // Process request
    resp := Response{
        ID:     req.ID,
        Result: fmt.Sprintf("Processed: %s", req.Data),
    }
    
    // Send response
    encoder := json.NewEncoder(stream)
    encoder.Encode(resp)
}

func sendRequest(host libp2p.Host, targetPeer peer.ID, request Request) (*Response, error) {
    ctx := context.Background()
    
    // Open stream
    stream, err := host.NewStream(ctx, targetPeer, protocol.ID(requestProtocol))
    if err != nil {
        return nil, err
    }
    defer stream.Close()
    
    // Send request
    encoder := json.NewEncoder(stream)
    if err := encoder.Encode(request); err != nil {
        return nil, err
    }
    
    // Receive response
    var resp Response
    decoder := json.NewDecoder(stream)
    if err := decoder.Decode(&resp); err != nil {
        return nil, err
    }
    
    return &resp, nil
}
```

### **Custom Protocol Implementation**

Building domain-specific protocols on top of libp2p.

```go
// Go: Custom file sharing protocol
package main

import (
    "context"
    "crypto/sha256"
    "fmt"
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/network"
    "github.com/libp2p/go-libp2p/core/protocol"
    "io"
    "os"
)

const fileProtocol = "/fileshare/1.0.0"

type FileShareProtocol struct {
    host  libp2p.Host
    files map[string][]byte // In-memory file storage
}

func NewFileShareProtocol(host libp2p.Host) *FileShareProtocol {
    fsp := &FileShareProtocol{
        host:  host,
        files: make(map[string][]byte),
    }
    
    host.SetStreamHandler(protocol.ID(fileProtocol), fsp.handleStream)
    return fsp
}

func (fsp *FileShareProtocol) handleStream(stream network.Stream) {
    defer stream.Close()
    
    // Read file hash request
    hashBytes := make([]byte, 32)
    _, err := io.ReadFull(stream, hashBytes)
    if err != nil {
        return
    }
    
    hash := fmt.Sprintf("%x", hashBytes)
    
    // Check if file exists
    if fileData, exists := fsp.files[hash]; exists {
        // Send file size first
        size := uint64(len(fileData))
        binary.BigEndian.PutUint64(hashBytes[:8], size)
        stream.Write(hashBytes[:8])
        
        // Send file data
        stream.Write(fileData)
    } else {
        // Send zero size to indicate file not found
        stream.Write(make([]byte, 8))
    }
}

func (fsp *FileShareProtocol) StoreFile(filename string, data []byte) string {
    // Calculate hash
    hash := sha256.Sum256(data)
    hashStr := fmt.Sprintf("%x", hash)
    
    // Store file
    fsp.files[hashStr] = data
    
    fmt.Printf("Stored file %s with hash %s\n", filename, hashStr)
    return hashStr
}

func (fsp *FileShareProtocol) RequestFile(ctx context.Context, peerID peer.ID, hash string) ([]byte, error) {
    // Open stream
    stream, err := fsp.host.NewStream(ctx, peerID, protocol.ID(fileProtocol))
    if err != nil {
        return nil, err
    }
    defer stream.Close()
    
    // Send hash request
    hashBytes, _ := hex.DecodeString(hash)
    stream.Write(hashBytes)
    
    // Read file size
    sizeBytes := make([]byte, 8)
    _, err = io.ReadFull(stream, sizeBytes)
    if err != nil {
        return nil, err
    }
    
    size := binary.BigEndian.Uint64(sizeBytes)
    if size == 0 {
        return nil, fmt.Errorf("file not found")
    }
    
    // Read file data
    fileData := make([]byte, size)
    _, err = io.ReadFull(stream, fileData)
    if err != nil {
        return nil, err
    }
    
    return fileData, nil
}
```

## **Performance Optimization and Monitoring**

### **Connection Management**

```go
// Go: Advanced connection management
package main

import (
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/p2p/net/connmgr"
    "time"
)

func setupConnectionManager() {
    // Create connection manager with limits
    cmgr, err := connmgr.NewConnManager(
        10,  // Low water mark
        100, // High water mark
        connmgr.WithGracePeriod(time.Minute),
    )
    if err != nil {
        panic(err)
    }
    
    // Create host with connection manager
    host, err := libp2p.New(
        libp2p.ConnectionManager(cmgr),
        // Enable connection gater for filtering
        libp2p.ConnectionGater(newConnectionGater()),
    )
    
    if err != nil {
        panic(err)
    }
    
    defer host.Close()
}

type connectionGater struct{}

func newConnectionGater() *connectionGater {
    return &connectionGater{}
}

func (cg *connectionGater) InterceptPeerDial(p peer.ID) bool {
    // Allow all peer dials
    return true
}

func (cg *connectionGater) InterceptAddrDial(p peer.ID, addr ma.Multiaddr) bool {
    // Filter addresses based on criteria
    return !isPrivateAddress(addr)
}

func isPrivateAddress(addr ma.Multiaddr) bool {
    // Check if address is in private range
    // Implementation depends on specific requirements
    return false
}
```

### **Metrics and Monitoring**

```go
// Go: Metrics collection
package main

import (
    "fmt"
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/metrics"
    "time"
)

func setupMetrics() {
    // Create metrics reporter
    reporter := metrics.NewBandwidthCounter()
    
    // Create host with metrics
    host, err := libp2p.New(
        libp2p.BandwidthReporter(reporter),
    )
    if err != nil {
        panic(err)
    }
    
    // Monitor metrics
    go func() {
        ticker := time.NewTicker(10 * time.Second)
        defer ticker.Stop()
        
        for range ticker.C {
            stats := reporter.GetBandwidthTotals()
            fmt.Printf("Bandwidth - In: %d bytes, Out: %d bytes\n", 
                stats.TotalIn, stats.TotalOut)
        }
    }()
    
    defer host.Close()
}
```


## **Security Best Practices**

### **Authentication and Authorization**

```go
// Go: Implementing peer authentication
package main

import (
    "context"
    "crypto/rand"
    "fmt"
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/crypto"
    "github.com/libp2p/go-libp2p/core/network"
    "github.com/libp2p/go-libp2p/core/peer"
    "github.com/libp2p/go-libp2p/core/protocol"
    "time"
)

type AuthenticatedProtocol struct {
    host         libp2p.Host
    trustedPeers map[peer.ID]bool
    challenges   map[peer.ID][]byte
}

const authProtocol = "/auth/1.0.0"

func NewAuthenticatedProtocol(host libp2p.Host) *AuthenticatedProtocol {
    ap := &AuthenticatedProtocol{
        host:         host,
        trustedPeers: make(map[peer.ID]bool),
        challenges:   make(map[peer.ID][]byte),
    }
    
    host.SetStreamHandler(protocol.ID(authProtocol), ap.handleAuth)
    return ap
}

func (ap *AuthenticatedProtocol) AddTrustedPeer(peerID peer.ID) {
    ap.trustedPeers[peerID] = true
}

func (ap *AuthenticatedProtocol) handleAuth(stream network.Stream) {
    defer stream.Close()
    
    remotePeer := stream.Conn().RemotePeer()
    
    // Check if peer is in trusted list
    if !ap.trustedPeers[remotePeer] {
        fmt.Printf("Rejecting untrusted peer: %s\n", remotePeer)
        return
    }
    
    // Generate challenge
    challenge := make([]byte, 32)
    rand.Read(challenge)
    ap.challenges[remotePeer] = challenge
    
    // Send challenge
    stream.Write(challenge)
    
    // Read signature
    signature := make([]byte, 64) // Ed25519 signature size
    _, err := stream.Read(signature)
    if err != nil {
        fmt.Printf("Failed to read signature: %v\n", err)
        return
    }
    
    // Verify signature
    if ap.verifyChallenge(remotePeer, challenge, signature) {
        stream.Write([]byte("AUTH_SUCCESS"))
        fmt.Printf("Successfully authenticated peer: %s\n", remotePeer)
    } else {
        stream.Write([]byte("AUTH_FAILED"))
        fmt.Printf("Authentication failed for peer: %s\n", remotePeer)
    }
    
    delete(ap.challenges, remotePeer)
}

func (ap *AuthenticatedProtocol) verifyChallenge(peerID peer.ID, challenge, signature []byte) bool {
    // Get peer's public key from peer store
    pubKey := ap.host.Peerstore().PubKey(peerID)
    if pubKey == nil {
        return false
    }
    
    // Verify signature
    valid, err := pubKey.Verify(challenge, signature)
    return err == nil && valid
}
```

### **Encrypted Communication Channels**

```rust
// Rust: Advanced encryption with custom protocols
use libp2p::{
    core::upgrade,
    noise::{Keypair, NoiseConfig, X25519Spec},
    tcp::TcpConfig,
    Transport,
};
use std::error::Error;

fn setup_secure_transport() -> Result<(), Box<dyn Error>> {
    // Generate noise keypair
    let noise_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&libp2p::identity::Keypair::generate_ed25519())?;
    
    // Create secure transport
    let transport = TcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(libp2p::yamux::YamuxConfig::default())
        .boxed();
    
    Ok(())
}

// Custom encryption for application data
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce
};

struct SecureChannel {
    cipher: ChaCha20Poly1305,
    nonce_counter: u64,
}

impl SecureChannel {
    fn new(key: &[u8; 32]) -> Self {
        Self {
            cipher: ChaCha20Poly1305::new(key.into()),
            nonce_counter: 0,
        }
    }
    
    fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let nonce = self.generate_nonce();
        let ciphertext = self.cipher.encrypt(&nonce, plaintext)?;
        Ok(ciphertext)
    }
    
    fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let nonce = self.generate_nonce();
        let plaintext = self.cipher.decrypt(&nonce, ciphertext)?;
        Ok(plaintext)
    }
    
    fn generate_nonce(&mut self) -> Nonce {
        let mut nonce = [0u8; 12];
        nonce[4..12].copy_from_slice(&self.nonce_counter.to_le_bytes());
        self.nonce_counter += 1;
        Nonce::from(nonce)
    }
}
```

### **Access Control and Permissions**

```go
// Go: Role-based access control
package main

import (
    "context"
    "encoding/json"
    "fmt"
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/network"
    "github.com/libp2p/go-libp2p/core/peer"
    "github.com/libp2p/go-libp2p/core/protocol"
    "time"
)

type Role string

const (
    RoleAdmin     Role = "admin"
    RoleModerator Role = "moderator"
    RoleUser      Role = "user"
    RoleGuest     Role = "guest"
)

type Permission string

const (
    PermissionRead   Permission = "read"
    PermissionWrite  Permission = "write"
    PermissionDelete Permission = "delete"
    PermissionAdmin  Permission = "admin"
)

type AccessControlList struct {
    peerRoles   map[peer.ID]Role
    rolePerms   map[Role][]Permission
    host        libp2p.Host
}

func NewAccessControlList(host libp2p.Host) *AccessControlList {
    acl := &AccessControlList{
        peerRoles: make(map[peer.ID]Role),
        rolePerms: map[Role][]Permission{
            RoleAdmin:     {PermissionRead, PermissionWrite, PermissionDelete, PermissionAdmin},
            RoleModerator: {PermissionRead, PermissionWrite, PermissionDelete},
            RoleUser:      {PermissionRead, PermissionWrite},
            RoleGuest:     {PermissionRead},
        },
        host: host,
    }
    
    return acl
}

func (acl *AccessControlList) AssignRole(peerID peer.ID, role Role) {
    acl.peerRoles[peerID] = role
    fmt.Printf("Assigned role %s to peer %s\n", role, peerID)
}

func (acl *AccessControlList) HasPermission(peerID peer.ID, perm Permission) bool {
    role, exists := acl.peerRoles[peerID]
    if !exists {
        role = RoleGuest // Default role
    }
    
    permissions := acl.rolePerms[role]
    for _, p := range permissions {
        if p == perm {
            return true
        }
    }
    return false
}

func (acl *AccessControlList) CreateSecureHandler(perm Permission, handler network.StreamHandler) network.StreamHandler {
    return func(stream network.Stream) {
        defer stream.Close()
        
        remotePeer := stream.Conn().RemotePeer()
        
        if !acl.HasPermission(remotePeer, perm) {
            fmt.Printf("Access denied for peer %s (required: %s)\n", remotePeer, perm)
            stream.Write([]byte("ACCESS_DENIED"))
            return
        }
        
        // Call original handler
        handler(stream)
    }
}

// Example usage with secure protocol
const secureDataProtocol = "/secure-data/1.0.0"

func setupSecureDataProtocol() {
    host, _ := libp2p.New()
    acl := NewAccessControlList(host)
    
    // Assign roles
    // acl.AssignRole(somePeerID, RoleAdmin)
    
    // Create secure handler that requires write permission
    secureHandler := acl.CreateSecureHandler(PermissionWrite, handleSecureData)
    host.SetStreamHandler(protocol.ID(secureDataProtocol), secureHandler)
}

func handleSecureData(stream network.Stream) {
    // Handle sensitive data operations
    fmt.Printf("Processing secure data request from %s\n", stream.Conn().RemotePeer())
    stream.Write([]byte("SECURE_DATA_RESPONSE"))
}
```

## **Advanced Networking Patterns**

### **Circuit Breaker Pattern**

```go
// Go: Circuit breaker for peer connections
package main

import (
    "context"
    "fmt"
    "sync"
    "time"
    "github.com/libp2p/go-libp2p/core/peer"
)

type CircuitState int

const (
    StateClosed CircuitState = iota
    StateOpen
    StateHalfOpen
)

type CircuitBreaker struct {
    mu                sync.RWMutex
    state            CircuitState
    failures         int
    maxFailures      int
    timeout          time.Duration
    nextAttempt      time.Time
    onStateChange    func(from, to CircuitState)
}

type PeerCircuitBreaker struct {
    breakers map[peer.ID]*CircuitBreaker
    mu       sync.RWMutex
}

func NewPeerCircuitBreaker() *PeerCircuitBreaker {
    return &PeerCircuitBreaker{
        breakers: make(map[peer.ID]*CircuitBreaker),
    }
}

func (pcb *PeerCircuitBreaker) GetBreaker(peerID peer.ID) *CircuitBreaker {
    pcb.mu.Lock()
    defer pcb.mu.Unlock()
    
    if breaker, exists := pcb.breakers[peerID]; exists {
        return breaker
    }
    
    breaker := &CircuitBreaker{
        state:       StateClosed,
        maxFailures: 5,
        timeout:     30 * time.Second,
        onStateChange: func(from, to CircuitState) {
            fmt.Printf("Circuit breaker for peer %s changed from %d to %d\n", 
                peerID, from, to)
        },
    }
    
    pcb.breakers[peerID] = breaker
    return breaker
}

func (cb *CircuitBreaker) Call(fn func() error) error {
    cb.mu.Lock()
    defer cb.mu.Unlock()
    
    // Check if circuit is open
    if cb.state == StateOpen {
        if time.Now().Before(cb.nextAttempt) {
            return fmt.Errorf("circuit breaker is open")
        }
        
        // Transition to half-open
        cb.setState(StateHalfOpen)
    }
    
    // Execute function
    err := fn()
    
    if err != nil {
        cb.onFailure()
        return err
    }
    
    cb.onSuccess()
    return nil
}

func (cb *CircuitBreaker) onSuccess() {
    if cb.state == StateHalfOpen {
        cb.setState(StateClosed)
    }
    cb.failures = 0
}

func (cb *CircuitBreaker) onFailure() {
    cb.failures++
    
    if cb.failures >= cb.maxFailures {
        cb.setState(StateOpen)
        cb.nextAttempt = time.Now().Add(cb.timeout)
    }
}

func (cb *CircuitBreaker) setState(state CircuitState) {
    if cb.state != state && cb.onStateChange != nil {
        cb.onStateChange(cb.state, state)
    }
    cb.state = state
}

// Usage example
func connectWithCircuitBreaker(host libp2p.Host, targetPeer peer.ID, breakers *PeerCircuitBreaker) error {
    breaker := breakers.GetBreaker(targetPeer)
    
    return breaker.Call(func() error {
        ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
        defer cancel()
        
        return host.Connect(ctx, peer.AddrInfo{ID: targetPeer})
    })
}
```

### **Load Balancing and Peer Selection**

```go
// Go: Intelligent peer selection
package main

import (
    "math/rand"
    "sort"
    "sync"
    "time"
    "github.com/libp2p/go-libp2p/core/peer"
)

type PeerMetrics struct {
    Latency       time.Duration
    Bandwidth     float64
    Reliability   float64
    LastSeen      time.Time
    FailureCount  int
    SuccessCount  int
}

type PeerSelector struct {
    mu      sync.RWMutex
    peers   map[peer.ID]*PeerMetrics
    weights map[peer.ID]float64
}

func NewPeerSelector() *PeerSelector {
    return &PeerSelector{
        peers:   make(map[peer.ID]*PeerMetrics),
        weights: make(map[peer.ID]float64),
    }
}

func (ps *PeerSelector) UpdateMetrics(peerID peer.ID, latency time.Duration, success bool) {
    ps.mu.Lock()
    defer ps.mu.Unlock()
    
    metrics, exists := ps.peers[peerID]
    if !exists {
        metrics = &PeerMetrics{
            Reliability: 1.0,
            LastSeen:    time.Now(),
        }
        ps.peers[peerID] = metrics
    }
    
    // Update latency with exponential moving average
    if metrics.Latency == 0 {
        metrics.Latency = latency
    } else {
        alpha := 0.8
        metrics.Latency = time.Duration(float64(metrics.Latency)*alpha + 
            float64(latency)*(1-alpha))
    }
    
    // Update reliability
    if success {
        metrics.SuccessCount++
    } else {
        metrics.FailureCount++
    }
    
    total := metrics.SuccessCount + metrics.FailureCount
    metrics.Reliability = float64(metrics.SuccessCount) / float64(total)
    metrics.LastSeen = time.Now()
    
    // Recalculate weight
    ps.calculateWeight(peerID, metrics)
}

func (ps *PeerSelector) calculateWeight(peerID peer.ID, metrics *PeerMetrics) {
    // Weight based on latency (lower is better), reliability (higher is better)
    latencyScore := 1.0 / (1.0 + float64(metrics.Latency.Milliseconds())/1000.0)
    reliabilityScore := metrics.Reliability
    freshnessScore := 1.0 / (1.0 + time.Since(metrics.LastSeen).Hours())
    
    weight := latencyScore * reliabilityScore * freshnessScore
    ps.weights[peerID] = weight
}

func (ps *PeerSelector) SelectBestPeer() peer.ID {
    ps.mu.RLock()
    defer ps.mu.RUnlock()
    
    if len(ps.weights) == 0 {
        return ""
    }
    
    // Find peer with highest weight
    var bestPeer peer.ID
    var bestWeight float64
    
    for peerID, weight := range ps.weights {
        if weight > bestWeight {
            bestWeight = weight
            bestPeer = peerID
        }
    }
    
    return bestPeer
}

func (ps *PeerSelector) SelectWeightedRandom() peer.ID {
    ps.mu.RLock()
    defer ps.mu.RUnlock()
    
    if len(ps.weights) == 0 {
        return ""
    }
    
    // Calculate total weight
    totalWeight := 0.0
    for _, weight := range ps.weights {
        totalWeight += weight
    }
    
    // Select random point
    target := rand.Float64() * totalWeight
    current := 0.0
    
    for peerID, weight := range ps.weights {
        current += weight
        if current >= target {
            return peerID
        }
    }
    
    // Fallback to first peer
    for peerID := range ps.weights {
        return peerID
    }
    
    return ""
}

func (ps *PeerSelector) SelectTopN(n int) []peer.ID {
    ps.mu.RLock()
    defer ps.mu.RUnlock()
    
    type peerWeight struct {
        ID     peer.ID
        Weight float64
    }
    
    var peers []peerWeight
    for peerID, weight := range ps.weights {
        peers = append(peers, peerWeight{ID: peerID, Weight: weight})
    }
    
    // Sort by weight (descending)
    sort.Slice(peers, func(i, j int) bool {
        return peers[i].Weight > peers[j].Weight
    })
    
    // Return top N
    result := make([]peer.ID, 0, n)
    for i := 0; i < len(peers) && i < n; i++ {
        result = append(result, peers[i].ID)
    }
    
    return result
}
```

## **Protocol Design Patterns**

### **Event-Driven Architecture**

```rust
// Rust: Event-driven libp2p application
use libp2p::{
    NetworkBehaviour,
    swarm::{Swarm, SwarmEvent},
};
use tokio::sync::mpsc;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AppEvent {
    PeerConnected(libp2p::PeerId),
    PeerDisconnected(libp2p::PeerId),
    MessageReceived {
        from: libp2p::PeerId,
        topic: String,
        data: Vec<u8>,
    },
    NetworkError(String),
}

pub struct EventDrivenApp {
    swarm: Swarm<MyBehaviour>,
    event_sender: mpsc::UnboundedSender<AppEvent>,
    event_receiver: mpsc::UnboundedReceiver<AppEvent>,
    handlers: HashMap<String, Box<dyn EventHandler>>,
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &AppEvent) -> Result<(), Box<dyn std::error::Error>>;
}

impl EventDrivenApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = libp2p::PeerId::from(local_key.public());
        
        let transport = libp2p::development_transport(local_key).await?;
        let behaviour = MyBehaviour::new();
        let swarm = Swarm::new(transport, behaviour, local_peer_id);
        
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        
        Ok(Self {
            swarm,
            event_sender,
            event_receiver,
            handlers: HashMap::new(),
        })
    }
    
    pub fn register_handler<T: EventHandler + 'static>(&mut self, name: String, handler: T) {
        self.handlers.insert(name, Box::new(handler));
    }
    
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            tokio::select! {
                // Handle swarm events
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await?;
                }
                
                // Handle application events
                Some(app_event) = self.event_receiver.recv() => {
                    self.dispatch_event(&app_event).await?;
                }
            }
        }
    }
    
    async fn handle_swarm_event(&mut self, event: SwarmEvent<MyBehaviourEvent>) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                let _ = self.event_sender.send(AppEvent::PeerConnected(peer_id));
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                let _ = self.event_sender.send(AppEvent::PeerDisconnected(peer_id));
            }
            SwarmEvent::Behaviour(behaviour_event) => {
                // Handle custom behaviour events
                self.handle_behaviour_event(behaviour_event).await?;
            }
            _ => {}
        }
        Ok(())
    }
    
    async fn dispatch_event(&self, event: &AppEvent) -> Result<(), Box<dyn std::error::Error>> {
        for (name, handler) in &self.handlers {
            if let Err(e) = handler.handle(event) {
                eprintln!("Handler {} failed: {}", name, e);
            }
        }
        Ok(())
    }
}

// Example event handlers
struct ConnectionHandler;

impl EventHandler for ConnectionHandler {
    fn handle(&self, event: &AppEvent) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            AppEvent::PeerConnected(peer_id) => {
                println!("New peer connected: {}", peer_id);
            }
            AppEvent::PeerDisconnected(peer_id) => {
                println!("Peer disconnected: {}", peer_id);
            }
            _ => {}
        }
        Ok(())
    }
}

struct MessageHandler;

impl EventHandler for MessageHandler {
    fn handle(&self, event: &AppEvent) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            AppEvent::MessageReceived { from, topic, data } => {
                println!("Message from {}: {} bytes on topic {}", 
                    from, data.len(), topic);
                
                // Process message based on topic
                match topic.as_str() {
                    "chat" => self.handle_chat_message(from, data)?,
                    "file" => self.handle_file_message(from, data)?,
                    _ => println!("Unknown topic: {}", topic),
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl MessageHandler {
    fn handle_chat_message(&self, from: &libp2p::PeerId, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let message = String::from_utf8_lossy(data);
        println!("Chat from {}: {}", from, message);
        Ok(())
    }
    
    fn handle_file_message(&self, from: &libp2p::PeerId, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        println!("File transfer from {}: {} bytes", from, data.len());
        Ok(())
    }
}
```

### **State Machine Pattern**

```go
// Go: Protocol state machine
package main

import (
    "fmt"
    "sync"
    "time"
    "github.com/libp2p/go-libp2p/core/peer"
)

type State int

const (
    StateIdle State = iota
    StateConnecting
    StateHandshaking
    StateConnected
    StateDisconnecting
    StateError
)

type Event int

const (
    EventConnect Event = iota
    EventConnected
    EventHandshakeComplete
    EventDisconnect
    EventError
    EventTimeout
)

type StateMachine struct {
    mu           sync.RWMutex
    currentState State
    peerID       peer.ID
    transitions  map[State]map[Event]State
    handlers     map[State]func()
    timeouts     map[State]time.Duration
    timer        *time.Timer
}

func NewStateMachine(peerID peer.ID) *StateMachine {
    sm := &StateMachine{
        currentState: StateIdle,
        peerID:       peerID,
        transitions:  make(map[State]map[Event]State),
        handlers:     make(map[State]func()),
        timeouts:     make(map[State]time.Duration),
    }
    
    // Define state transitions
    sm.defineTransitions()
    sm.defineHandlers()
    sm.defineTimeouts()
    
    return sm
}

func (sm *StateMachine) defineTransitions() {
    sm.transitions[StateIdle] = map[Event]State{
        EventConnect: StateConnecting,
    }
    
    sm.transitions[StateConnecting] = map[Event]State{
        EventConnected: StateHandshaking,
        EventError:     StateError,
        EventTimeout:   StateError,
    }
    
    sm.transitions[StateHandshaking] = map[Event]State{
        EventHandshakeComplete: StateConnected,
        EventError:            StateError,
        EventTimeout:          StateError,
    }
    
    sm.transitions[StateConnected] = map[Event]State{
        EventDisconnect: StateDisconnecting,
        EventError:      StateError,
    }
    
    sm.transitions[StateDisconnecting] = map[Event]State{
        EventConnect: StateIdle,
        EventError:   StateIdle,
        EventTimeout: StateIdle,
    }
    
    sm.transitions[StateError] = map[Event]State{
        EventConnect: StateConnecting,
    }
}

func (sm *StateMachine) defineHandlers() {
    sm.handlers[StateConnecting] = sm.handleConnecting
    sm.handlers[StateHandshaking] = sm.handleHandshaking
    sm.handlers[StateConnected] = sm.handleConnected
    sm.handlers[StateDisconnecting] = sm.handleDisconnecting
    sm.handlers[StateError] = sm.handleError
}

func (sm *StateMachine) defineTimeouts() {
    sm.timeouts[StateConnecting] = 30 * time.Second
    sm.timeouts[StateHandshaking] = 10 * time.Second
    sm.timeouts[StateDisconnecting] = 5 * time.Second
}

func (sm *StateMachine) SendEvent(event Event) error {
    sm.mu.Lock()
    defer sm.mu.Unlock()
    
    // Check if transition is valid
    if transitions, exists := sm.transitions[sm.currentState]; exists {
        if nextState, valid := transitions[event]; valid {
            fmt.Printf("Peer %s: %s -> %s (event: %s)\n", 
                sm.peerID, sm.stateString(sm.currentState), 
                sm.stateString(nextState), sm.eventString(event))
            
            sm.currentState = nextState
            
            // Cancel existing timer
            if sm.timer != nil {
                sm.timer.Stop()
            }
            
            // Execute state handler
            if handler, exists := sm.handlers[sm.currentState]; exists {
                go handler()
            }
            
            // Set timeout if defined
            if timeout, exists := sm.timeouts[sm.currentState]; exists {
                sm.timer = time.AfterFunc(timeout, func() {
                    sm.SendEvent(EventTimeout)
                })
            }
            
            return nil
        }
    }
    
    return fmt.Errorf("invalid transition from %s with event %s", 
        sm.stateString(sm.currentState), sm.eventString(event))
}

func (sm *StateMachine) GetState() State {
    sm.mu.RLock()
    defer sm.mu.RUnlock()
    return sm.currentState
}

func (sm *StateMachine) handleConnecting() {
    fmt.Printf("Peer %s: Initiating connection...\n", sm.peerID)
    // Simulate connection logic
    time.Sleep(2 * time.Second)
    sm.SendEvent(EventConnected)
}

func (sm *StateMachine) handleHandshaking() {
    fmt.Printf("Peer %s: Performing handshake...\n", sm.peerID)
    // Simulate handshake logic
    time.Sleep(1 * time.Second)
    sm.SendEvent(EventHandshakeComplete)
}

func (sm *StateMachine) handleConnected() {
    fmt.Printf("Peer %s: Connection established\n", sm.peerID)
}

func (sm *StateMachine) handleDisconnecting() {
    fmt.Printf("Peer %s: Disconnecting...\n", sm.peerID)
    // Simulate disconnection logic
    time.Sleep(1 * time.Second)
    sm.SendEvent(EventConnect) // Return to idle
}

func (sm *StateMachine) handleError() {
    fmt.Printf("Peer %s: Error state, waiting for retry\n", sm.peerID)
}

func (sm *StateMachine) stateString(state State) string {
    states := []string{"Idle", "Connecting", "Handshaking", "Connected", "Disconnecting", "Error"}
    if int(state) < len(states) {
        return states[state]
    }
    return "Unknown"
}

func (sm *StateMachine) eventString(event Event) string {
    events := []string{"Connect", "Connected", "HandshakeComplete", "Disconnect", "Error", "Timeout"}
    if int(event) < len(events) {
        return events[event]
    }
    return "Unknown"
}
```

## **Testing and Debugging**

### **Network Simulation and Testing**

```go
// Go: libp2p network testing framework
package main

import (
    "context"
    "fmt"
    "math/rand"
    "sync"
    "testing"
    "time"
    
    "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/host"
    "github.com/libp2p/go-libp2p/core/peer"
    "github.com/libp2p/go-libp2p/core/peerstore"
)

type NetworkSimulator struct {
    hosts       []host.Host
    connections map[string]bool
    latencies   map[string]time.Duration
    packetLoss  map[string]float64
    mu          sync.RWMutex
}

func NewNetworkSimulator(nodeCount int) *NetworkSimulator {
    sim := &NetworkSimulator{
        hosts:       make([]host.Host, 0, nodeCount),
        connections: make(map[string]bool),
        latencies:   make(map[string]time.Duration),
        packetLoss:  make(map[string]float64),
    }
    
    // Create