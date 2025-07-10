# Building a Voting Application on Solana with Anchor

## Introduction to the Voting Application

- **Purpose**: Create a decentralized **voting application** using **Solana blockchain** to enable instant vote calculation and user verification via **key pairs**.
- **Key Features**:
  - Manage accounts for polls and candidates.
  - Utilize **Program Derived Addresses (PDAs)** for secure and efficient lookups.
  - Build and test locally using a **Solana test validator**.
- **Comparison to Traditional Voting**:
  - Traditional systems rely on human verification and slow ledger updates.
  - Blockchain enables *instant vote tallying* and *secure verification*.

## Setting Up the Local Development Environment

### Prerequisites for Development
- **Rust**: Programming language for writing Solana smart contracts.
- **Solana Toolchain**: Tools for creating key pairs, compiling contracts, and running a local validator.
- **Anchor Framework**: Simplifies Solana smart contract development.
- **Node.js Package Manager**: Options include Yarn, NPM, Pnpm, or Bun for JavaScript dependencies.

### Step-by-Step Environment Setup
1. **Install Rust**:
   - Visit the **Anchor documentation** for installation instructions.
   - Run the following command in your terminal:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Follow prompts for standard installation.
   - Update the **PATH variable**:
     ```bash
     source $HOME/.cargo/env
     ```
   - Verify installation:
     ```bash
     rustup --version
     ```
     *Expected Output*: `rustup 1.27.1`

2. **Install Solana Toolchain**:
   - Copy and paste the installation command:
     ```bash
     sh -c "$(curl -sSfL https://release.solana.com/v1.18.18/install)"
     ```
   - Export the binary to the PATH:
     ```bash
     export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
     ```
   - Verify installation:
     ```bash
     solana --version
     ```
     *Expected Output*: `solana-cli 1.18.18`

3. **Install a Node.js Package Manager**:
   - Install **Yarn**, **NPM**, or another preferred package manager.
   - Example for Yarn:
     ```bash
     npm install -g yarn
     ```

4. **Install Anchor Version Manager (AVM)**:
   - Install Anchor to compile Solana smart contracts:
     ```bash
     cargo install --git https://github.com/coral-xyz/anchor avm --locked
     ```
   - Set Anchor version to 0.30.1:
     ```bash
     avm use 0.30.1
     ```
   - Verify Anchor installation:
     ```bash
     anchor --version
     ```
     *Expected Output*: `anchor-cli 0.30.1`

5. **Create a Solana dApp Project**:
   - Use `npx` to scaffold a Solana project:
     ```bash
     npx create-solana-dapp voting-dapp
     ```
   - Select options: **Next.js** for frontend, **Tailwiper** for styling, include a counter program.
   - Navigate to the project directory and install dependencies:
     ```bash
     cd voting-dapp
     npm install
     ```
   - Run the development server:
     ```bash
     npm run dev
     ```
   - Access the placeholder app at `http://localhost:3000`.

### Running a Local Solana Validator
- Start a **Solana test validator** to simulate the blockchain locally:
  ```bash
  solana-test-validator
  ```
- *Note*: On Windows, use **Windows Subsystem for Linux (WSL)** for compatibility.
- Verify the validator is running:
  - Check port **8899** for JSON RPC activity.
  - Observe slot processing (e.g., slot 1,300,000).
- Use **Solana Explorer** for local monitoring:
  - Visit `explorer.solana.com`.
  - Set a custom RPC URL to `http://localhost:8899`.
  - View live stats like finalized slots and transaction details.

## Architectural Design of the Voting Application

### Overview
- **Goal**: Build a voting app to allow users to vote for *crunchy* or *smooth* peanut butter.
- **Real-World Application**: Extends to decentralized elections (e.g., school representative voting).
- **Core Mechanism**: Use **public/private key pairs** for voter identification and instant vote counting.

