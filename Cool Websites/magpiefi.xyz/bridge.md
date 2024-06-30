
# What is a bridge?
Primarily, a bridge for cryptocurrency provides a way to transfer a token between two different blockchains without the need for a third-party intermediary.

A smart contract is used to create a two-way peg between two different chains by locking up or burning (destroying) a token on one blockchain in exchange for an equivalent amount of the same token or asset on the other blockchain. This is done so that the amount of said asset or token does not change, and the user can freely transfer assets between the two blockchains without the need for a third party.

Once the token is bridged/transferred to the new chain, it can be used just like any other token: to swap, provide liquidity, farm, etc.

Currently, there are quite a few different types of bridges that make it even more challenging to figure out what to use to go cross-chain.

> **With most bridges taking anywhere from fifteen minutes to a few hours (and some even taking a few days,) users trying to catch a deal or quickly join a farm on another chain might find themselves in a tough spot as prices and APR can change drastically in that time.**

The other major component of most bridges is that they require users to lock or burn their token in order to receive (or mint) the token or wrapped token on another blockchain. This can be a cause for confusion, or just a pain to deal with, as there are so many different options and you don’t know how long each option takes — sometimes it varies wildly, with the same bridge taking ten minutes one time and over a half-hour on another transfer.

_This is a must for the future of DeFi. As all blockchains are independent, decentralization cannot be fully realized and achieved without a secure way to communicate and swap between chains through a decentralized protocol._


# Solving Blockchain Bridge Problem

Back in August 2021, the biggest cryptocurrency heist up to that date saw [$600 million worth of digital assets stolen from Poly Network](https://www.reuters.com/technology/how-hackers-stole-613-million-crypto-tokens-poly-network-2021-08-12/) — a DeFi protocol focused on cross-chain asset swaps — only for the hacker to return the stolen assets a short while later.

Less than a year later in March 2022, the largest crypto theft in history resulted in [$625 million worth of digital assets stolen from the Ronin Bridge](https://gizmodo.com/hackers-steal-625-million-from-ronin-network-in-larges-1848724760) — a cross-chain bridge used by Axie Infinity players to transfer assets from Ethereum to the Ronin network. These persistent attacks proved that DeFi apps are only as robust as their underlying cross-chain bridges.

Born out of necessity to create a safer cross-chain environment,  [**Magpie**](https://magpiefi.xyz/) **is aiming to solve blockchain’s billion-dollar bridge problem**, by making cross-chain asset swaps more secure and efficient.

To do so, we had to reinvent the fundamental mechanics of blockchain bridges and look at the technology’s biggest vulnerabilities. So how did we manage to do that?


First and foremost, by making cross-chain asset swaps non-custodial — meaning that your digital assets are never held on a blockchain bridge. Instead, Magpie only leverages cross-chain bridges as a messaging service, using the Wormhole Guardian Network to communicate asset swap signals between blockchains.

Once the asset swap has been signalled and verified through 19 Wormhole Guardian Nodes, the Magpie core contract on the target chain receives the confirmation and your requested assets are sent to your wallet.

At press time, the total value-locked (TVL) across all cross-chain bridges exceeds $72 billion, down from an all-time high of over $253 billion back in December, according to [data by DefiLama](https://defillama.com/). It’s only natural for such a rapidly growing industry to attract its share of malicious actors.

As cross-chain applications gain more ground in our daily lives, non-custodial blockchain bridges will likely become the industry standard, eradicating bridge exploits and the vulnerabilities of custodial solutions.

Until that day comes, Magpie will be at the forefront of bridge development, making **cross-chain asset swaps user-friendly, secure, and more efficient**.