1. **Question:** What is the role of the `Application` interface in Cosmos SDK, and how does it interact with Tendermint?
   **Answer:** The `Application` interface defines the state machine for the blockchain, handling transactions via `CheckTx`, `DeliverTx`, and `Commit`. It interacts with Tendermint by receiving blocks and transactions through ABCI (Application Blockchain Interface), processing them, and responding with results to maintain consensus.

2. **Question:** Explain the difference between baseapp and a custom app in Cosmos SDK.
   **Answer:** Baseapp provides a foundational router for handling queries and transactions, including middleware like authentication. A custom app extends baseapp by integrating specific modules, keepers, and custom logic tailored to the chain's requirements.

3. **Question:** How does Cosmos SDK handle state transitions during block processing?
   **Answer:** State transitions occur in `DeliverTx` where transactions are executed atomically; if any fails, the block aborts. Use `MultiStore` for persistent state and `TransientStore` for temporary data, ensuring rollback on errors.

4. **Question:** What are keepers in Cosmos SDK, and why are they important?
   **Answer:** Keepers are module-specific objects that manage state access and modifications, enforcing encapsulation and modularity. They prevent direct store access, promoting secure and organized code.

5. **Question:** Describe the structure of a Cosmos SDK module and its key components.
   **Answer:** A module includes `AppModule` for lifecycle hooks, `Keeper` for state management, `Msg` types for transactions, `Query` services, and CLI/REST handlers. Implement `genesis` for initialization.

6. **Question:** How do you implement custom message types in a Cosmos SDK module?
   **Answer:** Define a struct implementing `sdk.Msg` with `Route`, `Type`, `Signers`, `GetSignBytes`, and `ValidateBasic`. Register it in the module's router and handler.

7. **Question:** What is the purpose of the `codec` in Cosmos SDK?
   **Answer:** The codec (e.g., Amino or Protobuf) serializes/deserializes data for storage, transactions, and communication. Use `MakeCodec` to register types and ensure compatibility.

8. **Question:** Explain how queries are handled in Cosmos SDK.
   **Answer:** Queries use gRPC or legacy ABCI, routed via `baseapp` to module queriers. Implement `Querier` functions that fetch data from stores without mutating state.

9. **Question:** What is the role of the `Context` object in transaction processing?
   **Answer:** `Context` provides block height, chain ID, gas meter, and store access. Pass it to keepers for contextual execution, and use `WithGasMeter` for metering.

10. **Question:** How does Cosmos SDK support multi-denomination assets?
    **Answer:** Use the `bank` module's `Keeper` to manage `sdk.Coins` with denominations. Implement custom minting/burning logic in modules like `x/mint`.

11. **Question:** Describe the IBC protocol and its layers in Cosmos SDK.
    **Answer:** IBC enables cross-chain communication via channels and packets. Layers include transport (TAO), light clients, and applications (ICS). Implement via `ibc` module.

12. **Question:** How do you establish an IBC connection between two chains?
    **Answer:** Use `MsgConnectionOpenInit`, `MsgConnectionOpenTry`, `MsgConnectionOpenAck`, and `MsgConnectionOpenConfirm` transactions, verifying client states and consensus.

13. **Question:** What is a relayer's role in IBC?
    **Answer:** Relayers monitor chains, relay headers for light client updates, and forward packets. Implement using Hermes or custom Go relayer for off-chain operation.

14. **Question:** Explain packet lifecycle in IBC.
    **Answer:** Packets are sent via `SendPacket`, relayed, received with `RecvPacket`, acknowledged with `AcknowledgePacket`, and timed out with `TimeoutPacket` if expired.

15. **Question:** How does IBC handle versioning?
    **Answer:** Connections and channels negotiate versions during handshake. Use `Version` fields in messages and implement upgrade logic for backward compatibility.

16. **Question:** What are ICS (Interchain Standards) and give an example.
    **Answer:** ICS define standardized IBC applications, e.g., ICS-20 for fungible token transfers. Implement by creating modules that handle specific packet data.

