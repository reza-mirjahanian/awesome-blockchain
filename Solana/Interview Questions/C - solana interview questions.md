

## Technical Depth — Core Protocol & Runtime

**Q1**  
*How do you mitigate account rent exhaustion in long-running Solana programs, especially when dealing with dynamic state growth?*  
*A:*  
Use `realloc` with rent-exemption guarantees via `Rent::minimum_balance`. Pre-calculate worst-case storage needs and enforce client-side padding. For dynamic state, implement chunked storage with linked accounts or off-chain metadata pointers.  
> *Rent exhaustion causes silent account deletion — a critical failure mode in DeFi protocols. Anchor’s `realloc` macro handles this, but manual CPI calls require explicit rent checks.*

**Q2**  
*Explain how you’d handle a program that must process >10k accounts in a single transaction under Solana’s 1.4M compute unit limit.*  
*A:*  
Split state across multiple transactions using Merkle proofs or pagination. Use `remaining_accounts` for bulk access but enforce strict account ordering and signature verification. Offload heavy reads to off-chain indexers and pass proofs on-chain.  
> *Solana’s per-transaction account limit is 64. Exceeding this requires architectural redesign — not optimization.*

**Q3**  
*Describe a scenario where CPI (Cross-Program Invocation) introduces reentrancy risk — and how you’d prevent it.*  
*A:*  
If Program A calls Program B, and B calls back into A before A finishes state mutation, state can be double-spent. Use guard flags (e.g., `is_processing: bool`) in account data or leverage Solana’s lack of dynamic dispatch to statically verify call graphs.  
> *Unlike EVM, Solana doesn’t have `call`/`delegatecall` ambiguity — but nested CPI with mutable refs can still cause logical reentrancy.*

**Q4**  
*How would you implement atomic multi-program state updates without violating Solana’s single-writer principle?*  
*A:*  
Use a coordinator account as the sole writer. All state-modifying CPIs must target this account. Alternatively, use PDA-derived “lock” accounts to serialize access. Never allow two programs to write to the same account in one TX.  
> *Violating single-writer causes runtime panics. Solana enforces this at the VM level — not a suggestion.*

**Q5**  
*What’s your strategy for handling transaction retries when a Solana RPC node returns “Blockhash not found”?*  
*A:*  
Implement exponential backoff with dynamic blockhash refresh. Cache recent blockhashes client-side and rotate on 429 or “BlockhashNotFound”. Use `getLatestBlockhash` with commitment=’finalized’ for high-value TXs.  
> *Clients that naively retry with stale blockhashes waste fees and congest mempools — a common UX failure in wallets.*

---

## System Design — Scalability & Architecture

**Q6**  
*Design a Solana-based orderbook that supports 50k TPS with sub-50ms latency. What layers do you offload off-chain?*  
*A:*  
Keep only settlement and slashing logic on-chain. Offload order matching, depth aggregation, and price feeds to off-chain relayers using ZK-proofs or threshold signatures for batch settlement. Use Serum-style event queues for on-chain triggers.  
> *On-chain orderbooks at scale are economically infeasible. Serum V3 moved matching off-chain — follow that pattern.*

**Q7**  
*How do you shard state across multiple PDAs to avoid account size limits (10MB) and reduce rent costs?*  
*A:*  
Hash-based sharding: `PDA = [program_id, base_seed, shard_id]`. Use consistent hashing (e.g., `murmur3(account_key) % N`) for deterministic routing. Store shard metadata in a root PDA.  
> *Monolithic state accounts hit rent and CU limits fast. Sharding is mandatory for protocols like NFT marketplaces or AMMs with high churn.*

**Q8**  
*Describe how you’d design a Solana program that must interact with Ethereum state (e.g., for cross-chain staking).*  
*A:*  
Use Wormhole or LayerZero for message passing. On Solana, store only the last verified root and validator set. Validate incoming VAA signatures on-chain via `secp256k1_recover`. Never trust off-chain relayers without slashing conditions.  
> *Bridges are top attack surface. Always verify signatures on-chain — don’t rely on relayer honesty.*

**Q9**  
*How would you implement a time-locked vault that releases funds only after a UNIX timestamp — without oracles?*  
*A:*  
Leverage slot-based time approximation: `Clock::get()?.slot > target_slot`. Precompute target slot using average slot time (400ms). Add 10% buffer for validator drift. For precision, use on-chain oracles like Pyth with slashing.  
> *Solana has no native wall-clock time. Slot-based time is “good enough” for most use cases — but not for legal deadlines.*

**Q10**  
*Design a gas-efficient NFT minting system that supports 100k mints in <5 minutes during a public sale.*  
*A:*  
Pre-generate PDAs and metadata off-chain. Use a single “mint coordinator” account to batch verify allowlists via Merkle proofs. Compress metadata using `borsh` and store off-chain (Arweave/IPFS). Mint via CPI to Token Program with pre-signed IXs.  
> *High-volume mints fail due to CU limits and congestion. Batching + off-chain proofs are non-negotiable.*

---

## Security & Economic Attacks

**Q11**  
*How do you prevent frontrunning in a Solana AMM when swap instructions are public in the mempool?*  
*A:*  
Use private mempools (Jito) or encrypted transactions (Shred). Alternatively, implement slippage checks server-side and submit TXs via private RPC endpoints. For open systems, use commit-reveal schemes with hashed parameters.  
> *Solana’s mempool is public. Frontrunning is rampant in DEXs — Raydium and Orca use Jito bundles for protection.*

**Q12**  
*What’s your mitigation for “account draining” attacks where a malicious program CPIs into your token vault?*  
*A:*  
Enforce strict ownership checks: `token_account.owner == program_id`. Use PDAs for vaults — never user-owned accounts. Validate all CPI targets via `invoke_signed` with correct seeds.  
> *The “Slope Finance” hack drained $5M via unchecked CPI ownership. Always verify `owner` and `mint` in token accounts.*

**Q13**  
*How would you defend against compute unit exhaustion attacks in a public-facing Solana program?*  
*A:*  
Cap loops with static bounds. Reject TXs if `compute_units_remaining < threshold` mid-execution. Use `msg!` to log CU usage in testnet. Precompute worst-case CUs and reject TXs client-side if estimated CU > 1.2M.  
> *CU exhaustion reverts TXs but still burns user fees. Denial-of-service via CU is a real economic attack.*

**Q14**  
*Describe how you’d implement slashing for a Solana validator staking pool that detects malicious behavior.*  
*A:*  
Store validator performance metrics (uptime, equivocation) off-chain via oracles. On-chain, use a slashing PDA that holds stake. Trigger slashing via multi-sig or DAO vote with cryptographic evidence (e.g., double-sign proofs).  
> *Solana doesn’t natively slash validators. Pools must implement their own — see Marinade or Jito for patterns.*

**Q15**  
*How do you prevent “signature malleability” attacks when verifying off-chain signed messages in your program?*  
*A:*  
Enforce canonical signatures: reject `s > N/2` for ECDSA (low-S rule). Use Ed25519 where possible — no malleability. Include chain ID and program ID in signed message to prevent replay.  
> *Malleability allows attackers to mutate TXs without invalidating sigs — critical for governance or withdrawal systems.*

