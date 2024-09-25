Protocol
--------

The ping protocol is a simple liveness check that peers can use to test the connectivity and performance between two peers. The libp2p ping protocol is different from the ping command line utility ([ICMP ping](https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol)), as it requires an already established libp2p connection.

The dialing peer sends a 32-byte payload of random binary data on an open stream. The listening peer echoes the same 32-byte payload back to the dialing peer. The dialing peer then measures the RTT from when it wrote the bytes to when it received them.