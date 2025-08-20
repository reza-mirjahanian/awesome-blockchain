# 🚀 Getting Started with Solana Development

- **Install Solana CLI** 🔧  
  Download and install from the official Solana documentation.  
  Use commands like `solana --version` to verify installation.

- **Set Up Rust Environment** 🦀  
  Install Rust via `rustup`.  
  Configure for Solana:  
  ```bash
  rustup component add rustfmt
  cargo install cargo-build-bpf --features=program
  ```

- **Development Tools** 🛠️  
  - Everything in plain Rust for programs (smart contracts).  
  - Use CLI tools: `cargo` for building, `solana` for deployment and interaction.  
  - Community libraries available in Rust, JavaScript, Python, and Java.

- **Hackathon Tip** ⏰  
  Hackathon starts in 2 days—focus on quick setup to build and deploy.

# 📦 Understanding Accounts in Solana

- **Core Concept: Everything is an Account** 🔑  
  - Accounts store data, programs, and tokens.  
  - *Analogy*: Think of accounts as files in a filesystem.

- **Account Limitations** ⚠️  
  - Maximum size: 10 MB.  
  - Must pay **rent** using SOL to store data.  
  - Preload 2 years of rent to make an account *rent-exempt*.

- **Types of Data in Accounts** 📊  
  - Native SOL tokens.  
  - Program code (executable accounts).  
  - User data and state.

- **Supplementary: Account Structure**  
  Accounts include:  
  - Lamports (balance in smallest SOL units).  
  - Owner (program that controls it).  
  - Data (byte array for storage).

# 🔗 Interacting with the Solana Blockchain

- **RPC Nodes** 🌐  
  - Send transactions via JSON-RPC.  
  - APIs in Rust, JavaScript; community libs in Python/Java.

- **Transactions and Instructions** 📝  
  - Transactions contain one or more **instructions** (like function calls).  
  - Instructions invoke programs with accounts and data.

- **Programs vs. State** 🚫  
  - Programs hold no state—state is in separate accounts.  
  - Enables **upgradeable smart contracts** without special patterns.

- **Example: Viewing Transactions** 👀  
  Use Solana Explorer (explorer.solana.com) to inspect transactions, balances, and instructions.

# 🪙 Token Program (SPL)

- **Overview** 💰  
  - From Solana Program Library (SPL).  
  - Deploy tokens via CLI or JavaScript—no Rust needed.

- **Creating a Token** 🛠️  
  ```bash
  spl-token create-token
  ```  
  Creates a new token mint.

- **Account Types in Token Program** 📑  
  - **Mint Account**: Tracks supply, mint/freeze authority (created once per token).  
  - **Token Account**: Holds tokens for users (multiple per token).

- **Fungible vs. Non-Fungible Tokens** 🎟️  
  - Same standard for both.  
  - NFT: Set decimals to 0, supply to 1, freeze mint.  
  - Enhances reusability of programs.

- **Wrapped SOL** 🔄  
  - Create wrapped SOL to treat native SOL as SPL token.  
  - Fund account, initialize with predefined mint.  
  ```javascript
  // JavaScript example using @solana/spl-token
  const wrappedSolAccount = await createWrappedSolAccount(connection, payer, amount);
  ```

# 🔑 Program Derived Accounts (PDAs)

- **Definition** 📍  
  - Special accounts without private keys—owned by programs.  
  - Can only sign via the owning program.

- **Benefits** 🌟  
  - Predictable addresses (discoverable off-chain).  
  - Track program state securely.  
  - Enable program-owned authorities (e.g., mint control).

- **Creating PDAs** 🛠️  
  - Derived from seeds + program ID via SHA-256 hash.  
  - Example in Rust:  
  ```rust
  use solana_program::pubkey::Pubkey;
  let (pda, bump) = Pubkey::find_program_address(&[seed.as_bytes()], &program_id);
  ```

