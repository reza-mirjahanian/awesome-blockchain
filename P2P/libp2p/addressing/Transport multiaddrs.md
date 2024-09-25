### IP and Name Resolution

Most libp2p transports use the IP protocol as a foundational layer, and as a
result, most transport multiaddrs will begin with a component that represents an
IPv4 or IPv6 address.

This may be an actual address, such as `/ip4/198.51.100` or
`/ip6/fe80::883:a581:fff1:833`, or it could be something that resolves to an IP
address, like a domain name.

libp2p will attempt to resolve "name-based" addresses into IP addresses. The
current [multiaddr protocol table][multiaddr-proto-table] defines four
resolvable or "name-based" protocols:

| protocol  | description                                                        |
|-----------|--------------------------------------------------------------------|
| `dns`     | Resolves DNS A and AAAA records into both IPv4 and IPv6 addresses. |
| `dns4`    | Resolves DNS A records into IPv4 addresses.                        |
| `dns6`    | Resolves DNS AAAA records into IPv6 addresses.                     |
| `dnsaddr` | Resolves multiaddrs from a special TXT record.                     |

When the `/dns` protocol is used, the lookup may result in both IPv4 and IPv6
addresses, in which case IPv6 will be preferred. To explicitly resolve to IPv4
or IPv6 addresses, use the `/dns4` or `/dns6` protocols, respectively.

Note that in some restricted environments, such as inside a web browser, libp2p
may not have access to the resolved IP addresses at all, in which case the
runtime will determine what IP version is used.

When a name-based multiaddr encapsulates another multiaddr, only the name-based
component is affected by the lookup process. For example, if `example.com`
resolves to `192.0.2.0`, libp2p will resolve the address
`/dns4/example.com/tcp/42` to `/ip4/192.0.2.0/tcp/42`.