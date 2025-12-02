

# Future of MEV and Validators in Cosmos with Maghnus Mareneck from Skip Protocol

When it comes to front running and sandwiching, we've moved away from having any opinion on these things. I can have my own personal opinions, but ultimately this is not our decision to make. **Skip** is neutral tooling that allows full expressivity of the sovereignty of individual chains and validators.

Toxic forms of MEV are going to go away. We might do it or someone else might do it, but I think eventually they will be pushed out of the ecosystem. We want to be here for many years, so we try to take a very long-term view in terms of how things will evolve, how our products can be helpful, and how we make Cosmos the most welcoming and user-friendly experience possible.

My background: I got into crypto in 2016 as a co-founder of an early NFT marketplace called Scent that sold tweets as NFTs. After that, I became interested in computer science and finance, worked at a hedge fund, and then worked on products like Confluence in Jira at Atlassian. I met my co-founder Barry during college, and we were interested in the intersection of financial systems and engineering products.

MEV firmly fit into that category. We started Skip looking at different MEV products already offered and saw a gap. Flashbots offered a purely extraction-focused product, but when we entered Cosmos, we saw a need for something more customizable - what we call more "sovereign" - where app chains and validators can decide what MEV markets they want to use and how to configure them.

MEV is an encompassing word that categorizes everything that happens between when a user submits a transaction and when it ends up on chain. What happens in that process? Does it get censored, reordered, delayed? Many of these things can be bad and represent a toxic user experience, but not all are necessarily bad.

If you submit a transaction that causes an arbitrage opportunity, that creates profit for someone else. You should probably be rewarded for that, or at least the network should be rewarded. There should be competition around capturing that opportunity. This leads to the idea of block space markets - what do blockchains sell? They sell their block space based on their blocks.

Our idea was that if we created good MEV tooling, we could help accelerate financial sustainability by having blockchains sell their block space in open markets that return value to the native chain.

When it comes to different kinds of MEV, validators can pick on their node what kinds of MEV they want to allow. This is a sovereignty question. In Cosmos, there are two parties: individual validators and their decisions, and governance that decides collectively.

For things where chain-wide agreement makes sense, governance is a great tool. For example, a minimum commission to ensure validators don't go down to zero and then shoot up the ranks and "rug" everyone when they increase it to 20 or 50 percent. MEV falls in this category - it's a redistribution of funds just like transaction fees or taxes.

If a chain thinks it's important to have bounds around MEV, those should be expressed through governance. Then validators can operate within those bounds to express their own preferences. For example, there's a Commonwealth thread for Juno to place 25% minimum to the validator and 25% minimum to stakers, then validators can choose their own distribution within that framework.

To get started with Skip, the process is simple. First, we need to identify that you're really a validator. We have a setup process on skip.money/registration where you can sign a challenge with your operator address (the same one used for governance votes). If you don't want to do that, you can delegate a message vote authorization via authz to a different address and pull it back later.

After signing that challenge, you get an API key, which is important for the next step: getting your node set up to receive bundles from Skip. This takes about five minutes and involves using a patched version of Tendermint called MEV Tendermint, which is open source. It's minimal in its changes and doesn't modify core Tendermint functionality like signing.

You just add a couple config variables including your API key. When your node starts up, it pings the Skip relayer and says "I have this API key and I'm associated with a real validator. When I'm up next in the block, you should send me bundles so I can put them in my block."

When it's your turn next, you ping Skip, which knows you're up next and sends you bundles. These bundles include payments to your validator and get included at the top of the block. The rest of the block gets filled by your normal mempool. We're not a censorship vector - we don't build the whole block, only a couple transactions at the very top where most MEV is anyway. You sign the block as normal.

To implement this, you need to use a different version of Tendermint. You add an extra line in your go.mod file with a replace statement to replace Tendermint with MEV Tendermint, then recompile that binary and restart it through Cosmovisor.

We maintain versions 34.19 through 34.24, and we'll have a version ready as soon as upstream makes a new minor release. The only changes to MEV Tendermint are around the P2P module. The main difference is the concept of a separate mempool.

You have your regular mempool that operates untouched, and then you have what we call the sidecar mempool, used for gossiping MEV transactions. We have this separate mempool to keep it private. Regular mempool gossip spams all nodes you're connected to as peers, but the sidecar is very selective - it only receives from the relayer you specify (probably Skip, but could be another) and only sends to nodes in your own network.

For example, if you have a Horcrux cluster, it only sends to other nodes in your cluster. If you have sentries, it only sends to your validator. This provides privacy. The second concept is bundling - we send MEV transactions that should be put into bundles. We might send three transactions that together constitute a bundle needing to be executed next to each other. The sidecar knows how to receive individual transactions marked as MEV and put them into designated bundles so when it's time to create the block, you reap those transactions in the right order.