### Data Structure Design
- **Accounts**:
  - **Poll Account**: Stores poll metadata.
    - Fields:
      - `poll_id`: Unique identifier (U64).
      - `description`: Poll question or details (String, max 280 characters).
      - `poll_start`: Unix timestamp for poll start (U64).
      - `poll_end`: Unix timestamp for poll end (U64).
      - `candidate_amount`: Number of candidates in the poll (U64).
    - Seeds for PDA: `poll_id`.
  - **Candidate Account**: Stores candidate-specific data.
    - Fields:
      - `candidate_name`: Name of the candidate (String, max 32 characters).
      - `candidate_votes`: Vote count (U64).
    - Seeds for PDA: `poll_id`, `candidate_name`.

- **Instructions**:
  - `initialize_poll`: Creates a new poll with provided parameters.
  - `initialize_candidate`: Adds a candidate to a poll and increments candidate count.
  - `vote`: Increments the vote count for a specific candidate.

### Program Derived Addresses (PDAs)
- **Purpose**: Enable efficient and secure account lookups.
- **Poll Account PDA**:
  - Seed: `poll_id` (e.g., incremental numbers 1, 2, 3).
  - Allows lookup of polls by ID.
- **Candidate Account PDA**:
  - Seeds: `poll_id`, `candidate_name`.
  - Ties candidates to specific polls for unique identification.
- **Importance**: Predefining PDAs prevents the need for costly account restructuring later.

## Writing the Voting Smart Contract

### Project Setup
- Open the project in an IDE like **Visual Studio Code**.
- Navigate to the `anchor` folder, specifically `programs/src/lib.rs`.
- Update the program name from `counter` to `voting`:
  ```rust
  use anchor_lang::prelude::*;

  declare_id!("YOUR_PROGRAM_ID_HERE");

  #[program]
  pub mod voting {
      use super::*;
  }
  ```
- Ensure **Anchor version 0.30.1** is used in dependencies.
- Build the project to verify setup:
  ```bash
  cd anchor
  anchor build
  ```

### Instruction 1: Initialize Poll
- **Purpose**: Create a poll account with specified metadata.
- **Code**:
  ```rust
  pub fn initialize_poll(
      ctx: Context<InitializePoll>,
      poll_id: u64,
      description: String,
      poll_start: u64,
      poll_end: u64
  ) -> Result<()> {
      let poll = &mut ctx.accounts.poll;
      poll.poll_id = poll_id;
      poll.description = description;
      poll.poll_start = poll_start;
      poll.poll_end = poll_end;
      poll.candidate_amount = 0;
      Ok(())
  }
  ```
- **Context Struct**:
  ```rust
  #[derive(Accounts)]
  #[instruction(poll_id: u64)]
  pub struct InitializePoll<'info> {
      #[account(
          init,
          payer = signer,
          space = 8 + Poll::INIT_SPACE,
          seeds = [poll_id.to_le_bytes().as_ref()],
          bump
      )]
      pub poll: Account<'info, Poll>,
      #[account(mut)]
      pub signer: Signer<'info>,
      pub system_program: Program<'info, System>,
  }
  ```
- **Poll Struct**:
  ```rust
  #[account]
  #[derive(InitSpace)]
  pub struct Poll {
      pub poll_id: u64,
      #[max_len(280)]
      pub description: String,
      pub poll_start: u64,
      pub poll_end: u64,
      pub candidate_amount: u64,
  }
  ```
- **Key Notes**:
  - Use `#[account(init)]` to create the poll account.
  - `space = 8 + Poll::INIT_SPACE` reserves space for the account (8 bytes for discriminator + calculated space).
  - `#[max_len(280)]` limits the description to 280 characters.
  - Seeds ensure unique PDA for each poll.

### Instruction 2: Initialize Candidate
- **Purpose**: Add a candidate to a poll and increment the poll’s candidate count.
- **Code**:
  ```rust
  pub fn initialize_candidate(
      ctx: Context<InitializeCandidate>,
      _poll_id: u64,
      candidate_name: String
  ) -> Result<()> {
      let candidate = &mut ctx.accounts.candidate;
      let poll = &mut ctx.accounts.poll;
      candidate.candidate_name = candidate_name;
      candidate.candidate_votes = 0;
      poll.candidate_amount += 1;
      Ok(())
  }
  ```
