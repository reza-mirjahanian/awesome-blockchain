Query Processing in Blockchain Applications
===========================================

Overview
--------

This text describes the process of handling a query in a blockchain application, from the initial call to the final output.

Key Points
----------

### 1\. Query Reception and Processing

-   **Initial Call**: The process begins with a call to `ABCIQueryWithOptions()`.
-   **Receiver**: A full-node receives and processes the query.
-   **Nature of Queries**:
    -   Not part of consensus
    -   Not broadcasted to the entire network
    -   Don't require network-wide agreement

### 2\. Application Query Handling

-   **Environment**: Queries are handled in an environment that understands application-specific types and has a state copy.
-   **Key Components**:
    -   `baseapp`: Implements ABCI `Query()` function and handles gRPC queries
    -   `GRPCQueryRouter`: Used by `baseapp` to route queries to relevant modules

### 3\. Query Routing Process

1.  Query route is parsed
2.  Matched with a fully-qualified service method name
3.  Request is relayed to the relevant module
4.  gRPC handler in the module:
    -   Recognizes the query
    -   Retrieves appropriate values from stores
    -   Returns a response

### 4\. Response Generation and Handling

-   **Response Type**: `abci.ResponseQuery`
-   **Processing**: `client.Context Query()` routine receives the response

### 5\. CLI Response

-   **Unmarshalling**: Application codec unmarshals the response to JSON
-   **Output**: `client.Context` prints the output to the command line
-   **Output Formats**:
    -   Text (default, converted from JSON to YAML)
    -   JSON
    -   YAML


With a call to `ABCIQueryWithOptions()`, `MyQuery` is received by a [full-node](https://docs.cosmos.network/v0.50/learn/advanced/encoding) which then processes the request. Note that, while the RPC is made to the consensus engine (e.g. CometBFT) of a full-node, queries are not part of consensus and so are not broadcasted to the rest of the network, as they do not require anything the network needs to agree upon.

Read more about ABCI Clients and CometBFT RPC in the [CometBFT documentation](https://docs.cometbft.com/v0.37/spec/rpc/).

Application Query Handling[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#application-query-handling "Direct link to Application Query Handling")
----------------------------------------------------------------------------------------------------------------------------------------------------------------------

When a query is received by the full-node after it has been relayed from the underlying consensus engine, it is at that point being handled within an environment that understands application-specific types and has a copy of the state. [`baseapp`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp) implements the ABCI [`Query()`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#query) function and handles gRPC queries. The query route is parsed, and it matches the fully-qualified service method name of an existing service method (most likely in one of the modules), then `baseapp` relays the request to the relevant module.

Since `MyQuery` has a Protobuf fully-qualified service method name from the `staking` module (recall `/cosmos.staking.v1beta1.Query/Delegations`), `baseapp` first parses the path, then uses its own internal `GRPCQueryRouter` to retrieve the corresponding gRPC handler, and routes the query to the module. The gRPC handler is responsible for recognizing this query, retrieving the appropriate values from the application's stores, and returning a response. Read more about query services [here](https://docs.cosmos.network/v0.50/build/building-modules/query-services).

Once a result is received from the querier, `baseapp` begins the process of returning a response to the user.

Response[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#response "Direct link to Response")
----------------------------------------------------------------------------------------------------------------

Since `Query()` is an ABCI function, `baseapp` returns the response as an [`abci.ResponseQuery`](https://docs.cometbft.com/master/spec/abci/abci.html#query-2) type. The `client.Context` `Query()` routine receives the response and.

### CLI Response[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#cli-response "Direct link to CLI Response")

The application [`codec`](https://docs.cosmos.network/v0.50/learn/advanced/encoding) is used to unmarshal the response to a JSON and the `client.Context` prints the output to the command line, applying any configurations such as the output type (text, JSON or YAML).

client/context.go
```
func(ctx Context)printOutput(out []byte)error{
var err error
if ctx.OutputFormat =="text"{
		out, err = yaml.JSONToYAML(out)
if err !=nil{
return err
}
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/client/context.go#L341-L349)

And that's a wrap! The result of the query is outputted to the console by the CLI.