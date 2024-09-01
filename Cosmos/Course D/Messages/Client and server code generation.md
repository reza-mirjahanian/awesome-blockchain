The Cosmos SDK uses Protobuf definitions to generate client and server code:

-   The `MsgServer` interface defines the server API for the `Msg` service. Its implementation is described in the [`Msg` services documentation (opens new window)](https://docs.cosmos.network/v0.45/building-modules/msg-services.html).
-   Structures are generated for all RPC requests and response types.



If you want to dive deeper when it comes to messages, the `Msg` service, and modules, see:

-   The Cosmos SDK documentation on [`Msg` service (opens new window)](https://docs.cosmos.network/v0.45/building-modules/msg-services.html).
-   The Cosmos SDK documentation on messages and queries, addressing how to define messages using `Msg` services - [Amino `LegacyMsg` (opens new window)](https://docs.cosmos.network/v0.45/building-modules/messages-and-queries.html#legacy-amino-legacymsgs).