- **Context Struct**:
  ```rust
  #[derive(Accounts)]
  #[instruction(poll_id: u64, candidate_name: String)]
  pub struct InitializeCandidate<'info> {
      #[account(
          mut,
          seeds = [poll_id.to_le_bytes().as_ref()],
          bump
      )]
      pub poll: Account<'info, Poll>,
      #[account(
          init,
          payer = signer,
          space = 8 + Candidate::INIT_SPACE,
          seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
          bump
      )]
      pub candidate: Account<'info, Candidate>,
      #[account(mut)]
      pub signer: Signer<'info>,
      pub system_program: Program<'info, System>,
  }
  ```
- **Candidate Struct**:
  ```rust
  #[account]
  #[derive(InitSpace)]
  pub struct Candidate {
      #[max_len(32)]
      pub candidate_name: String,
      pub candidate_votes: u64,
  }
  ```
- **Key Notes**:
  - Poll account is mutable to increment `candidate_amount`.
  - Candidate account uses two seeds: `poll_id` and `candidate_name`.
  - `#[max_len(32)]` limits candidate names to 32 characters.

### Instruction 3: Vote
- **Purpose**: Increment the vote count for a specific candidate.
- **Code**:
  ```rust
  pub fn vote(
      ctx: Context<Vote>,
      _poll_id: u64,
      _candidate_name: String
  ) -> Result<()> {
      let candidate = &mut ctx.accounts.candidate;
      candidate.candidate_votes += 1;
      Ok(())
  }
  ```
- **Context Struct**:
  ```rust
  #[derive(Accounts)]
  #[instruction(poll_id: u64, candidate_name: String)]
  pub struct Vote<'info> {
      #[account(
          seeds = [poll_id.to_le_bytes().as_ref()],
          bump
      )]
      pub poll: Account<'info, Poll>,
      #[account(
          mut,
          seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
          bump
      )]
      pub candidate: Account<'info, Candidate>,
      pub signer: Signer<'info>,
  }
  ```
- **Key Notes**:
  - No account creation, so no `system_program` or `init` required.
  - Candidate account is mutable to update `candidate_votes`.
  - Signer is included for transaction authorization.

## Testing the Smart Contract with Anchor-Bankrun

### Setting Up the Testing Environment
- **Tool**: **Anchor-Bankrun** for testing Solana smart contracts.
- Install Anchor-Bankrun:
  ```bash
  npm install anchor-bankrun
  ```
- **Setup Steps**:
  1. Copy the compiled binary (`voting.so`) from `target/deploy` to a `fixtures` folder.
  2. Copy the Interface Definition Language (IDL) file (`voting.json`) from `target/idl` to the project root.
  3. Import program types from `target/types/voting.ts`.
- **Test File Configuration** (`voting.spec.ts`):
  ```typescript
  import * as anchor from "@coral-xyz/anchor";
  import { Program } from "@coral-xyz/anchor";
  import { Voting } from "../target/types/voting";
  const IDL = require("../target/idl/voting.json");

  describe("voting", () => {
      let provider, votingProgram;
      const votingAddress = new anchor.web3.PublicKey("YOUR_PROGRAM_ID_HERE");

      beforeAll(async () => {
          const { startAnchor } = require("anchor-bankrun");
          const context = await startAnchor("", [{ name: "voting", programId: votingAddress }], []);
          provider = context.banksClient;
          votingProgram = new Program<Voting>(IDL, provider);
      });
  });
  ```

### Test 1: Initialize Poll
- **Purpose**: Verify poll account creation with correct metadata.
- **Code**:
  ```typescript
  it("Initialize Poll", async () => {
      const pollId = new anchor.BN(1);
      const description = "What is your favorite type of peanut butter?";
      const pollStart = new anchor.BN(0);
      const pollEnd = new anchor.BN(1800000000); // Future timestamp

      await votingProgram.methods
          .initializePoll(pollId, description, pollStart, pollEnd)
          .rpc();

      const pollAddress = anchor.web3.PublicKey.findProgramAddressSync(
          [pollId.toArrayLike(Buffer, "le", 8)],
          votingAddress
      )[0];

      const poll = await votingProgram.account.poll.fetch(pollAddress);
      console.log(poll);

      expect(poll.pollId.toNumber()).toEqual(1);
      expect(poll.description).toEqual(description);
      expect(poll.pollStart.toNumber()).toBeLessThan(poll.pollEnd.toNumber());
  });
  ```
