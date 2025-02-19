Signer
-----------------------------------------------------------------------------------------------------------------------

Signer specifies which field should be used to determine the signer of a message for the Cosmos SDK. This field can be used for clients as well to infer which field should be used to determine the signer of a message.

Read more about the signer field [here](https://docs.cosmos.network/v0.50/build/building-modules/messages-and-queries).

Method,Field,Message Added In
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

`method_added_in`, `field_added_in` and `message_added_in` are annotations to denotate to clients that a field has been supported in a later version. This is useful when new methods or fields are added in later versions and that the client needs to be aware of what it can call.

The annotation should be worded as follow:

```
option (cosmos_proto.method_added_in) = "cosmos-sdk v0.50.1";
option (cosmos_proto.method_added_in) = "x/epochs v1.0.0";
option (cosmos_proto.method_added_in) = "simapp v24.0.0";
```


Amino
--------------------------------------------------------------------------------------------------------------------

The amino codec was removed in `v0.50+`, this means there is not a need register `legacyAminoCodec`. To replace the amino codec, Amino protobuf annotations are used to provide information to the amino codec on how to encode and decode protobuf messages.

note

Amino annotations are only used for backwards compatibility with amino. New modules are not required use amino annotations.