

The EncodingConfig structure is the last important part of the app.go file. The goal of this structure is to define the codecs that will be used throughout the app.

```
simapp/params/encoding.go
// EncodingConfig specifies the concrete encoding types to use for a given app.
// This is provided for compatibility between protobuf and amino implementations.
type EncodingConfig struct {
	InterfaceRegistry types.InterfaceRegistry
	Codec             codec.Codec
	TxConfig          client.TxConfig
	Amino             *codec.LegacyAmino
}
```

https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/simapp/params/encoding.go#L9-L16


Here are descriptions of what each of the four fields means:

-   `InterfaceRegistry`: The `InterfaceRegistry` is used by the Protobuf codec to handle interfaces that are encoded and decoded (we also say "unpacked") using [`google.protobuf.Any`](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/any.proto). `Any` could be thought as a struct that contains a `type_url` (name of a concrete type implementing the interface) and a `value` (its encoded bytes). `InterfaceRegistry` provides a mechanism for registering interfaces and implementations that can be safely unpacked from `Any`. Each application module implements the `RegisterInterfaces` method that can be used to register the module's own interfaces and implementations.
    -   You can read more about `Any` in [ADR-019](https://docs.cosmos.network/v0.50/build/architecture/adr-019-protobuf-state-encoding).
    -   To go more into details, the Cosmos SDK uses an implementation of the Protobuf specification called [`gogoprotobuf`](https://github.com/cosmos/gogoproto). By default, the [gogo protobuf implementation of `Any`](https://pkg.go.dev/github.com/cosmos/gogoproto/types) uses [global type registration](https://github.com/cosmos/gogoproto/blob/master/proto/properties.go#L540) to decode values packed in `Any` into concrete Go types. This introduces a vulnerability where any malicious module in the dependency tree could register a type with the global protobuf registry and cause it to be loaded and unmarshaled by a transaction that referenced it in the `type_url` field. For more information, please refer to [ADR-019](https://docs.cosmos.network/v0.50/build/architecture/adr-019-protobuf-state-encoding).
-   `Codec`: The default codec used throughout the Cosmos SDK. It is composed of a `BinaryCodec` used to encode and decode state, and a `JSONCodec` used to output data to the users (for example, in the [CLI](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#cli)). By default, the SDK uses Protobuf as `Codec`.
-   `TxConfig`: `TxConfig` defines an interface a client can utilize to generate an application-defined concrete transaction type. Currently, the SDK handles two transaction types: `SIGN_MODE_DIRECT` (which uses Protobuf binary as over-the-wire encoding) and `SIGN_MODE_LEGACY_AMINO_JSON` (which depends on Amino). Read more about transactions [here](https://docs.cosmos.network/v0.50/learn/advanced/transactions).
-   `Amino`: Some legacy parts of the Cosmos SDK still use Amino for backwards-compatibility. Each module exposes a `RegisterLegacyAmino` method to register the module's specific types within Amino. This `Amino` codec should not be used by app developers anymore, and will be removed in future releases.

An application should create its own encoding config. See an example of a `simappparams.EncodingConfig` from `simapp`:

```
simapp/params/encoding.go
type EncodingConfig struct {
	InterfaceRegistry types.InterfaceRegistry
	Codec             codec.Codec
	TxConfig          client.TxConfig
	Amino             *codec.LegacyAmino
}
```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/simapp/params/encoding.go#L11-L16)