---

## Optimization & Performance

**Q16**  
*What techniques do you use to reduce Borsh serialization overhead in high-frequency Solana programs?*  
*A:*  
Avoid nested structs. Use fixed-size arrays instead of Vecs. Pre-serialize static data into byte arrays. For dynamic data, use custom `try_from_slice_unchecked` with manual bounds checks.  
> *Borsh adds 5-15% CU overhead. In高频 programs (e.g., gaming), every CU counts — optimize serialization like you optimize assembly.*

**Q17**  
*How do you profile and optimize a Solana program that consistently hits 1.4M CU limit?*  
*A:*  
Use `solana-program-test` with `compute_budget::set_compute_unit_limit`. Log CU usage at each function via `sol_log_compute_units!()`. Replace loops with lookup tables. Inline small functions. Avoid `Pubkey::find_program_address` in hot paths — cache PDAs.  
> *Mainnet CU limits are hard caps. Optimization isn’t optional — it’s survival.*

**Q18**  
*What’s your approach to minimizing rent costs for a protocol with 1M+ state accounts?*  
*A:*  
Use account compression (Solana’s new state compression feature). Store data in Merkle trees anchored on-chain. For active accounts, enforce rent delegation via `assign` to rent-payer PDAs. Archive cold data off-chain.  
> *Rent for 1M accounts ≈ 2,000 SOL/year. Compression reduces this by 1000x — mandatory for scalable apps.*

**Q19**  
*How do you optimize Solana transaction size when batching 50+ instructions?*  
*A:*  
Use address lookup tables (ALTs) to replace full Pubkeys with 1-byte indices. Compress instruction data with custom binary formats. Avoid redundant accounts — reuse `writable` and `signer` flags efficiently.  
> *Without ALTs, 50 IXs exceed 1232-byte packet limit. ALTs are non-optional for batch operations.*

**Q20**  
*Describe how you’d implement zero-copy deserialization for Solana account data to avoid heap allocation.*  
*A:*  
Use `#[repr(packed)]` structs with `bytemuck::Pod`. Access account data via `&[u8]` slices and cast with `try_from_slice_unchecked`. Validate alignment and size manually. Never use `Vec` or `String` in account structs.  
> *Heap allocation in Solana programs is expensive and risky. Zero-copy is standard in high-perf programs like Mango v4.*

---

## Tooling & DevOps

**Q21**  
*How do you set up a local Solana test validator that mimics mainnet rent and CU behavior?*  
*A:*  
Use `solana-test-validator --compute-unit-limit 1400000 --rent 1900000000`. Add `--clone` for mainnet accounts. Set `--slots-per-epoch 32` to match mainnet epoch timing.  
> *Local tests with default settings lie. Always mimic mainnet constraints — or your program will fail on deployment.*

**Q22**  
*What’s your CI/CD pipeline for Solana programs that includes fuzzing and formal verification?*  
*A:*  
GitHub Actions: build → Anchor test → halmos (symbolic execution) → solana-fuzz (AFL-based) → deploy to devnet. Use `seahorn-sol` for invariant checking. Gate mainnet deploys on 95% branch coverage.  
> *Solana programs are immutable. Fuzzing isn’t optional — see the “Cashio” hack for why.*

**Q23**  
*How do you monitor Solana program performance and errors in production?*  
*A:*  
Log structured events via `emit!` (Anchor) or custom logs. Ship to Elasticsearch via `solana-logs` forwarder. Alert on `ProgramError` codes and CU > 1.3M. Use Helius or Triton for RPC-level tracing.  
> *Silent failures kill protocols. Log everything — even “expected” errors.*

**Q24**  
*Describe your strategy for zero-downtime upgrades of a live Solana program with millions in TVL.*  
*A:*  
Deploy new program to new address. Migrate state via CPI from old program (with user consent). Use feature flags for gradual rollout. Freeze deposits during migration. Test rollback via shadow deployments.  
> *Solana program upgrades are not in-place. State migration is manual — plan for weeks, not hours.*

**Q25**  
*How do you secure private keys for program upgrade authority in a multi-sig setup?*  
*A:*  
Use Squads or Gnosis Safe with 3/5 threshold. Store keys in HSM or AWS KMS. Rotate keys quarterly. Require 48hr timelock for upgrades. Log all upgrade attempts on-chain.  
> *Upgrade authority compromise = total loss. Treat it like nuclear codes.*

---

## Cross-Functional & Collaboration

**Q26**  
*How do you communicate Solana’s 400ms slot time limitation to frontend teams building real-time dashboards?*  
*A:*  
Educate on probabilistic finality: “400ms is average — not guaranteed”. Push for client-side buffering and optimistic UI updates. Use websockets for slot subscriptions. Never promise sub-second finality.  
> *Frontend teams assume “blockchain = instant”. This mismatch causes UX disasters — manage expectations early.*

**Q27**  
*What’s your process for onboarding auditors to a complex Solana program with 50+ PDAs?*  
*A:*  
Provide state transition diagrams, threat model doc, and test vectors. Freeze code 2 weeks pre-audit. Grant read-only access to devnet deployment. Pay for adversarial testing — not just line review.  
> *Auditors miss bugs if context is missing. Invest in documentation — it’s cheaper than a hack.*

**Q28**  
*How do you handle disagreements with product managers who demand “EVM-like” features on Solana?*  
*A:*  
Show cost/risk analysis: “Reentrancy guard adds 15K CUs and 0.1 SOL/tx in rent”. Propose Solana-native alternatives (e.g., event queues instead of callbacks). Prototype both — let data decide.  
> *Forcing EVM patterns on Solana causes failures. Educate — don’t capitulate.*

**Q29**  
*Describe how you’d work with DevOps to ensure Solana RPC nodes don’t become a single point of failure.*  
*A:*  
Deploy multi-region RPC cluster (GCP + AWS). Use load balancer with health checks. Cache blockhashes and account states client-side. Fall back to public RPCs (Helius, QuickNode) with rate limiting.  
> *RPC downtime = app downtime. Own the full stack — don’t blame “the blockchain”.*

**Q30**  
*How do you document Solana program invariants for future maintainers?*  
*A:*  
Embed invariants as `assert!` statements with comments. Generate docs via `cargo doc` with state diagrams. Record failure scenarios in `NOTES.md`. Use `solana-security-txt` for contact and bug bounty info.  
> *Programs outlive teams. Document like your users’ funds depend on it — because they do.*

---

## Edge Cases & Failure Recovery

**Q31**  
*What do you do when your Solana program hits “AccountNotRentExempt” during a CPI call?*  
*A:*  
Pre-check rent balance via `Rent::minimum_balance(size)`. If insufficient, revert and instruct client to top up rent. Never auto-top-up — it enables griefing. For PDAs, ensure seeds include rent payer.  
> *Rent errors are silent killers. Always check before CPI — not after.*

