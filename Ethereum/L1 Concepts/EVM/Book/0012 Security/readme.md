

---

# 🧠 **1. FOUNDATIONAL CONCEPTS**



---

# 🚨 **2. CATEGORIES OF EVM VULNERABILITIES**

| Category                    | Examples                                    |
| --------------------------- | ------------------------------------------- |
| **Reentrancy**              | `DAO`, ERC777 callback-based attacks        |
| **Arithmetic Errors**       | Integer over/underflow (pre-Solidity 0.8)   |
| **Access Control**          | Misconfigured owner, public admin methods   |
| **DoS (Denial of Service)** | Gas griefing, selfdestruct griefing         |
| **Block Dependency**        | `block.timestamp`, `block.number` abuse     |
| **Uninitialized Storage**   | Shared slot overlap                         |
| **Front-running & MEV**     | DEX slippage, auctions                      |
| **Delegatecall Misuse**     | Proxy pattern hijacks, logic swapping       |
| **Storage Collision**       | Upgradeable contracts not respecting layout |
| **Insecure Randomness**     | `block.timestamp % X`, `blockhash`          |

---

# 🛠️ **3. REENTRANCY**

### 🧨 Description:

* Occurs when an external contract is called **before state changes**, allowing **reentrant calls** to the same function.

### 🔁 Vulnerable Solidity

```solidity
mapping(address => uint256) public balances;

function withdraw() external {
    uint256 amount = balances[msg.sender];
    require(amount > 0);
    (bool sent,) = msg.sender.call{value: amount}(""); // External call first ❌
    require(sent);
    balances[msg.sender] = 0; // State change after ❌
}
```

### ✅ Secure Pattern (Checks-Effects-Interactions)

```solidity
function withdraw() external {
    uint256 amount = balances[msg.sender];
    require(amount > 0);
    balances[msg.sender] = 0; // Effect first ✅
    (bool sent,) = msg.sender.call{value: amount}("");
    require(sent);
}
```

### 🛡️ Advanced: Reentrancy Guard

```solidity
bool locked;

modifier noReentrant() {
    require(!locked);
    locked = true;
    _;
    locked = false;
}
```

---

# ➗ **4. INTEGER OVERFLOW / UNDERFLOW**

### ⚠️ Vulnerable (Solidity <0.8.0)

```solidity
uint8 x = 255;
x += 1; // x = 0 due to overflow ❌
```

### ✅ SafeMath (Pre-0.8)

```solidity
x = x.add(1); // Throws on overflow
```

### ✅ Native Checks (Solidity ≥0.8)

* Overflow/underflow now cause **reverts by default**

---

# 🏁 **5. BLOCK VARIABLES MISUSE**

| Variable          | Risk                                     |
| ----------------- | ---------------------------------------- |
| `block.timestamp` | Miners can manipulate within \~15s range |
| `block.number`    | Unreliable for randomness                |
| `blockhash`       | Only last 256 blocks valid               |

### ❌ Vulnerable Randomness

```solidity
uint256 rand = uint256(keccak256(abi.encodePacked(block.timestamp, msg.sender))) % 100;
```

### ✅ Secure Randomness

* Use **Chainlink VRF** or **commit-reveal** schemes.

---

# 📦 **6. DELEGATECALL EXPLOITS**

### 🔥 Description:

* `delegatecall` allows executing code **in the context of the calling contract**.
* Used in **proxies**, but leads to storage hijack if not isolated.

### ❌ Vulnerable Proxy

```solidity
function upgrade(address _impl) public {
    (bool success,) = _impl.delegatecall(abi.encodeWithSignature("upgrade()")); // Executes logic contract maliciously
}
```

### 🛡️ Secure Proxy Pattern

* Lock upgrade with `onlyOwner`
* Use **EIP-1967** slots to prevent collisions

---

# ⚠️ **7. STORAGE COLLISIONS IN UPGRADEABLE CONTRACTS**

| Cause                   | Effect                     |
| ----------------------- | -------------------------- |
| No storage gap in proxy | New vars override old vars |
| Mismatched layout       | Unpredictable behavior     |

### ✅ Solution:

```solidity
uint256[50] private __gap; // Reserved slots in base contract
```

---

# 🧼 **8. UNINITIALIZED STORAGE POINTERS**

### ❌ Vulnerable

