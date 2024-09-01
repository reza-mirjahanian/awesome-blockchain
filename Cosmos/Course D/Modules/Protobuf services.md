Each module defines two Protobuf services:

-   **`Msg`:** a set of RPC methods related one-to-one to Protobuf request types, to handle messages.
-   **`Query`:** gRPC query service, to handle queries.
-   

### `Msg` service

Regarding the `Msg` service, keep in mind:

-   Best practice is to define the `Msg` Protobuf service in the `tx.proto` file.
-   Each module should implement the `RegisterServices` method as part of the `AppModule` interface. This lets the application know which messages and queries the module can handle.
-   Service methods should use a *keeper*, which encapsulates knowledge about the storage layout and presents methods for updating the state.
-   

### gRPC `Query` service

For the gRPC `Query` service, keep in mind:

-   Best practice is to define the `Query` Protobuf service in the `query.proto` file.
-   It allows users to query the state using gRPC.
-   Each gRPC endpoint corresponds to a service method, named with the `rpc` prefix inside the gRPC `Query` service.
-   It can be configured under the `grpc.enable` and `grpc.address` fields in `app.toml`.

Protobuf generates a `QueryServer` interface containing all the service methods for each module. Modules implement this `QueryServer` interface by providing the concrete implementation of each service method in separate files. These implementation methods are the handlers of the corresponding gRPC query endpoints. This division of concerns across different files makes the setup safe from a re-generation of files by Protobuf.



gRPC-Gateway REST endpoints support external clients that may not wish to use gRPC. The Cosmos SDK provides a gRPC-gateway REST endpoint for each gRPC service.