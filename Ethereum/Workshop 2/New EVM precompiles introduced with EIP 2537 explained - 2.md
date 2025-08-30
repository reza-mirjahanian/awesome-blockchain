# 🔧 **Understanding Precompiles in the EVM: A Deep Code Walkthrough**

---

## 🌐 **Precompiles: The Hidden Powerhouses of the EVM**

- Precompiles are **native functions** built directly into the Ethereum Virtual Machine.
- They are **not written in Solidity** or any high-level language, but implemented at the **protocol level** in the client software (e.g., Geth, Nethermind, **Revm**).
- 🎯 Purpose: Perform **computationally intensive operations** more efficiently than regular smart contracts.

> 💡 *Why use precompiles?*  
> Because they run in **native code**, they avoid the overhead of EVM interpretation, making them **faster and cheaper** for specific cryptographic or mathematical operations.

---

## 🧱 **Architecture of Precompiles in Revm (Rust EVM)**

Revm is a **high-performance, modular EVM implementation in Rust**, designed not just for execution but as a **framework** for experimentation.

### 🔗 Key Abstraction: The `PrecompileProvider` Trait

```rust
pub trait PrecompileProvider {
    fn run(&self, address: Address, input: &[u8]) -> PrecompileResult;
    fn contains(&self, address: &Address) -> bool;
}
```

- ✅ `contains(address)` → Checks if an address is a precompile.
- ✅ `run(address, input)` → Executes the precompile logic if the address is valid.

> 🧠 This trait enables **pluggable precompile sets**, allowing developers to **customize the EVM** for testing new EIPs.

---

## 🧩 **How Precompiles Are Structured Internally**

### 📦 `EthereumPrecompileProvider` – The Main Implementation

This struct holds:
- A `HashMap<Address, Precompile>` — mapping addresses to function pointers.
- A `HashSet<Address>` — for fast lookup via `contains()`.

```rust
struct EthereumPrecompileProvider {
    precompiles: Precompiles,
}
```

Where `Precompiles` is a wrapper around:
- `inner: HashMap<Address, PrecompileFn>`
- `addresses: HashSet<Address>`

> 🚀 This design enables **O(1) lookup** and efficient dispatch during EVM execution.

---

## ⚙️ **Execution Flow: From CALL to Precompile**

When the EVM encounters a `CALL` opcode:

1. 🧭 **Address Resolution**
   - Is the target address a **precompile**?
   - The EVM calls `contains(address)` on the `PrecompileProvider`.

2. ✅ **If Yes**:
   - The EVM **skips contract bytecode execution**.
   - Instead, it invokes the **native precompile function** via the function pointer.

3. ❌ **If No**:
   - The EVM loads and interprets the **contract bytecode** normally.

> 🔄 This decision happens during **frame creation** in the EVM’s call stack.

---

## 🛠️ **Adding New Precompiles: EIP-2537 as a Case Study**

EIP-2537 introduces **BLS12-381 elliptic curve operations** to enable advanced cryptography:
- BLS signatures
- zk-SNARKs
- Threshold signatures
- Ethereum 2.0 consensus layer compatibility

### 📌 New Precompiles Added:
| Address | Operation | Description |
|--------|----------|-------------|
| `0x0A` | `BLS12_381_G1_ADD` | Adds two points on G1 group |
| `0x0B` | `BLS12_381_G1_MUL` | Multiplies a G1 point by a scalar |
| `0x0C` | `BLS12_381_G2_ADD` | Adds two points on G2 group |
| `0x0D` | `BLS12_381_G2_MUL` | Multiplies a G2 point by a scalar |
| `0x0E` | `BLS12_381_PAIRING` | Computes pairing check (useful for BLS signature verification) |

> 🔐 These operations are **critical for Ethereum’s scalability and security roadmap**.

---

## 🧱 **Implementation Structure in Revm**

### 🧱 `Precompiles::new(spec_id: SpecId)` – The Factory Pattern

The `Precompiles` type is constructed based on the **network fork** (via `SpecId`), ensuring backward compatibility.

```rust
impl Precompiles {
    pub fn new(spec_id: SpecId) -> Self {
        match spec_id {
            SpecId::HOMESTEAD => homestead(),
            SpecId::BYZANTIUM => byzantium(),
            SpecId::ISTANBUL => istanbul(),
            SpecId::MUIR_GLACIER => muir_glacier(),
            SpecId::BERLIN => berlin(),
            SpecId::LONDON => london(),
            SpecId::MERGE => merge(),
            SpecId::SHANGHAI => shanghai(),
            SpecId::CANCUN => cancun(),
            SpecId::PRAGUE => prague(), // ← EIP-2537 lands here
            _ => unimplemented!(),
        }
    }
}
```

