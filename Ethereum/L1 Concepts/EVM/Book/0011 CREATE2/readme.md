

---

## 🔹 **1. FOUNDATIONAL CONCEPTS**

### ❓ What is `CREATE2`?

* `CREATE2` is an opcode that allows you to deploy a contract at a **deterministic address** based on:

  1. **Sender**
  2. **Salt**
  3. **Bytecode**
  4. **Constructor arguments**

This allows contract addresses to be **predicted before deployment**, enabling **meta-transactions, counterfactual contracts**, and **off-chain logic**.

---

## 🔹 **2. `CREATE2` vs `CREATE` (Comparison Table)**

| Feature              | `CREATE`                    | `CREATE2`                                                  |
| -------------------- | --------------------------- | ---------------------------------------------------------- |
| Address Predictable? | ❌ No                        | ✅ Yes                                                      |
| Used In              | Regular contract deployment | Meta-transactions, proxy patterns, upgradeability          |
| Formula for Address  | Hash of sender and nonce    | `keccak256(0xFF ++ sender ++ salt ++ keccak256(bytecode))` |
| Introduced In        | EVM pre-EIP-1014            | **EIP-1014**                                               |

---

## 🔹 **3. `CREATE2` ADDRESS FORMULA**

The contract address from `CREATE2` is:

```text
address = keccak256(0xFF ++ deployer_address ++ salt ++ keccak256(init_code))[12:]
```

* `0xFF` — Constant
* `deployer_address` — 20-byte address
* `salt` — 32-byte user-defined value
* `init_code` — Creation code (bytecode + constructor args)

---

## 🔹 **4. BASIC EXAMPLE IN SOLIDITY**

### ✅ **Factory Contract using `CREATE2`**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleContract {
    uint256 public value;

    constructor(uint256 _value) {
        value = _value;
    }
}

contract Create2Factory {
    event Deployed(address addr, uint256 salt);

    function deploy(bytes memory bytecode, uint256 _salt) public returns (address) {
        address addr;

        assembly {
            addr := create2(0, add(bytecode, 0x20), mload(bytecode), _salt)
            if iszero(extcodesize(addr)) { revert(0, 0) }
        }

        emit Deployed(addr, _salt);
        return addr;
    }

    function computeAddress(bytes memory bytecode, uint256 _salt) public view returns (address) {
        bytes32 hash = keccak256(abi.encodePacked(
            bytes1(0xff),
            address(this),
            _salt,
            keccak256(bytecode)
        ));
        return address(uint160(uint256(hash)));
    }
}
```

---

## 🔹 **5. GETTING BYTECODE OF A CONTRACT**

### 📌 Deploying `SimpleContract` via Factory

To use `CREATE2`, you need the **init code**:

```solidity
// Create the init code off-chain or using inline Solidity
bytes memory bytecode = abi.encodePacked(
    type(SimpleContract).creationCode,
    abi.encode(42)
);
```

---

## 🔹 **6. EDGE CASES AND ERROR HANDLING**

| Case                      | Behavior                                                  |
| ------------------------- | --------------------------------------------------------- |
| Re-deploy with same salt  | ❌ Fails with `create2` revert                             |
| Wrong bytecode length     | ❌ `extcodesize == 0`, revert manually in assembly         |
| Constructor args mismatch | ❌ Deployment fails, need to ensure correct ABI            |
| Gas too low               | ❌ Same as `CREATE` — `create2` fails silently in assembly |

---

## 🔹 **7. ADVANCED USAGE PATTERNS**

### ✅ **1. Counterfactual Contracts**

* Precompute address using salt and bytecode
* Sign transaction that interacts with it
* Deploy only when needed

This is used in **meta-transactions**.

### ✅ **2. Upgradeable Proxy Contracts**

* Precompute proxy address
* Deploy a new logic contract
* Point proxy to new logic

---

## 🔹 **8. UTILITY: CREATE2 ADDRESS CALCULATION IN GO**

```go
package main

import (
    "crypto/sha256"
    "fmt"
    "github.com/ethereum/go-ethereum/crypto"
    "github.com/ethereum/go-ethereum/common"
)

func create2Address(deployer, salt, bytecodeHash []byte) common.Address {
    data := []byte{0xff}
    data = append(data, deployer...)
    data = append(data, salt...)
    data = append(data, bytecodeHash...)
    hash := crypto.Keccak256(data)
    return common.BytesToAddress(hash[12:])
}

func main() {
    deployer := common.HexToAddress("0x123...").Bytes()
    salt := common.LeftPadBytes([]byte("my_salt"), 32)
    bytecodeHash := crypto.Keccak256([]byte{ /* your contract bytecode */ })

    address := create2Address(deployer, salt, bytecodeHash)
    fmt.Println("CREATE2 Address:", address.Hex())
}
```

---

## 🔹 **9. SECURITY CONSIDERATIONS**

| Risk               | Description                               | Mitigation                                                    |
| ------------------ | ----------------------------------------- | ------------------------------------------------------------- |
| Address collision  | Reusing salt/bytecode                     | Always hash salts or append randomness                        |
| Front-running      | Attackers can precompute and deploy first | Use unpredictable salts (e.g., `keccak256(tx.origin, nonce)`) |
| Incorrect bytecode | Will lead to invalid address computation  | Always hash the same creation code used in deployment         |

---

## 🔹 **10. CREATE2 WITH SALT LIBRARY (Best Practice)**

```solidity
function getSalt(address user, uint256 nonce) public pure returns (bytes32) {
    return keccak256(abi.encodePacked(user, nonce));
}
```

* This prevents replay attacks and collision.

---

## 🔹 **11. CREATE2 IN PRACTICE (OpenZeppelin `MinimalProxy`)**

OpenZeppelin uses `CREATE2` to deploy minimal proxies:

```solidity
Clones.cloneDeterministic(implementation, salt);
```

You can then predict:

```solidity
Clones.predictDeterministicAddress(implementation, salt, deployer);
```

---

## 🔹 **12. DEBUGGING CREATE2**

* Use `eth_getCode` to check if the contract was deployed
* Log emitted events with deployed address
* Ensure salt is consistent between prediction and deployment

---

## 🔹 **13. USE IN OTHER LANGUAGES**

### ✅ **Rust (ethers-rs)**

```rust
use ethers::utils::keccak256;
use ethers::types::{Address, H256};

fn compute_create2_address(deployer: Address, salt: H256, init_code_hash: H256) -> Address {
    let mut data = vec![0xff];
    data.extend_from_slice(deployer.as_bytes());
    data.extend_from_slice(salt.as_bytes());
    data.extend_from_slice(init_code_hash.as_bytes());

    let hash = keccak256(&data);
    Address::from_slice(&hash[12..])
}
```

---

## 🔹 **14. SUMMARY CHECKLIST FOR CREATE2 MASTERY**

* ✅ Understand and compute deterministic address
* ✅ Encode salt properly
* ✅ Validate bytecode and constructor args
* ✅ Avoid collisions and front-running
* ✅ Predict address on-chain and off-chain
* ✅ Debug with `extcodesize`, logs, and `eth_getCode`

---

Ready to move on to **meta-transactions, proxies, or CREATE2 exploit scenarios** next?