17. **Question:** How do you implement a custom IBC module?
    **Answer:** Extend `ibc_porttypes.IBCModule` with handlers for `OnChanOpenInit`, `OnRecvPacket`, etc. Bind ports and register with the IBC router.

18. **Question:** Describe client states in IBC.
    **Answer:** Client states track the consensus state of the counterparty chain. Use Tendermint clients for header verification and freezing on misbehavior.

19. **Question:** What is the purpose of proofs in IBC?
    **Answer:** Proofs (Merkle proofs) verify state from another chain without full sync. Include in relay messages to ensure trustless communication.

20. **Question:** How does IBC handle timeouts and ordering?
    **Answer:** Channels can be ordered (sequential) or unordered. Timeouts use `TimeoutHeight` or `TimeoutTimestamp`; relay before expiry or process timeout.

21. **Question:** Explain Tendermint's BFT consensus mechanism.
    **Answer:** Tendermint achieves 1/3 fault tolerance via rounds of propose, prevote, precommit. Validators vote with stake; finalize on +2/3 precommits.

22. **Question:** What is the role of the mempool in Tendermint?
    **Answer:** Mempool stores pending transactions, checks validity via `CheckTx`, and prioritizes by gas/fee. Customize with `MempoolOption` for application needs.

23. **Question:** How does Tendermint handle block proposal?
    **Answer:** Proposers are selected round-robin by stake. They broadcast proposals; validators vote. Use `Propose` timeout for liveness.

24. **Question:** Describe the validator set updates in Tendermint.
    **Answer:** Validator sets change at epoch boundaries via `EndBlock` ABCI, updating power and addresses. Tendermint applies in the next block.

25. **Question:** What is evidence handling in Tendermint?
    **Answer:** Evidence of Byzantine behavior (e.g., double signing) is submitted via `Evidence` pool, verified, and punished by slashing in `DeliverTx`.

26. **Question:** How do you configure Tendermint for a custom chain?
    **Answer:** Edit `config.toml` for P2P, RPC, consensus params like `timeout_commit`. Integrate with Cosmos SDK app via ABCI.

27. **Question:** Explain fast sync vs. full sync in Tendermint.
    **Answer:** Fast sync downloads blocks without execution, verifying headers. Full sync executes all transactions. Use `fast_sync = true` for quicker bootstrapping.

28. **Question:** What are Tendermint's light clients?
    **Answer:** Light clients verify headers with minimal resources, using sequential or skipping verification. Useful for IBC and mobile apps.

29. **Question:** How does Tendermint ensure liveness?
    **Answer:** Through timeouts (propose, prevote, precommit) and +2/3 voting thresholds. If stalled, increment round and select new proposer.

30. **Question:** Describe the ABCI interface methods.
    **Answer:** Key methods: `CheckTx` for validation, `BeginBlock/DeliverTx/EndBlock` for execution, `Commit` for persistence, `Query` for reads.

31. **Question:** How is Golang used in Cosmos SDK development?
    **Answer:** Cosmos SDK is written in Go; use interfaces for modularity, goroutines for concurrency in relayers, and error handling with `sdkerrors`.

32. **Question:** Explain dependency injection in Cosmos SDK.
    **Answer:** Use `AppModule` to provide keepers via constructors, injecting dependencies like stores and other keepers during app initialization.

33. **Question:** What are some Go best practices for Cosmos modules?
    **Answer:** Use immutable structs, avoid global state, employ context for cancellation, and write unit tests with `testify` and mock keepers.

34. **Question:** How do you handle concurrency in Cosmos SDK?
    **Answer:** Transactions are processed sequentially in blocks, but use mutexes in keepers for shared state. For queries, ensure read-only access.

35. **Question:** Describe error handling in Cosmos SDK.
    **Answer:** Wrap errors with `sdkerrors.Register` and `sdkerrors.Wrap`. Use `ABCIErrors` for ABCI responses, propagating stack traces.

36. **Question:** How to optimize Go code for gas efficiency in Cosmos?
    **Answer:** Minimize allocations, use pools for objects, profile with `pprof`, and meter operations with `GasMeter`.

37. **Question:** What is the role of interfaces in Cosmos SDK?
    **Answer:** Interfaces like `Keeper`, `Handler` enable decoupling and testing. Implement multiple for different modules.

