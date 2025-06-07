## Gas Optimization in Solidity Smart Contracts

### 1. **Core Principles**
- **Gas Mechanics**: Every EVM opcode has fixed gas costs. Storage writes (SSTORE: 20k gas new, 5k update) are 100x costlier than arithmetic ops (ADD: 3 gas) .
- **Key Strategy**: Minimize storage operations, maximize computation, and leverage EVM's 32-byte word alignment.

### 2. **Data Structures Optimization**
#### **Mappings vs Arrays**
```solidity
// Inefficient: Array iteration (O(n))
address[] users;
function findUser(address user) public view returns (bool) {
    for(uint i=0; i<users.length; i++) { // Costs more gas with length
        if(users[i] == user) return true;
    }
    return false;
}

// Efficient: Mapping (O(1))
mapping(address => bool) public isUser;
```
- **Pros**: Mappings avoid iteration costs .
- **Cons**: Mappings can't be iterated; use hybrid (mapping + array) for iteration needs.

#### **Fixed vs Dynamic Arrays**
```solidity
uint256[] dynamicArray;  // Costs extra gas for push/pop
uint256[10] fixedArray;  // More efficient for known sizes
```
- Use fixed-size arrays when length is predetermined .

---

### 3. **Variable Handling**
#### **Packing Structs**
```solidity
// Inefficient (3 slots)
struct Unpacked {
    uint64 a;  // 8 bytes
    address b; // 20 bytes
    bool c;    // 1 byte
} // Total: 29 bytes → 3 slots

// Efficient (1 slot)
struct Packed {
    uint64 a;  // 8 bytes
    address b; // 20 bytes
    bool c;    // 1 byte
} // Total: 29 bytes → 1 slot (32 bytes)
```
- **Rule**: Group variables ≤32 bytes; order matters (declaration order = storage layout) .

#### **Constants and Immutables**
```solidity
uint256 constant MAX_SUPPLY = 1000;      // Embedded in bytecode
address immutable owner = msg.sender;    // Set once at deploy
```
- Saves ~35% gas vs regular storage variables .

---

### 4. **Function Optimization**
#### **Memory vs Calldata**
```solidity
function process(uint256[] memory data) {} // Copies to memory (costly)
function process(uint256[] calldata data) external {} // Reads from calldata (cheaper)
```
- Use `calldata` for read-only external arrays/strings .

#### **Short-Circuiting**
```solidity
// Cheap check first: Saves gas if fails often
require(userExists(user) && balance[user] > 100, "Error");
```
- Place low-cost ops (e.g., `userExists`) before heavy ops (e.g., balance checks) .

#### **Error Handling**
```solidity
error InsufficientBalance(uint available); // Custom error (cheaper)
require(balance >= amount, "InsufficientBalance"); // String error (costlier)
```
- Custom errors save ~50% gas vs string messages .

---

### 5. **Control Flow**
#### **Loop Optimization**
```solidity
uint256 sum = 0;
for (uint256 i = 0; i < arr.length; i++) { // Reads length every iteration
    sum += arr[i];
}

// Optimized:
uint256 len = arr.length; // Cache length
for (uint256 i = 0; i < len; ) {
    sum += arr[i];
    unchecked { i++; } // Skips overflow checks (saves gas)
}
```
- **Gas Savings**: Caching length + `unchecked` math saves ~10-20% gas .

#### **Batching Operations**
```solidity
// Inefficient: Multiple external calls
function airdrop(address[] calldata users, uint256 amount) external {
    for(uint i=0; i < users.length; i++) {
        token.transfer(users[i], amount); // High base cost per call
    }
}

// Efficient: Single call with batched data
function batchTransfer(address[] calldata users, uint256[] calldata amounts) external {
    require(users.length == amounts.length, "Length mismatch");
    for(uint i=0; i < users.length; i++) {
        _transfer(users[i], amounts[i]); // Internal call
    }
}
```
- Reduces external call overhead (base 700 gas/call) .

---

