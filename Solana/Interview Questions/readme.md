

https://solanacompass.com/

### Core Solana Concepts & Architecture

1.  **Q: Explain the key innovations of Solana's architecture (Proof of History, Tower BFT, etc.) that enable its high throughput.**
    *   **A:** PoH provides a cryptographic clock for transaction ordering, decoupling time from consensus. Tower BFT is a PoH-optimized version of PBFT that uses this clock for faster leader rotation and finality. This allows validators to process transactions in parallel (Sealevel) and store state efficiently (Cloudbreak).

2.  **Q: What is a Solana "slot" and how does it differ from an Ethereum "block"?**
    *   **A:** A slot is a unit of time (~400ms) designated by the PoH clock for a leader to produce a block. Multiple slots can contain blocks, and a block can be spread across multiple slots. This is a temporal concept vs. Ethereum's discrete, sequential block-based model.

3.  **Q: Describe the lifecycle of a transaction on Solana from creation to confirmation.**
    *   **A:** 1) Client creates and signs tx. 2) Sent to a leader's RPC node. 3) Leader bundles txs into an entry and appends to the PoH stream. 4) Entry is gossiped to other validators. 5) Validators vote on the PoH sequence (Tower BFT). 6) After a supermajority of votes, the state is finalized.

4.  **Q: What is the role of the Leader in Solana's consensus mechanism?**
    *   **A:** The leader for a given slot is responsible for producing entries to the PoH stream, batching transactions, and gossiping the block to the rest of the network. The role rotates among validators based on staked weight.

5.  **Q: How does Solana's fee market work to prevent spam?**
    *   **A:** Fees are composed of a base fee per signature and a per-write-byte fee. Additionally, there is a **state rent** mechanism where accounts must maintain a minimum balance or be purged. Most importantly, **priority fees** (tips) can be added to prioritize transaction inclusion during network congestion.

6.  **Q: What is a Solana Epoch and what happens during an epoch boundary?**
    *   **A:** An epoch is a period of ~2-3 days (~432,000 slots). At the boundary, staking rewards are distributed, the leader schedule for the next epoch is calculated based on the new stake weights, and voting credits are tallied.

7.  **Q: Explain the concept of "rent" in Solana. How do rent-exempt accounts work?**
    *   **A:** Rent is a cost for storing data on-chain. An account is rent-exempt if its balance is above a threshold (e.g., 2 years worth of rent). Exempt accounts are not purged. This conserves global state space.

8.  **Q: What are the different types of accounts in Solana?**
    *   **A:** **Executable:** Programs (smart contracts). **Non-Executable:** Data accounts. **System-owned:** Standard data accounts. **Program-derived address (PDA):** Accounts derived from a program's ID, lacking a private key, and owned by that program.

9.  **Q: What is a Program Derived Address (PDA)? Why is it a critical concept?**
    *   **A:** A PDA is an address derived from a program ID and a set of seeds (e.g., "user_wallet"). It does not have a corresponding private key, allowing programs to programmatically sign for them (`invoke_signed`). This is fundamental for Cross-Program Invocation (CPI) and state management.

10. **Q: How does Solana's parallel execution (Sealevel) work?**
    *   **A:** The runtime uses the information in a transaction (all accounts it will read from or write to) to schedule non-overlapping transactions in parallel. Transactions that touch the same accounts are executed sequentially.

### Rust & Solana Program Development

11. **Q: What are the three core traits a Solana program must implement using the `anchor_lang` framework?**
    *   **A:** `#[derive(Accounts)]` for validation, `#[account]` for data serialization/deserialization, and the program logic function typically wrapped in `#[program]`.

12. **Q: In vanilla Solana programs (without Anchor), how do you deserialize incoming instruction data?**
    *   **A:** Use the `solana_program::program_error::ProgramError` and `bytemuck` or `borsh` crates to cast a slice of the instruction data into a Rust struct. `borsh::from_slice(&instruction_data)` is common.

13. **Q: What is the purpose of the `entrypoint!` macro?**
    *   **A:** It defines the main entry point that the Solana runtime calls to process instructions. It wraps the provided function to handle the calling convention between the runtime and the program.

14. **Q: How do you safely increment a integer in a Solana account without causing an integer overflow?**
    *   **A:** Use Rust's checked arithmetic methods: `my_number.checked_add(1).unwrap()` or handle the `Option` returned by `checked_add` gracefully to avoid panics.

15. **Q: Explain the process of Cross-Program Invocation (CPI). How does a program sign for a PDA in a CPI?**
    *   **A:** A CPI is when one program calls an instruction on another. You use `invoke` or `invoke_signed`. To have the calling program sign as a PDA, you use `invoke_signed`, providing the seeds used to derive the PDA and the program ID.

16. **Q: What is the re-entrancy vulnerability and how does Solana's architecture inherently protect against it?**
    *   **A:** Re-entrancy is when a contract is interrupted and called again before its first execution finishes. Solana programs are **stateless**; they cannot call other programs recursively within the same transaction. All CPIs are executed to completion before the calling program resumes, eliminating classic re-entrancy.