38. **Question:** How do you use Protobuf in Cosmos SDK?
    **Answer:** Define messages with `.proto` files, generate Go code with `protoc`, and register with codec for serialization.

39. **Question:** Explain testing strategies for Go in Cosmos.
    **Answer:** Use unit tests for keepers, integration tests with `simapp`, and fuzzing for robustness. Mock dependencies with `gomock`.

40. **Question:** How to handle large data in Go for Cosmos?
    **Answer:** Use streams for I/O, paginate queries, and compress with `gzip` if needed, mindful of gas costs.

41. **Question:** What is staking in Cosmos SDK?
    **Answer:** Staking locks tokens to validators for security. Use `x/staking` module for delegation, rewards, and unbonding.

42. **Question:** Explain the slashing mechanism.
    **Answer:** Slashing penalizes validators for downtime or double-signing by burning stake. Configure params like `slash_fraction_downtime`.

43. **Question:** How does governance work in Cosmos?
    **Answer:** Proposals are submitted, voted on by stakers, and executed if passed. Use `x/gov` for text, param change, or software upgrades.

44. **Question:** Describe delegation and redelegation.
    **Answer:** Delegation assigns tokens to validators; redelegation switches without unbonding wait. Limit redelegations to prevent abuse.

45. **Question:** What are rewards distribution in Cosmos?
    **Answer:** Rewards from inflation and fees are distributed proportionally to staked amounts. Use `DistributionKeeper` for claims.

46. **Question:** How to implement custom governance proposals?
    **Answer:** Define a new `ProposalContent` type, implement `ProposalHandler`, and register with gov router.

47. **Question:** Explain unbonding periods.
    **Answer:** Unbonding locks tokens for a period (e.g., 21 days) to prevent attacks. Configure in `staking.Params`.

48. **Question:** What is the role of validators in governance?
    **Answer:** Validators vote with their stake; delegators can override. Quorum and threshold params control passage.

49. **Question:** How does Cosmos handle inflation?
    **Answer:** Inflation mints new tokens via `x/mint`, distributed as rewards. Adjust rate based on bonded ratio.

50. **Question:** Describe jail and unjail mechanisms.
    **Answer:** Jailing removes validators for offenses; unjail after self-delegation and message. Prevents repeated faults.

51. **Question:** What is a chain upgrade in Cosmos SDK?
    **Answer:** Upgrades apply code changes at a specific height via gov proposal. Use `x/upgrade` for planning and halting.

52. **Question:** How do you plan a software upgrade?
    **Answer:** Submit `SoftwareUpgradeProposal` with height, name, and info. Nodes halt at height if not upgraded.

53. **Question:** Explain module migrations.
    **Answer:** Use `Migrator` in modules to transform state during upgrades, registering handlers for version jumps.

54. **Question:** What is the downgrade process?
    **Answer:** Downgrades are rare; use `DowngradeVerification` to prevent accidental runs on higher states.

55. **Question:** How to test chain upgrades?
    **Answer:** Use `upgrades` package in tests, simulating halt and restart with new binary.

56. **Question:** Describe genesis migrations.
    **Answer:** Export genesis, modify, and import with migration scripts for initial state adjustments.

57. **Question:** What are version maps in upgrades?
    **Answer:** Version maps track module versions; during upgrade, run migrations to target version.

58. **Question:** How does Cosmos handle fork upgrades?
    **Answer:** Soft forks via gov; hard forks require coord off-chain, with new genesis.

59. **Question:** Explain upgrade handlers.
    **Answer:** Register `UpgradeHandler` funcs that execute during `ApplyUpgrade`, performing state changes.

60. **Question:** What is the role of signals in upgrades?
    **Answer:** Nodes signal readiness via `Signal` messages; not mandatory but useful for coordination.

61. **Question:** How to optimize block processing time in Cosmos?
    **Answer:** Parallelize non-conflicting txs if possible, index stores efficiently, and reduce I/O with caching.

62. **Question:** Explain gas metering in Cosmos SDK.
    **Answer:** Gas tracks computation; define costs in handlers, use `InfiniteGasMeter` for genesis, enforce limits to prevent DoS.

