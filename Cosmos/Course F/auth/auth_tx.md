The default implementation of `client.TxConfig` is instantiated by `NewTxConfig` in `x/auth/tx` module.

x/auth/tx/config.go
```
// NewTxConfig returns a new protobuf TxConfig using the provided ProtoCodec and sign modes. The
// first enabled sign mode will become the default sign mode.
// NOTE: Use NewTxConfigWithHandler to provide a custom signing handler in case the sign mode
// is not supported by default (eg: SignMode_SIGN_MODE_EIP_191).
funcNewTxConfig(protoCodec codec.ProtoCodecMarshaler, enabledSignModes []signingtypes.SignMode) client.TxConfig {
returnNewTxConfigWithHandler(protoCodec,makeSignModeHandler(enabledSignModes))
}
```