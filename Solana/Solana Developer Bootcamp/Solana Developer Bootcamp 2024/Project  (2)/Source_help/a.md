
---

## 🧠 **Overview: What This Contract Does**

This smart contract implements a **voting system** on Solana using the Anchor framework. It lets users:

1. **Initialize a poll** (name, description, voting start/end).
2. **Add candidates** to a poll.
3. **Vote** for candidates in a poll.

Each poll and candidate is a separate Solana **account**, and Anchor’s macros define how these are initialized and validated.

---

## 🗺️ **Mind Map: Anchor Voting Contract**

```
Voting Program (voting)
│
├── #[program]
│   └── voting::*
│       ├── initialize_poll(ctx, poll_id, start_time, end_time, name, description)
│       │   └── Sets up a new PollAccount with metadata
│       ├── initialize_candidate(ctx, poll_id, candidate)
│       │   └── Creates CandidateAccount under the Poll
│       └── vote(ctx, poll_id, candidate)
│           └── Increments candidate_votes if poll is active
│
├── #[derive(Accounts)]
│   ├── InitializePoll
│   │   ├── signer: user paying for the account creation
│   │   ├── poll_account: seeded by "poll", poll_id
│   │   └── system_program
│   ├── InitializeCandidate
│   │   ├── signer
│   │   ├── poll_account
│   │   ├── candidate_account: seeded by poll_id + candidate
│   │   └── system_program
│   └── Vote
│       ├── signer
│       ├── poll_account (must exist)
│       └── candidate_account (must exist)
│
├── #[account] Data Structures
│   ├── PollAccount
│   │   ├── poll_name: String (max 32 chars)
│   │   ├── poll_description: String (max 280 chars)
│   │   ├── poll_voting_start: u64
│   │   ├── poll_voting_end: u64
│   │   └── poll_option_index: u64 (tracks # of candidates)
│   └── CandidateAccount
│       ├── candidate_name: String (max 32 chars)
│       └── candidate_votes: u64
│
└── #[error_code]
    ├── VotingNotStarted
    └── VotingEnded
```

---

## 🔄 **Rust vs Solidity Concepts Mapping**

| Solidity             | Anchor (Rust)                      |
| -------------------- | ---------------------------------- |
| `contract Voting {}` | `#[program] pub mod voting {}`     |
| `mapping` / `struct` | `#[account]` (state accounts)      |
| `msg.sender`         | `Signer<'info>` in `Context`       |
| `require()`          | `if ... { return Err(...); }`      |
| `constructor`        | `initialize_poll(...)`             |
| `event`              | Manual via logs or CPI (not shown) |

---

## 🧠 Conceptual Anchors

### 🍱 1. **Accounts Are First-Class Citizens**

* Each "entity" (poll, candidate) is a separate **Solana account**.
* Accounts are defined using `#[account]` and passed into instruction handlers.

### 🧾 2. **PDA (Program Derived Address)**

* `seeds = [...]` → deterministic address generation.
* Ensures uniqueness per poll/candidate.

### 🧭 3. **Context Structs (`#[derive(Accounts)]`)**

* Defines which accounts the function expects.
* Anchor auto-validates ownership, constraints, seeds, etc.

---

## 🧩 Function Breakdown

### 🟢 `initialize_poll`

* Creates a poll account (PDA from `"poll"` + `poll_id`).
* Stores poll metadata.

### 🟨 `initialize_candidate`

* Creates a candidate account for a specific poll.
* Uses `poll_id` + `candidate` as seeds.
* Increments `poll_option_index` in the poll.

### 🔴 `vote`

* Gets Solana timestamp using `Clock::get()?.unix_timestamp`.
* Checks if poll is active.
* Increments vote count for a candidate.

---


