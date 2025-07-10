## 🗳️ **Solana Voting dApp: Full Local Setup and Smart Contract Walkthrough**

---

### 📌 **Project Overview**

* Build a **voting dApp** using **Solana**, **Rust**, and **Anchor**.
* Enable **decentralized voting** using **public-private key pairs**.
* Votes are instantly verifiable and counted on-chain using **program-derived addresses (PDAs)**.
* Compare to traditional voting:

  * *Manual verification*
  * *Delayed results*
  * *Susceptible to tampering*

---

### ⚙️ **Local Development Environment Setup**

#### 1. **Install Rust**

* Visit the [Rust installation site](https://www.rust-lang.org/tools/install)
* Command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* Reload shell:

```sh
source $HOME/.cargo/env
```

* Verify:

```sh
rustup --version
```

---

#### 2. **Install Solana CLI**

* Command:

```sh
sh -c "$(curl -sSfL https://release.solana.com/v1.18.18/install)"
```

* Add to path:

```sh
export PATH="/path/to/solana/bin:$PATH"
```

* Verify:

```sh
solana --version
```

---

#### 3. **Install Anchor Framework**

* Use Cargo:

```sh
cargo install --git https://github.com/coral-xyz/anchor avm --locked
```

* Use specific Anchor version:

```sh
avm use 0.30.1
```

* Check version:

```sh
anchor --version
```

---

#### 4. **Create Project Scaffold**

* Scaffold the dApp:

```sh
npx create-solana-dapp
```

* Select:

  * **Project name:** `voting-dapp`
  * **Framework:** `Next.js`
  * **UI:** `TailwindCSS`

* Navigate and install dependencies:

```sh
cd voting-dapp
npm install
npm run dev
```

* App runs on `http://localhost:3000`

---

### 🧪 **Run Local Validator**

```sh
solana-test-validator
```

* Runs on port `8899`
* Mimics a real validator node
* Use with local CLI and UI explorer:

  * **Explorer:** `explorer.solana.com → Custom RPC URL → http://127.0.0.1:8899`

---

## 🧱 **Smart Contract Architecture**

### 📑 **Accounts**

#### 1. **Poll Account**

Stores poll-specific metadata.

```rust
pub struct Poll {
    pub poll_id: u64,
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}
```

* **Seeds**: `["poll", poll_id]`

---

#### 2. **Candidate Account**

Stores candidate info and vote count.

```rust
pub struct Candidate {
    pub candidate_name: String,
    pub candidate_votes: u64,
}
```

* **Seeds**: `["candidate", poll_id, candidate_name]`

---

### 🧾 **Instructions (Functions)**

#### 1. `initialize_poll`

Initializes a poll with metadata.

**Inputs:**

* `poll_id`: `u64`
* `description`: `String`
* `poll_start`, `poll_end`: `u64`

**Implementation:**

```rust
#[derive(Accounts)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [b"poll", &poll_id.to_le_bytes()],
        bump,
        payer = signer,
        space = 8 + Poll::INIT_SPACE
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}
```

---

#### 2. `initialize_candidate`

Adds a candidate to a poll.

**Inputs:**

* `poll_id`: `u64`
* `candidate_name`: `String`

**Implementation:**

```rust
#[derive(Accounts)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll", &poll_id.to_le_bytes()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init,
        seeds = [b"candidate", &poll_id.to_le_bytes(), candidate_name.as_bytes()],
        bump,
        payer = signer,
        space = 8 + Candidate::INIT_SPACE
    )]
    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}
```

* Increments `poll.candidate_amount`

---

#### 3. `vote`

Increases the candidate's vote count.

**Inputs:**

* `poll_id`: `u64`
* `candidate_name`: `String`

**Implementation:**

```rust
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account()]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"candidate", &poll_id.to_le_bytes(), candidate_name.as_bytes()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,
}
```

* Increments `candidate.candidate_votes`

---

## 🧪 **Testing With Anchor Bankrun**

### 🧰 Setup

Install:

```sh
npm install anchor-bankrun
```

Add `.so` and `.json` IDL to:

* `/fixtures/voting.so`
* `/fixtures/voting.json`

Update `Anchor.toml`:

```toml
[programs.localnet]
voting = "YourVotingProgramPublicKey"
```

---

### ✅ Sample Test: `initialize_poll`

```ts
it("Initialize poll", async () => {
  await votingProgram.methods
    .initializePoll(
      new anchor.BN(1),
      "Best Peanut Butter?",
      new anchor.BN(0),
      new anchor.BN(1893456000) // year 2030
    )
    .rpc();

  const [pollAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("poll"), new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
    votingProgram.programId
  );

  const poll = await votingProgram.account.poll.fetch(pollAddress);

  expect(poll.pollId.toNumber()).to.equal(1);
  expect(poll.description).to.equal("Best Peanut Butter?");
});
```

---

### ✅ Sample Test: `initialize_candidate`

```ts
await votingProgram.methods
  .initializeCandidate("crunchy", new anchor.BN(1))
  .rpc();

const [candAddr] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("candidate"), new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("crunchy")],
  votingProgram.programId
);

const candidate = await votingProgram.account.candidate.fetch(candAddr);
expect(candidate.candidateName).to.equal("crunchy");
expect(candidate.candidateVotes.toNumber()).to.equal(0);
```

---

### ✅ Sample Test: `vote`

```ts
await votingProgram.methods
  .vote("crunchy", new anchor.BN(1))
  .rpc();

const [candAddr] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("candidate"), new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("crunchy")],
  votingProgram.programId
);

const candidate = await votingProgram.account.candidate.fetch(candAddr);
expect(candidate.candidateVotes.toNumber()).to.equal(1);
```

---

### 🛠️ **Troubleshooting & Tips**

* **Always mark mutability:**

  ```rust
  #[account(mut)]
  ```

* **Ensure PDA seeds and bumps are correct.**

* **Use `anchor build` early and often.**

* **Validate account updates with logs:**

  ```rust
  msg!("Votes for candidate: {}", candidate.candidate_votes);
  ```

* **Common Errors:**

  * "account not mutable"
  * "seeds don't match"
  * "insufficient lamports" (missing payer)

---

### 📐 **Program Derived Addresses (PDAs)**

**Poll PDA:**

```rust
seeds = [b"poll", &poll_id.to_le_bytes()]
```

**Candidate PDA:**

```rust
seeds = [b"candidate", &poll_id.to_le_bytes(), candidate_name.as_bytes()]
```

* Avoid collisions
* Enable easy lookup by front-end
* Deterministic and unique

---

### 🌐 **Project Scaffold Overview**

```
voting-dapp/
├── programs/
│   └── voting/
│       └── src/
│           └── lib.rs
├── web/
│   └── pages/
│       └── index.tsx
├── tests/
│   └── voting.spec.ts
├── fixtures/
│   ├── voting.so
│   └── voting.json
```

* `programs`: Rust smart contract using Anchor
* `web`: Next.js front-end scaffold
* `tests`: Anchor Bankrun tests
* `fixtures`: compiled binary and IDL for testing

---

### 🔧 **Developer Workflow**

1. **Write contract logic** in `lib.rs`
2. **Run:**

```sh
anchor build
```

3. **Copy `.so` & `.json`** to `fixtures/`
4. **Run tests:**

```sh
anchor test --skip-local-validator --skip-deploy
```

5. **Check logs** and account values
6. **Repeat (iterate)**

---