- **Use Cases** 🔍  
  - Auction systems: Hash creator + item for unique auction account.  
  - Bridges (e.g., Wormhole): Predictable mints from external addresses.

- **Related Concept: Associated Token Accounts** 🔗  
  - PDAs for user tokens: Derived from wallet + mint.  
  - Predictable, simplifies transfers without knowing exact addresses.

# 📚 Solana Program Library (SPL)

- **Reference Implementations** 📖  
  - Token program, AMM (Automated Market Maker).  
  - Borrow-lending with liquidations and cross-collateral.  
  - Utility: Memo program for transaction notes.

- **Repository** 🔗  
  GitHub: solana-labs/solana-program-library.  
  Explore for modifying or using existing programs.

- **Documentation** 📄  
  Extensive guides on token interactions, creating fungible/NFTs.

- **Supplementary: Memo Program Example** 📝  
  Add metadata to transactions:  
  ```bash
  solana program deploy --program-id MemoProgramAddress memo_program.so
  ```

# ⚛️ Scaffold for Quick Prototyping

- **DB Scaffold Project** 🏗️  
  - Bootstrap React app + simple Rust "Hello World" program.  
  - Unopinionated—add your own serialization (e.g., Borsh, Anchor).

- **Setup Steps** 🚀  
  1. Clone repository.  
  2. Run `yarn` for JavaScript dependencies.  
  3. Build Rust program with `cargo build-bpf`.  
  4. Deploy and interact.

- **Features** ✨  
  - Integrates with Phantom wallet.  
  - Airdrop SOL on devnet/testnet.  
  - Hooks for program interaction.

- **Serialization Options** 🔄  
  - Borsh (Rust native).  
  - Anchor: Generates client code, simplifies boilerplate.  
  - Protobuf or custom.

- **Example: Anchor Usage** 📦  
  Anchor is a framework for Solana programs:  
  ```rust
  #[program]
  pub mod hello_world {
      use super::*;
      pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
          Ok(())
      }
  }
  ```

# 🔄 Tokenization and Interoperability

- **Tokenization Beyond Currency** 💡  
  - Track ownership rights (e.g., car, options strikes).  
  - API for program-to-program interactions.

- **Non-Fungible Tokens (NFTs)** 🎨  
  - Not just art—represent unique assets.  
  - Visible in wallets, even if not tradable.

- **Integration with Serum DEX** 📈  
  - Central limit order book.  
  - Deploy markets via UI: Add token, start trading.  
  - Build on tokenized assets for composability.

- **Program Interfaces** 🤝  
  - Straightforward for Rust/JavaScript.  
  - Enables Primitives like markets, lending.

# ⚙️ Common CLI Interactions

- **Creating Tokens** 🪙  
  ```bash
  spl-token create-token --decimals 9
  ```

- **Minting Tokens** ➕  
  ```bash
  spl-token mint <TOKEN_ADDRESS> <AMOUNT>
  ```

- **Transferring Tokens** 🔄  
  ```bash
  spl-token transfer <TOKEN_ADDRESS> <AMOUNT> <RECIPIENT>
  ```

- **Airdrop SOL** 💸  
  ```bash
  solana airdrop 1 --url devnet
  ```

- **Querying Accounts** 🔍  
  ```bash
  solana account <ACCOUNT_ADDRESS>
  ```

# 📏 Transaction and Instruction Limits

- **Transaction Size** 📏  
  - Limited by UDP: ~1,200 bytes.  
  - ~30 accounts per transaction (32 bytes each).

- **Compute Units** ⚡  
  - 200,000 per instruction.  
  - Flat fee based on signers—no rebates.

- **Cross-Program Invocation (CPI)** 🔗  
  - Stack limit: 4 calls deep.  
  - All accounts must be passed upfront.

- **Workarounds** 🔄  
  - Split into multiple transactions.  
  - Use PDAs to reduce account count via predictability.

# 🎲 Handling Randomness

- **Current Limitations** 🚫  
  - No built-in VRF (Verifiable Random Function).

