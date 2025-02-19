Although it is technically feasible to proceed to create a novel `MsgService`, the recommended approach is to define a Protobuf `Msg` service. Each module has exactly one Protobuf `Msg` service defined in `tx.proto` and there is an RPC service method for each message type in the module. The Protobuf message service implicitly defines the interface layer of the state, mutating processes contained within the module.

How does all of this translate into code? Here is an example `MsgService` from the [`bank` module (opens new window)](https://docs.cosmos.network/v0.46/modules/bank/):

```
// Msg defines the bank Msg service.
service Msg {
  // Send defines a method for sending coins from one account to another account.
  rpc Send(MsgSend) returns (MsgSendResponse);

  // MultiSend defines a method for sending coins from some accounts to other accounts.
  rpc MultiSend(MsgMultiSend) returns (MsgMultiSendResponse);
}
```

In this example:

-   Each `Msg` service method has exactly **one argument**, such as `MsgSend`, which must implement the `sdk.Msg` interface and a Protobuf response.
-   The **standard naming convention** is to call the RPC argument `Msg<service-rpc-name>` and the RPC response `Msg<service-rpc-name>Response`.