**Q32**  
*How do you recover from a stuck transaction that’s clogging your program’s state machine?*  
*A:*  
Design state machines with timeouts: if `last_updated_slot < current_slot - 1000`, allow force-reset via admin or DAO. Log stuck TXs off-chain for manual intervention. Never rely on “eventual inclusion”.  
> *Stuck TXs halt protocols. Build escape hatches — or your app becomes abandonware.*

**Q33**  
*Describe how you’d handle a Solana network fork that causes your program’s state to diverge.*  
*A:*  
Monitor fork detection via `RpcClient::get_slot_with_commitment(‘finalized’)`. If fork detected, pause deposits and trigger state reconciliation via off-chain oracle. Use checkpointing every 100 slots.  
> *Forks are rare but catastrophic. Have a runbook — don’t wing it.*

**Q34**  
*What’s your response when a user reports “Transaction too large” for a multi-instruction bundle?*  
*A:*  
Switch to address lookup tables (ALTs). Split bundle into multiple TXs with nonce accounts for ordering. Compress instruction data using binary encoding. Educate user on MTU limits.  
> *1232-byte limit is hard. ALTs solve this — but require pre-registration.*

**Q35**  
*How do you debug a Solana program that works on devnet but fails on mainnet with “InvalidAccountData”?*  
*A:*  
Check rent differences: mainnet accounts may be rent-exhausted. Validate account discriminators match exactly. Use `solana account --output json` to compare raw data. Test with mainnet state clones.  
> *Devnet ≠ mainnet. Always test with production data — or fail in production.*

---

## Economic Design & Tokenomics

**Q36**  
*How do you design fee collection in a Solana program to avoid dust accumulation and rent drain?*  
*A:*  
Sweep fees to a central vault every 100 transactions. Use rent-delegated accounts for fee storage. Denominate fees in SOL, not tokens — avoids multi-currency accounting.  
> *Dust fees are economically irrational. Sweep or burn — don’t let them rot.*

**Q37**  
*Describe how you’d implement a bonding curve for a Solana token with <1% slippage at 100 SOL volume.*  
*A:*  
Use constant-product formula with virtual reserves. Cache price off-chain and submit via oracle. On-chain, verify price within 1% of last accepted. Use TWAP to prevent oracle manipulation.  
> *On-chain curves at scale are CU-prohibitive. Hybrid on/off-chain is the only viable approach.*

**Q38**  
*How do you prevent MEV extraction in a Solana staking rewards distributor?*  
*A:*  
Randomize reward distribution order using on-chain VRF (e.g., Switchboard). Distribute in batches with Merkle proofs. Never use predictable iteration (e.g., by Pubkey).  
> *Predictable reward order = MEV paradise. Randomize or get picked clean.*

**Q39**  
*What’s your strategy for handling token inflation in a Solana program that calculates APY?*  
*A:*  
Fetch real-time inflation rate via `RpcClient::get_inflation_rate()`. Cache for 1 epoch. Use geometric mean for APY calculation. Display “estimated” APY with disclaimer.  
> *Solana inflation changes quarterly. Hardcoding rates causes user lawsuits.*

**Q40**  
*How do you design a Solana token vesting contract that survives wallet changes or key loss?*  
*A:*  
Tie vesting to identity (e.g., GitHub OAuth via Civic) not wallet. Use PDA escrow with social recovery multisig. Allow beneficiary to update wallet via signed message from old key.  
> *Wallets are lost. Design for human failure — not cryptographic purity.*

---

## Advanced Cryptography

**Q41**  
*How do you verify Ed25519 signatures on-chain in a Solana program without exceeding CU limits?*  
*A:*  
Use `solana_program::ed25519_program` via CPI. Pre-verify signatures off-chain and submit as instruction data. For bulk, use Merkle trees of signatures.  
> *On-chain Ed25519 verification costs ~85K CUs. Avoid in loops — batch or pre-verify.*

**Q42**  
*Describe how you’d implement zk-proof verification (e.g., Groth16) in a Solana program.*  
*A:*  
Offload heavy pairing operations to off-chain oracles. On-chain, verify only the final scalar multiplication and subgroup checks. Use precompiled circuits for common ops (e.g., SHA256).  
> *Full zk-verification exceeds 1.4M CUs. Hybrid verification is the only practical approach on Solana today.*

**Q43**  
*How do you prevent hash collision attacks when using SHA256 for Merkle proofs in Solana?*  
*A:*  
Use domain separation: `SHA256(“solana_merkle” || leaf_data)`. Enforce fixed-width leaves. Reject proofs with depth > 32. Log root changes for monitoring.  
> *Collision attacks are theoretical but catastrophic. Domain separation costs nothing — always use it.*

**Q44**  
*What’s your approach to implementing BLS signature aggregation for validator sets on Solana?*  
*A:*  
Aggregate off-chain. On-chain, verify only the final signature against the aggregated pubkey. Use `alt_bn128` precompiles via CPI. Cache validator set roots to avoid repeated aggregation.  
> *BLS on-chain is expensive. Aggregate off-chain — submit only the proof.*

**Q45**  
*How do you securely generate random numbers in a Solana program for NFT minting?*  
*A:*  
Use Switchboard VRF or Chainlink VRF. Never use `Clock::slot` or `recent_blockhash` — predictable. For low-stakes, use commit-reveal with hashed client entropy.  
> *“Random” from slot or hash is trivially exploitable. Pay for real randomness — or get rugged.*

---

## Governance & DAOs

**Q46**  
*How do you prevent vote manipulation in a Solana DAO that uses token-weighted voting?*  
*A:*  
Snapshot balances at proposal start. Use Merkle proofs for vote eligibility. Enforce 1-week lockup post-vote to prevent flash loans.  
> *Without snapshotting, whales can borrow tokens to swing votes — see “Build Finance” hack.*

**Q47**  
*Describe how you’d implement quadratic voting on Solana without exceeding CU limits.*  
*A:*  
Calculate vote cost off-chain: `cost = votes^2`. Submit cost and signature. On-chain, verify signature and deduct cost from voter’s account. Use PDA for vote tally.  
> *On-chain quadratic math is CU-heavy. Shift computation off-chain — keep verification on-chain.*

**Q48**  
*How do you handle proposal execution that requires multiple program upgrades in a Solana DAO?*  
*A:*  
Stage upgrades: Proposal → Vote → Timelock → Execute. Use separate “executor” PDA with limited permissions. Require 3-day delay between approval and execution.  
> *Rushed upgrades kill protocols. Timelocks save lives — enforce them.*

**Q49**  
*What’s your strategy for preventing Sybil attacks in a Solana DAO with low-barrier membership?*  
*A:*  
Require proof-of-humanity (e.g., BrightID) or proof-of-stake (minimum 10 SOL). Use social graph analysis off-chain. Weight votes by tenure, not just token count.  
> *Sybil attacks turn DAOs into plutocracies. Identity matters — don’t pretend it doesn’t.*