> 🔄 Each function returns a set of precompiles active at that fork.

---

## 🏗️ **Fork Evolution of Precompiles**

| Fork | Precompiles Added |
|------|-------------------|
| 🏘️ **Homestead** | `ecrecover`, `sha256`, `ripemd160`, `identity` |
| 🧨 **Byzantium** | `bn256_add`, `bn256_mul`, `modexp` |
| 🌪️ **Istanbul** | `blake2b_f` (for ZKPs) |
| 🏜️ **Cancun** | No new precompiles |
| 🏛️ **Prague** | `bls12_381_*` series (EIP-2537) |

> 📈 This shows Ethereum’s **gradual expansion** of cryptographic capabilities.

---

## 🔤 **Inside a Precompile: G1 Add Example**

```rust
pub const BLS12_381_G1_ADD: PrecompileWithAddress = PrecompileWithAddress {
    address: Address::from_low_u64_be(0x0B),
    function: bls12_381_g1_add,
};
```

### 🔍 Breakdown:
- `address: 0x0B` → Matches EIP-2537 spec.
- `function: bls12_381_g1_add` → A **function pointer** to the actual implementation.

### 🧮 Function Signature:
```rust
fn bls12_381_g1_add(input: &[u8]) -> PrecompileResult;
```

- Takes raw bytes as input (encoded G1 points).
- Returns a `PrecompileResult`:
  - `Ok(output)` on success
  - `Err(error)` on failure (e.g., invalid point, gas limit)

> 🧮 The actual math involves **finite field arithmetic** over large primes — too complex for EVM bytecode, hence the need for a precompile.

---

## 🔗 **How Smart Contracts Use Precompiles**

Solidity doesn’t expose precompiles directly, but they’re used under the hood:

### Example: `ecrecover`
```solidity
function recover(bytes32 hash, uint8 v, bytes32 r, bytes32 s) 
    public pure returns (address) {
    return ecrecover(hash, v, r, s); // → Calls precompile at 0x01
}
```

### Manual CALL to a Precompile:
```solidity
function callBlsAdd(bytes memory input) public returns (bytes memory) {
    bool success;
    bytes memory result;

    assembly {
        success := call(
            gas(),             // Gas provided
            0x0B,              // BLS12-381 G1 Add precompile
            0,                 // No ether transferred
            add(input, 32),    // Input data (skip length)
            mload(input),      // Input length
            0,                 // Output starts at 0
            128                // Output is 128 bytes (compressed G1 point)
        )
        result := mload(0x40)   // Get free memory pointer
        mstore(0x40, add(result, 128))
        mstore(result, 128)
        returndatacopy(add(result, 32), 0, 128)
    }

    require(success, "BLS add failed");
    return result;
}
```

> ⚠️ Manual calls require **correct input encoding** and **gas estimation**.

---

## 🧪 **Why Revm is Ideal for EIP Experimentation**

Revm’s **modular design** makes it perfect for testing new precompiles:

### ✅ Advantages:
- 🧩 **Trait-based architecture** → Swap precompile sets easily.
- 🛠️ **Fork-aware construction** → Simulate future upgrades.
- 🚀 **Rust performance** → Fast execution for benchmarking.
- 📦 **Extensible** → Add your own precompile in minutes.

### Example: Adding a Custom Precompile
```rust
fn my_custom_precompile(input: &[u8]) -> PrecompileResult {
    // Your logic here
    Ok(PrecompileOutput {
        output: vec![1, 2, 3],
        cost: 100,
        gas_used: 100,
    })
}

// Then register it:
precompiles.insert(Address::from_low_u64_be(0x100), my_custom_precompile);
```

> 🛠️ This enables **rapid prototyping** of new cryptographic primitives.

---

## 🧮 **Gas Model for Precompiles**

Each precompile defines its **gas cost function**:
- Often based on **input size** (e.g., `modexp` uses a formula based on exponent length).
- Some are **fixed cost** (e.g., `ecrecover`).
- EIP-2537 precompiles use **complex gas formulas** based on operation type and input size.

> 💸 Goal: Prevent DoS attacks while remaining **affordable for legitimate use**.

---

## 🔐 **Security Considerations**

Precompiles are **trusted code** — bugs can be catastrophic.

### Risks:
- ❌ **Invalid curve points** → Can lead to vulnerabilities.
- ❌ **Side-channel attacks** → In native implementations.
- ❌ **Incorrect gas pricing** → Enables spam attacks.