17. **Q: How do you create and initialize a new PDA account from within a program?**
    *   **A:** Use the `system_instruction::create_account` and then `invoke_signed` with the seeds and bump seed for the PDA. Alternatively, in Anchor, use the `#[account(init, payer = user, space = 8 + 32, seeds = [...], bump)]` constraint.

18. **Q: What is the significance of the `8` in `space = 8 + ...` when initializing an account?**
    *   **A:** The first 8 bytes are reserved for the account discriminator. Anchor automatically prepends this discriminator to identify the account type within your program.

19. **Q: How does Anchor's `Accounts` struct and the `#[account(...)]` macro provide security?**
    *   **A:** It performs pre-flight validation on all incoming accounts *before* the program logic runs. This includes checking signers, ownership, deserialization, and custom constraints, centralizing security checks and reducing boilerplate.

20. **Q: What is a common seed for deriving a user-specific PDA?**
    *   **A:** The user's public key is a very common seed (e.g., `&[b"user-stats", user.key.as_ref()]`). This creates a unique PDA for each user.

21. **Q: How do you handle errors gracefully in an Anchor program?**
    *   **A:** Define custom error codes using `#[error_code]` and then return them using `err!(MyError::SomeVariant)` or use `require!(condition, MyError::SomeVariant)` for assertions.

22. **Q: What is the purpose of the `Clock` sysvar? Name a use case.**
    *   **A:** It provides a trusted source of on-chain time (slot, epoch, unix timestamp). A use case is enforcing a timelock on a withdrawal function.

23. **Q: Why is it important to validate every public key passed into your program?**
    *   **A:** To prevent someone from passing in a maliciously crafted account that your program did not intend to interact with. You must check the `key`, `owner`, and `is_signer` properties.

24. **Q: What does the `mut` constraint do in an Anchor `#[derive(Accounts)]` struct?**
    *   **A:** It enforces that the account must be mutable (`mut`), as the program intends to write data to it.

25. **Q: How would you implement a royalty mechanism for an NFT sale within a program?**
    *   **A:** Calculate the royalty amount from the sale price. In the transfer/swap instruction, perform CPIs to the System Program to transfer Lamports from the buyer's account to the creator's wallet(s), not just the seller's.

26. **Q: What is the difference between `init` and `init_if_needed` in Anchor?**
    *   **A:** `init` will always initialize a new account, failing if it already exists. `init_if_needed` checks existence first and only initializes if it's not already present. **Use with extreme caution** as it can be a source of replay attacks if not handled correctly.

27. **Q: Describe how you would implement a voting escrow model (like veToken) on Solana.**
    *   **A:** Users lock tokens in a program for a time. The program mints governance power (veTokens) as a linear function of lock amount and duration. These veTokens are non-transferable and stored in a user-specific PDA. Voting weight is calculated on-chain when needed for proposals.

28. **Q: How do you ensure a program upgrade is safe and doesn't break existing data?**
    *   **A:** 1) Thorough testing on devnet/testnet. 2) Ensure the new program's data structures are backwards compatible. 3) Use a buffer deployment to test the upgrade on mainnet-beta with a multisig. 4) Consider immutability for critical contracts.

29. **Q: What is a Solana "loader" and what are the two types?**
    *   **A:** The loader is the program responsible for loading and executing programs. The **BPF Loader** deploys upgradable programs. The **BPF Upgradeable Loader** allows for deploying, upgrading, and closing programs.

30. **Q: How would you design a program to be gas efficient?**
    *   **A:** Minimize compute units: use efficient data structures, avoid heavy loops, use `checked_arithmetic`, pack data tightly, and leverage PDAs for efficient storage. Profile instructions using `solana-logging` or `Sage`.

### Advanced Development & Security

31. **Q: What is a sandwich attack and how can an AMM mitigate it?**
    *   **A:** A bot spots a victim's trade, front-runs it to drive up price, and back-runs it to profit. Mitigations: trade execution in multiple ticks, fee tiers, and transaction routing that minimizes MEV extraction.

32. **Q: Explain how you would implement a permissioned swap pool (e.g., for a whitelist).**
    *   **A:** Store a whitelist of allowed mints or user wallets in the program's state. In the swap instruction, validate that the input mint or user's key exists in the whitelist PDA before executing the trade.

33. **Q: What is a flash loan on Solana? How is it implemented?**
    *   **A:** A loan that must be borrowed and repaid within the same transaction. Implemented by having the program logic require the borrowed amount is returned by the end of the instruction, using CPIs to lend and then recall the funds.

34. **Q: How do you prevent signature malleability in your program?**
    *   **A:** Use Ed25519's strict signature verification (e.g., `ed25519_dalek::PublicKey::verify_strict`). The `signer` check in Anchor/Runtime is already protected against this.

35. **Q: What is a "double spend" in the context of Solana transactions?**
    *   **A:** It's not a traditional double-spend. It's about a transaction being included multiple times if it has a reusable blockhash. Solution: use a recent blockhash and design idempotent instructions where possible, using nonces or state flags.

