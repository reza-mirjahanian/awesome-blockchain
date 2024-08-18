`Addresses` and `PubKey`s are both public information that identifies actors in the application. `Account` is used to store authentication information. The basic account implementation is provided by a `BaseAccount` object.

Each account is identified using `Address` which is a sequence of bytes derived from a public key. In the Cosmos SDK, we define 3 types of addresses that specify a context where an account is used:

-   `AccAddress` identifies users (the sender of a `message`).
-   `ValAddress` identifies validator operators.
-   `ConsAddress` identifies validator nodes that are participating in consensus. Validator nodes are derived using the **`ed25519`** curve.

These types implement the `Address` interface:

types/address.go
```
type Address interface{
Equals(Address)bool
Empty()bool
Marshal()([]byte,error)
MarshalJSON()([]byte,error)
Bytes()[]byte
String()string
Format(s fmt.State, verb rune)
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/types/address.go#L126-L134)

Address construction algorithm is defined in [ADR-28](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-028-public-key-addresses.md). Here is the standard way to obtain an account address from a `pub` public key:

```
sdk.AccAddress(pub.Address().Bytes())

```

Of note, the `Marshal()` and `Bytes()` method both return the same raw `[]byte` form of the address. `Marshal()` is required for Protobuf compatibility.

For user interaction, addresses are formatted using [Bech32](https://en.bitcoin.it/wiki/Bech32) and implemented by the `String` method. The Bech32 method is the only supported format to use when interacting with a blockchain. The Bech32 human-readable part (Bech32 prefix) is used to denote an address type. Example:

types/address.go
```
func(aa AccAddress)String()string{
if aa.Empty(){
return""
}
	key := conv.UnsafeBytesToStr(aa)
ifIsAddrCacheEnabled(){
		accAddrMu.Lock()
defer accAddrMu.Unlock()
		addr, ok := accAddrCache.Get(key)
if ok {
return addr.(string)
}
}
returncacheBech32Addr(GetConfig().GetBech32AccountAddrPrefix(), aa, accAddrCache, key)
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/types/address.go#L299-L316)

|  | Address Bech32 Prefix |
| --- |  --- |
| Accounts | cosmos |
| --- |  --- |
| Validator Operator | cosmosvaloper |
| Consensus Nodes | cosmosvalcons |

### Public Keys[​](https://docs.cosmos.network/v0.50/learn/beginner/accounts#public-keys "Direct link to Public Keys")

Public keys in Cosmos SDK are defined by `cryptotypes.PubKey` interface. Since public keys are saved in a store, `cryptotypes.PubKey` extends the `proto.Message` interface:

crypto/types/types.go
```
// PubKey defines a public key and extends proto.Message.
type PubKey interface{
	proto.Message
Address() Address
Bytes()[]byte
VerifySignature(msg, sig []byte)bool
Equals(PubKey)bool
Type()string
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/types/types.go#L8-L17)

A compressed format is used for `secp256k1` and `secp256r1` serialization.

-   The first byte is a `0x02` byte if the `y`\-coordinate is the lexicographically largest of the two associated with the `x`\-coordinate.
-   Otherwise the first byte is a `0x03`.

This prefix is followed by the `x`\-coordinate.

Public Keys are not used to reference accounts (or users) and in general are not used when composing transaction messages (with few exceptions: `MsgCreateValidator`, `Validator` and `Multisig` messages). For user interactions, `PubKey` is formatted using Protobufs JSON ([ProtoMarshalJSON](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/codec/json.go#L14-L34) function). Example:

client/keys/output.go
```
// NewKeyOutput creates a default KeyOutput instance without Mnemonic, Threshold and PubKeys
funcNewKeyOutput(name string, keyType keyring.KeyType, a sdk.Address, pk cryptotypes.PubKey)(KeyOutput,error){
	apk, err := codectypes.NewAnyWithValue(pk)
if err !=nil{
return KeyOutput{}, err
}
	bz, err := codec.ProtoMarshalJSON(apk,nil)
if err !=nil{
return KeyOutput{}, err
}
return KeyOutput{
		Name:    name,
		Type:    keyType.String(),
		Address: a.String(),
		PubKey:string(bz),
},nil
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/client/keys/output.go#L23-L39)