### Mitigations:
- ✅ **Input validation** at entry point.
- ✅ **Constant-time algorithms** to avoid timing leaks.
- ✅ **Formal verification** where possible.

> 🛡️ Precompiles are **audited heavily** before inclusion in a hard fork.

---

## 🌍 **Real-World Use Cases of Precompiles**

| Precompile | Use Case |
|----------|--------|
| `ecrecover` | Signature verification (Meta-transactions, Wallets) |
| `sha256` | Data integrity, Merkle proofs |
| `bn256_add/mul` | zk-SNARK verification (e.g., Tornado Cash) |
| `modexp` | RSA encryption/decryption |
| `bls12_381_*` | Ethereum 2.0 consensus, BLS signatures, threshold crypto |

> 🌐 These enable **privacy, scalability, and interoperability** across dApps.

---

## 🧬 **Future of Precompiles**

Ethereum continues to expand its precompile set for:
- 🧠 **ZK-friendly operations** (e.g., Poseidon hash)
- 🔗 **Cross-chain messaging**
- 🧮 **Advanced math** (e.g., FFT, matrix ops)
- 🤖 **Machine learning primitives** (long-term vision)

> 🚀 Precompiles are **Ethereum’s extensibility mechanism** — a bridge between EVM limitations and real-world crypto needs.

---

## 🧰 **Developer Tips for Working with Precompiles**

- 🔍 **Always check the EIP spec** for input/output formats.
- 💡 Use **assembly** for direct control over calls.
- ⛽ Simulate gas costs — precompiles can still be expensive.
- 🧪 Test with **invalid inputs** to ensure robustness.
- 📊 Monitor **client support** — not all precompiles are enabled on all networks.

> 🛠️ Tools like **Hardhat, Foundry** can help simulate precompile behavior.

---

## 🧱 **Deep Dive: The `Precompile` Type**

```rust
pub type PrecompileFn = fn(&[u8], u64) -> PrecompileResult;
```

- Takes: `input: &[u8]`, `gas_limit: u64`
- Returns: `PrecompileResult` with:
  - `output: Vec<u8>`
  - `gas_used: u64`
  - `error: Option<PrecompileError>`

> 🔄 This uniform interface allows the EVM to treat all precompiles the same way.

---

## 🗺️ **Memory Layout & Input Encoding**

Precompiles expect **specific byte layouts**:

### Example: `BLS12_381_G1_ADD`
Input: 256 bytes
- First 128 bytes: G1 point P (x, y coordinates, 64 bytes each)
- Next 128 bytes: G1 point Q

Output: 128 bytes (compressed representation of P + Q)

> 📏 Misaligned or malformed input → `PrecompileError::InvalidInput`

---

## 🧪 **Testing Precompiles in Revm**

Revm allows **unit testing precompiles in isolation**:

```rust
#[test]
fn test_bls_g1_add() {
    let input = [0u8; 256]; // Valid zero points
    let result = bls12_381_g1_add(&input, 100000);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().output.len(), 128);
}
```

> ✅ Enables **fast feedback loop** during development.

---

## 🔄 **Dynamic vs. Static Precompile Registration**

- **Static**: All precompiles known at compile time (current model).
- **Dynamic**: Proposals to allow **on-chain registration** of precompiles (e.g., for L2s).

> 🧩 Dynamic precompiles could enable **custom VM extensions** in rollups.

---

## 🧠 **Conceptual Role of Precompiles in the EVM Stack**

```
+---------------------+
|   Smart Contracts   | ← High-level logic (Solidity)
+---------------------+
|     EVM Bytecode    | ← Interpreted opcodes
+---------------------+
|    Precompiles      | ← Native functions (fast path)
+---------------------+
|   Ethereum Client   | ← Rust/Go/C++ implementation
+---------------------+
|    Host Machine     | ← CPU, Memory, OS
+---------------------+
```

> ⬆️ Precompiles sit **between bytecode and native execution**, offering a hybrid performance model.

---

## 🧰 **Tooling & Debugging Precompile Calls**

- **Tenderly** → Simulate and debug precompile failures.
- **Geth RPC Tracer** → Step through precompile execution.
- **Foundry** → Test with `vm.etch()` to mock precompile behavior.

> 🔍 Use `CALL` traces to inspect input/output and gas usage.

---

## 🧬 **Mathematical Foundations: BLS12-381 Curve**

- Pairing-friendly elliptic curve.
- Security level: ~128 bits.
- Used in:
  - Ethereum 2.0 validator signatures
  - Zcash’s zk-SNARKs
  - Filecoin’s proofs

> 🧮 The `pairing` precompile computes **bilinear pairings**, enabling **aggregate signature verification**.

---

