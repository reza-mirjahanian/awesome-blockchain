---
title: What is Archive Data on Ethereum?
description: >-
  Archive data is data on the blockchain that is older than 128 blocks, which is
  approximately 4 epochs or 25.6 minutes old
subtitle: >-
  Archive data is data on the blockchain that is older than 128 blocks, which is
  approximately 4 epochs or 25.6 minutes old
url: 'https://docs.alchemy.com/docs/what-is-archive-data-on-ethereum'
slug: docs/what-is-archive-data-on-ethereum
---

# Archive Data

**Archive data** is data on the blockchain that is **older than 128 blocks**. Archive data is at least 25.6 minutes old because one block can be created every 12 seconds. Archive data is also at least 4 epochs old (128 slots) because there are 32 slots per epoch, and 1 block can be validated in each slot. Because archive data is at least 4 epochs old, the commitment level for archive data is considered "finalized". This data is used to store information about past transactions and events that have occurred on the blockchain network. This data is used by users to help them understand the history of the network and to make sure that all transactions and events that have occurred on the network are valid.

# Full Nodes

Full nodes store the current and most recent blockchain states (up to the last 128 blocks) and participate in validating newly added blocks. They can process transactions, execute smart contracts, and query/serve blockchain data. They can also access some historical data (via tracing) but are inefficient for this task.

# Archive Nodes

Archive nodes store the same information as full nodes and all previous states of the blockchain(data older than 128 blocks). Running an archive node requires the most investment in hardware, running costs, technical expertise, and experience. Archive nodes build archival blockchain data quickly and efficiently, and they’re useful for querying arbitrary historical data, like a user’s balances on a specific block.

Only an archive node can serve API requests for certain RPC methods older than 128 blocks. The Ethereum JSON-RPC and Websocket APIs include several methods which require access to an archive node.

# Methods that require Archive Data

Requests for data older than the most recent 128 blocks require access to archive data. The following methods include a parameter for specifying a block number for the request:

* [eth\_getBalance](/reference/eth-getbalance)
* [eth\_call](/reference/eth-call)
* [eth\_getCode](/reference/eth-getcode)
* [eth\_getStorageAt](/reference/eth-getstorageat)
* [eth\_call](/reference/eth-call)

<Info>
  These methods can also be used to get non-archive data. Archive data access is required only if you request data older than 128 blocks using these methods.
</Info>

# Use cases for Archive Data

Here are two use-cases for Ethereum archive data:

## 1. Auditing historical information for blockchains

If you're building a service to audit a blockchain or gather specific pieces of historic data, archive data is ideal. A good use-case would be if you were building a blockchain explorer (Etherscan), an on-chain analytics tool (Dune Analytics), or a cryptocurrency wallet.

These services rely on archive nodes to query and serve up old state data for users. For example, you can get information about the first block mined on Ethereum using Etherscan.

## 2. dApp development

dApps that need to access data older than 128 blocks require access to archive data.

Examples of dApps that may need access to an archive data include:

On-chain reputation services (e.g. DegenScore) that track user activity over a large period of time. Governance platforms (e.g., Tally, Snapshot) that allow users to discuss and vote on governance proposals.


