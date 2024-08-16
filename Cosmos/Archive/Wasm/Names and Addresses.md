# Names and Addresses

Almost all blockchains use addresses to identify external actors via a hash of a public key, and many newer ones extended this to identify on-chain "smart contracts" with unique addresses as well.

On-chain addresses are represented by the use of a concise, immutable binary format, typically 20 or 32 bytes long, often derived from a hashing function. However, there are many human-readable representations of these binary addresses, which are shown to clients.

For example,

-   [Bech32](https://en.bitcoin.it/wiki/Bech32)  `bc1qc7slrfxkknqcq2jevvvkdgvrt8080852dfjewde450xdlk4ugp7szw5tk9`, hex  `0x8617E340B3D01FA5F11F306F4090FD50E238070D`,
-   [checksummed hex](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-55.md)  `0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed`,
-   and even  [large integers](https://research.kudelskisecurity.com/2018/01/16/blockchains-how-to-steal-millions-in-264-operations/)  `3040783849904107057L`  .

## Addr[​](https://docs.cosmwasm.com/docs/architecture/addresses#addr "Direct link to heading")

Addresses in Cosmos SDK are 20-character long strings and contain security checks - such as chain-prefix in Bech32, checksums in Bech32, and checksummed hex (EIP55). Since CosmWasm is an extension of Cosmos SDK, it follows the same address rules; wallets, smart contracts, and modules have an identifier address with a defined prefix.  `cosmos1...`  for gaia,  `wasm1...`  for CosmWasm testnets.

For passing address to contracts, pass it as a string and then validate the input to an:  **Addr**.  [Addr](https://github.com/CosmWasm/cosmwasm/blob/v0.14.0/packages/std/src/addresses.rs#L31)  is just a wrapper around a plain string that provides useful helper functions such as string validation to an address.

There is another representation of an address, called  _Canonical Addresses_, that tackles the case of change in human representation, but this is rare.

For example Bitcoin  [moved from Base58 to Bech32](https://en.bitcoin.it/wiki/BIP_0173)  encoding along with SegWit, and Rise is also  [moving from Lisk format to Bech32](https://medium.com/rise-vision/introducing-rise-v2-521a58e1e9de#41d5)  in the v2 upgrade.

This means that if  `message.signer`  is always the string address that signed the transaction and I use it to look up your account balance, if this encoding ever changed, then you lose access to your account. We need a stable identifier to work with internally.

This is where we define a  _Canonical Address_. This is defined to be stable and unique. That is, for one given account, there is only ever one canonical address (for the life of the blockchain). We define a  _canonical address_  format that potentially multiple string addresses can be converted to. It can be transformed back and forth without any changes