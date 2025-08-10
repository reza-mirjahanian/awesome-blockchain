## 🔄 Transaction Processing Lifecycle in HyperGrid

---

### 🛡️ **Validator Involvement**

1. **Transaction Initiation** 📤

   * Users send **transaction requests** (*tx requests*) to the network.
   * A **validator** is chosen based on **predetermined selection rules**.

2. **Processing & Proof Generation** ⚙️

   * **Firedancer Execution Engine** – *Executes transactions rapidly within each Grid for real-time state transitions*.
   * Chosen validator uses **Sonic ZK-Coprocessor** to **construct a slot** and **generate a Zero-Knowledge Proof (ZK proof)**.

3. **Settlement & Data Availability** 📦

   * **ZK proofs** are **batched** and submitted to **Solana L1** for settlement.
   * **Transaction logs** sent to **Celestia** to guarantee **data availability**.

4. **Security & Efficiency** 🚀

   * Combines **advanced cryptography** with **distributed architecture** for **fast, secure transaction processing**.

---

### 🔁 **State Transition Flow**

1. **Tracking State Changes** 🧾

   * From **state(N)** → **state(N+1)**.
   * All **intermediate events** are **hashed** to form **Proof of History (PoH)**.

2. **Validator Verification** 🔍

   * PoH is **verifiable by validators**, confirming **legitimacy of state transitions**.

3. **PoH Function** 📏

   ```
   PoH{n,n+1} = H(Sn ∥ E{n,n+1} ∥ Sn+1)
   ```

   * **H** – Hash function
   * **Sn** – State before transition
   * **E{n,n+1}** – Events between states
   * **Sn+1** – State after transition