63. **Question:** What are performance bottlenecks in Cosmos?
    **Answer:** State access, crypto ops, and consensus. Profile with `trace` and optimize hot paths.

64. **Question:** How to scale Cosmos chains?
    **Answer:** Use IBC for sharding, optimize modules, or migrate to faster consensus like Tendermint++.

65. **Question:** Describe indexing in Cosmos SDK.
    **Answer:** Use `x/tx` for transaction indexing by tags; enable in config for faster queries.

66. **Question:** How to handle high throughput?
    **Answer:** Increase block size/gas limit, use mempool prioritization, and hardware optimizations.

67. **Question:** What is state pruning?
    **Answer:** Prune old states with `pruning` config (e.g., keep_last=100) to reduce storage.

68. **Question:** Explain caching in keepers.
    **Answer:** Use `CacheContext` for batched reads/writes, committing only on success.

69. **Question:** How to monitor performance?
    **Answer:** Use Prometheus metrics from SDK, tracking gas, block time, and tx throughput.

70. **Question:** Describe optimization for IBC.
    **Answer:** Batch relays, use efficient light clients, and minimize proof sizes.

71. **Question:** What are common security vulnerabilities in Cosmos SDK?
    **Answer:** Reentrancy in handlers, improper auth, and oracle attacks. Audit with formal verification.

72. **Question:** How to secure private keys in validators?
    **Answer:** Use HSMs or TMKMS for signing, avoid exposing in config.

73. **Question:** Explain DDoS protection in Cosmos.
    **Answer:** Rate limit RPC/P2P, use sentries, and configure mempool to drop low-fee txs.

74. **Question:** What is misbehavior detection?
    **Answer:** Detect via evidence in Tendermint, slash in app layer for double-sign or downtime.

75. **Question:** How to audit Cosmos modules?
    **Answer:** Use static analysis (golint), fuzzing, and third-party audits focusing on economic attacks.

76. **Question:** Describe secure upgrade processes.
    **Answer:** Verify binaries, use multi-sig gov, and test on testnets before mainnet.

77. **Question:** What are economic security considerations?
    **Answer:** Design tokenomics to incentivize honesty, with slashing and long unbonding.

78. **Question:** How to prevent front-running?
    **Answer:** Use commit-reveal schemes or order txs by nonce/time.

79. **Question:** Explain secure IBC implementations.
    **Answer:** Validate proofs strictly, freeze clients on invalid headers, and rate-limit channels.

80. **Question:** What is role-based access in modules?
    **Answer:** Use auth via signers and capabilities for fine-grained control.

81. **Question:** How to write unit tests for Cosmos modules?
    **Answer:** Use `testing.T`, mock keepers with interfaces, and test invariants.

82. **Question:** Explain integration testing in Cosmos.
    **Answer:** Use `simapp` to simulate full app, submit txs, and query states.

83. **Question:** What are invariants in Cosmos SDK?
    **Answer:** Invariants are checks run in `EndBlock` to detect state corruption, panicking on failure.

84. **Question:** How to debug Cosmos nodes?
    **Answer:** Use logs with `log_level`, delve for Go debugging, and trace ABCI calls.

85. **Question:** Describe e2e testing for chains.
    **Answer:** Spin up local nets with starport or igniter, automate txs and assertions.

86. **Question:** What is fuzz testing in Cosmos?
    **Answer:** Use go-fuzz on msg validation and handlers to find edge cases.

87. **Question:** How to test IBC interactions?
    **Answer:** Use two local chains and a relayer, verifying packet flows.

88. **Question:** Explain benchmarking in Cosmos.
    **Answer:** Use `go test -bench` on critical paths, measuring TPS and latency.

89. **Question:** What are mock clients for testing?
    **Answer:** Implement mock IBC clients to simulate counterparty without full chains.

90. **Question:** How to handle testnet deployments?
    **Answer:** Use genesis coordination, faucets, and monitoring for stability.

91. **Question:** What is the crisis module in Cosmos SDK?
    **Answer:** Crisis halts chain on invariant breaks, requiring gov to resume after fix.

