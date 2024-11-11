### Keypair [#](https://solana.com/docs/intro/wallets#keypair)

A [*keypair*](https://solana.com/docs/terminology#keypair) is a securely generated [*secret key*](https://solana.com/docs/intro/wallets#secret-key) and its cryptographically-derived [*public key*](https://solana.com/docs/intro/wallets#public-key). A secret key and its corresponding public key are together known as a *keypair*. A wallet contains a collection of one or more keypairs and provides some means to interact with them.

### Public key [#](https://solana.com/docs/intro/wallets#public-key)

The [*public key*](https://solana.com/docs/terminology#public-key-pubkey) (commonly shortened to *pubkey*) is known as the wallet's *receiving address* or simply its *address*. The wallet address **may be shared and displayed freely**. When another party is going to send some amount of cryptocurrency to a wallet, they need to know the wallet's receiving address. Depending on a blockchain's implementation, the address can also be used to view certain information about a wallet, such as viewing the balance, but has no ability to change anything about the wallet or withdraw any tokens.

### Secret key [#](https://solana.com/docs/intro/wallets#secret-key)

The [*secret key*](https://solana.com/docs/terminology#private-key) (also referred to as *private key*) is required to digitally sign any transactions to send cryptocurrencies to another address or to make any changes to the wallet. The secret key **must never be shared**. If someone gains access to the secret key to a wallet, they can withdraw all the tokens it contains. If the secret key for a wallet is lost, any tokens that have been sent to that wallet's address are **permanently lost**.

Security [#](https://solana.com/docs/intro/wallets#security)
------------------------------------------------------------

Different wallet solutions offer different approaches to keypair security, interacting with the keypair, and signing transactions to use/spend the tokens. Some are easier to use than others. Some store and back up secret keys more securely. Solana supports multiple types of wallets so you can choose the right balance of security and convenience.

**If you want to be able to receive SOL tokens on the Solana blockchain, you first will need to create a wallet.**

Supported Wallets [#](https://solana.com/docs/intro/wallets#supported-wallets)
------------------------------------------------------------------------------

Several browser and mobile app based wallets support Solana. Find some options that might be right for you on the [Solana Wallets](https://solana.com/wallets) page.

For advanced users or developers, the [command-line wallets](https://docs.solanalabs.com/cli/wallets) may be more appropriate, as new features on the Solana blockchain will always be supported on the command line first before being integrated into third-party solutions.