**Q50**  
*How do you design a Solana program that allows DAOs to veto malicious transactions post-execution?*  
*A:*  
Implement “circuit breaker” PDA. If 66% of DAO votes within 100 slots, revert state via backup snapshot. Log all veto attempts for transparency.  
> *Post-execution veto is controversial but necessary for high-risk protocols. Document tradeoffs clearly.*

---

## DeFi-Specific Challenges

**Q51**  
*How do you prevent impermanent loss exploitation in a Solana AMM during volatile markets?*  
*A:*  
Use concentrated liquidity (like Uniswap V3) with dynamic fee tiers. Offload price updates to oracles with 1% deviation limits. Freeze pools during >50% price swings.  
> *Standard constant-product AMMs bleed LPs in volatility. Concentrated liquidity is the only defense.*

**Q52**  
*Describe how you’d implement闪电贷 (flash loans) on Solana without reentrancy risks.*  
*A:*  
Use a “lender” PDA that holds funds. Borrower must return + fee in same TX. Verify return amount before CPI exit. Never allow nested borrows.  
> *Solana’s lack of reentrancy makes flash loans safer — but logic errors still cause hacks.*

**Q53**  
*How do you handle oracle failure in a Solana lending protocol during a market crash?*  
*A:*  
Freeze borrows and liquidations. Allow manual price setting by DAO with 48hr timelock. Use fallback oracles (median of 3 sources). Pause interest accrual.  
> *Oracle failure during crashes = death spiral. Have a manual override — automated systems fail when you need them most.*

**Q54**  
*What’s your approach to liquidation bot competition in Solana lending protocols?*  
*A:*  
Use Dutch auctions: start with 5% discount, increase by 1% every slot. This reduces bot wars and gives fair value to liquidators.  
> *First-price liquidations cause gas wars and user losses. Dutch auctions align incentives.*

**Q55**  
*How do you prevent “sandwich attacks” in a Solana DEX aggregator?*  
*A:*  
Route through private mempools (Jito). Use slippage tolerance per pool. Reject routes with >2% price impact. Add random delay to order submission.  
> *Aggregators are MEV magnets. Privacy and slippage control are your only weapons.*

---

## NFT & Gaming

**Q56**  
*How do you prevent “rug pulls” in a Solana NFT mint where creators can update metadata?*  
*A:*  
Lock metadata URI after first sale. Store hash of metadata on-chain. Allow updates only via DAO vote. Use Metaplex’s `verify_creator` for provenance.  
> *Mutable metadata = rug pull vector. Freeze it — or lose user trust.*

**Q57**  
*Describe how you’d implement on-chain rarity traits for 10k Solana NFTs without exceeding account limits.*  
*A:*  
Store trait hashes in a Merkle tree. On-chain, store only root and index. Verify traits via Merkle proof during sales or claims.  
> *Storing 10k traits on-chain costs 100+ SOL in rent. Compression is mandatory.*

**Q58**  
*How do you handle mass NFT airdrops (100k+ recipients) on Solana without bankrupting the sender?*  
*A:*  
Use compressed NFTs (cNFTs). Airdrop via Merkle proofs — recipients claim by submitting proof. Batch claims in single TXs using ALTs.  
> *Traditional airdrops cost 0.01 SOL/recipient — 1000 SOL for 100k. cNFTs reduce this to 1 SOL.*

**Q59**  
*What’s your strategy for preventing bot sniping in Solana NFT public sales?*  
*A:*  
Use captcha or proof-of-humanity off-chain. Implement fair launch via Merkle allowlists. Add 5-minute randomized delay to sale start.  
> *Bots drain sales in seconds. Human verification is ugly but necessary.*

**Q60**  
*How do you design a Solana on-chain game with 100ms response times?*  
*A:*  
Keep only state transitions and fraud proofs on-chain. Run game logic off-chain with ZK verification. Use WebSockets for real-time updates.  
> *On-chain games are illusions. Offload everything — keep only settlement on-chain.*

---

## Identity & Compliance

**Q61**  
*How do you implement KYC in a Solana DeFi protocol without storing PII on-chain?*  
*A:*  
Use zero-knowledge proofs (e.g., Polygon ID) to verify KYC off-chain. Store only attestation hash on-chain. Allow revocation via issuer signature.  
> *Storing PII on-chain violates GDPR. ZK proofs are the only compliant path.*

**Q62**  
*Describe how you’d handle OFAC sanctions in a Solana token contract.*  
*A:*  
Maintain off-chain sanctions list. Freeze transfers to sanctioned addresses via DAO vote. Log all freezes on-chain for auditability.  
> *Ignoring sanctions = protocol shutdown. Compliance isn’t optional — build it in.*

**Q63**  
*How do you prevent Sybil attacks in a Solana airdrop targeting unique humans?*  
*A:*  
Require Worldcoin or BrightID verification. Use social graph analysis to detect duplicates. Limit 1 claim per verified identity.  
> *Airdrop farmers drain budgets. Identity verification is the cost of fairness.*

**Q64**  
*What’s your approach to implementing age restrictions for Solana NFT mints?*  
*A:*  
Integrate with Civic’s age verification. Store only “over_18: bool” hash on-chain. Allow challenge via DAO if false positive.  
> *Age gates require legal compliance. Don’t guess — use certified providers.*

**Q65**  
*How do you design a Solana program that complies with MiCA regulations for token issuers?*  
*A:*  
Embed issuer DID and whitepaper hash in token metadata. Freeze transfers until prospectus is approved. Log all issuer actions for 5 years.  
> *MiCA is coming. Bake compliance into token design — don’t bolt it on.*

---

## Interoperability

**Q66**  
*How do you handle Solana <> Ethereum token bridging with 15-sec finality mismatch?*  
*A:*  
Use optimistic verification: lock on Ethereum, mint on Solana after 30 mins. For speed, use economic bonds with slashing for fraud.  
> *Finality mismatch causes race conditions. Optimistic or ZK bridges are the only safe options.*

**Q67**  
*Describe how you’d implement跨链 (cross-chain) governance for a Solana DAO that controls Ethereum contracts.*  
*A:*  
Use Wormhole to relay votes. On Ethereum, require 66% quorum from Solana vote snapshot. Verify Solana signatures via `secp256k1` precompile.  
> *Cross-chain governance is complex. Snapshot votes on source chain — execute on target.*

**Q68**  
*How do you prevent replay attacks when bridging messages between Solana and Polygon?*  
*A:*  
Include chain ID and nonce in signed message. Store used nonces in a bitmap on both chains. Reject nonces older than 100 blocks.  
> *Replay attacks drain bridges. Nonce + chain ID is the minimum defense.*

**Q69**  
*What’s your strategy for handling failed cross-chain transactions due to gas spikes?*  
*A:*  
Implement auto-retry with dynamic gas pricing. Allow manual override via DAO. Refund users in native token if failure exceeds 1 hour.  
> *Gas spikes kill cross-chain UX. Auto-retry + refund is the user-friendly solution.*

