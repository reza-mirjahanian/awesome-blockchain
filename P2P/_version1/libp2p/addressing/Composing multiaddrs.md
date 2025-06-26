As shown above, protocol addresses can be composed within a multiaddr in a way that mirrors the composition of protocols within a networking stack.

The terms generally used to describe composition of multiaddrs are "encapsulation" and "decapsulation", and they essentially refer to adding and removing protocol components from a multiaddr, respectively.

#### Encapsulation

A protocol is said to be "encapsulated within" another protocol when data from an "inner" protocol is wrapped by another "outer" protocol, often by re-framing the data from the inner protocol into the type of packets, frames or datagrams used by the outer protocol.

Some examples of protocol encapsulation are HTTP requests encapsulated within TCP/IP streams, or TCP segments themselves encapsulated within IP datagrams.

The multiaddr format was designed so that addresses encapsulate each other in the same manner as the protocols that they describe. The result is an address that begins with the "outermost" layer of the network stack and works progressively "inward". For example, in the address `/ip4/198.51.100/tcp/80/ws`, the outermost protocol is IPv4, which encapsulates TCP streams, which in turn encapsulate WebSockets.

All multiaddr implementations provide a way to *encapsulate* two multiaddrs into a composite. For example, `/ip4/198.51.100` can encapsulate `/tcp/42` to become `/ip4/198.51.100/tcp/42`.

#### Decapsulation

Decapsulation takes a composite multiaddr and removes an "inner" multiaddr from it, returning the result.

For example, if we start with `/ip4/198.51.100/tcp/1234/ws` and decapsulate `/ws`, the result is `/ip4/198.51.100/tcp/1234`.

It's important to note that decapsulation returns the original multiaddr up to the last occurrence of the decapsulated multiaddr. This may remove more than just the decapsulated component itself if there are more protocols encapsulated within it. Using our example above, decapsulating either `/tcp/1234/ws` *or* `/tcp/1234` from `/ip4/198.51.100/tcp/1234/ws` will result in `/ip4/198.51.100`. This is unsurprising if you consider the utility of the `/ip4/198.51.100/ws` address that would result from simply removing the `tcp` component.