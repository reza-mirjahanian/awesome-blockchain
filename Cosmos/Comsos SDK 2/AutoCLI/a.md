
- The `autocli` (also known as `client/v2`) package is a [Go library](https://pkg.go.dev/cosmossdk.io/client/v2/autocli) for generating CLI

- Autocli generates CLI commands and flags directly from your **protobuf messages**, including options, input parameters, and output parameters. This means that you can easily add a CLI interface to your application without having to manually create and manage commands.


## Example:

By default, it generates commands for each gRPC services. The commands are named based on the name of the service method.

```
service MyService {
  rpc MyMethod(MyRequest) returns (MyResponse) {}
}
```
`autocli` would generate a command named `my-method` for the `MyMethod` method. The command will have flags for each field in the `MyRequest` message.

-------------------------

To further enhance your CLI experience with Cosmos SDK-based blockchains, you can use `hubl`. `hubl` is a tool that allows you to query any Cosmos SDK-based blockchain using the new AutoCLI feature of the Cosmos SDK. With `hubl`, you can easily configure a new chain and query modules with just a few simple commands.

For more information on `hubl`, including how to configure a new chain and query a module, see the [Hubl documentation](https://docs.cosmos.network/main/tooling/hubl).

---------