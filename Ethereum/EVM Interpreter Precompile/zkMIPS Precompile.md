**📌 What is a Precompile in zkMIPS?**  
- **Special-purpose circuit/table** designed to handle *specific cryptographic operations* (e.g., **hashing**, **elliptic pairings**, etc.).  
- Executes computation **outside the CPU table**, preventing transformation into standard MIPS instructions.  
- **Benefits:**  
  - 🚀 Significant performance boost for crypto-heavy workloads.  
  - ⏱ Reduced proof generation time.  
  - 💰 Lower computational cost.  

---

**🛠 How Precompiles Improve zkMIPS Performance**  
- Example: **SHA-256 Precompile**  
  - ~40% reduction in cost.  
  - Proof generation time cut by **50%**.  
- Keeps **proof size unchanged** while accelerating execution.  

---

**⚙ Using Precompiles in zkMIPS**  
- Exposed as **System Calls (syscalls)** → executed via *MIPS syscall instructions*.  
- SDK Integration:  
  - Include the **ZKM runtime** in your project.  
  - Replace slow cryptographic library calls with **precompile function calls**.  
- Example Usage:  
  - Replace `keccak256` from TinyKeccak with precompile-based `keccak256`.  
  - Result: Proof time drops dramatically while proof size remains constant.  

---

**🛠 Steps to Implement a Custom Precompile**  
### **1️⃣ Implement the Precompile Interface**
- Works like a **function** called from the main program.  
- Invokes the relevant syscall internally.  
- Example: Compress Precompile Interface  
  - Calls syscall for **SHA-256 extend** and **compress** phases.  

---

### **2️⃣ Implement Syscall Functions**
- **Syscall Basics:**  
  - Special MIPS instruction for system-level functionality.  
  - Requires a **unique syscall number** assignment.  
- **Implementation Steps:**  
  - Pass syscall number via **Register #2**.  
  - Pass inputs via **Register #4**.  
  - Emulator logic fetches register values → executes crypto computation.  
- **State Transitions:**  
  - Move crypto computations to **custom tables** outside the CPU table.  
  - Maintain **data dependency tracking** using memory/CPU table lookups.  

---

### **3️⃣ Implement STARK Circuits**  
- Follow **four main sub-steps** for efficiency:  

  **📍 Step A: Table Structure**  
  - Typically **two tables** per syscall:  
    1. **Core Table** – Main crypto logic.  
    2. **Sponge Table** – Input/output handling (memory read/write).  
  - Optimization: Merge into one table if logic is simple.  

  **📍 Step B: Column & Constraint Design**  
  - **Sponge Table** should have:  
    - ⏱ Timestamp (clock cycle)  
    - 📍 Addresses  
    - 📦 Input/Output values  
    - ⚑ Conditional flags  
  - Constraints are:  
    - Native evaluation on field elements.  
    - Recursive evaluation for proof compatibility.  
    - Max constraint degree = **3** (reduce with intermediate columns if needed).  

  **📍 Step C: Trace Generation**  
  - Generate table traces with **timestamp**, I/O values, flags.  
  - Compute intermediate values internally (not as input).  
  - **Power of Two Rule**: Pad table length with empty rows to nearest 2ⁿ.  

  **📍 Step D: Cross-Table Lookup**  
  - Verifies **data dependencies** between tables.  
  - Example lookups:  
    - Sponge Table ↔ Memory Table  
    - Sponge Table ↔ CPU Table  
    - Sponge Table ↔ Logic Table  
  - Match **timestamp + input value** pairs to ensure integrity.  

---

**💡 Additional Implementation Notes**  
- **Data Dependency Handling:**  
  - If precompile reads memory → push read ops into **Memory Trace Table**.  
  - If writes occur → ensure correct write verification across related tables.  
- **Recursive Proofing Context:**  
  - Constraints are validated both directly and in recursive aggregation.  
