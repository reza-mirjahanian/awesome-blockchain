**Execution clients**, formerly known as *eth1 clients*, are implementing Ethereum [execution layer](https://github.com/ethereum/execution-specs) tasked with processing and broadcasting transactions and managing the state. They receive transactions via p2p, run the computations for each transaction using the [Ethereum Virtual Machine](https://ethereum.org/en/developers/docs/evm/) to update the state and ensure that the rules of the protocol are followed. Execution clients can be configured as full nodes which holds the state, historical blockchain including receipts, or as an archive node which retains also all historical states.


[Overview Table](https://epf.wiki/#/wiki/EL/el-clients?id=overview-table)
-------------------------------------------------------------------------

Current execution clients used in production are:

| Client | Language | Developer | Status |
| --- |  --- |  --- |  --- |
| [Besu](https://github.com/hyperledger/besu) | Java | Hyperledger | Production |
| [Erigon](https://github.com/ledgerwatch/erigon) | Go | Ledgerwatch | Production |
| [Geth](https://github.com/ethereum/go-ethereum) | Go | Ethereum Foundation | Production |
| [Nethermind](https://github.com/NethermindEth/nethermind) | C# | Nethermind | Production |
| [Reth](https://github.com/paradigmxyz/reth) | Rust | Paradigm | Production |


There are more execution clients that are in active development and haven't reached maturity yet or has been used in the past:

| Client | Language | Developer | Status |
| --- |  --- |  --- |  --- |
| [Nimbus](https://github.com/status-im/nimbus-eth1) | Nim | Nimbus | Development |
| [Silkworm](https://github.com/erigontech/silkworm) | C++ | Erigon | Development |
| [JS Client](https://github.com/ethereumjs/ethereumjs-monorepo) | Typescript | Ethereum Foundation | Development |
| [ethrex](https://github.com/lambdaclass/ethrex) | Rust | LambdaClass | Development |
| [Akula](https://github.com/akula-bft/akula) | Rust | Akula Developers | Deprecated |
| [Aleth](https://github.com/ethereum/aleth) | C++ | Aleth Developers | Deprecated |
| [Mana](https://github.com/mana-ethereum/mana) | Elixir | Mana Developers | Deprecated |
| [OpenEthereum](https://github.com/openethereum/parity-ethereum) | Rust | Parity | Deprecated |
| [Trinity](https://github.com/ethereum/trinity) | Python | OpenEthereum | Deprecated |