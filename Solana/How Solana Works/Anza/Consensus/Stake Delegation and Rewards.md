# Stake Delegation & Rewards 🪙

> Stakers delegate tokens to validators for voting; rewards are earned through participation, with slashing risk for misbehavior.

---

## 👥 Core Roles

- **Vote account (`VoteState`):**
  - Tracks votes, `credits`, `root_slot`, `commission`, and authorized keys
  - Holds commission lamports (not stake weight)
- **Stake account (`StakeStateV2`):**
  - Owned by staker; holds lamports, `stake`, `voter_pubkey`, `credits_observed`
  - Records activation/deactivation epochs, authorized staker/withdrawer
- **Passive delegation:** Multiple stake accounts → single vote account without validator interaction
- **Rewards pools:** 256 pre-created at genesis for high-volume reward redemption

---

## 🔄 Delegation Lifecycle

1. **Initialize & Delegate:**  
   Set `voter_pubkey`, copy vote `credits` to `credits_observed`, begin activation (warmup applies)
2. **Authorize Roles:**  
   Update `authorized_staker`/`authorized_withdrawer` (lock-up may require custodian)
3. **Deactivate:**  
   Schedule deactivation epoch; cooldown while still earning rewards/slashable
4. **Withdraw:**  
   Remove lamports beyond `effective + activating` stake (usually after cooldown)
5. **Lock-up:**  
   Restricts withdrawals/authority changes until epoch met or custodian approves

---

## 🗳️ Vote Program Basics

- **Initialize:** Define `node_pubkey`, authorized voter/withdrawer, commission
- **Authorize:** Change authorized voter/withdrawer (with or without seed derivation)
- **Vote:** Submit votes under Tower BFT lockout rules
- **Commission:** Percentage of delegator rewards taken by validator at claim time

---

## 💰 Rewards Mechanics

- **Inflation-funded:** Fixed epoch budget; rewards available post-epoch
- **Points System:**  
  - **Credits** earned when votes become roots  
  - **Points = credits × stake**
  - Total network points → per-point value for epoch
- **Redemption:** Rewards = stake’s points × point value; split between stake & vote accounts (commission deducted)
- **Economics:** Lower participation boosts per-point value for active validators/stakers

---

## 🧊 Warmup, Cooldown & Withdrawal

- **Warmup rate:** Max 25% net stake change/epoch; split into `effective` & `activating`
- **Cooldown symmetry:** Mirrors warmup; stake is `deactivating` but still active for rewards & slashing
- **Withdrawal:** Only excess over `effective + activating` is withdrawable
- **Bootstrap stakes:** Skip warmup; earn rewards on effective portion immediately

---

## ✅ Design Benefits

- Single commission covers all delegators to a vote account
- Each stake independently claims rewards without resetting global credits
- Clear custody split: vote program handles slashing; stake program manages funds & delegation