Keyring[​](https://docs.cosmos.network/v0.50/learn/beginner/accounts#keyring "Direct link to Keyring")
------------------------------------------------------------------------------------------------------

A `Keyring` is an object that stores and manages accounts. In the Cosmos SDK, a `Keyring` implementation follows the `Keyring` interface:

crypto/keyring/keyring.go
```
// Keyring exposes operations over a backend supported by github.com/99designs/keyring.
type Keyring interface{
// Get the backend type used in the keyring config: "file", "os", "kwallet", "pass", "test", "memory".
Backend()string
// List all keys.
List()([]*Record,error)
// Supported signing algorithms for Keyring and Ledger respectively.
SupportedAlgorithms()(SigningAlgoList, SigningAlgoList)
// Key and KeyByAddress return keys by uid and address respectively.
Key(uid string)(*Record,error)
KeyByAddress(address sdk.Address)(*Record,error)
// Delete and DeleteByAddress remove keys from the keyring.
Delete(uid string)error
DeleteByAddress(address sdk.Address)error
// Rename an existing key from the Keyring
Rename(from, to string)error
// NewMnemonic generates a new mnemonic, derives a hierarchical deterministic key from it, and
// persists the key to storage. Returns the generated mnemonic and the key Info.
// It returns an error if it fails to generate a key for the given algo type, or if
// another key is already stored under the same name or address.
//
// A passphrase set to the empty string will set the passphrase to the DefaultBIP39Passphrase value.
NewMnemonic(uid string, language Language, hdPath, bip39Passphrase string, algo SignatureAlgo)(*Record,string,error)
// NewAccount converts a mnemonic to a private key and BIP-39 HD Path and persists it.
// It fails if there is an existing key Info with the same address.
NewAccount(uid, mnemonic, bip39Passphrase, hdPath string, algo SignatureAlgo)(*Record,error)
// SaveLedgerKey retrieves a public key reference from a Ledger device and persists it.
SaveLedgerKey(uid string, algo SignatureAlgo, hrp string, coinType, account, index uint32)(*Record,error)
// SaveOfflineKey stores a public key and returns the persisted Info structure.
SaveOfflineKey(uid string, pubkey types.PubKey)(*Record,error)
// SaveMultisig stores and returns a new multsig (offline) key reference.
SaveMultisig(uid string, pubkey types.PubKey)(*Record,error)
	Signer
	Importer
	Exporter
	Migrator
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/keyring/keyring.go#L57-L105)

The default implementation of `Keyring` comes from the third-party [`99designs/keyring`](https://github.com/99designs/keyring) library.

A few notes on the `Keyring` methods:

-   `Sign(uid string, msg []byte) ([]byte, types.PubKey, error)` strictly deals with the signature of the `msg` bytes. You must prepare and encode the transaction into a canonical `[]byte` form. Because protobuf is not deterministic, it has been decided in [ADR-020](https://docs.cosmos.network/v0.50/build/architecture/adr-020-protobuf-transaction-encoding) that the canonical `payload` to sign is the `SignDoc` struct, deterministically encoded using [ADR-027](https://docs.cosmos.network/v0.50/build/architecture/adr-027-deterministic-protobuf-serialization). Note that signature verification is not implemented in the Cosmos SDK by default, it is deferred to the [`anteHandler`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#antehandler).

proto/cosmos/tx/v1beta1/tx.proto
```
messageSignDoc{
// body_bytes is protobuf serialization of a TxBody that matches the
// representation in TxRaw.
bytes body_bytes =1;
// auth_info_bytes is a protobuf serialization of an AuthInfo that matches the
// representation in TxRaw.
bytes auth_info_bytes =2;
// chain_id is the unique identifier of the chain this transaction targets.
// It prevents signed transactions from being used on another chain by an
// attacker
string chain_id =3;
// account_number is the account number of the account in state
uint64 account_number =4;
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/proto/cosmos/tx/v1beta1/tx.proto#L50-L66)

-   `NewAccount(uid, mnemonic, bip39Passphrase, hdPath string, algo SignatureAlgo) (*Record, error)` creates a new account based on the [`bip44 path`](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) and persists it on disk. The `PrivKey` is **never stored unencrypted**, instead it is [encrypted with a passphrase](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/armor.go) before being persisted. In the context of this method, the key type and sequence number refer to the segment of the BIP44 derivation path (for example, `0`, `1`, `2`, ...) that is used to derive a private and a public key from the mnemonic. Using the same mnemonic and derivation path, the same `PrivKey`, `PubKey` and `Address` is generated. The following keys are supported by the keyring:

-   `secp256k1`

-   `ed25519`

-   `ExportPrivKeyArmor(uid, encryptPassphrase string) (armor string, err error)` exports a private key in ASCII-armored encrypted format using the given passphrase. You can then either import the private key again into the keyring using the `ImportPrivKey(uid, armor, passphrase string)` function or decrypt it into a raw private key using the `UnarmorDecryptPrivKey(armorStr string, passphrase string)` function.

### Create New Key Type[​](https://docs.cosmos.network/v0.50/learn/beginner/accounts#create-new-key-type "Direct link to Create New Key Type")

To create a new key type for using in keyring, `keyring.SignatureAlgo` interface must be fulfilled.

crypto/keyring/signing\_algorithms.go
```

// SignatureAlgo defines the interface for a keyring supported algorithm.
type SignatureAlgo interface{
Name() hd.PubKeyType
Derive() hd.DeriveFn
Generate() hd.GenerateFn

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/keyring/signing_algorithms.go#L10-L15)

The interface consists in three methods where `Name()` returns the name of the algorithm as a `hd.PubKeyType` and `Derive()` and `Generate()` must return the following functions respectively:

crypto/hd/algo.go
```
type(
	DeriveFn   func(mnemonic, bip39Passphrase, hdPath string)([]byte,error)
	GenerateFn func(bz []byte) types.PrivKey
)

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/hd/algo.go#L28-L31)

Once the `keyring.SignatureAlgo` has been implemented it must be added to the [list of supported algos](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/keyring/keyring.go#L217) of the keyring.

For simplicity the implementation of a new key type should be done inside the `crypto/hd` package. There is an example of a working `secp256k1` implementation in [algo.go](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/hd/algo.go#L38).

#### Implementing secp256r1 algo[​](https://docs.cosmos.network/v0.50/learn/beginner/accounts#implementing-secp256r1-algo "Direct link to Implementing secp256r1 algo")

Here is an example of how secp256r1 could be implemented.

First a new function to create a private key from a secret number is needed in the secp256r1 package. This function could look like this:

```
// cosmos-sdk/crypto/keys/secp256r1/privkey.go
// NewPrivKeyFromSecret creates a private key derived for the secret number
// represented in big-endian. The `secret` must be a valid ECDSA field element.
funcNewPrivKeyFromSecret(secret []byte)(*PrivKey,error){
var d =new(big.Int).SetBytes(secret)
if d.Cmp(secp256r1.Params().N)>=1{
returnnil, errorsmod.Wrap(errors.ErrInvalidRequest,"secret not in the curve base field")
}
    sk :=new(ecdsa.PrivKey)
return&PrivKey{&ecdsaSK{*sk}},nil
}

```

After that `secp256r1Algo` can be implemented.

```
// cosmos-sdk/crypto/hd/secp256r1Algo.go
package hd
import(
"github.com/cosmos/go-bip39"
"github.com/cosmos/cosmos-sdk/crypto/keys/secp256r1"
"github.com/cosmos/cosmos-sdk/crypto/types"
)
// Secp256r1Type uses the secp256r1 ECDSA parameters.
const Secp256r1Type =PubKeyType("secp256r1")
var Secp256r1 = secp256r1Algo{}
type secp256r1Algo struct{}
func(s secp256r1Algo)Name() PubKeyType {
return Secp256r1Type
}
// Derive derives and returns the secp256r1 private key for the given seed and HD path.
func(s secp256r1Algo)Derive() DeriveFn {
returnfunc(mnemonic string, bip39Passphrase, hdPath string)([]byte,error){
        seed, err := bip39.NewSeedWithErrorChecking(mnemonic, bip39Passphrase)
if err !=nil{
returnnil, err
}
        masterPriv, ch :=ComputeMastersFromSeed(seed)
iflen(hdPath)==0{
return masterPriv[:],nil
}
        derivedKey, err :=DerivePrivateKeyForPath(masterPriv, ch, hdPath)
return derivedKey, err
}
}
// Generate generates a secp256r1 private key from the given bytes.
func(s secp256r1Algo)Generate() GenerateFn {
returnfunc(bz []byte) types.PrivKey {
        key, err := secp256r1.NewPrivKeyFromSecret(bz)
if err !=nil{
panic(err)
}
return key
}
}

```

Finally, the algo must be added to the list of [supported algos](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/keyring/keyring.go#L217) by the keyring.

```
// cosmos-sdk/crypto/keyring/keyring.go
funcnewKeystore(kr keyring.Keyring, cdc codec.Codec, backend string, opts ...Option) keystore {
// Default options for keybase, these can be overwritten using the
// Option function
    options := Options{
        SupportedAlgos:       SigningAlgoList{hd.Secp256k1, hd.Secp256r1},// added here
        SupportedAlgosLedger: SigningAlgoList{hd.Secp256k1},
}
...

```

Hereafter to create new keys using your algo, you must specify it with the flag `--algo` :

`simd keys add myKey --algo secp256r1`