- **Performance Advice:**  
  - Minimize extra columns while ensuring constraint degree ≤ 3.  
  - Merge small, simple tables where possible.  

---

**🔍 Example Flow: Using SHA-256 Extend Precompile**  
1. **Assign Syscall Number** → e.g., 256.  
2. **Pass Input** → Register #4.  
3. **Run Inside Emulator** → Identify syscall, process SHA-256 extend logic.  
4. **Trace Tables Creation** → Core logic table + sponge table with timestamped I/O.  
5. **Cross-Table Lookups** → Confirm memory integrity & match with CPU table.  
6. **Proof Generation** → Optimized 2ⁿ padded table for STARK.  

---

**⛓ Related ZK & Performance Concepts**  
- **STARKs:** Scalable proof system without trusted setup, perfect for CPU/crypto verification.  
- **Table Lookup Arguments:** A method for *ensuring state consistency* across parallel execution tables.  
- **Recursive Proof Aggregation:** Allows batching many small proofs into a single final proof for L1 submission.  
- **Syscall Numbering Strategy:** Use a reserved & documented map to avoid developer collisions.  
- **Crypto-heavy workloads use cases:** zkEVM rollups, zkVM-based verifiable computations, blockchain light clients.  



# What is Precompile in zkVM? 🚀

- **Definition**: Precompiles are special-purpose circuit tables designed for proving one or a few cryptographic operations, such as hashing or elliptic curve pairing.
  - They handle cryptographic computations in dedicated circuits or tables *outside* the main CPU tables.
  - This avoids transforming complex cryptographic logic into MIPS instructions, which can be inefficient.
- **Performance Benefits** 💨:
  - Precompiles bring **significant improvements** in proof generation for cryptographic ops.
  - Example: Running SHA-256 precompile on zkMIPS reduced costs by ~40% and proofing time by half 📉.
- **Related Concepts**:
  - In zero-knowledge proofs (ZKPs), precompiles optimize for operations that are computationally intensive in general-purpose circuits.
  - Similar to Ethereum's precompiles (e.g., for ECDSA recovery), which offload ops to native code for gas efficiency.
  -Think of precompiles as "shortcut circuits" ⚡ to bypass slow instruction-by-instruction proving.

# How to Use Precompile in zkMIPS? 🛠️

- **Integration Method**: Precompiles are provided as system calls, executed via MIPS syscall instructions.
  - Supported syscalls include cryptographic hash functions like Keccak-256 and SHA-256.
- **Implementation Steps**:
  - Add the zkMIPS SDK (zkm-runtime) to your project.
  - Replace standard library calls with precompile versions.
    - Example: Use Keccak-256 precompile instead of tiny-keccak library for hashing.
- **Demo Example** 🎥:
  - Running Rust EVM on zkMIPS using a project template.
  - Compute code hash of account information.
    - With tiny-keccak: ~156 seconds proof time, ~450-500 KB proof size.
    - With precompile: Reduced to ~38 seconds, proof size remains similar.
- **Supplementary Info**:
  - zkMIPS emulates a MIPS processor in a ZK-friendly way, allowing verifiable execution of programs.
  - Precompiles reduce proof overhead by specializing circuits, akin to how GPUs accelerate specific tasks vs. CPUs.
  - Pro tip: Always benchmark proof times before/after to quantify gains 📊.

# Implementing Custom Precompile on zkMIPS 🔧

- **Overview**: Developers can implement custom precompiles during the proving phase without altering the verifier.
  - Three main steps: Implement precompile interface, syscall functions, and STARK circuits.
- **Step 1: Precompile Interface** 📝:
  - Treat it as a regular function in your Rust program.
  - Invoke syscall inside to handle precompile logic.
    - Example: For SHA-256 compress, call syscall for extend and compress functions.
  - Related: Interfaces abstract syscall details, similar to API wrappers in software development.

# Syscall Functions Implementation ⚙️

