A priori, a node cannot know if it is behind a NAT / firewall or if it is publicly reachable. Knowing its NAT status is essential for the node to be well-behaved in the network: A node that's behind a NAT doesn't need to advertise its (undiable) addresses to the rest of the network, preventing superfluous dials from other peers. Furthermore, it might actively seek to improve its connectivity by finding a relay server, which would allow other peers to establish a relayed connection.

To determine if it is located behind a NAT, nodes use the `autonat` protocol. Using this protocol, the node requests other peers to dial its presumed public addresses. If a couple of these dial attempts succeed, the node can be reasonably certain that it is not located behind a NAT. Likewise, if a couple of these dial attempts fail, this is a strong indicator that a NAT is blocking incoming connections.