**Q70**  
*How do you design a Solana program that reads Bitcoin UTXO state?*  
*A:*  
Use a federated oracle (e.g., Chainlink) to submit Bitcoin block headers. Verify Merkle proofs on-chain. Never trust single oracle — require 5/7 signers.  
> *Bitcoin SPV on Solana is possible but expensive. Federated oracles are the pragmatic choice.*

---

## Monitoring & Alerting

**Q71**  
*How do you detect and alert on Solana program exploits in real-time?*  
*A:*  
Monitor for anomalous state changes: e.g., `token_balance_delta > 1000 SOL`. Use Forta-like agents with custom rules. Alert via PagerDuty and SMS.  
> *Exploits happen in seconds. Real-time monitoring is your immune system.*

**Q72**  
*Describe how you’d set up canary transactions to detect Solana RPC node failures.*  
*A:*  
Send heartbeat TXs every 30 sec to devnet. If 3 fail, trigger failover to backup RPC. Measure latency — alert if >2 sec.  
> *RPC failures are silent. Canary TXs are your early warning system.*

**Q73**  
*How do you monitor for rent exhaustion across 10k+ Solana accounts?*  
*A:*  
Off-chain bot scans accounts daily via RPC. If `rent_epoch < current_epoch - 2`, alert owner. Auto-top-up for critical accounts.  
> *Rent exhaustion is slow death. Scan proactively — don’t wait for user complaints.*

**Q74**  
*What’s your approach to detecting MEV bots targeting your Solana protocol?*  
*A:*  
Log all transaction origins. Cluster by IP and wallet patterns. Flag TXs with >50% slippage or rapid-fire submissions.  
> *MEV bots leave footprints. Track them — and adjust your design.*

**Q75**  
*How do you alert on Solana validator downtime affecting your program’s finality?*  
*A:*  
Monitor `RpcClient::get_health()`. If >33% of validators down, pause deposits. Use decentralized oracle (e.g., Pyth) for network health.  
> *Validator outages break finality. Pause before you lose funds.*

---

## Disaster Recovery

**Q76**  
*Describe your runbook for responding to a Solana program exploit draining user funds.*  
*A:*  
1. Pause deposits via circuit breaker.  
2. Fork state to recovery program.  
3. Allow users to prove ownership and reclaim.  
4. Post-mortem within 24hr.  
> *Speed saves funds. Have the runbook printed — not in a wiki.*

**Q77**  
*How do you recover from a Solana upgrade that breaks backward compatibility with your state?*  
*A:*  
Deploy migration program. Allow users to opt-in to migrate state. Freeze old program. Offer incentives (e.g., airdrop) for migration.  
> *Backward-incompatible upgrades are user-hostile. Make migration painless — or they’ll abandon you.*

**Q78**  
*What’s your strategy for handling a Solana network halt (e.g., due to bug)?*  
*A:*  
Pause all on-chain activity. Communicate via Twitter and Discord. Prepare state rollback script. Test on devnet before mainnet.  
> *Network halts cause panic. Over-communicate — silence breeds rumors.*

**Q79**  
*How do you restore user trust after a Solana protocol hack?*  
*A:*  
1. Full transparency: post exploit TX, root cause.  
2. Reimburse 100% via treasury or insurance.  
3. Third-party audit before relaunch.  
> *Trust is harder to rebuild than code. Over-compensate — or die.*

**Q80**  
*Describe how you’d handle a private key leak for your Solana program’s upgrade authority.*  
*A:*  
1. Immediately rotate keys via multi-sig.  
2. Deploy dummy program to old address to trap attackers.  
3. Notify all users via on-chain event.  
> *Key leaks are existential. Assume breach — act faster than the attacker.*

---

## Advanced Tooling

**Q81**  
*How do you use Solana’s new “state compression” feature to store 1M NFTs under 100 SOL rent?*  
*A:*  
Store NFT data in Merkle tree off-chain. Anchor root hash on-chain. Use `ConcurrentMerkleTree20` for proofs. Verify ownership via `verifyLeaf` CPI.  
> *State compression reduces rent by 1000x. Mandatory for mass adoption.*

**Q82**  
*Describe how you’d implement on-chain program debugging using Solana’s `msg!` and log scanners.*  
*A:*  
Prefix logs with `[DEBUG]` and structured JSON. Use `solana-logs` to forward to ELK stack. Correlate by TX signature. Never log secrets.  
> *On-chain debugging is painful. Structure logs like you’ll grep them at 3AM.*

**Q83**  
*How do you leverage address lookup tables (ALTs) to reduce transaction costs for frequent users?*  
*A:*  
Register user’s common accounts (token vaults, PDAs) in ALT. Replace 32-byte Pubkeys with 1-byte indices. Update ALT weekly.  
> *ALTs cut TX size by 70%. Essential for DeFi power users.*

**Q84**  
*What’s your approach to fuzzing Solana programs with structured inputs (e.g., valid account states)?*  
*A:*  
Use `arbitrary` crate to generate valid `AccountInfo` structs. Mutate discriminators and data sizes. Focus on boundary conditions: rent, CU, account counts.  
> *Random fuzzing misses Solana-specific bugs. Structure your fuzzers.*

**Q85**  
*How do you integrate formal verification (e.g., Seahorn) into your Solana CI pipeline?*  
*A:*  
Convert BPF bytecode to LLVM IR. Specify invariants (e.g., “token balance never negative”). Run nightly — gate deploys on no violations.  
> *Formal verification catches logical errors unit tests miss. Invest in it — or pay later.*

---

## Performance at Scale

**Q86**  
*How do you handle 10k TPS in a Solana program that updates global state (e.g., price feed)?*  
*A:*  
Offload to oracles. On-chain, store only latest price and epoch. Use optimistic updates: allow 1% deviation before rejecting.  
> *Global state updates don’t scale. Offload or die.*

**Q87**  
*Describe how you’d shard a Solana AMM pool to support 1M+ LPs.*  
*A:*  
Partition LPs by fee tier or asset pair. Use separate PDAs per shard. Route trades via off-chain aggregator. Rebalance shards weekly.  
> *Monolithic pools hit account limits. Shard early — don’t wait for failure.*

**Q88**  
*How do you optimize Solana transaction confirmation times during network congestion?*  
*A:*  
Use priority fees (compute budget IX). Submit via private RPC (Jito). Bundle with unrelated TXs to fill packets.  
> *Congestion kills UX. Priority fees are your toll road — pay to play.*

**Q89**  
*What’s your strategy for reducing Solana RPC latency for global users?*  
*A:*  
Deploy RPC nodes in AWS us-west, eu-central, ap-southeast. Use Cloudflare load balancer with geo-routing. Cache frequent queries (e.g., token balances).  
> *Latency varies by region. Deploy globally — or lose international users.*

**Q90**  
*How do you design a Solana program that survives 10x traffic spikes during NFT mints?*  
*A:*  
Rate-limit mints per wallet. Queue requests off-chain. Process in batches via ALTs. Use CDN for metadata.  
> *Traffic spikes kill unprepared programs. Throttle and queue — don’t try to scale instantly.*

---

## Token Standards & Extensions

