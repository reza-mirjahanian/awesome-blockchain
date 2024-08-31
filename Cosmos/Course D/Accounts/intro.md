he different digital key schemes are implemented in different SDK packages:

-   **[secp256k1 (opens new window)](https://www.secg.org/sec2-v2.pdf)**, as implemented in the SDK's `crypto/keys/secp256k1` package: is the most common and the same one used for Bitcoin.
-   **[secp256r1 (opens new window)](https://www.secg.org/sec2-v2.pdf)**, as implemented in the SDK's `crypto/keys/secp256r1` package.
-   **[tm-ed25519 (opens new window)](https://ed25519.cr.yp.to/ed25519-20110926.pdf)**, as implemented in the SDK's `crypto/keys/ed25519` package: is supported only for consensus validation.
-   

Accounts
---------

The [`BaseAccount` (opens new window)](https://github.com/cosmos/cosmos-sdk/blob/bf11b1bf1fa0c52fb2cd51e4f4ab0c90a4dd38a0/x/auth/types/auth.pb.go#L31-L36)object provides the basic account implementation that stores authentication information.


Address
-------

An address is public information normally used to reference an account. Addresses are derived from public keys using [ADR-28 (opens new window)](https://github.com/cosmos/cosmos-sdk/blob/master/docs/architecture/adr-028-public-key-addresses.md). Three types of addresses specify a context when an account is used:

-   [`AccAddress` (opens new window)](https://github.com/cosmos/cosmos-sdk/blob/1dba6735739e9b4556267339f0b67eaec9c609ef/types/address.go#L129)identifies users, which are the sender of a message.
-   [`ValAddress` (opens new window)](https://github.com/cosmos/cosmos-sdk/blob/23e864bc987e61af84763d9a3e531707f9dfbc84/types/address.go#L298)identifies validator operators.
-   [`ConsAddress` (opens new window)](https://github.com/cosmos/cosmos-sdk/blob/23e864bc987e61af84763d9a3e531707f9dfbc84/types/address.go#L448)identifies validator nodes that are participating in consensus. Validator nodes are derived using the [ed25519 (opens new window)](https://www.cryptosys.net/pki/manpki/pki_eccsafecurves.html)curve
-   


Keyring
-------

The keyring object stores and manages multiple accounts. It implements the [`Keyring` (opens new window)](https://github.com/cosmos/cosmos-sdk/blob/bf11b1bf1fa0c52fb2cd51e4f4ab0c90a4dd38a0/crypto/keyring/keyring.go#L55)interface in the Cosmos SDK. You will make use of keyrings from the command-line in the exercises.