## 🛠️ **Building Your Own Precompile: Step-by-Step**

1. 📝 Define the **spec** (address, input/output format, gas model).
2. 🔧 Implement the **function** in Rust (or Go/C++ for other clients).
3. 🧪 Write **unit tests** for edge cases.
4. 🔗 Register it in the `PrecompileProvider` for your target fork.
5. 🧪 Integrate into a **testnet client**.
6. 📄 Submit an **EIP** for standardization.

> 🚀 This is how **EIP-2537** was born — from research to mainnet.

---

## 🧩 **Extensibility: Precompiles as Plug-ins**

Imagine:
```rust
trait PrecompilePlugin {
    fn register(&mut self, precompiles: &mut Precompiles);
}
```

Allows third parties to **inject custom precompiles** into the EVM.

> 🔌 Useful for:
> - Enterprise chains
> - Research testnets
> - Domain-specific rollups

---

## 🧠 **Performance Comparison: Precompile vs. Solidity**

| Operation | Precompile Gas | Solidity Equivalent Gas | Speedup |
|---------|----------------|--------------------------|--------|
| `ecrecover` | ~3,000 | ~20,000+ | ~6x |
| `bn256_add` | ~150 | ~400,000 | ~2,600x |
| `bls12_381_mul` | ~10,000 | Would time out | ∞ |

> ⚡ Precompiles make **impossible operations possible** on-chain.

---

## 🧭 **Address Space for Precompiles**

Precompiles live in the **low-address range**:
- `0x01` to `0xFF`
- Reserved for **protocol-level functions**

> ⚠️ Addresses above `0x10000` are for contracts; **no overlap**.

---

## 🧰 **Common Precompile Addresses**

| Address | Name | EIP |
|-------|------|-----|
| `0x01` | `ecrecover` | Core |
| `0x02` | `sha256` | Core |
| `0x03` | `ripemd160` | Core |
| `0x04` | `identity` | Core |
| `0x05` | `modexp` | 198 |
| `0x06` | `bn256_add` | 196 |
| `0x07` | `bn256_mul` | 196 |
| `0x08` | `bn256_pairing` | 197 |
| `0x09` | `blake2b` | 152 |
| `0x0A`–`0x0E` | `bls12_381_*` | 2537 |

> 📋 This table is **essential** for low-level developers.

---

## 🧱 **Revm’s Precompile Module Structure**

```
precompiles/
├── mod.rs
├── homestead.rs
├── byzantium.rs
├── istanbul.rs
├── cancun.rs
├── prague/
│   ├── mod.rs
│   ├── bls12_381/
│   │   ├── g1_add.rs
│   │   ├── g1_mul.rs
│   │   ├── g2_add.rs
│   │   ├── g2_mul.rs
│   │   └── pairing.rs
│   └── bls12_381.rs ← exports all
```

> 📁 Clean, modular, and **easy to extend**.

---

## 🧪 **Error Handling in Precompiles**

Possible errors:
- `OutOfGas`
- `InvalidInput`
- `Failure` (e.g., point not on curve)

These are **not reverts** — they are **precompile-specific failures**.

> 🛑 On error, the `CALL` returns `false` and reverts state unless `STATICCALL` or `DELEGATECALL`.

---

## 🧠 **Why Not Implement Everything as Precompiles?**

Trade-offs:
- 🔐 **Security risk**: More native code = more attack surface.
- 🧱 **Complexity**: Harder to audit and upgrade.
- 🔄 **Flexibility**: Smart contracts can be upgraded; precompiles cannot.
- 📦 **Bloat**: Not every use case justifies a precompile.

> 🎯 Only **widely used, performance-critical** operations become precompiles.


# ⚙️ How Precompiles are Implemented in the EVM
### A Deep Dive into `revm` and EIP-2537

---

# 🦀 The Right Tool: Why `revm`?

* **`revm`** is a high-performance, flexible **Rust implementation of the EVM**.
* **Key Strengths:**
    * 💨 **Fast:** Built in Rust, known for its speed.
    * 🧩 **Highly Flexible & Customizable:** It's designed as an EVM *framework*, not just an EVM.
    * 🔬 **Ideal for Experimentation:** Makes it easy to prototype and test new EVM features, like EIPs that introduce new precompiles.
* **Our Approach:**
    * We will analyze the specific Pull Request (PR) that added **EIP-2537** to the `revm` codebase to understand the implementation process.

---

# 🏛️ The Architecture: The `EthHandler` Trait

* This trait acts as the central **interface for `revm`'s execution logic**. Precompiles are a core part of this logic.
* **EVM as a Framework:**
    * `revm` uses Rust's "Associated Types" to define various aspects of the EVM.
    * One of these is the `precompiles` type.