**Q91**  
*How do you implement transfer hooks for SPL Token 2022 in a Solana program?*  
*A:*  
Derive hook PDA from mint. In `transfer_checked`, CPI to hook program. Validate hook returns `success` before proceeding.  
> *Transfer hooks enable compliance. Use them for royalties, freezes, or taxes.*

**Q92**  
*Describe how you’d add confidential transfers to an existing SPL Token using Solana’s ZK feature.*  
*A:*  
Wrap token in ZK-enabled mint. Use `apply_pending_balance` for shielded transfers. Verify proofs via `zk_token_proof` program.  
> *Confidential transfers require new mints. Plan migration early.*

**Q93**  
*How do you prevent metadata spoofing in Solana NFTs using Metaplex standards?*  
*A:*  
Verify `update_authority` matches trusted creator. Check `creators[0].verified == true`. Reject if URI domain not in allowlist.  
> *Fake NFTs drain users. Verify provenance — don’t trust metadata blindly.*

**Q94**  
*What’s your approach to implementing token vesting with clawback for SPL Token 2022?*  
*A:*  
Use `TransferFeeConfig` to lock tokens. Allow clawback via `withdraw_withheld_tokens_from_mint` by issuer. Log all clawbacks.  
> *Clawback is legally sensitive. Document terms — and log every action.*

**Q95**  
*How do you handle token decimals mismatch when swapping between SPL tokens with 6 vs 9 decimals?*  
*A:*  
Normalize to 9 decimals internally. Use `checked_mul/div` to avoid overflow. Display “estimated” amounts to users.  
> *Decimal mismatches cause silent losses. Normalize early — and warn users.*

---

## Validator & Node Ops

**Q96**  
*How do you configure a Solana validator to prioritize your program’s transactions?*  
*A:*  
Run your own validator. Use `--prioritize-transactions` with your program ID. Stake heavily to get voting power.  
> *Priority is economic. Run validators or pay others to prioritize you.*

**Q97**  
*Describe how you’d detect and mitigate Solana validator censorship of your program’s TXs.*  
*A:*  
Monitor inclusion rate via multiple RPCs. If <90%, submit via alternative validators. Report to Solana Foundation.  
> *Censorship is rare but possible. Monitor — and have backups.*

**Q98**  
*How do you optimize Solana validator performance for low-latency program execution?*  
*A:*  
Use NVMe SSDs. Disable unnecessary plugins. Set `--limit-ledger-size`. Tune `--accounts-db-skip-shrink`.  
> *Slow validators hurt your users. Own your infra — or suffer.*

**Q99**  
*What’s your strategy for handling Solana epoch transitions that reset your program’s state?*  
*A:*  
Cache epoch in state account. On `Clock::epoch` change, trigger state reset via CPI. Test epoch transitions in CI.  
> *Epoch transitions break unprepared programs. Handle them explicitly.*

**Q100**  
*How do you secure Solana validator private keys against physical theft?*  
*A:*  
Use HSM (e.g., AWS CloudHSM). Require multi-person access. Store backups in bank vaults. Rotate keys quarterly.  
> *Validator keys = network security. Treat them like nuclear launch codes.*

---

## Remaining Questions (101-200)

*(Due to length constraints, here are condensed versions — full answers follow same pattern: concise, actionable, Solana-specific.)*

**Q101**  
*How do you prevent “dust” accumulation in Solana token accounts?*  
*A:* Sweep to fee vault if balance < rent exemption. Auto-close if zero balance after sweep.

**Q102**  
*Describe how you’d implement time-weighted APY calculation in a Solana staking program.*  
*A:* Store `reward_per_token_stored` and `last_update_time`. Calculate delta on each interaction.

**Q103**  
*How do you handle Solana program downgrades after a failed upgrade?*  
*A:* Deploy old version to new address. Migrate state back via user opt-in. Never overwrite program data.

**Q104**  
*What’s your approach to testing Solana programs under adversarial network conditions?*  
*A:* Use `solana-test-validator` with `--faucet-sol` and random packet loss injection.

**Q105**  
*How do you prevent “griefing” attacks where users spam your Solana program with invalid TXs?*  
*A:* Charge upfront fee in SOL. Refund if valid. Use CAPTCHA off-chain for high-risk endpoints.

**Q106**  
*Describe how you’d implement a Solana program that requires multi-block computation.*  
*A:* Break into state machine. Store intermediate state in PDA. Resume via client-triggered IX.

**Q107**  
*How do you optimize Solana Anchor programs for minimal binary size?*  
*A:* Disable `idl` in prod. Use `--features cpi`. Strip debug symbols. Avoid heavy dependencies.

**Q108**  
*What’s your strategy for handling Solana’s 10MB account size limit in data-heavy programs?*  
*A:* Use account compression or off-chain storage with on-chain Merkle roots.

**Q109**  
*How do you prevent “front-running” of governance votes on Solana?*  
*A:* Use commit-reveal: hash vote → reveal later. Shuffle reveal order via VRF.

**Q110**  
*Describe how you’d implement fractional NFT ownership on Solana.*  
*A:* Wrap NFT in tokenized vault. Issue ERC-20-like tokens representing shares. Use Merkle proofs for claims.

**Q111**  
*How do you handle Solana transaction versioning (v0) with address lookup tables?*  
*A:* Always use `VersionedTransaction`. Fallback to legacy if ALT not supported.

**Q112**  
*What’s your approach to securing Solana program environment variables?*  
*A:* Never hardcode. Fetch from PDAs or oracles. Encrypt secrets using threshold encryption.

**Q113**  
*How do you prevent “oracle lag” attacks in Solana DeFi protocols?*  
*A:* Reject prices older than 20 slots. Use TWAP. Allow manual override during volatility.

**Q114**  
*Describe how you’d implement Solana program-level circuit breakers.*  
*A:* Track error rates. If >5% in 100 slots, pause via admin or DAO vote.

**Q115**  
*How do you handle Solana’s lack of floating-point math in financial calculations?*  
*A:* Use fixed-point: `u64` with 6 decimals. Libraries like `solana-fixed` for safe ops.

**Q116**  
*What’s your strategy for migrating users from SPL Token to Token 2022?*  
*A:* Airdrop new tokens 1:1. Burn old on transfer. Use wrapper contract for backward compatibility.

**Q117**  
*How do you prevent “signature reuse” in Solana off-chain signed messages?*  
*A:* Include nonce and expiration in signed data. Store used nonces in bitmap.

**Q118**  
*Describe how you’d implement Solana program upgradability with feature flags.*  
*A:* Store flags in PDA. Check flags before executing new logic. Allow DAO to toggle.

**Q119**  
*How do you handle Solana’s 1232-byte packet size limit for complex transactions?*  
*A:* Use address lookup tables. Compress instruction data. Split into multiple TXs.

**Q120**  
*What’s your approach to testing Solana programs with mocked clock and slot?*  
*A:* Use `solana-program-test` with `set_clock` and `warp_to_slot`.

