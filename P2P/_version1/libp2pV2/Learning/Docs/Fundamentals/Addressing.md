A `multiaddress` (often abbreviated `multiaddr`), is a convention for encoding multiple layers of addressing information into a single "future-proof" path structure. It [defines](https://github.com/libp2p/specs/blob/master/addressing/README.md) human-readable and machine-optimized encodings of common transport and overlay protocols and allows many layers of addressing to be combined and used together


For example: `/ip4/192.0.2.0/udp/1234` encodes two protocols along with their essential addressing information. The `/ip4/192.0.2.0` informs us that we want the `192.0.2.0` loopback address of the IPv4 protocol, and `/udp/1234` tells us we want to send UDP packets to port `1234`.


Now I can start [handing out multiaddrs to all my friends](https://docs.libp2p.io/concepts/appendix/glossary/#peer-routing), of the form `/ip4/198.51.100.0/tcp/4242/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N`. Combining my "location multiaddr" (my IP and port) with my "identity multiaddr" (my libp2p `PeerId`), produces a new multiaddr containing both key pieces of information.

Now not only do my friends know where to find me, anyone they give that address to can verify that the machine on the other side is really me, or at least, that they control the private key for my `PeerId`. They also know (by virtue of the `/p2p/` protocol id) that I'm likely to support common libp2p interactions like opening connections and negotiating what application protocols we can use to communicate.