- **Run Test**:
  ```bash
  anchor test --skip-local-validator --skip-deploy
  ```
- **Expected Output**: Poll account initialized with `poll_id = 1`, correct description, and `poll_start < poll_end`.

### Test 2: Initialize Candidate
- **Purpose**: Verify candidate account creation and poll’s candidate count increment.
- **Code**:
  ```typescript
  it("Initialize Candidate", async () => {
      const pollId = new anchor.BN(1);
      const candidateNames = ["crunchy", "smooth"];

      for (const name of candidateNames) {
          await votingProgram.methods
              .initializeCandidate(pollId, name)
              .rpc();

          const candidateAddress = anchor.web3.PublicKey.findProgramAddressSync(
              [pollId.toArrayLike(Buffer, "le", 8), Buffer.from(name)],
              votingAddress
          )[0];

          const candidate = await votingProgram.account.candidate.fetch(candidateAddress);
          console.log(`${name} candidate:`, candidate);
          expect(candidate.candidateVotes.toNumber()).toEqual(0);
      }
  });
  ```
- **Expected Output**: Candidates “crunchy” and “smooth” initialized with zero votes.

### Test 3: Vote
- **Purpose**: Verify vote increment for a candidate.
- **Code**:
  ```typescript
  it("Vote", async () => {
      const pollId = new anchor.BN(1);
      const candidateName = "smooth";

      await votingProgram.methods
          .vote(pollId, candidateName)
          .rpc();

      const smoothAddress = anchor.web3.PublicKey.findProgramAddressSync(
          [pollId.toArrayLike(Buffer, "le", 8), Buffer.from(candidateName)],
          votingAddress
      )[0];

      const smoothCandidate = await votingProgram.account.candidate.fetch(smoothAddress);
      console.log("Smooth candidate after vote:", smoothCandidate);
      expect(smoothCandidate.candidateVotes.toNumber()).toEqual(1);
  });
  ```
- **Expected Output**: Candidate “smooth” has one vote after execution.

### Debugging Tips
- **Common Issues**:
  - Missing `mut` keyword on accounts can prevent updates (e.g., `candidate_votes` not incrementing).
  - Incorrect seed order in PDAs can lead to invalid account lookups.
- **Solutions**:
  - Add `#[account(mut)]` for accounts that need modification.
  - Use `msg!` in Rust to log values for debugging:
    ```rust
    msg!("Candidate votes: {}", candidate.candidate_votes);
    ```
  - Check **Solana Stack Exchange** for error resolution, ensuring all relevant code is provided.

## Best Practices for Solana Development

- **Build Early and Often**:
  - Run `anchor build` frequently to catch errors early.
- **Use Local Validator**:
  - Test on `solana-test-validator` to iterate quickly without network costs.
- **Leverage Anchor Macros**:
  - `#[derive(InitSpace)]` simplifies account space calculation.
  - `#[max_len]` enforces string length limits.
- **Testing with Anchor-Bankrun**:
  - Skip heavy validator setup with `--skip-local-validator` and `--skip-deploy`.
  - Validate account data with `expect` assertions.
- **PDA Design**:
  - Plan seeds carefully to ensure efficient and unique account identification.
  - Avoid restructuring accounts later by defining PDAs upfront.

## Next Steps

- **Frontend Integration**:
  - Use **Next.js** to build a user interface for the voting app.
  - Connect to the Solana smart contract using the generated IDL.
- **Social Features**:
  - Enhance the app with social interactions (e.g., sharing poll results).
- **Productionization**:
  - Implement server-side caching for poll IDs and descriptions.
  - Perform incremental lookups for displaying polls on the frontend.
- **Resources**:
  - **Solana Documentation**: For CLI and toolchain details.
  - **Anchor Documentation**: For framework-specific guidance.
  - **Solana Stack Exchange**: For community support and error resolution.