- **Basics**: Syscalls are MIPS instructions; implement logic and state transitions.
- **Key Sub-Steps**:
  - Assign a unique syscall number to your custom function.
  - Pass inputs via registers (e.g., syscall num in register 2, inputs in register 4).
  - In the emulator: Retrieve register values and handle logic based on syscall number.
    - Example: For SHA-256 extend, follow the algorithm from relevant specs/paper.
- **State Transition**:
  - Move cryptographic computations outside CPU tables by creating new tables.
  - Generate table traces for these new tables.
  - Maintain data dependencies: Push memory read/write ops to memory trace table for lookups.
    - Also applies to CPU table interactions.
- **Enhancements**:
  - Syscalls in MIPS are like interrupts in modern OSes, handling privileged operations.
  - In ZK context, ensure transitions are provable without revealing data 🔒.

# Implementing STARK Circuits 📐

- **Approach**: Follow four parallel steps for clarity (consider them iteratively in practice).
- **Step 1: Determine Tables**:
  - Typically, two tables per syscall:
    - **Control Table**: Handles precompile logic and computation.
    - **Sponge Table**: Manages input/output, memory reads/writes.
  - Rows needed: Usually one row per sponge table invocation; variable for control based on complexity.
  - Optimization: Merge into one table if logic is simple to reduce overhead.
- **Step 2: Define Columns and Constraints**:
  - **Sponge Table Columns**:
    - Timestamp (clock cycle) ⏰.
    - Addresses and values for inputs/outputs.
    - Conditional flags.
    - Example: SHA-256 extend includes flags, I/O values, addresses, timestamp.
  - **Constraints**:
    - Evaluated natively on field elements and recursively for STARK compatibility.
    - Max degree: 3; add intermediate columns if higher.
    - Types: Flag constraints, address change constraints.
    - Use cross-table lookups for memory read/write verification.
  - **Control Table Columns**:
    - Timestamp, input/output values.
    - Intermediate values during computation.
    - Conditional flags.
    - Example: SHA-256 extend table with computation intermediates.
  - **Constraints**:
    - Conditional flag constraints.
    - Computation constraints.
    - Verify ops (e.g., XOR, AND) via cross-table lookups.
- **Related Concepts**:
  - STARKs (Scalable Transparent ARguments of Knowledge) use algebraic intermediate representations for proofs.
  - Constraints ensure circuit integrity, like polynomials in PLONK or R1CS in other ZK systems.
  - Emoji insight: Tables are like spreadsheets in ZK – rows prove steps, columns hold variables 📑.

# Trace Generation for Tables 🔍

- **Process**: Add info to generate execution traces.
- **Sponge Table Inputs**:
  - Addresses for I/O values.
  - Timestamp.
  - Flag values.
- **Control Table Inputs**:
  - Timestamp.
  - Flag values.
  - Input/output values.
  - Intermediates computed during generation.
- **Padding Requirement**:
  - Trace length must be power of 2 for STARKs.
  - Add padded (empty) rows at the end.
- **Supplementary**:
  - Traces represent program execution step-by-step, verifiable via Fiat-Shamir heuristic in ZKPs.
  - Similar to execution traces in debuggers, but optimized for polynomial commitments.

# Cross-Table Lookups for Data Dependencies 🔗

- **Purpose**: Verify consistency across tables (e.g., input/output matching).
- **Implementation**:
  - Specify data and apply filters for which rows/columns to check.
  - Examples of pairs:
    - Sponge and Control: Match timestamps and I/O values.
    - Sponge and Memory: Verify reads/writes.
    - Sponge and CPU: Ensure state consistency.
    - Control and Logic: For ops like XOR/AND.
- **How-To**:
  - Define lookup arguments: Columns like timestamp, input values.
  - Filter to relevant rows (e.g., where flags indicate active computation).
- **Enhancements**:
  - Lookups are like database joins in ZK, ensuring no tampering across components.
  - In advanced ZKVMs, they enable modular circuit design, reducing overall complexity 🧩.