[Application-specific rollup](#application-specific-rollup)

A rollup that supports only a certain type of application or functionality. For example, a rollup that supports only ETH and ERC20 token transfers, or DEX swaps. This is in contrast to general-purpose rollups which support arbitrary computations.

[Arithmetic circuit](#arithmetic-circuit)

The computation model of ZK-SNARKs.

[Blind signature](#blind-signature)

A type of digital signature where the signer signs a message without knowing its content, allowing the signer to remain anonymous.

[Blobs](#blobs)

The data that a rollup publishes to its L1/data availability (DA) layer. They consist of the L2 transactions that are rolled up, along with some metadata. Blobs are introduced as a new transaction type within Ethereum with EIP-4844, and has rollup scaling specifically in mind. Blobs persist on Ethereum’s Beacon Chain ephemerally.

[Blobspace](#blobspace)

The storage area within Ethereum’s Beacon nodes where blobs are published by rollups and ephemerally stored.

[Block](#block)

An ordered list of transactions and chain-related metadata that gets bundled together and published to the L1/DA layer. Nodes execute the transactions contained within blocks to change the rollup chain’s state. Protocol rules dictate what constitutes a valid block, and invalid blocks are skipped over.

[Bridge](#bridge)

A message-passing protocol between two blockchains. At its most basic, a token bridge consists of a smart contract which can escrow funds on one side of the bridge, and instruct the release or minting of corresponding assets on the other side, but bridges could also support arbitrary messages. How these instructions are validated is a critical factor in assessing the trust assumptions of a bridge.

[Bulletproofs](#bulletproofs)

A type of zero-knowledge proof that enables efficient range proofs and other mathematical proofs without requiring trusted setup.

[Bytecode-compatible (EVM-compatible) ZK-EVM](#bytecode-compatible-zk-evm)

A ZK-Rollup that can interpret and execute Ethereum bytecode, with some changes made to certain components of the EVM, such as different gas costs, or some unsupported features. (Type 2.5 and 3 in Vitalik's classification).

[Bytecode-equivalent ZK-EVM](#bytecode-equivalent-zk-evm)

A ZK-Rollup that can interpret and execute EVM bytecode. All EVM opcodes and pre-compiles are supported and provable through the ZK-EVM circuit, but gas costs for EVM opcodes might be changed. (Type 2.5 in Vitalik's classification).

[Bytecode](#bytecode)

Generally, an instruction set designed for efficient execution by a software interpreter or a virtual machine. Unlike human-readable source code, bytecode is expressed numerically. Within the context of rollups, often related to the concept of compatibility: whether the bytecode of programs on the rollup are capable of being run as-is on Ethereum as well, and vice versa.

[Canonically bridged token](#canonically-bridged-token)

Asset bridged from Ethereum L1 via a canonical (enshrined into the L2) bridge. These assets are originally minted on Ethereum, they are using Ethereum as a canonical source ledger and they are bridged using a lock-mint canonical bridge.

[Challenge period](#challenge-period)

In optimistic rollups, the window of time wherein network participants can assert that some fraud was included in a prior block. Most optimistic rollups currently specify a challenge window of 7 days. By extending the period, there is more time for participants to guard against fraud (invalid state transitions), but also more time until withdrawals gets enabled.

[Circuit](#circuit)

A program written for the purpose of being proven within a proving system. A circuit is a mathematical representation of the computation to be executed, and is therefore sometimes referred to as an arithmetic circuit. Circuits can be written in different languages, ranging from low-level to high-level.

[Client](#client)

Sometimes labelled interchangeably as a “node”, they are tasked with processing transactions and managing the blockchains's state. They run the computations for each transaction according to the rollup's virtual machine and protocol rules. If comparing to Ethereum clients, these would be execution clients such as Geth, as opposed to consensus clients.

[Commitment Scheme](#commitment-scheme)

A cryptographic primitive that (i) allows one (usually a prover) to commit to a chosen statement and send this commitment to the verifier while keeping it hidden to others, and (ii) allows verifier to check that some statement is consistent with such commitment. The commitment scheme should satisfy two properties: (i) it should be binding; that is, once the prover has committed she is bound to the message she has committed, (ii) it should be hiding; that is, the commitment reveals nothing new about the message to the verifier.

[Compatibility](#compatibility)

The degree to which a rollup can make use of existing patterns, code, and tooling from another blockchain or development environment. Today, this typically refers to how well rollups and developers thereon can make use of the Ethereum Virtual Machine, existing smart contracts, and other Ethereum infrastructure.

[Completeness](#completeness)

One of three properties of zero-knowledge proofs, completeness states that if a statement is true, an honest verifier will be convinced of this by an honest prover.

[Consensus](#consensus)

An agreement on the latest and correct state of a blockchain. Unlike L1 blockchains which coordinate participating nodes with consensus rules, rollups rely on L1s for reaching consensus by checking the state of the rollup smart contract deployed thereon.

[DA Bridge](#da-bridge)

System that verifies that data has been made available. It takes the form of a smart contract verifying a consensus or, if the data is verified directly by either downloading the full data or sampling, of an enshrined bridge.

[DA layer](#da-layer)

An infrastructure that is used to make publish data so that it's available to the public. They take the form of Data Availability Committees (DACs) or blockchains. Not to confuse with the layer responsible with ordering, since ordering and DA can be separated.

[Data Availability Committee (DAC)](#dac)

A set of members whose task is attesting and ensuring that the data is available for the public. An onchain DAC verifier checks that a threshold of signatures from the DAC members is reached before considering a data commitment as available and therefore valid to be used in the system.

[Danksharding](#danksharding)

A sharding design proposed for Ethereum. Instead of providing more space for transactions, Ethereum sharding provides more space for blobs of data. Verifying a blob simply requires checking that the blob is available - that it can be downloaded from the network. The data space in these blobs is expected to be used by rollup protocols. In comparison to other sharding mechanisms, where there are a fixed number of shards with distinct blocks and distinct block proposers, in Danksharding there is only one proposer that chooses all transactions and all data that go into that slot.

[Data availability](#data-availability)

The property of a rollup's data being reachable by any node retrieving the data that were rolled up and executed to reach the proposed state. Data availability (DA), specifically decoupling it from the rollup nodes themselves, is one of the preeminent factors which allows a rollup to scale securely. A rollup is faced with a decision of what to use as a DA layer to guarantee that any node can retrieve this data--permissionlessly under any circumstance. For this reason, using Ethereum for DA currently provides the strongest security guarantees. If data is stored somewhere other than a permissionless L1, then the project is not a rollup, but rather a validium or an optimium.

[Determinism](#determinism)

The concept of some function or process having a single outcome that is knowable by all participants. In the context of a rollup, it generally refers to the new state being calculable given a prior state and a list of transactions to be executed.

[Equivalence](#equivalence)

A perfect degree of compatibility; where one system or concept is indistinguishable from another in the domain being compared. In the context of rollups, it generally refers to the proximity to the EVM and to Ethereum architecture.

[Escape hatch](#escape-hatch)

The facility for any user of a rollup to exit the system with their assets under any circumstance. Most relevant in rollups with a centralized proposer, wherein users do not have the ability to propose blocks, but can nonetheless exit the rollup by interacting with a smart contract on L1.

[Ethereum-equivalent ZK-EVM](#ethereum-equivalent-zk-evm)

A ZK-Rollup that can prove the correctness of an actual Ethereum L1 block in its ZK-EVM circuit. No changes made to any part of Ethereum--EVM but also peripheral things like storage structures and hash functions. (Type 1 in Vitalik's classification).

[Ethereum Virtual Machine (EVM)](#ethereum-virtual machine)

A stack-based virtual machine that executes bytecode. In Ethereum, the execution model specifies how the system state is altered given a series of bytecode instructions and a small tuple of environmental data. In the context of rollups, the EVM is a choice of execution environment that rollups could implement, as in the case of EVM ZK-Rollups (ZK-EVMs) and EVM optimistic rollups.

[EVM-equivalent ZK-EVM](#evm-equivalent-zk-evm)

A ZK-Rollup that can interpret and execute EVM bytecode. All EVM opcodes and pre-compiles are supported and provable through the \[ZK-EVM\](#ZK-EVM) circuit. (Type 2 in Vitalik's classification).

[Execution environment](#execution-environment)

Refers to both the environment where transactions are processed, constituted by the execution clients, as well as the virtual machine model which the clients run.

[Exit window](#exit-window)

The amount of time that users have to exit a system before an unwanted upgrade. It takes into account upgrade delays, forced transaction delays and other time factors. To be considered Stage 1, a rollup needs to have an exit window of at least 7d if upgrades are initiated by a permissioned actor less decentralized than a Security Council. For Stage 2, a rollup needs at least 30d in all cases outside of onchain provable bugs.

[Externally bridged token](#externally-bridged-token)

Asset bridged to L2 via non-canonical bridge (lock-mint or burn-mint). These assets are either originally minted on Ethereum L1 but they are not using a canonical bridge or — more likely — they are originally minted on some other L1 (e.g. Polygon, Avalanche, BSC) and are bridged to L2 via non-canonical bridge.

[Fast Fourier Transform (FFT)](#fast-fourier-transform)

An algorithm that computes the Discrete Fourier Transform (DFT) of a sequence in a more efficient manner; that is, taking O(n \* log(n)) instead of O(n \* n). It is used for extremely fast multiplication of large numbers, multiplication of polynomials, and extremely fast generation and recovery of erasure codes.

[Fiat-Shamir heuristic](#fiat-shamir-heuristic)

A method of converting interactive proofs into non-interactive proofs, commonly used in zero-knowledge protocols.

[Finality](#finality)

Strongest confirmation rule that can be given on the ordering of transactions. On Ethereum, a transaction is finalized when the corresponding epoch becomes final, which currently takes around 15 mins from transaction inclusion. A rollup transaction can be said to be final when the corresponding data is published to L1 and its ordering cannot be reverted. If outputs, i.e. state diffs are published, then also a proof proving their correctness must be verified to consider the transaction final.

[Fraud proof](#fraud-proof)

Also referred to as a fault proof, it is the construction of an assertion that fraud was perpetrated on an optimistic rollup. More concretely, that an invalid state transition took place according to the protocol rules. The submitter of a fraud proof would expect a reward from the optimistic rollup protocol for helping maintain the integrity of the system.

[FRI](#fri)

A proximity test method that is used to determine whether a set of points is mostly on a polynomial with a degree less than a specified value. It resembles the FFT but the arithmetic complexity of its prover is strictly linear and that of the verifier is strictly logarithmic.

[Gas limit](#gas-limit)

The maximum amount of gas a transaction or block may consume.

[Gas price](#gas-price)

Price of one unit of gas specified in a transaction. The token used to pay for gas is usually Ether, but rollups can use other custom tokens.

[Gas](#gas)

A virtual fuel used to execute smart contracts on a rollup. The EVM (or other VM within the rollup) uses an accounting mechanism to correspond the consumption of gas to the consumption of computing resources, and to limit the consumption of computing resources.

[General-purpose rollup](#general-purpose-rollup)

A rollup that supports arbitrary computations, allowing for the development of arbitrary applications and functionalities.

[Geth (Go Ethereum)](#geth)

An Ethereum execution client, meaning it handles transactions, deployment and execution of smart contracts and contains an embedded computer known as the Ethereum Virtual Machine. Running Geth alongside a consensus client turns a computer into a full Ethereum node, or validator.

[Halo](#halo)

The first recursive proof composition without a trusted setup. A proof verifies the correctness of another instance of itself, meaning that the latest mathematical output (one single proof) contains within it a proof that all prior claims to the relevant secret knowledge have themselves been sufficiently proven through a similar process. It allows any amount of computational effort and data to produce a short proof that can be checked quickly.

[Hash](#hash)

A fixed-length fingerprint of variable-size input, produced by a hash function.

[Homomorphic encryption](#homomorphic-encryption)

A type of encryption that allows computations to be performed on encrypted data without decrypting it, making it useful for privacy-preserving computation.

[Interoperability](#interoperability)

Refers to the capability of different blockchain networks to communicate and share data. It enables the transfer of assets and information between blockchains, facilitating functionality and collaboration between blockchain ecosystems.

[Keccak](#keccak)

Cryptographic hash function used in Ethereum.

[KZG commitments](#kzg-commitments)

A polynomial commitment scheme that allows a prover to compute a commitment to a polynomial, with the properties that this commitment can later be opened at any position: the prover shows that the value of the polynomial at a certain position is equal to a claimed value. KZG is widely used as it’s applicable both for univariate and k-variate polynomials, is efficient for batch proofs, and is able to generate many proofs at once relatively fast. It is also proof generation time efficient: the time for prover to commit to a polynomial is linear on the degree of the polynomial.

[Lagrange Interpolation](#lagrange-interpolation)

A formula that helps to find a polynomial which takes on certain values at arbitrary points. Thanks to Lagrange Interpolation the time for a prover to commit to a polynomial in KZG is linear in the degree of the polynomial.

[Language-compatible ZK-EVM](#language-compatible-zk-evm)

A ZK-Rollup that can interpret and execute Solidity or other high-level-language source code. This code is compiled into bytecode that differs from that which the EVM runs. (Type 4 in Vitalik's classification).

[Layer 1](#layer-1)

Layer 1 (L1) is a blockchain that is self-reliant on its validator set for its security and consensus properties. Ethereum is an example of a layer 1. Blockchains started receiving the moniker of layer 1 once layer 2 became a meaningful area of development.

[Layer 2](#layer-2)

Layer 2 (L2) is a category of technical solutions aimed to scale the base layer in a trust minimized way. This category includes solutions like rollups as well as state channels and plasma. Other solutions are able to scale further, but with the introduction of additional trust assumptions, which are therefore not trust minimized. Sometimes the term Layer 2 is used to refer to include these solutions too, like validiums and optimiums, but to distinguish between trust minimized and non trust minimized solutions they are often referred to as "light" L2s, opposed to "strong" L2s like rollups.

[Layer 3](#layer-3)

Intuitively, L3s are projects that follow a similar structure to the L2 <> L1 structure but with an L2 underneath. It's not entirely defined yet if this implies that the token escrow must be on the L2, or the proof system. Also, certain cases like projects using aggregation layers make the distinction between L2 and L3 fuzzy as certain components start to look less like a traditional blockchain. How to properly define what is a layer in the first place is still an open question.

[Liquidity network](#liquidity-network)

A category of token bridge that makes use of liquidity providers on the destination chain to facilitate the transfer of tokens.

[Liveness](#liveness)

Liveness refers to the ability of a system to respond to requests and to process them in a timely manner. In the context of L2s, it refers to the ability of settling transactions, proofs and state roots to the base layer.

[Lookup table](#lookup-table)

Lookup tables express a relation between variables in the format of a table. A prover can rely on such a table of precomputed values in generating a proof without having to do bit by bit arithmetic. These tables can help handle hash functions within circuits in a more friendly manner (that is speeding up decryption and reducing memory requirements).

[Maximal Extractable Value (MEV)](#maximal-extractable value)

The maximum profit that can be extracted from block production by including, excluding, and reordering transactions in a block. In rollups, sequencers and provers are the network participants that receive rewards for building and proving blocks.

[Merkle proofs](#merkle-proofs)

Hashing the pairs of values at each level and climbing up the (Merkle) Tree until you obtain the root hash. Merkle proofs help check if the data belongs to a set without having to store the entire set.

[Merkle tree](#merkle-tree)

A hash-based data structure in which each leaf node is a hash of a block of data, and each non-leaf node is a hash of its children. The root of the tree is a cryptographic fingerprint of the entire data structure. Merkle trees (Merkle Patricia Tries) are used in Ethereum to efficiently store key-value pairs.

[Modular blockchains](#modular-blockchains)

A blockchain that fully outsources at least one of the 4 components (Consensus, Data Availability, Execution, Settlement) to an external chain. For example, rollup is a modular blockchain as it handles transaction executions off-chain and ‘outsources’ data availability and consensus to Ethereum.

[Multi-proof system](#multi-proof-system)

A rollup settlement concept that relies on a combination of multiple different proving systems. For example, a combination of fraud proof and validity proof. The goal is to reduce reliance on a single system-type or implementation. A more complex example of a multi-proof system: if anyone submits two conflicting state roots to a prover and both roots pass, that prover is turned off.

[Natively minted token](#natively-minted-token)

Asset that are natively minted on an L2, using either this L2 as their original ledger or using a “multi-ledger” approach with a burn-mint bridge.

[Network](#network)

A constellation of nodes (peers) that communicate via a peer-to-peer protocol, for example, in propagating transactions and blocks to other nodes.

[Node](#node)

A software client that participates in the network.

[One-time pad](#one-time-pad)

A type of encryption that uses a random key that is only used once to encrypt a message, providing perfect secrecy.

[Opcode](#opcode)

A binary instruction code that defines a specific operation or computational task that the EVM or other VMs can perform. In the context of rollups, opcode- and bytecode-level compatibility refers to the ability to execute smart contracts with or without modification.

[Operator](#operator)

An operator is the entity charged with managing a rollup and progressing its state. An rollup operator can be a centralized sequencer, proposer, prover, challenger, pauser of admin that is able to perform upgrades.

[Optimistic rollup](#optimistic-rollup)

A rollup that optimistically updates state with the possibility of fraud proofs being generated to revert faulty state roots.

[Optimium](#optimium)

An off-chain solution that uses fraud proofs for settlement and publishes the data offchain, therefore requiring an additional trust assumption.

[Permissionless](#permissionless)

Anyone willing should be able to join and leave the network at any time, without causing significant disturbance to the network or being detrimental to the party in question. No single entity should have the power to allowlist or blocklist participants.

[Plasma](#plasma)

Plasma is an offchain scaling solution that is able to support offchain data availability by allowing users to exit even when the data is unavailable. Usually, though, Plasma projects require users to frequently monitor the chain (e.g. at least once per week) to ensure that they can exit in time if necessary. There are attempts to support general smart contracts, but the exit mechanism is often limited to simpler applications.

[Polynomial commitment](#polynomial-commitment)

A commitment scheme that commits to a polynomial.

[Pre-compiles](#pre-compiles)

Special contracts that include complex computations, but do not require the EVM overhead. For example, hashing operations and signature schemes.

[Proof system](#proof-system)

The infrastructure that allows projects to verify their state transitions. It is composed by onchain verifiers and offchain provers. The main two flavors are optimistic and ZK proof systems, but they can be combined in a hybrid model. In general though, if a system is able to accept state roots optimistically, even if it has a ZK component, it is considered an optimistic proof system.

[Proposer](#proposer)

In the context of L2s, the actor that proposes a claimed state root on L1. The term is also used in the context of Ethereum to refer to the actor that proposes a new block.

[Proto-danksharding (EIP-4844)](#proto-danksharding)

The proposal that implemented most of the logic and architecture that make up a full Danksharding spec (eg. transaction formats, verification rules), but not yet actually implementing any sharding (i.e. data availability sampling). In a proto-danksharding implementation, all validators and users still have to directly validate the availability of the full data. The main difference with calldata is that blobs do not interact with the execution layer and are deleted after a certain period of time.

[Prover](#prover)

An entity that generates the cryptographic proof to convince the verifier that the statement is true. In a ZK-Rollup, the prover generates the ZK (validity) proof to submit to the verifier contract.

[Random oracle](#random-oracle)

A theoretical cryptographic primitive that is used to model the behavior of hash functions in security proofs.

[Rollup-as-a-service](#rollup-as-a-service)

An SDK or service that allows anyone to launch rollups quickly. Emphasis may be placed on the ability to customize the modular components of a rollup: VM, DA layer, proof system.

[Rollup](#rollup)

A blockchain that inherits consensus and data availability from another blockchain called L1. Rollups enable trust minimized bridges with the base layer via proof systems, either optimistic or zero-knowledge. A rollup without a bridge, or without considering the bridge, is called a sovereign rollup.

[RPC (Remote Procedure Call)](#rpc)

A protocol that a program uses to request a service from a program located on another computer in a network without having to understand the network details.

[Scalability](#scalability)

Scalability refers to the ability of a system to handle a growing amount of transactions or its potential to accommodate them. In the context of decentralized blockchains like Ethereum, scalability solutions only make sense when they are able to do so in a trust minimized manner and while maintaining decentralization.

[Schnorr signature](#schnorr-signature)

A type of digital signature that is more efficient than traditional digital signatures and can be used in conjunction with threshold signatures to enable multisignature schemes.

[Secure multiparty computation](#secure-multiparty-computation)

A type of computation where multiple parties jointly compute a function without revealing their inputs to each other, allowing for privacy-preserving computation.

[Security Council](#security-council)

A Security Council is a sufficiently decentralized set of members that is able to upgrade a system. A properly set up Security Council consists of at least 8 members with a threshold greater than 75%. What 'sufficiently decentralized' means is fundamentally subjective and L2BEAT evaluates each case individually. A Security Council is allowed to instantly upgrade Stage 1 rollups.

[Sequencer](#sequencer)

A party responsible for ordering and executing transactions on the rollup. The sequencer verifies transactions, compresses the data into a block, and submits the data related to it to enable state reconstruction to Ethereum L1 as a single transaction. The data can be either transaction data or state diffs.

[Settlement](#settlement)

The mechanism with which the execution of rollup blocks and the resultant state is verified and possible disputes are resolved. In the context of rollups or other modular blockchains, it often refers to the proof system used--validity (ZK) or fraud proofs, or a combination thereof. Sometimes it will refer to this mechanism along with where the mechanism's outputs are ultimately published and verified, as in Ethereum being a settlement layer by verifying the proofs and allowing for withdrawals.

[SNARK](#snark)

Short for "succinct non-interactive argument of knowledge", a SNARK is a widely used type of zero-knowledge proof that is short and fast to verify. Different kinds of SNARKs are usually systematized by proof size, verification time, and type of setup. The most famous SNARKs are Groth16, PLONK/Marlin, Bulletproofs, and STARKs.

[STARK](#stark)

Short for "scalable transparent argument of knowledge", a STARK is a type of zero-knowledge proof that resolves one of the primary weaknesses of ZK-SNARKs, its reliance on a "trusted setup”. STARKs also come with much simpler cryptographic assumptions, avoiding the need for elliptic curves, pairings, and the knowledge-of-exponent assumption and instead relying purely on hashes and information theory. This means that they are secure even against attackers with quantum computers.

[State root](#state-root)

A cryptographic hash succinctly representing a state using a Merkle tree.

[State update](#state-update)

A mechanism that allows to update the claimed state of a project. It usually involves verifying a state transition proof, but it can also be done in a trusted manner by a permissioned actor.

[Succinctness](#succinctness)

A property of ZKP that stands for the following terms: (i) the proof of statement is shorter than the statement itself, (ii) the time to verify the proof is faster than just to evaluate the function from scratch.

[Threshold cryptography](#threshold-cryptography)

A type of cryptography where multiple parties jointly hold a secret key and must cooperate to perform cryptographic operations, enabling more secure and decentralized system.

[Time Delay](#time-delay)

In regards to upgradeability, a predefined amount of time that must elapse before the rollup smart contracts or parameters are updated. This protects users from malicious upgrades by giving them time to exit the rollup before upgrades come into effect. Note that time delays only make sense if users' exits cannot be censored.

[Time to inclusion](#time-to-inclusion)

Time between L2 (or L3) tx submission and inclusion in an L1 block such that it can be reorged only if the L1 reorgs. If the project operators can reorg transactions even after tx data submission on L1, the transaction is not yet considered included in the canonical L2 (or L3) chain.

[Trusted setup (ceremony)](#trusted-setup)

Generation of a piece of data that must then be used for some cryptographic protocol to run. Generating this data requires some secret information. The "trust" comes from the fact that a person generates a secret, uses it to generate the data, and then publishes the data and forgets the secret. Once the data is generated, and the secrets are forgotten, no further participation from the creators of the ceremony is required. There are two types of trusted setup: (i) trusted setup per circuit where it is generated from scratch for each circuit, (ii) trusted universal (updatable) setup where it can be used for as many circuits as we want.

[TVL](#tvl)

We define TVL to be the total amount of value that can be accessed on a project. Currently, L2BEAT only tracks fungible assets, but it could be extended in the future. Currently, TVL is calculated as the sum of canonical bridged tokens, externally bridged tokens and native tokens.

[Upgradeability](#upgradeability)

The ability for rollup smart contracts and parameters used in a rollup to be updated by holders of an admin key. Upgradeability represents a vector of risk for users, and should be decentralized and combined with time delays for greater security guarantees.

[Validator](#validator)

In the context of L2s, a Validator is an actor that validates the correctness of state transitions. For optimistic rollups this corresponds to challengers, and for ZK rollups this corresponds to the onchain verifier

[Validity proof](#validity-proof)

The output of a cryptographic proving system attesting to correct computation. ZK-Rollups use succinct validity proofs (also called zero-knowledge proofs) to prove a batch of rollup transactions and blocks were properly executed. Validity proofs are submitted to a verifier, such as an Ethereum smart contract, which accepts them if properly constructed.

[Validium](#validium)

An off-chain solution that uses validity proofs for settlement and publishes the data offchain, therefore requiring an additional trust assumption.

[Verifier](#verifier)

An entity in a ZK-Rollup, often a smart contract, that verifies zero-knowledge proofs submitted by a prover.

[Verkle tree](#verkle-tree)

A data storage format of which you can make a proof that it contains some pieces of data to anyone who has the root of the tree. While similar to Merkle Patricia Trees, key differences include a much wider tree format which leads to smaller proof sizes.

[Volition](#volition)

A hybrid data availability mode, where the user can choose whether to place data on-chain or off-chain.

[Zero-knowledge](#zero-knowledge)

A cryptographic technology and sub-discipline of cryptography that allows an individual to prove that a statement or computation is true without revealing any additional information.

[ZK-Rollup](#zk-rollup)

A rollup that uses ZKPs (also often called validity proofs) to validate the correctness of the state transition function and update the rollup state. This is one of two main types of rollup constructions, along with optimistic rollups.