- **Approaches** 🛠️  
  - **Commit-Reveal Scheme**: Hash secret off-chain, reveal later.  
    Ideal for raffles/games where operator isn't a participant.  
  - Oracles for external randomness (upcoming sessions).  
  - Avoid clock/slot as source—predictable.

- **Example: Commit-Reveal** 🔒  
  1. Commit hash of random number.  
  2. After entries close, reveal and verify.  
  ```rust
  // Pseudo-code
  let commit = hash(random + salt);
  // Store commit in account
  // Later: verify hash(revealed) == commit
  ```

# 🔒 Security Best Practices

- **Input Validation** ✅  
  - Majority of code: Validate accounts, owners, programs.  
  - Check token program ID matches expected.

- **Common Errors** ⚠️  
  - Spoofed accounts: Ensure ownership.  
  - Missing checks on writable accounts.

- **Example from Token Swap** 📊  
  In processor.rs:  
  - Extract accounts in order.  
  - Validate each (e.g., is mint authority correct?).  
  ```rust
  let accounts_iter = &mut accounts.iter();
  let swap_account = next_account_info(accounts_iter)?;
  if swap_account.owner != program_id {
      return Err(ProgramError::IncorrectProgramId);
  }
  ```

- **Parallel Execution** ⚡  
  - Design for parallelism: Avoid global writable accounts.  
  - Separate markets/bids for independence.

# 🤝 Cross-Program Interactions

- **Calling Other Programs** 🔄  
  - Use CPI to invoke instructions in other programs.  
  - Pass all accounts in graph upfront.

- **Validation** 🔍  
  - Declare constants for expected program IDs.  
  - Compare passed accounts.

- **Token Transfers** 💸  
  - Involves: Token program, mint (read-only), source/dest accounts.  
  - Non-overlapping accounts allow parallel processing.

- **PDA Signatures** ✍️  
  - Program can "sign" its PDAs via runtime flag.  
  - Use `invoke_signed` for program-owned actions.

# 🛡️ Customizing Tokens and Programs

- **Custom Minting** 🔧  
  - Fork token program, deploy your own.  
  - Alternative: Use PDA-owned mint for vesting/schedules.

- **Vesting Example** ⏳  
  - Program owns mint authority.  
  - Release tokens based on slots/clock.

- **Rewarding Users** 🎁  
  - Off-chain tracker for Serum trades.  
  - Claim method in program.  
  - Burn tokens on usage.

- **Governance Tokens** 🗳️  
  - Distribute via claims or airdrops based on ownership.

# 🔍 Querying and Discovery

- **RPC Calls** 📡  
  - `getProgramAccounts`: All accounts for a program.  
  - `getTokenAccountsByOwner`: Tokens for a user.

- **Efficiency** ⚡  
  - Indexed for fast queries.  
  - Use for discovering PDAs/associated accounts.

- **Example: Get Token Accounts** 🔍  
  ```javascript
  const tokenAccounts = await connection.getTokenAccountsByOwner(owner, { programId: TOKEN_PROGRAM_ID });
  ```

# 📈 Advanced Topics: Bridges and Oracles

- **Wormhole Bridge** 🌉  
  - Bridges Ethereum (ERC-20, soon 721/1155) to Solana.  
  - Uses PDAs for predictable mints from ETH addresses.

- **Oracles** 🔮  
  - For external data/randomness.  
  - Upcoming: Sessions on integration.

- **Parallelism in Design** ⚙️  
  - Independent writable accounts (e.g., per market in DEX).  
  - Enhances throughput.

# 🏦 Borrow-Lending Platform

- **Features** 💼  
  - Supports liquidations, multiple markets.  
  - Cross-collateral.

- **Design Tip** 📐  
  - Separate permissioned vs. permissionless instructions.  
  - Reduces account needs per transaction.

- **Example Structure** 🏗️  
  - Markets as separate PDAs.  
  - User positions discoverable via owner queries.