92. **Question:** Explain the auth module.
    **Answer:** Auth handles accounts, signers, and tx authentication. Use `StdTx` or `AuthTx`.

93. **Question:** How does the bank module work?
    **Answer:** Bank manages coin transfers, balances, and vesting. Use `SendCoins` safely.

94. **Question:** Describe the distribution module.
    **Answer:** Distribution allocates rewards from fees/inflation to validators/delegators.

95. **Question:** What is the evidence module?
    **Answer:** Evidence collects and handles Byzantine proofs for slashing.

96. **Question:** Explain the mint module.
    **Answer:** Mint controls inflation, minting tokens based on params and bonding.

97. **Question:** How does the params module function?
    **Answer:** Params stores subspace params, changeable via gov proposals.

98. **Question:** Describe the slashing module.
    **Answer:** Slashing applies penalties for infractions, tracking history.

99. **Question:** What is the upgrade module?
    **Answer:** Upgrade coordinates chain halts and signals for software updates.

100. **Question:** Explain the vesting module.
    **Answer:** Vesting locks coins with release schedules, used for grants.

101. **Question:** How to integrate CosmWasm with Cosmos SDK?
    **Answer:** Use `x/wasm` module for smart contracts, registering keepers and handlers.

102. **Question:** What are capabilities in Cosmos SDK?
    **Answer:** Capabilities are keys for secure object access, used in IBC and auth.

103. **Question:** Explain ADR (Architecture Decision Records) in Cosmos.
    **Answer:** ADRs document design choices; follow for custom modules.

104. **Question:** How does Cosmos handle oracles?
    **Answer:** Use custom modules or IBC to feed external data securely.

105. **Question:** Describe NFT module in Cosmos.
    **Answer:** `x/nft` manages non-fungible tokens with ownership and transfers.

106. **Question:** What is the group module?
    **Answer:** Group enables multi-sig like decisions with weighted voting.

107. **Question:** Explain feegrant module.
    **Answer:** Feegrant allows accounts to pay fees for others, with allowances.

108. **Question:** How does authz module work?
    **Answer:** Authz grants authorizations for actions on behalf of others.

109. **Question:** What is consensus params?
    **Answer:** Params like block size, configurable via gov for Tendermint.

110. **Question:** Describe historical headers.
    **Answer:** Store past headers for light client verification in IBC.

111. **Question:** How to implement custom CLI commands?
    **Answer:** Use Cobra in module, adding subcommands for txs and queries.

112. **Question:** Explain REST API in Cosmos.
    **Answer:** Use `lcd` for legacy REST, or gRPC-gateway for modern APIs.

113. **Question:** What are events in Cosmos SDK?
    **Answer:** Events are emitted in txs for indexing and notifications, e.g., `ctx.EventManager().EmitEvent`.

114. **Question:** How to use telemetry?
    **Answer:** Enable metrics collection for monitoring with Prometheus.

115. **Question:** Describe snapshotting.
    **Answer:** Snapshots allow fast state sync for new nodes, configured in app.

116. **Question:** What is state sync?
    **Answer:** State sync downloads recent state instead of replaying history.

117. **Question:** Explain P2P configuration.
    **Answer:** Configure seeds, persistent peers, and max connections in config.

118. **Question:** How does Cosmos handle privacy?
    **Answer:** Use zk-proofs in custom modules or shielded pools.

119. **Question:** What are rollups in Cosmos context?
    **Answer:** Use Optimint or custom for rollups, settling on Cosmos chain.

120. **Question:** Describe sovereign chains.
    **Answer:** Sovereign chains are independent, using Cosmos SDK without hub dependency.

121. **Question:** How to fork Cosmos SDK?
    **Answer:** Clone repo, modify modules, and build custom binary.

122. **Question:** Explain SDK versioning.
    **Answer:** Use semantic versioning; pin dependencies with go.mod.

123. **Question:** What is the role of starport?
    **Answer:** Starport scaffolds new chains and modules quickly.

124. **Question:** How to contribute to Cosmos SDK?
    **Answer:** Follow CONTRIBUTING.md, submit PRs with tests and docs.