There was a big change in 34.23 where they moved P2P gossip packages to things called envelopes for more reliability. We supported that new P2P mechanism but also backported it so you can receive both kinds of packets, which is why 23 is an optional upgrade on Juno where you can use 21 or 23 and they're interoperable.

Running Skip doesn't require special hardware. There might be an extra 20 milliseconds in the reap process, probably from having additional transactions. With current volume, running Skip usually increases the number of transactions in your block by an average of three. One of those is usually a payment to the validator, which can be substantial - recently there was a bundle on Juno that paid the validator 28 Juno in one transaction.

We didn't modify signing or consensus - just added a new peering connection to receive transactions over P2P from that peer, which are treated differently once they get into the mempool.

For monitoring, we built a new set of Prometheus metrics added to the regular ones. MEV Tendermint adds six or seven new metrics. One shows if you're connected to the relayer, which has been very reliable. We've only seen disconnections when we modified our infrastructure because Tendermint caches IPs.

There are also metrics for how many bundles you've received and how big your sidecar mempool is. We'll soon be publishing profit metrics. The system has been live on Juno for two months, on Osmosis for a couple weeks, and on Terra 2 for a couple days. Over 100 validators are running the software with no failures or app hashes, so we're confident in its stability.

After setup, everything operates automatically. You set your payment address where you want funds to go (defaults to your staking address) and your payment percentage - how much MEV goes to you versus the network. Skip is free and doesn't take fees, so 100% is paid out to the validator, who decides where that revenue accrues.

Some validators keep 100% but use it to pay for their relayer, which can be confusing when people see them keeping 100% on dashboards. We're working on a way to show what these funds are being used for, especially if they're funding public goods.

For testing, we have constantly running bundle feeders that submit test bundles. When you get set up on testnet, you quickly receive a bundle and can see how it works. Your Prometheus metrics show when the bundle was received and at what block height. You can inspect the bundle to see the searcher paid this address this amount, with half sent to you and half to the network if you set a 50/50 split.

By default, Skip filters out front running and sandwiching type bundles. These are easy to detect because when a searcher submits a bundle, they sign it with a private key associated with a public address. We find the signer, verify they signed with the address they claimed, and compare that to the signers of individual transactions.

If they signed the first transaction but not the second, that's probably an attempted front run. If they signed the first and third but not the second, that's definitely a sandwich. If they didn't sign the first but signed the second, that's a back run, which we allow.

If a validator turns off that protection, we would allow anything, making the product similar to Flashbots. We've opted to keep these protections on by default until governance decides otherwise. We don't have front running or sandwiching in Cosmos already because we have first-come-first-serve mempools where transactions are ordered in the order they arrive. You can't insert your transaction above someone else, so there's no concept of front running, only back running.

Skip's mission is to maintain this status quo unless chains want to disable it and have a front running ecosystem. If that's the case, we'll support it because we're an unopinionated product.

Looking at the future of MEV in Cosmos, Osmosis produces the vast majority of MEV because every swap on Osmosis usually creates some MEV by unbalancing pools, creating arbitrage opportunities to rebalance them. Osmosis is one of the deepest thinkers on MEV in the entire crypto space.

We've had conversations with Sunny about where MEV will go. The general sense is that toxic forms of MEV will go away. Catalysts for this will include threshold encryption with encrypted mempools so information doesn't leak out. This is something you can almost exclusively do on Cosmos chains because you can modify consensus to enable it. Once we have ABCI++ and vote extensions, you'll have the ability to do this.

There's also been clear demand from validators not to enable these things. When we first entered the ecosystem, many validators were skeptical of us and distrustful of MEV as a concept because they saw it as a problem happening in Ethereum that hadn't come to Cosmos yet.

We classified MEV into two buckets: types that require a lookahead of the mempool to take advantage of transactions already there (front running and sandwiching), and MEV that already happens on Cosmos by submitting transactions after someone else's (back running), which is normal on any exchange.

We created a marketplace around this ordering to reduce spam and latency from back running and wasting block space. Instead, there's a priced auction for that ordering so value accrues back to validators and the network instead of just to searchers and their expensive infrastructure.

We want to support validators who are struggling with token prices down and chains being expensive to validate, especially for smaller validators. But we also consider the user experience and what happens if we maximize profit from MEV extraction. Osmosis was pitched as the first MEV-resistant DEX, which is exciting for traders sick of getting sandwiched on Uniswap. If we made Osmosis an MEV-extractive DEX, what would that mean for its vision?

Cosmos is an ecosystem where this wouldn't fly anyway because you can always create new chains. If users don't like MEV on Ethereum, they're stuck with Ethereum. In Cosmos, you're always exposed to someone creating a new app chain offering services you don't want to offer. Cosmos will adapt and build what users want because it's so open.

We rely on feedback from validators - they're our customers and the only ones that allow us to survive. All our time and focus has been on how to support validators in the context of Cosmos. We want to build something validators are proud to run. We've seen validators get set up with Skip, decide to share revenue back to stakers, and tweet about it, with stakers excited by that proposition.