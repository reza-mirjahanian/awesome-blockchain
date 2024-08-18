in the Cosmos SDK, an *account* designates a pair of *public key* `PubKey` and *private key* `PrivKey`. The `PubKey` can be derived to generate various `Addresses`, which are used to identify users (among other parties) in the application. `Addresses` are also associated with [`message`s](https://docs.cosmos.network/v0.50/build/building-modules/messages-and-queries#messages) to identify the sender of the `message`. The `PrivKey` is used to generate [digital signatures](https://docs.cosmos.network/v0.50/learn/beginner/accounts#signatures) to prove that an `Address` associated with the `PrivKey` approved of a given `message`.

For HD key derivation the Cosmos SDK uses a standard called [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki). The BIP32 allows users to create an HD wallet (as specified in [BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)) - a set of accounts derived from an initial secret seed. A seed is usually created from a 12- or 24-word mnemonic. A single seed can derive any number of `PrivKey`s using a one-way cryptographic function. Then, a `PubKey` can be derived from the `PrivKey`. Naturally, the mnemonic is the most sensitive information, as private keys can always be re-generated if the mnemonic is preserved

```
     Account 0                         Account 1                         Account 2

+------------------+              +------------------+               +------------------+
|                  |              |                  |               |                  |
|    Address 0     |              |    Address 1     |               |    Address 2     |
|        ^         |              |        ^         |               |        ^         |
|        |         |              |        |         |               |        |         |
|        |         |              |        |         |               |        |         |
|        |         |              |        |         |               |        |         |
|        +         |              |        +         |               |        +         |
|  Public key 0    |              |  Public key 1    |               |  Public key 2    |
|        ^         |              |        ^         |               |        ^         |
|        |         |              |        |         |               |        |         |
|        |         |              |        |         |               |        |         |
|        |         |              |        |         |               |        |         |
|        +         |              |        +         |               |        +         |
|  Private key 0   |              |  Private key 1   |               |  Private key 2   |
|        ^         |              |        ^         |               |        ^         |
+------------------+              +------------------+               +------------------+
         |                                 |                                  |
         |                                 |                                  |
         |                                 |                                  |
         +--------------------------------------------------------------------+
                                           |
                                           |
                                 +---------+---------+
                                 |                   |
                                 |  Master PrivKey   |
                                 |                   |
                                 +-------------------+
                                           |
                                           |
                                 +---------+---------+
                                 |                   |
                                 |  Mnemonic (Seed)  |
                                 |                   |
                                 +-------------------+
```

In the Cosmos SDK, keys are stored and managed by using an object called a Keyring

Keys, accounts, addresses, and signatures
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The principal way of authenticating a user is done using [digital signatures](https://en.wikipedia.org/wiki/Digital_signature). Users sign transactions using their own private key. Signature verification is done with the associated public key. For on-chain signature verification purposes, we store the public key in an `Account` object (alongside other data required for a proper transaction validation).

In the node, all data is stored using Protocol Buffers serialization.

The Cosmos SDK supports the following digital key schemes for creating digital signatures:

-   `secp256k1`, as implemented in the [Cosmos SDK's `crypto/keys/secp256k1` package](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/keys/secp256k1/secp256k1.go).
-   `secp256r1`, as implemented in the [Cosmos SDK's `crypto/keys/secp256r1` package](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/keys/secp256r1/pubkey.go),
-   `tm-ed25519`, as implemented in the [Cosmos SDK `crypto/keys/ed25519` package](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/crypto/keys/ed25519/ed25519.go). This scheme is supported only for the consensus validation.
-   

| Address length in bytes | Public key length in bytes | Used for transaction authentication | Used for consensus (cometbft) |
| :-: |  :-: |  :-: |  :-: |
| `secp256k1` | 20 | 33 | yes | no |
| :-: |  :-: |  :-: |  :-: |  :-: |
| `secp256r1` | 32 | 33 | yes | no |
| `tm-ed25519` | \-- not used -- | 32 | no | yes |