36. **Q: How can you implement a time-delayed or timelocked transaction?**
    *   **A:** Store a `unlock_timestamp` or `unlock_slot` in the account state. In the withdrawal function, use the `Clock` sysvar to check the current time and require that `clock.unix_timestamp >= unlock_timestamp`.

37. **Q: What is a "delegate" in the Token Program and what are its security implications?**
    *   **A:** A delegate is an authority granted by the token owner to manage a subset (or all) of their tokens. It's powerful. Programs must rigorously check that any instruction using a delegate is indeed signed by the *delegate's* key, not just the owner's.

38. **Q: Describe a common pitfall when closing an account and how to avoid it.**
    *   **A:** Failing to set the account's data to zero and/or the lamports to zero before closing. This can allow the new owner of the address to claim the lamports and potentially fake old state. Always zero out data and transfer lamports in one atomic instruction.

39. **Q: What is a "bump seed" and why do we store it in a PDA?**
    *   **A:** The bump seed is a value (usually the first that results in a valid PDA) used in the derivation. We often store it in the PDA's data to avoid having to re-derive it with `find_program_address` every time, saving compute units.

40. **Q: How would you design a program to be upgradeable from the start?**
    *   **A:** Use the BPF Upgradeable Loader. Separate logic from state. Ensure state accounts are owned by the program, not the loader, so they persist across upgrades. Use versioned APIs or feature flags if necessary.

41. **Q: What is a "program simulation" and how is it used in testing?**
    *   **A:** Using the `solana-program-test` crate to run programs in a local, simulated runtime environment. It's fast and ideal for unit testing program logic without a full validator node.

42. **Q: How do you perform integration testing for Solana programs?**
    *   **A:** Use the `solana-validator` crate (`test-validator`) to spin up a local validator node. Use BanksClient or the RPC client to send transactions and test interactions between multiple programs in a more realistic environment.

43. **Q: What is the purpose of the `#[interface]` macro in Anchor?**
    *   **A:** It allows you to call a vanilla Solana program (that doesn't use Anchor) from an Anchor program, by defining an interface with the instruction IDs and account structs.

44. **Q: Explain how you would implement a decentralized random number generator (RNG).**
    *   **A:** Use a combination of a recent, difficult-to-predict blockhash and a user-provided seed, hashed together. For stronger randomness, use an oracle like Switchboard's VRF (Verifiable Random Function).

45. **Q: What are Solana's compute units and how do you optimize for them?**
    *   **A:** Compute units are the measure of computational effort. Optimize by: using `return Ok(())` early, pre-calculating values off-chain, using efficient loops, and minimizing heap allocations.

46. **Q: How do you handle dynamic pricing (e.g., for a bonding curve) in a program?**
    *   **A:** Implement the pricing formula (e.g., linear, exponential) inside the program instruction. Calculate the price based on the current supply stored in the program's state. Use fixed-point math or very large integers to avoid floating-point imprecision.

47. **Q: What is a "wormhole" in Solana and how does it relate to cross-chain communication?**
    *   **A:** Wormhole is a generic message-passing protocol. It uses a network of "guardian" validators to attest to events on one chain (e.g., a token burn on Solana), which can then be relayed to another chain (e.g., to mint a wrapped token on Ethereum).

48. **Q: How would you design an anti-bot mechanism for a token launch?**
    *   **A:** Proof-of-Work captchas, transaction limits per wallet, requiring a prior NFT holder, or using a permissioned launchpad program that handles fair distribution.

49. **Q: What is a "shadow drive" and how is it used?**
    *   **A:** Shadow Drive is decentralized storage on the Solana blockchain. Programs can store hashes of off-chain data on-chain (in an account) and store the actual data on Shadow Drive, leveraging its cheaper storage costs.

50. **Q: Explain a challenge and solution for building an order book on-chain.**
    *   **A:** Challenge: High storage and compute cost for storing and matching orders. Solution: Use a central limit order book (CLOB) program that stores orders in a efficient data structure (e.g., a slab) and has designated market makers to provide liquidity and matching.

### Token & NFT Standards (SPL, Metaplex)

51. **Q: What is the difference between a Mint account and a Token Account in the SPL Token program?**
    *   **A:** The **Mint Account** holds the metadata of the token (supply, decimals, mint/freeze authority). A **Token Account** holds a specific wallet's balance of tokens from a specific mint.

52. **Q: How do you create a new fungible token using the SPL Token program?**
    *   **A:** 1) Create a new Mint account. 2) Initialize that mint with decimals and authorities. 3) Create a Token Account for a user. 4) Mint tokens to that user's Token Account.

53. **Q: What is the Associated Token Account (ATA) and why is it important?**
    *   **A:** It's a PDA-derived token account address for a specific wallet and mint. It ensures a canonical, discoverable address for a user's tokens, preventing the confusion of having multiple token accounts for the same mint.

54. **Q: How would you transfer an SPL token from within your program?**
    *   **A:** Perform a CPI to the SPL Token program's `transfer