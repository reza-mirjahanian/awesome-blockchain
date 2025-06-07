

***

## Data Types and Storage

Choosing the right data types and managing storage efficiently are fundamental to gas optimization. Storage operations are among the most expensive in the EVM.

### **Variable Packing**

The EVM processes data in 256-bit (32-byte) slots. Packing multiple smaller data types into a single storage slot can significantly reduce gas costs.

| Data Type | Size (bytes) |
| :--- | :--- |
| `bool` | 1 |
| `uint8` / `int8` | 1 |
| `uint16` / `int16` | 2 |
| `uint32` / `int32` | 4 |
| `uint64` / `int64` | 8 |
| `uint128` / `int128` | 16 |
| `uint256` / `int256` | 32 |
| `address` | 20 |
| `bytes32` | 32 |

**Inefficient (Unpacked)**
```solidity
contract Unpacked {
    uint64 public a; // Occupies a full 32-byte slot
    uint256 public b; // Occupies a full 32-byte slot
    uint64 public c; // Occupies a full 32-byte slot
}
```

**Efficient (Packed)**
```solidity
contract Packed {
    uint64 public a; // Packed into a single slot
    uint64 public c; // Packed into the same slot as 'a'
    uint256 public b; // Occupies its own 32-byte slot
}
```

In the `Packed` contract, `a` and `c` are placed in the same storage slot, saving one `SSTORE` operation (20,000 gas for a new slot) during deployment.

### **`bytes` vs. `string`**

For short, fixed-size strings, `bytes32` is more gas-efficient than `string` or `bytes`.

| Operation | `bytes32` | `string` / `bytes` |
| :--- | :--- | :--- |
| **Storage** | Cheaper (1 slot) | More expensive (1 slot for length + data) |
| **Passing** | Cheaper | More expensive |

**Use Case:** Storing a short, unchanging name.

```solidity
contract NameRegistry {
    // More gas-efficient for names up to 32 bytes
    mapping(address => bytes32) public names;

    function setName(bytes32 newName) public {
        names[msg.sender] = newName;
    }
}
```

### **`mapping` vs. `array`**

| Feature | `mapping` | `array` (dynamic) |
| :--- | :--- | :--- |
| **Lookup** | `O(1)` | `O(n)` (if searching) |
| **Addition** | `O(1)` | Amortized `O(1)` |
| **Deletion** | Can't be cleared easily | `pop()` is `O(1)` |
| **Gas Cost**| Generally cheaper | More expensive due to length tracking |

Use `mapping` for key-value data retrieval. Arrays are better for iterable data, but loops over large storage arrays can be very costly.

***

## Function and Code Structure

Optimizing function visibility, logic, and overall contract structure can lead to substantial gas savings.

### **Function Visibility**

| Visibility | Description | Gas Implication |
| :--- | :--- | :--- |
| `external` | Can only be called from other contracts or via transactions. | Cheaper, as arguments are read directly from `calldata`. |
| `public` | Can be called internally or externally. | More expensive for external calls, as arguments are copied to `memory`. |
| `internal` | Can only be called from within the current contract or derived contracts. | - |
| `private` | Can only be called from within the current contract. | - |

Always use `external` instead of `public` for functions that will only be called externally.

### **Short-Circuiting**

In logical operations (`&&` and `||`), the EVM will stop evaluating as soon as the outcome is determined. Order your conditions from least to most expensive.

**Inefficient**
```solidity
function check(uint256 _a, uint256 _b) public view returns (bool) {
    // expensive_computation() is always called
    return expensive_computation(_a) && _b > 0;
}
```

**Efficient**
```solidity
function check(uint256 _a, uint256 _b) public view returns (bool) {
    // expensive_computation() is only called if _b > 0
    return _b > 0 && expensive_computation(_a);
}
```

### **Unchecked Math**

Since Solidity 0.8.0, arithmetic operations revert on overflow and underflow. If you are certain that an operation will not over/underflow, you can use an `unchecked` block to save the gas required for these checks.

```solidity
// Safe to use unchecked as 'i' will not overflow in a practical scenario
for (uint256 i = 0; i < length; i++) {
    unchecked {
        // ... operations with i ...
    }
}
```

### **`++i` vs `i++`**

In loops, using `++i` is slightly cheaper than `i++`. `i++` creates a temporary copy of `i` before incrementing it, while `++i` increments the value directly.

| Operation | Gas Cost (approx.) |
| :--- | :--- |
| `i++` | Higher |
| `++i` | Lower |

```solidity
// Preferred
for (uint i = 0; i < 10; ++i) {
    // ...
}
```

***

## Memory and Calldata

Properly utilizing memory and calldata can prevent unnecessary and expensive storage operations.

