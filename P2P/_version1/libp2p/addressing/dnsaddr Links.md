#### dnsaddr Links

A libp2p-specific DNS-backed format, `/dnsaddr` resolves addresses from a `TXT`
record associated with the `_dnsaddr` subdomain of a given domain.

Note that this is different from [dnslink](https://dnslink.io/), which uses
`TXT` records to reference content addressed objects.

For example, resolving `/dnsaddr/libp2p.io` will perform a `TXT` lookup for
`_dnsaddr.libp2p.io`. If the result contains entries of the form
`dnsaddr=<multiaddr>`, the embedded multiaddrs will be parsed and used.

For example, asking the DNS server for the TXT records of one of the bootstrap
nodes, `ams-2.bootstrap.libp2p.io`, returns the following records:

```
> dig +short _dnsaddr.ams-2.bootstrap.libp2p.io txt
"dnsaddr=/dns4/ams-2.bootstrap.libp2p.io/tcp/443/wss/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
"dnsaddr=/ip6/2604:1380:2000:7a00::1/tcp/4001/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
"dnsaddr=/ip4/147.75.83.83/tcp/4001/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
"dnsaddr=/ip6/2604:1380:2000:7a00::1/udp/4001/quic/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
"dnsaddr=/ip4/147.75.83.83/udp/4001/quic/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
"dnsaddr=/dns6/ams-2.bootstrap.libp2p.io/tcp/443/wss/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
```

The `dnsaddr` lookup serves a similar purpose to a standard A-record DNS lookup,
however there are differences that can be important for some use cases. The most
significant is that the `dnsaddr` entry contains a full multiaddr, which may
include a port number or other information that an A-record lacks, and it may
even specify a non-IP transport. Also, there are cases in which the A-record
already serves a useful purpose; using `dnsaddr` allows a second "namespace" for
libp2p registrations.