A Protobuf Query service processes [`queries`](https://docs.cosmos.network/v0.50/build/building-modules/messages-and-queries#queries). Query services are specific to the module in which they are defined, and only process `queries` defined within said module. They are called from `BaseApp`'s [`Query` method](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#query).



### gRPC Service

When defining a Protobuf `Query` service, a `QueryServer` interface is generated for each module with all the service methods:



### Calling queries from the State Machine

The Cosmos SDK v0.47 introduces a new `cosmos.query.v1.module_query_safe` Protobuf annotation which is used to state that a query that is safe to be called from within the state machine, for example:

-   a Keeper's query function can be called from another module's Keeper,
-   ADR-033 intermodule query calls,
-   CosmWasm contracts can also directly interact with these queries.

If the `module_query_safe` annotation set to `true`, it means:

-   The query is deterministic: given a block height it will return the same response upon multiple calls, and doesn't introduce any state-machine breaking changes across SDK patch versions.
-   Gas consumption never fluctuates across calls and across patch versions.

If you are a module developer and want to use `module_query_safe` annotation for your own query, you have to ensure the following things:

-   the query is deterministic and won't introduce state-machine-breaking changes without coordinated upgrades
-   it has its gas tracked, to avoid the attack vector where no gas is accounted for on potentially high-computation queries.