125. **Question:** Describe Cosmos Hub specifics.
    **Answer:** Hub is the central chain for IBC, with ATOM token and governance.

126. **Question:** What is Gravity Bridge?
    **Answer:** Bridge for Ethereum-Cosmos transfers using IBC and peg zones.

127. **Question:** Explain Peg Zones.
    **Answer:** Peg zones lock assets on one chain, mint representations on another.

128. **Question:** How does Cosmos integrate with Ethereum?
    **Answer:** Use Ethermint for EVM compatibility or IBC with adapters.

129. **Question:** What is Celestia integration?
    **Answer:** Use for data availability layer, separating execution.

130. **Question:** Describe Rollkit.
    **Answer:** Framework for sovereign rollups on Cosmos SDK.

131. **Question:** How to handle chain halts?
    **Answer:** Use gov to propose recovery, coordinate off-chain.

132. **Question:** Explain long-range attacks.
    **Answer:** Prevent with social consensus and checkpointing.

133. **Question:** What are eclipse attacks?
    **Answer:** Mitigate with diverse peers and sentry nodes.

134. **Question:** How to secure RPC endpoints?
    **Answer:** Use TLS, auth, and firewall restrictions.

135. **Question:** Describe sybil resistance.
    **Answer:** Stake weighting prevents sybil via economic cost.

136. **Question:** What is the role of VRF in Cosmos?
    **Answer:** Use for random proposer selection in future upgrades.

137. **Question:** Explain ABCI++.
    **Answer:** Enhanced ABCI with `ProcessProposal` and `VerifyVoteExtension`.

138. **Question:** How does Cosmos handle MEV?
    **Answer:** Use auction mechanisms or fair ordering in mempool.

139. **Question:** What are threshold signatures?
    **Answer:** Use for multi-validator setups to aggregate signs.

140. **Question:** Describe interchain accounts.
    **Answer:** ICS-27 allows controlling accounts cross-chain via IBC.

141. **Question:** How to implement ICS-20?
    **Answer:** Handle fungible token packets with denom tracing.

142. **Question:** What is ICS-721?
    **Answer:** Standard for NFT transfers over IBC.

143. **Question:** Explain packet forwarding middleware.
    **Answer:** Middleware to route packets through multiple hops.

144. **Question:** How does IBC handle fees?
    **Answer:** Use ICS-29 for relayer incentives with fees on packets.

145. **Question:** What are client upgrades in IBC?
    **Answer:** Upgrade clients on-chain without breaking connections.

146. **Question:** Describe Hermes relayer.
    **Answer:** Rust-based relayer for IBC, configurable via TOML.

147. **Question:** How to debug IBC issues?
    **Answer:** Check client states, proofs, and relayer logs.

148. **Question:** What is the solo machine client?
    **Answer:** IBC client for single-signer chains like wallets.

149. **Question:** Explain beefy client.
    **Answer:** Light client for Substrate in Cosmos IBC.

150. **Question:** How does Cosmos support DeFi?
    **Answer:** Build modules for lending, DEX like Osmosis.

151. **Question:** What is the oracle module?
    **Answer:** Custom module for feeding prices, e.g., BandChain integration.

152. **Question:** Describe DAO on Cosmos.
    **Answer:** Use gov or group module for on-chain DAOs.

153. **Question:** How to implement bridges?
    **Answer:** Use IBC or custom pegs with multi-sig oracles.

154. **Question:** What are zones in Cosmos?
    **Answer:** Independent chains connected via hub.

155. **Question:** Explain consumer chains.
    **Answer:** Chains securing via interchain security from providers.

156. **Question:** How does interchain security work?
    **Answer:** Validators from provider stake on consumer, sharing security.

157. **Question:** What is replicated security?
    **Answer:** Validators run multiple chains with shared stake.

158. **Question:** Describe mesh security.
    **Answer:** Cross-staking between chains for mutual security.

159. **Question:** How to optimize validator nodes?
    **Answer:** Use SSDs, high RAM, and prune states regularly.

160. **Question:** What are sentry nodes?
    **Answer:** Proxy nodes protecting validators from DDoS.