### **`calldata` vs `memory` for Function Arguments**

For `external` functions, use `calldata` for read-only dynamic data types (arrays, strings) to avoid copying them to `memory`.

| Location | Description | Use Case |
| :--- | :--- | :--- |
| `calldata` | Read-only data from an external call. | Arguments for `external` functions. |
| `memory` | Temporary, modifiable data that exists during function execution. | Temporary variables within functions. |

**Efficient `calldata` Usage**
```solidity
function processData(uint256[] calldata data) external pure returns (uint256) {
    uint256 sum = 0;
    for (uint256 i = 0; i < data.length; i++) {
        sum += data[i];
    }
    return sum;
}
```

### **Caching Storage Variables in Memory**

Reading from storage (`SLOAD`) is expensive. If you need to read a storage variable multiple times within a function, cache it in a `memory` variable.

**Inefficient**
```solidity
uint256 public value;

function calculate() public view returns (uint256) {
    // Multiple SLOADs
    uint256 a = value * 2;
    uint256 b = value + 5;
    return a + b;
}
```

**Efficient**
```solidity
uint256 public value;

function calculate() public view returns (uint256) {
    // Single SLOAD
    uint256 cachedValue = value;
    uint256 a = cachedValue * 2;
    uint256 b = cachedValue + 5;
    return a + b;
}
```

***

## Advanced Techniques

For further optimization, consider these advanced strategies.

### **Using `immutable` and `constant`**

If a state variable's value is known at compile time, declare it as `constant`. If it's set in the constructor and never changed, use `immutable`. The compiler replaces reads of these variables with their actual values, saving `SLOAD` operations.

| Keyword | Set At | Gas Cost |
| :--- | :--- | :--- |
| `constant`| Compile-time | Very Low |
| `immutable`| Deployment-time (constructor) | Low |

```solidity
contract Constants {
    // Value is directly embedded in the bytecode
    uint256 public constant MY_CONSTANT = 123;

    // Value is set in the constructor and stored in code, not storage
    address public immutable OWNER;

    constructor() {
        OWNER = msg.sender;
    }
}
```

### **Custom Errors**

Since Solidity 0.8.4, custom errors are cheaper than `require` statements with string messages. They result in smaller contract bytecode and lower gas costs on revert.

**Less Efficient**
```solidity
require(msg.sender == owner, "Not the owner");
```

**More Efficient**
```solidity
error NotOwner();

// ...

if (msg.sender != owner) {
    revert NotOwner();
}
```

### **Inline Assembly**

For fine-grained control, `assembly` can be used to perform operations more efficiently than high-level Solidity. This is an advanced technique and should be used with caution as it bypasses many of Solidity's safety checks.

**Use Case: Reading the keccak256 of a packed struct without creating a memory struct.**

```solidity
struct MyStruct {
    address addr;
    uint256 val;
}

function getStructHash(address _addr, uint256 _val) public pure returns (bytes32) {
    bytes32 hash;
    assembly {
        // Allocate memory for the struct
        let ptr := mload(0x40)
        mstore(ptr, _val)
        mstore(add(ptr, 0x20), _addr) // Note: address is 20 bytes, but we use a 32-byte slot
        hash := keccak256(ptr, 0x40) // Hash the 64 bytes
    }
    return hash;
}
```

### Pros and Cons of Gas Optimization Techniques

| Technique | Pros | Cons |
| :--- | :--- | :--- |
| **Variable Packing**| Reduces storage costs. | Can make data access slightly more complex. |
| **Using `immutable`**| Significant gas savings on reads. | Value cannot be changed after deployment. |
| **Custom Errors** | Cheaper reverts, smaller contract size. | Requires Solidity version 0.8.4+. |
| **Inline Assembly**| Maximum control and potential for high optimization. | Complex, error-prone, reduces readability, bypasses safety checks. |

***

### Real-World Projects & Usage

* **Uniswap V2/V3:** Heavily optimized for gas efficiency, especially in the core swapping and liquidity provision logic. They use many of the techniques described above, including tight variable packing and careful memory management.
* **Aave:** As a lending protocol, Aave's contracts handle complex calculations. Gas optimization is crucial to make borrowing and lending affordable for users.
* **ERC721A (Azuki):** An implementation of the NFT standard optimized for minting multiple tokens in a single transaction, significantly reducing gas costs for users during a mint event.

***

### Next Steps Suggestion:

For those looking to deepen their expertise, the next logical step is to study **Yul**, Solidity's intermediate assembly language. Understanding and writing directly in Yul provides maximum control over the EVM, allowing for hyper-optimization beyond what's achievable with inline assembly in Solidity. This knowledge is particularly valuable for building low-level infrastructure, complex libraries, and highly efficient smart contracts.