**Q121**  
*How do you prevent “account reuse” attacks in Solana programs?*  
*A:* Check account discriminators. Reject if data doesn’t match expected struct.

**Q122**  
*Describe how you’d implement Solana program-level rate limiting.*  
*A:* Store `last_tx_slot` per user. Reject if `current_slot - last_tx_slot < 10`.

**Q123**  
*How do you handle Solana’s lack of native string support in account data?*  
*A:* Use fixed-size `[u8; N]` arrays. Validate UTF-8 off-chain. Never store dynamic strings.

**Q124**  
*What’s your strategy for securing Solana program upgrade authority keys?*  
*A:* Multi-sig with timelock. Store in HSM. Rotate quarterly. Require 3/5 signers.

**Q125**  
*How do you prevent “integer overflow” in Solana financial calculations?*  
*A:* Use `checked_add/mul`. Panic on overflow — never wrap. Test with fuzzing.

**Q126**  
*Describe how you’d implement Solana program-level access control lists (ACLs).*  
*A:* Store allowed Pubkeys in PDA. Verify `signer` against list before execution.

**Q127**  
*How do you handle Solana’s 64-account limit per transaction in complex protocols?*  
*A:* Use `remaining_accounts` with strict ordering. Offload to multiple TXs.

**Q128**  
*What’s your approach to optimizing Solana program logs for production?*  
*A:* Prefix with `[ERROR]/[WARN]`. Structured JSON. Ship to ELK. Never log secrets.

**Q129**  
*How do you prevent “replay attacks” in Solana off-chain signed instructions?*  
*A:* Include recent blockhash and nonce. Reject if blockhash older than 150 slots.

**Q130**  
*Describe how you’d implement Solana program-level emergency pauses.*  
*A:* Store `paused: bool` in PDA. Check at start of every IX. Allow toggle via multi-sig.

**Q131**  
*How do you handle Solana’s lack of native event system for off-chain indexing?*  
*A:* Emit structured logs via `msg!`. Use `solana-logs` forwarder to Kafka.

**Q132**  
*What’s your strategy for securing Solana RPC endpoints against DDoS?*  
*A:* Use Cloudflare rate limiting. Require API keys. Deploy multi-region.

**Q133**  
*How do you prevent “timestamp manipulation” in Solana time-based logic?*  
*A:* Use `Clock::slot` not wall time. Add buffer for validator drift.

**Q134**  
*Describe how you’d implement Solana program-level fee tiers.*  
*A:* Store tier in user PDA. Charge different fees based on tier. Update via governance.

**Q135**  
*How do you handle Solana’s 1.4M CU limit in recursive or nested programs?*  
*A:* Flatten recursion. Use iterative loops. Pre-calculate max depth.

**Q136**  
*What’s your approach to testing Solana programs with corrupted account data?*  
*A:* Fuzz with random byte arrays. Test discriminator mismatches. Panic on invalid data.

**Q137**  
*How do you prevent “governance takeover” in Solana DAOs with low quorum?*  
*A:* Require minimum 20% participation. Use quadratic voting. Add timelock.

**Q138**  
*Describe how you’d implement Solana program-level allowlists with Merkle proofs.*  
*A:* Store Merkle root in PDA. Verify proof in IX. Update root via governance.

**Q139**  
*How do you handle Solana’s lack of native random number generation?*  
*A:* Use VRF (Switchboard). Never use `slot` or `blockhash`.

**Q140**  
*What’s your strategy for securing Solana program metadata (e.g., name, symbol)?*  
*A:* Store hash on-chain. Verify off-chain metadata matches hash. Allow updates via multi-sig.

**Q141**  
*How do you prevent “account data overwrite” in Solana CPI calls?*  
*A:* Verify `account_info` ownership and data length before CPI. Never assume safety.

**Q142**  
*Describe how you’d implement Solana program-level vesting schedules.*  
*A:* Store `start_time`, `duration`, `cliff` in PDA. Calculate releasable amount on-demand.

**Q143**  
*How do you handle Solana’s 10k PDA derivation limit per IX?*  
*A:* Pre-derive and cache. Use `find_program_address` off-chain. Pass as `remaining_accounts`.

**Q144**  
*What’s your approach to optimizing Solana Anchor IDL for minimal size?*  
*A:* Strip docs in prod. Use short names. Avoid complex nested types.

**Q145**  
*How do you prevent “signature forgery” in Solana off-chain approvals?*  
*A:* Verify `secp256k1` or `ed25519` on-chain. Include chain ID and program ID in message.

**Q146**  
*Describe how you’d implement Solana program-level referral rewards.*  
*A:* Store referrer in user PDA. On action, CPI to transfer reward to referrer.

**Q147**  
*How do you handle Solana’s lack of native batch processing?*  
*A:* Use `invoke_signed` in loops. Enforce strict account ordering. Use ALTs.

**Q148**  
*What’s your strategy for securing Solana program configuration parameters?*  
*A:* Store in PDA. Allow updates only via multi-sig or DAO. Log all changes.

**Q149**  
*How do you prevent “integer underflow” in Solana token accounting?*  
*A:* Use `checked_sub`. Panic on underflow — never wrap. Test with edge cases.

**Q150**  
*Describe how you’d implement Solana program-level cooldown periods.*  
*A:* Store `last_action_slot` in PDA. Reject if `current_slot - last_action_slot < threshold`.

**Q151**  
*How do you handle Solana’s 32-byte Pubkey limitation for complex addressing?*  
*A:* Use PDA seeds for sharding. Never try to store >32 bytes in address.

**Q152**  
*What’s your approach to testing Solana programs with malicious validators?*  
*A:* Use test validator with custom slashing rules. Simulate equivocation and downtime.

**Q153**  
*How do you prevent “metadata poisoning” in Solana NFT marketplaces?*  
*A:* Verify `update_authority`. Cache metadata off-chain. Allow user reporting.

**Q154**  
*Describe how you’d implement Solana program-level loyalty points.*  
*A:* Store points in PDA. Award on actions. Allow redemption via CPI to token program.

**Q155**  
*How do you handle Solana’s lack of native decimal support in pricing?*  
*A:* Use `u64` with 6 decimals. Libraries like `rust_decimal` off-chain.

**Q156**  
*What’s your strategy for securing Solana program event logs?*  
*A:* Emit structured events. Use indexed fields for filtering. Never log secrets.

**Q157**  
*How do you prevent “account initialization race conditions” in Solana?*  
*A:* Use PDA with unique seeds. Check `lamports == 0` before init. Never assume atomicity.

**Q158**  
*Describe how you’d implement Solana program-level staking with auto-compounding.*  
*A:* Store `last_compound_slot`. Calculate rewards on-demand. Compound via user-triggered IX.

**Q159**  
*How do you handle Solana’s 10MB account data limit for large datasets?*  
*A:* Use account compression or off-chain storage with on-chain hashes.

**Q160**  
*What’s your approach to optimizing Solana program binary size for faster deployment?*  
*A:* Strip debug symbols. Use `opt-level = 'z'`. Avoid heavy dependencies.