### 6. **Storage Management**
#### **Freeing Storage**
```solidity
uint256 public data;
function deleteData() external {
    delete data; // Refunds 15k gas (zeroes storage)
}
```
- Refunds: 15k gas per storage slot zeroed .

#### **Bitmap for Booleans**
```solidity
uint8 public flags; // 1 slot for 8 booleans
uint8 constant FLAG_A = 1 << 0;
function setFlagA(bool enable) external {
    flags = enable ? flags | FLAG_A : flags & ~FLAG_A;
}
```
- Stores 256 booleans in one slot .

---

### 7. **Advanced Techniques**
#### **Inline Assembly**
```solidity
function readSlot(uint256 slot) public view returns (bytes32 value) {
    assembly {
        value := sload(slot)
    }
}
```
- **Use Cases**: Precise storage/memory control. **Risk**: Bypasses Solidity safety checks .

#### **Minimal Proxy Pattern**
```solidity
contract MinimalProxy {
    address immutable implementation;
    constructor(address _impl) { implementation = _impl; }
    fallback() external payable {
        address impl = implementation;
        assembly {
            calldatacopy(0, 0, calldatasize())
            let result := delegatecall(gas(), impl, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch result
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }
}
```
- **Gas Savings**: ~50% cheaper deployment vs full clones .

---

### 8. **Compiler & Deployment**
#### **Optimizer Settings**
```javascript
// hardhat.config.js
module.exports = {
  solidity: {
    version: "0.8.23",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200, // Trade-off: 200 for dApps (lower deploy cost), 10k for complex logic (lower runtime cost)
      },
    },
  },
};
```
- Higher `runs` optimizes for execution; lower `runs` for deployment .

#### **Contract Size**
- Limit: 24KB (EIP-170). Solutions:
  - Use libraries (e.g., `using SafeERC20 for IERC20`)
  - Modularize logic across contracts .

---

### 9. **Real-World Pitfalls**
#### **Reentrancy Risks**
```solidity
// Vulnerable: State update after external call
function withdraw() external {
    uint256 amount = balances[msg.sender];
    (bool success,) = msg.sender.call{value: amount}("");
    require(success, "Transfer failed");
    balances[msg.sender] = 0; // Too late!
}

// Secure: Checks-Effects-Interactions
function withdraw() external {
    uint256 amount = balances[msg.sender];
    balances[msg.sender] = 0; // Update state first
    (bool success,) = msg.sender.call{value: amount}(""); // Interaction last
    require(success, "Transfer failed");
}
```
- Always follow Checks-Effects-Interactions .

#### **Uint8 Gas Trap**
```solidity
uint8 a = 1; // Costs more gas than uint256 in computations
uint256 b = a + 1; // EVM converts to 32 bytes
```
- **Exception**: Use small uints **only** in packed structs .

---

### 10. **Optimization Comparison Table**
| **Technique**         | **Gas Saved** | **Complexity** | **Use Case**               |
|------------------------|---------------|----------------|----------------------------|
| Mappings vs Arrays     | ~89%  | Low            | Key-value lookups          |
| Storage Packing        | ~20-50%       | Medium         | Structs with small fields  |
| `unchecked` Math       | ~5-10%        | Low            | Overflow-safe loops        |
| Custom Errors          | ~50%          | Low            | All reverts                |
| Minimal Proxies        | ~50% deploy   | High           | Clone deployments          |

---

### 11. **Edge Cases & Tricks**
- **Constructor Optimization**: 
  ```solidity
  contract C {
      bytes32 immutable merkleRoot;
      constructor(bytes32 _merkleRoot) {
          merkleRoot = _merkleRoot; // Set once
      }
  }
  ```
- **Dead Code Elimination**: Remove unused functions to reduce contract size .
- **Event Data**: Cheaper than storage but **not** accessible on-chain .

---

### Next Step Suggestion: 
**Dive into Yul Intermediate Representation** for low-level control over EVM bytecode. This advanced topic enables manual optimization of critical logic beyond Solidity's capabilities, such as custom memory layouts and opcode-level gas tuning. Explore Solidity's `assembly {}` blocks and Yul syntax to start.