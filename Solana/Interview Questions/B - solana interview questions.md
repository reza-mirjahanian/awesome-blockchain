


## I. Core Solana Architecture & Runtime (Q1–Q40)

**Q1.** How does Solana’s Sealevel runtime enable parallel execution, and what are the main pitfalls developers must avoid?
*A:* Sealevel runs transactions in parallel if they touch disjoint accounts. Pitfalls: forgetting to declare all read/write accounts, leading to runtime errors or serialization bottlenecks. Best practice: always predeclare accounts explicitly.

> Importance: Ensures apps scale efficiently, core skill for Solana devs.

**Q2.** Explain how Solana’s optimistic concurrency differs from Ethereum’s transaction model.
*A:* Ethereum executes serially; Solana allows speculative parallel execution. Conflict resolution is via account locks, not global ordering. This reduces throughput bottlenecks but requires precise account modeling.

**Q3.** Why might a transaction fail under `AccountInUse` and how do you debug it?
*A:* Another transaction locks the same account simultaneously. Debug via `simulateTransaction`, restructure instruction batching, or split hot accounts into PDAs.

**Q4.** How would you optimize state-heavy programs on Solana that frequently exceed the account size limit (10 MB)?
*A:* Use multiple PDAs for sharded state, store large data off-chain with Merkle commitments, or upgrade to account compression.

**Q5.** What’s the difference between `invoke_signed` and `invoke`, and when is misuse dangerous?
*A:* `invoke` calls another program with provided accounts; `invoke_signed` allows PDAs to sign via seeds. Misuse → PDA forgery or replay attacks. Best practice: never leak seeds client-side.

**Q6.** Describe a real-world failure caused by missing `realloc` security checks in Solana programs.
*A:* Attackers over-allocate accounts to bypass rent or overflow memory. Fix: always validate new size against program logic and enforce `owner` checks.

**Q7.** What happens if two cross-program invocations mutate the same account?
*A:* Solana runtime enforces single mutability; runtime will reject with `AccountBorrowFailed`. Requires transaction restructuring.

**Q8.** Why is account serialization format critical, and what’s the tradeoff between Borsh and custom layouts?
*A:* Serialization defines backward compatibility. Borsh = safe and portable; custom layouts = gas efficiency but higher risk of human error.

**Q9.** How does Solana prevent re-entrancy, and when could you still be vulnerable?
*A:* Accounts are locked, preventing concurrent state changes. Vulnerability arises if program delegates authority incorrectly across CPIs.

**Q10.** What’s the effect of Solana’s single leader per slot design on transaction liveness guarantees?
*A:* Ensures deterministic ordering per slot, but under network partition, leader failure = slot skipped. Clients must retry until finalization (\~32 slots).

---

## II. Advanced Rust & Program Design (Q41–Q80)

**Q41.** How do you enforce ownership invariants in Solana Rust programs?
*A:* Always check `account.owner == program_id` before mutating. Add explicit authority checks (signer, seeds).

**Q42.** What are the risks of unchecked `unsafe` in Solana Rust contracts?
*A:* Memory corruption, bypassing runtime checks → exploitable vulnerabilities. Best practice: isolate unsafe in audited libraries only.

**Q43.** How would you design an upgradeable Solana program while avoiding admin key centralization?
*A:* Use governance DAO (e.g., Realms) to manage upgrade authority. Multisig > single key.

**Q44.** Why is zero-copy deserialization often used in Solana, and what’s the downside?
*A:* Eliminates serialization overhead. Downside: strict memory alignment → brittle upgrades.

**Q45.** How do you prevent integer overflows in Solana programs handling tokens?
*A:* Use `checked_add`, `checked_mul`. Wrap in custom math utils. Avoid plain `+` in financial ops.

---

## III. Tokenomics & SPL Standards (Q81–Q110)

**Q81.** Compare SPL Token Program vs custom token program. When build your own?
*A:* SPL = battle-tested, ecosystem standard. Custom needed for advanced rules (e.g., dynamic supply, conditional transfers).

**Q82.** How do you implement a streaming token vesting contract efficiently?
*A:* Use PDA escrow accounts with linear release via slot timestamps. Avoid per-slot state writes; compute lazily on claim.

**Q83.** Explain token account rent exemptions and how failing to enforce them can cause user fund loss.
*A:* Accounts with insufficient SOL for rent are closed, tokens lost. Always top-up or design rent-free PDAs.

**Q84.** Why are Associated Token Accounts (ATA) safer than custom token accounts?
*A:* Unique PDA ensures no duplicate ownership. Prevents phishing and token misdirection.

---

## IV. Security & Auditing (Q111–Q140)

**Q111.** Outline common Solana-specific attack vectors not seen in Ethereum.
*A:* Account inflation, PDA forgery, runtime `realloc` misuse, CPI recursion abuse.

**Q112.** How would you detect race conditions across parallel Solana transactions?
*A:* Simulate with conflicting tx, fuzz with custom cluster. Use metrics on `AccountInUse` reverts.

**Q113.** Explain Solana’s rent-exemption DoS vector.
*A:* Attackers spam near-empty accounts to exhaust state budget. Mitigation: enforce minimum SOL balances.

---

## V. System Design & Scalability (Q141–Q170)

**Q141.** How do you shard state across PDAs to support 10M+ users?
*A:* Hash user IDs into PDA namespaces. Use Merkle roots for global proofs.

**Q142.** When would you use account compression (Merkle trees) in Solana?
*A:* For NFT mints or large user registries. Cuts on-chain state from GBs → MBs.

**Q143.** How would you design Solana program state to support L2 rollups?
*A:* Store minimal commitments, offload execution to L2, settle via zk or fraud proofs.

---

## VI. Cross-Chain & Integrations (Q171–Q185)

**Q171.** What are the main risks of using Wormhole bridge in Solana?
*A:* Smart contract exploits (past \$320M hack), validator collusion. Mitigation: multisig verification + light clients.

**Q172.** How do you validate Ethereum-origin assets on Solana securely?
*A:* Rely on canonical bridge contracts with multisig verification. Long-term → zk light client verification.

---

## VII. Teamwork, Compliance, & Ops (Q186–Q200)

**Q186.** How would you explain Solana’s parallelism model to non-technical stakeholders?
*A:* Like multiple checkout lines at a grocery store: if shoppers don’t need the same cashier (account), they proceed in parallel.

**Q187.** How do you ensure HIPAA/GDPR compliance if building healthcare dApps on Solana?
*A:* Keep PHI off-chain; store hashes/commitments only. Encrypt client-side; use access-controlled PDAs.

**Q188.** How do you handle conflicts between product managers pushing features and auditors blocking them?
*A:* Prioritize security → explain that unsafe shortcuts may cause catastrophic exploits. Negotiate phased rollouts.

**Q189.** What metrics would you track in production Solana programs?
*A:* Tx success rate, `AccountInUse` collisions, CPI depth, rent top-ups.

**Q190.** What’s your approach to handling Solana runtime updates breaking deployed programs?
*A:* Maintain canary cluster for pre-release testing. Pin versions in CI/CD.

**Q200.** What soft skills distinguish senior Solana developers in cross-functional teams?
*A:* Translating low-level runtime details into business risk terms; mentoring juniors; mediating between engineers and compliance.


