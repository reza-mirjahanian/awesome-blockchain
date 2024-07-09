## Gaining Security Awareness with Substrate-Based Blockchains and Rust

---

**Welcome!** In this article, we will explore security awareness when working with Substrate-based blockchains, especially using the Rust programming language. Whether you are developing or auditing smart contracts, this guide is for you. 

---

### Overview

- **Article Basis**: This article highlights the top 10 vulnerabilities in Substrate-based blockchains using Rust.
- **Goal**: Guide you through these vulnerabilities, explain their risks, impact, and how to mitigate them.

### Vulnerabilities Covered

1. **Insecure Randomness**
2. **Storage Exhaustion**
3. **Unbounded Decoding**
4. **Insufficient Benchmarking**
5. **Arbitrary Execution**
6. **Cross-Consensus Messaging (XCM) Denial of Service**
7. **Unsafe Arithmetic**
8. **Unsafe Conversion**
9. **Replay Issues**
10. **Outdated Crates**
11. **Verbosity Issues**

---

### Detailed Breakdown

#### 1. Insecure Randomness

- **Issue**: Reliance on weak or predictable randomness sources.
- **Impact**: Manipulation by malicious actors, crucial in applications like lotteries or voting.
- **Mitigation**: Use the Verifiable Random Function from the "pallet-babe".

**Example**:
- **Insecure Approach**: Utilizes hashes of the previous 81 blocks.
- **Secure Approach**: Use "compute_randomness" and "randomness_change_epoch" functions.

#### 2. Storage Exhaustion

- **Issue**: Inadequate charging for storage use, leading to system slowdown and higher operational costs.
- **Impact**: Attackers can exploit low storage costs to bloat storage.
- **Mitigation**: Implement checks to ensure cost proportional to storage use and set storage limits.

**Example**:
- **Analogy**: Like a public storage facility with cheap rental rates leading to overcrowding.
- **Mitigation**: Add "cost_per_byte" field and calculate storage costs accordingly.

#### 3. Unbounded Decoding

- **Issue**: No depth limit set for decoding objects, leading to stack overflow.
- **Impact**: Highly nested calls can disrupt network operations.
- **Mitigation**: Set a depth limit using "decode_with_depth_limit".

**Example**:
- **Analogy**: A truck with an excessive load blocking a low bridge, causing a traffic jam.
- **Mitigation**: Implement "decode_with_depth_limit" to prevent stack overflow.

#### 4. Insufficient Benchmarking

- **Issue**: Incorrect benchmarking of extrinsics (functions) in Substrate.
- **Impact**: Slows down the network and allows system spam.
- **Mitigation**: Perform benchmarks using worst-case scenarios.

**Example**:
- **Analogy**: A bus company charging the same price for all distances leading to financial loss.
- **Mitigation**: Set accurate benchmarks for computational costs.

#### 5. Arbitrary Execution

- **Issue**: Improperly configured XCM allowing unauthorized actions.
- **Impact**: Unauthorized actions or system disruptions.
- **Mitigation**: Limit usage of XCM executes and send operations.

**Example**:
- **Analogy**: A high-security door set to let anyone in mode.
- **Mitigation**: Change "everything" value to a more detailed "safe_call_filter".

#### 6. XCM Denial of Service

- **Issue**: Inadequate setup allowing system overload by spamming XCM messages.
- **Impact**: Bottleneck in XCM queues disrupting network functionality.
- **Mitigation**: Properly set up XCM to filter incoming messages.

**Example**:
- **Analogy**: A post office overloaded with packages causing a traffic jam.
- **Mitigation**: Filter messages before processing to prevent overload.

#### 7. Unsafe Arithmetic

- **Issue**: Potential overflows or underflows in mathematical operations.
- **Impact**: Incorrect calculations leading to inconsistencies.
- **Mitigation**: Use "checked_add" or "checked_sub" functions.

**Example**:
- **Insecure Approach**: Direct subtraction without checks.
- **Secure Approach**: Use checked arithmetic functions to prevent errors.

#### 8. Unsafe Conversion

- **Issue**: Converting numeric types without adequate checks causing errors.
- **Impact**: Overflows or incorrect values leading to system inconsistencies.
- **Mitigation**: Use "try_into" or "saturating_into" for safe conversions.

**Example**:
- **Insecure Approach**: Direct downcast without checks.
- **Secure Approach**: Use safe conversion methods.

#### 9. Replay Issues

- **Issue**: Improper handling of transaction nonces allowing repeat transactions.
- **Impact**: Network congestion and potential system slowdown.
- **Mitigation**: Implement nonce checks to prevent transaction repetition.

**Example**:
- **Insecure Approach**: No nonce verification.
- **Secure Approach**: Add nonce verification in the "process_transaction" function.

#### 10. Outdated Crates

- **Issue**: Using outdated or unsafe versions of dependencies.
- **Impact**: Exposes the system to known vulnerabilities and incompatibility issues.
- **Mitigation**: Use the latest versions of dependencies and ensure consistent versioning.

**Example**:
- **Insecure Approach**: Dependencies from different Substrate versions.
- **Secure Approach**: Align dependencies to the same version.

#### 11. Verbosity Issues

- **Issue**: Lack of detailed logs making it hard to diagnose issues.
- **Impact**: Delayed identification and resolution of critical issues.
- **Mitigation**: Implement comprehensive logging and regularly review logs.

**Example**:
- **Insecure Approach**: No logging in state updates.
- **Secure Approach**: Add events to log state changes.

---


[Top-10 Vulnerabilities in Substrate-based Blockchains Using Rust | by Bloqarl | Rektoff | Medium](https://medium.com/rektoff/top-10-vulnerabilities-in-substrate-based-blockchains-using-rust-d454279521ff)

---