161. **Question:** Explain seed nodes.
    **Answer:** Bootstrap peers for new nodes to join network.

162. **Question:** How to manage keys?
    **Answer:** Use keyring backends like os, file, or ledger.

163. **Question:** What is the ledger integration?
    **Answer:** Hardware wallet support for signing.

164. **Question:** Describe multisig accounts.
    **Answer:** Accounts requiring k-of-n signatures for txs.

165. **Question:** How to use vesting accounts?
    **Answer:** Create with cliffs and periods for gradual release.

166. **Question:** What are module accounts?
    **Answer:** Autonomous accounts for modules, no keys needed.

167. **Question:** Explain base accounts.
    **Answer:** Standard accounts with pubkey and sequence.

168. **Question:** How does Cosmos handle HD wallets?
    **Answer:** Use BIP44 paths for derivation.

169. **Question:** What is the address format?
    **Answer:** Bech32 with prefixes like cosmos1.

170. **Question:** Describe signature schemes.
    **Answer:** Support secp256k1 and ed25519.

171. **Question:** How to implement custom auth?
    **Answer:** Extend `AnteHandler` for validation.

172. **Question:** What are decorators?
    **Answer:** Ante decorators for gas, sig verify, etc.

173. **Question:** Explain tx broadcasting.
    **Answer:** Use RPC `broadcast_tx_sync` or async.

174. **Question:** How to query txs?
    **Answer:** Use `/txs` endpoint with hashes or tags.

175. **Question:** What is websocket subscription?
    **Answer:** Subscribe to events like new blocks or txs.

176. **Question:** Describe gRPC in Cosmos.
    **Answer:** Modern query/tx interface, faster than REST.

177. **Question:** How to enable CORS?
    **Answer:** Configure in app.toml for API access.

178. **Question:** What is the halt height?
    **Answer:** Config to stop node at specific height.

179. **Question:** Explain min gas prices.
    **Answer:** Validator-set min fees to accept txs.

180. **Question:** How does pruning work?
    **Answer:** Options: default, nothing, everything, custom.

181. **Question:** What are IAVL trees?
    **Answer:** Immutable AVL for state storage, versioned.

182. **Question:** Describe rocksdb vs. goleveldb.
    **Answer:** Rocksdb faster for large data, configurable.

183. **Question:** How to backup states?
    **Answer:** Export genesis or use snapshots.

184. **Question:** What is the app hash?
    **Answer:** Merkle root of state after commit.

185. **Question:** Explain validator pubkeys.
    **Answer:** Tendermint uses ed25519 for signing.

186. **Question:** How to change validator keys?
    **Answer:** Edit priv_validator_key.json carefully.

187. **Question:** What is double sign protection?
    **Answer:** Use persistent state to prevent old signs.

188. **Question:** Describe remote signer.
    **Answer:** TMKMS for secure external signing.

189. **Question:** How to monitor validators?
    **Answer:** Use tenderduty or cosmosvisor.

190. **Question:** What is cosmosvisor?
    **Answer:** Tool for auto-upgrades without downtime.

191. **Question:** Explain chain ids.
    **Answer:** Unique identifiers to prevent replay attacks.

192. **Question:** How to genesis a chain?
    **Answer:** Use `gentx` for validators, collect and add-genesis.

193. **Question:** What are faucets?
    **Answer:** Services dispensing test tokens.

194. **Question:** Describe testnets.
    **Answer:** Like Gaia for Cosmos Hub testing.

195. **Question:** How to join a network?
    **Answer:** Sync from genesis or state sync, add peers.

196. **Question:** What is the explorer?
    **Answer:** Tools like Hub explorer for block viewing.

197. **Question:** Explain wallets.
    **Answer:** Keplr or Cosmostation for user interaction.

198. **Question:** How does Cosmos support NFTs?
    **Answer:** Via ICS-721 or custom modules.

199. **Question:** What is GameFi on Cosmos?
    **Answer:** Build games with tokens and IBC transfers.

200. **Question:** Describe future of Cosmos SDK.
    **Answer:** Focus on modularity, scalability with ABCI++, and interchain innovations.

    