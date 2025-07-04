Common transport interfaces 
------------------------------------------------------------------------------------------------------------------------

Transports are defined in terms of two core operations, **listening** and **dialing**.

Listening means that you can accept incoming connections from other peers, using whatever facility is provided by the transport implementation. For example, a TCP transport on a unix platform could use the `bind` and `listen` system calls to have the operating system route traffic on a given TCP port to the application.

Dialing is the process of opening an outgoing connection to a listening peer. Like listening, the specifics are determined by the implementation, but every transport in a libp2p implementation will share the same programmatic interface.