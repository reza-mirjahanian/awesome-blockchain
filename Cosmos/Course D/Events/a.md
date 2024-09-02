An event is an object that contains information about the execution of applications. Events are used by service providers like block explorers and wallets to track the execution of various messages and index transactions.

Events are implemented as an alias of the ABCI `event` type in the form `{eventType}.{attributeKey}={attributeValue}` in the Cosmos SDK.

Events allow application developers to attach additional information. This means that transactions might be queried using events:

```
// Events allow application developers to attach additional information to
// ResponseBeginBlock, ResponseEndBlock, ResponseCheckTx, and ResponseDeliverTx.
// Later, transactions may be queried using these events.
message Event {
  string                  type       = 1;
  repeated EventAttribute attributes = 2 [
    (gogoproto.nullable) = false,
    (gogoproto.jsontag)  = "attributes,omitempty"
  ];
}

```