* **The Power of Flexibility:**
    * This design allows any developer to implement their own set of precompiles.
    * It's perfect for an EIP author who wants to quickly create a working EVM prototype with their new precompile included.

---

# 📜 The Blueprint: `PrecompileProvider` Trait

* Any set of precompiles must follow the rules defined by the `PrecompileProvider` trait.
* **It defines two essential methods:**
    * `run(address, input_bytes)`
        * Takes the precompile's address and input data.
        * **Executes the actual precompile logic.**
    * `contains(address) -> bool`
        * Takes an address.
        * **Returns `true` or `false` if the address belongs to a precompile.** This is a crucial check.

---

# 🚦 EVM Execution Flow: A Fork in the Road

When the EVM executes a `CALL` opcode to another address, it must make a decision.

1.  **Check Address:** The EVM uses the `contains(address)` method.
2.  **Decision Point:**
    * **If `true` ✅:** The address is a precompile.
        * The EVM calls the `run` method, executing the **natively implemented, hardcoded function**.
        * This bypasses the normal interpreter.
    * **If `false` ❌:** The address is a regular smart contract.
        * The EVM loads the **smart contract bytecode** from that address.
        * It executes the code through the interpreter, one opcode at a time.


---

# 🧱 The Concrete Implementation: `EthPrecompileProvider`

* This is the actual implementation of the `PrecompileProvider` trait for the Ethereum Mainnet EVM.
* **How its `run` method works:**
    1.  Takes the target `address` and `input` bytes.
    2.  First, it checks if the address is a *known* precompile. If not, it returns immediately.
    3.  If it is, it finds the corresponding precompile function.
    4.  **The function pointer is dereferenced and called with the input data.** 🧠
    5.  It captures the result of the execution (success or error) and returns it.

---

# 🗺️ Data Structure: The `Precompiles` Struct

* This is the struct that actually holds all the precompile information for a given EVM configuration.
* **It has two key fields:**
    * `inner: HashMap<Address, PrecompileFunction>`
        * This is a **hash map** (like a dictionary or key-value store).
        * **Key:** The precompile's `Address` (e.g., `0x01` for `ecrecover`).
        * **Value:** A `PrecompileFunction`, which is a **function pointer** to the native Rust code that implements the logic.
    * `addresses: HashSet<Address>`
        * A simple set of all precompile addresses for very fast lookups (this is what powers the `contains` method).

---

# 🏭 The Factory: The `Precompiles` Constructor

* The precompiles are not static; they change with network upgrades (hard forks).
* **The `Precompiles::new(spec_id)` Method:**
    * It takes a `spec_id` as an argument.
    * The `spec_id` identifies the specific Ethereum hard fork (e.g., `Homestead`, `Byzantium`, `Prague`).
    * Based on the `spec_id`, the constructor calls a specific function (e.g., `homestead()`, `prague()`) to build the correct set of precompiles for that fork.

---

# 🔄 The Evolution of Precompiles Through Forks

This design beautifully tracks the history of Ethereum's upgrades.

* **🐣 Homestead:**
    * Starts with the first four precompiles: `ecrecover`, `sha256`, `ripemd160`, and `identity`.
* **🏛️ Byzantium:**
    * Takes all precompiles from `Homestead`.
    * **Adds** new precompiles for curve operations (`BN128`) and modular exponentiation.
* **🏙️ Prague (with EIP-2537):**
    * Takes all precompiles from the previous fork (`Cancun`).
    * **Adds** all the new BLS12-381 curve operations as defined in the EIP.

---

# ✨ Inside EIP-2537's Implementation

* The `prague()` constructor function is where the magic happens.
* **The Logic:**
    1.  First, it calls `cancun()` to get the base set of all existing precompiles.
    2.  Then, it calls a function from the `bls12_381` module.
    3.  This function returns an **iterator** of all the new precompiles (G1add, G1mul, etc.).
    4.  These new precompiles are added to the set returned by `cancun()`.

---

#🔬 Anatomy of a Single Precompile: `G1_ADD`

Let's look at one specific precompile added by EIP-2537.

* It's defined as a `PrecompileWithAddress` constant.
* **This struct has two fields:**
    * `address: Address`
        * This is the permanent on-chain address for this operation.
        * As defined in the EIP spec, the address for `G1_ADD` is `0x0b`.
    * `function: PrecompileFunction`
        * This is a **pointer** to the actual Rust function that contains the complex cryptographic logic for performing an addition operation on the BLS12-381 curve.

This directly links the EIP's specification to the executable code.