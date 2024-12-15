Comparison Table[​](https://docs.cosmos.network/v0.52/learn/advanced/grpc_rest#comparison-table "Direct link to Comparison Table")
----------------------------------------------------------------------------------------------------------------------------------

| Name | Advantages | Disadvantages |
| --- |  --- |  --- |
| gRPC | \- can use code-generated stubs in various languages\- supports streaming and bidirectional communication (HTTP2)\- small wire binary sizes, faster transmission | \- based on HTTP2, not available in browsers\- learning curve (mostly due to Protobuf) |
| --- |  --- |  --- |
| REST | \- ubiquitous\- client libraries in all languages, faster implementation | \- only supports unary request-response communication (HTTP1.1)\- bigger over-the-wire message sizes (JSON) |
| CometBFT RPC | \- easy to use | \- bigger over-the-wire message sizes (JSON) |


### Swagger[​](https://docs.cosmos.network/v0.52/learn/advanced/grpc_rest#swagger "Direct link to Swagger")

A [Swagger](https://swagger.io/) (or OpenAPIv2) specification file is exposed under the `/swagger` route on the API server. Swagger is an open specification describing the API endpoints a server serves, including description, input arguments, return types and much more about each endpoint.

Enabling the `/swagger` endpoint is configurable inside `~/.simapp/config/app.toml` via the `api.swagger` field, which is set to false by default.

For application developers, you may want to generate your own Swagger definitions based on your custom modules. The Cosmos SDK's [Swagger generation script](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/scripts/protoc-swagger-gen.sh) is a good place to start.


An Overview of All Endpoints[​](https://docs.cosmos.network/v0.52/learn/advanced/grpc_rest#an-overview-of-all-endpoints "Direct link to An Overview of All Endpoints")
----------------------------------------------------------------------------------------------------------------------------------------------------------------------

Each node exposes the following endpoints for users to interact with a node, each endpoint is served on a different port. Details on how to configure each endpoint is provided in the endpoint's own section.

-   the gRPC server (default port: `9090`),
-   the REST server (default port: `1317`),
-   the CometBFT RPC endpoint (default port: `26657`).