```solidity
struct Data { uint a; }
Data data;

function set(uint _a) public {
    Data memory ptr;
    ptr.a = _a;  // Doesn't modify `data` – just local memory
}
```

### ✅ Fix

```solidity
function set(uint _a) public {
    data.a = _a;
}
```

---

# 💣 **9. SELFDESTRUCT GRIEFING**

* Anyone can **send ETH to your contract** via `selfdestruct`.
* This **breaks invariants** in contracts relying on `address(this).balance`.

### ❌ Assumption

```solidity
require(address(this).balance == deposits[msg.sender]); // May fail
```

---

# 🦹 **10. FRONT-RUNNING & MEV**

### 🏦 Example: DEX

```solidity
function buy(uint expectedPrice) external {
    require(price <= expectedPrice); // Front-runnable!
}
```

### 🛡️ Solutions

* **Commit-reveal**
* **MEV-resistant auctions**
* Use **`eip-1559` max fees**

---

# 🧪 **11. DENIAL OF SERVICE (DOS)**

### 📛 Gas Griefing

```solidity
function distribute() public {
    for (uint i = 0; i < users.length; i++) {
        users[i].call{value: 1 ether}(""); // Any revert blocks entire loop ❌
    }
}
```

### ✅ Fix

* Use **pull over push** pattern.

---

# 🧠 **12. EXAMPLES IN OTHER LANGUAGES**

---

## ✅ **Rust (Reentrancy Simulation)**

```rust
#[ink::contract]
mod reentrancy {
    #[ink(storage)]
    pub struct Vault {
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
    }

    impl Vault {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { balances: Default::default() }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let caller = self.env().caller();
            let value = self.env().transferred_value();
            self.balances.insert(caller, value);
        }

        #[ink(message)]
        pub fn withdraw(&mut self) {
            let caller = self.env().caller();
            let amount = *self.balances.get(&caller).unwrap_or(&0);
            self.balances.insert(caller, 0);
            self.env().transfer(caller, amount).unwrap(); // Transfer after effect ✅
        }
    }
}
```

---

## ✅ **Go (Delegatecall Address Hijack Simulation)**

```go
// Simplified pseudo-code in Go
type Proxy struct {
    Implementation Address
}

func (p *Proxy) Upgrade(newImpl Address) {
    if !caller.IsOwner() {
        panic("unauthorized")
    }
    p.Implementation = newImpl
    // Delegatecall simulation
    newImpl.RunLogicInContext(p.Storage)
}
```

---

## ✅ **C++ (Storage Collision)**

```cpp
class Proxy {
    int logic_address;
    int slot1;
};

class LogicV1 {
    int slot1; // Interferes with proxy slot1
};

class LogicV2 {
    int slot1; // Unintended overwrite
    int slot2;
};
```

---

# 📊 **13. VULNERABILITY COMPARISON TABLE**

| Vulnerability          | Exploit Type     | Preventative Pattern                          | Solidity Version Impact |
| ---------------------- | ---------------- | --------------------------------------------- | ----------------------- |
| Reentrancy             | Call-back        | Checks-Effects-Interactions, ReentrancyGuard  | All                     |
| Overflow/Underflow     | Arithmetic       | SafeMath / Solidity ≥0.8                      | <0.8 critical           |
| Block.timestamp misuse | Manipulable RNG  | Chainlink VRF                                 | All                     |
| delegatecall misuse    | Logic hijack     | Storage isolation, access control             | All                     |
| Storage collision      | State corruption | Use `__gap`, adhere to layout                 | All upgradeables        |
| Selfdestruct griefing  | ETH pollution    | Avoid strict balance invariants               | All                     |
| Front-running          | MEV              | Commit-reveal, private tx, delay confirmation | All                     |

---

# 🏁 **14. AUDIT CHECKLIST**

* ✅ `call`, `delegatecall`, `selfdestruct` usage?
* ✅ Gas-heavy loops or dynamic arrays?
* ✅ Owner/admin modifiers enforced?
* ✅ Upgradeable contract layout preserved?
* ✅ Balance checks? Reentrancy guards?
* ✅ Usage of `block.timestamp` for critical logic?
* ✅ All inputs validated (e.g., slippage, limits)?
* ✅ Storage collisions tested?

---

Ready to go deeper into **EVM internals**, **gas griefing**, or **zero-day real-world examples from exploits like The DAO, Parity Wallet, or Harvest Finance**?