**Q161**  
*How do you prevent “signature malleability” in Solana ECDSA verification?*  
*A:* Enforce low-S rule. Reject `s > N/2`. Use canonical signatures.

**Q162**  
*Describe how you’d implement Solana program-level whitelists for beta features.*  
*A:* Store allowed Pubkeys in PDA. Check `signer` against list. Update via admin.

**Q163**  
*How do you handle Solana’s lack of native mutex for shared state?*  
*A:* Use single-writer principle. Never allow concurrent writes. Serialize via PDA locks.

**Q164**  
*What’s your strategy for securing Solana program upgrade transactions?*  
*A:* Require multi-sig. Add 48hr timelock. Log all upgrade attempts.

**Q165**  
*How do you prevent “overflow in token transfers” in Solana?*  
*A:* Use `checked_add/sub`. Panic on overflow. Test with max values.

**Q166**  
*Describe how you’d implement Solana program-level gas refunds.*  
*A:* Track CUs used. Refund SOL if `CUs < threshold`. Use `get_compute_unit_price`.

**Q167**  
*How do you handle Solana’s 64KB stack limit in complex programs?*  
*A:* Avoid large local variables. Use heap via `Box` sparingly. Optimize data structures.

**Q168**  
*What’s your approach to testing Solana programs with network partitions?*  
*A:* Use test validator with `--faucet-sol` and simulated forks. Test state reconciliation.

**Q169**  
*How do you prevent “oracle price manipulation” in Solana DeFi?*  
*A:* Use median of 3 oracles. Add 1% deviation limit. Allow manual override.

**Q170**  
*Describe how you’d implement Solana program-level affiliate tracking.*  
*A:* Store affiliate ID in user PDA. On conversion, CPI to reward affiliate.

**Q171**  
*How do you handle Solana’s lack of native sleep or delay functions?*  
*A:* Use slot-based timers. Store `target_slot`. Check `Clock::slot` on entry.

**Q172**  
*What’s your strategy for securing Solana program admin keys?*  
*A:* Multi-sig with HSM. Require 3/5 signers. Rotate quarterly. Log all actions.

**Q173**  
*How do you prevent “reentrancy via CPI” in Solana programs?*  
*A:* Use guard flags. Never call untrusted programs with mutable accounts.

**Q174**  
*Describe how you’d implement Solana program-level discount codes.*  
*A:* Store valid codes in PDA. Verify code before applying discount. Track usage.

**Q175**  
*How do you handle Solana’s 1.4M CU limit in programs with heavy loops?*  
*A:* Unroll loops. Use lookup tables. Pre-calculate results off-chain.

**Q176**  
*What’s your approach to testing Solana programs with corrupted program data?*  
*A:* Fuzz with random opcodes. Test invalid BPF instructions. Panic on corruption.

**Q177**  
*How do you prevent “governance proposal spam” in Solana DAOs?*  
*A:* Require deposit (refundable). Minimum token threshold. DAO veto power.

**Q178**  
*Describe how you’d implement Solana program-level geofencing.*  
*A:* Use off-chain IP geolocation. Store allowed regions in PDA. Verify on entry.

**Q179**  
*How do you handle Solana’s lack of native HTTP calls in programs?*  
*A:* Use oracles. Never try on-chain. Fetch data off-chain, submit via IX.

**Q180**  
*What’s your strategy for securing Solana program configuration updates?*  
*A:* Require multi-sig. Add timelock. Log all changes. Allow DAO override.

**Q181**  
*How do you prevent “account data truncation” in Solana Borsh deserialization?*  
*A:* Check `data.len()` before deserializing. Panic on mismatch. Use `try_from_slice`.

**Q182**  
*Describe how you’d implement Solana program-level achievement systems.*  
*A:* Store achievements in PDA. Award on conditions. Allow display via CPI.

**Q183**  
*How do you handle Solana’s 32KB log limit per transaction?*  
*A:* Log only critical errors. Use structured codes. Ship verbose logs off-chain.

**Q184**  
*What’s your approach to optimizing Solana program startup time?*  
*A:* Minimize global constructors. Avoid heavy init. Lazy-load data.

**Q185**  
*How do you prevent “signature replay across chains” in Solana?*  
*A:* Include chain ID in signed message. Verify on-chain. Reject unknown chains.

**Q186**  
*Describe how you’d implement Solana program-level subscription billing.*  
*A:* Store `next_billing_slot` in PDA. Charge on access if past due. Allow grace period.

**Q187**  
*How do you handle Solana’s lack of native cron jobs?*  
*A:* Use client-triggered IX. Store `last_run_slot`. Check on entry.

**Q188**  
*What’s your strategy for securing Solana program metadata updates?*  
*A:* Require `update_authority` signature. Log all changes. Allow user verification.

**Q189**  
*How do you prevent “integer division truncation” in Solana financial math?*  
*A:* Use fixed-point. Multiply before divide. Round explicitly.

**Q190**  
*Describe how you’d implement Solana program-level leaderboards.*  
*A:* Store scores in PDAs. Update on actions. Allow querying top N.

**Q191**  
*How do you handle Solana’s 10MB account size limit for historical data?*  
*A:* Archive off-chain. Store only recent data on-chain. Use Merkle proofs for history.

**Q192**  
*What’s your approach to testing Solana programs with high rent pressure?*  
*A:* Use test validator with `--rent 2x`. Simulate rent exhaustion. Test recovery.

**Q193**  
*How do you prevent “metadata URI tampering” in Solana NFTs?*  
*A:* Store hash of metadata on-chain. Verify off-chain content matches hash.

**Q194**  
*Describe how you’d implement Solana program-level cooldown for NFT mints.*  
*A:* Store `last_mint_slot` per wallet. Reject if `current_slot - last_mint_slot < threshold`.

**Q195**  
*How do you handle Solana’s lack of native floating-point in APY calculations?*  
*A:* Use fixed-point math. Libraries like `solana-fixed`. Avoid division where possible.

**Q196**  
*What’s your strategy for securing Solana program emergency keys?*  
*A:* Air-gapped multi-sig. Stored in bank vaults. Require 5/7 signers. Test annually.

**Q197**  
*How do you prevent “account data padding attacks” in Solana?*  
*A:* Validate account size exactly. Reject if `data.len() != expected_size`.

**Q198**  
*Describe how you’d implement Solana program-level referral trees.*  
*A:* Store parent in user PDA. On action, reward parent, grandparent, etc. via CPI.

**Q199**  
*How do you handle Solana’s 64-instruction limit per transaction?*  
*A:* Use `invoke_signed` for batching. Split into multiple TXs. Use ALTs.

**Q200**  
*What’s your approach to securing Solana program source code post-deployment?*  
*A:* Publish on IPFS. Verify on-chain hash matches. Allow audits. Never obfuscate.

---

> **Final Importance Note**: These questions target *production-grade* Solana development — where mistakes cost millions. Prioritize security, economic incentives, and adversarial thinking over theoretical purity. The best Solana developers think like attackers, build like accountants, and ship like Navy SEALs.

