 ## Storage Optimization

### State Variable Packing

Solidity stores state variables in 32-byte slots. Packing smaller data types together can save significant gas.

```solidity
// Bad - Uses 3 storage slots
contract Inefficient {
    uint8 a;     // Slot 0 (31 bytes wasted)
    uint256 b;   // Slot 1
    uint8 c;     // Slot 2 (31 bytes wasted)
}

// Good - Uses 2 storage slots
contract Efficient {
    uint8 a;     // Slot 0
    uint8 c;     // Slot 0 (packed)
    uint256 b;   // Slot 1
}
```

### Storage vs Memory vs Calldata

| Location | Gas Cost | Persistence | Use Case |
|----------|----------|-------------|----------|
| Storage | Write: 20,000 (cold), 5,000 (warm)<br>Read: 2,100 (cold), 100 (warm) | Permanent | State variables |
| Memory | ~3 gas per 32 bytes | Function scope | Temporary complex data |
| Calldata | Cheapest (read-only) | Function scope | External function parameters |

```solidity
// Expensive - copies array to memory
function bad(uint[] memory data) external {
    // Process data
}

// Cheap - reads directly from calldata
function good(uint[] calldata data) external {
    // Process data
}
```

### SSTORE Optimizations

```solidity
contract StoragePatterns {
    uint256 public value;
    
    // Bad - Multiple SSTORE operations
    function inefficient() external {
        value = 1;  // 20,000 gas
        value = 2;  // 5,000 gas
        value = 3;  // 5,000 gas
    }
    
    // Good - Single SSTORE
    function efficient() external {
        uint256 temp = 1;
        temp = 2;
        temp = 3;
        value = temp;  // 20,000 gas only
    }
    
    // Storage to storage copies
    mapping(uint => uint) data;
    
    function copyStorage(uint from, uint to) external {
        // Bad - Two storage reads
        data[to] = data[from];
        
        // Good - Cache in memory
        uint temp = data[from];
        data[to] = temp;
    }
}
```

## Loop Optimizations

### Loop Patterns

```solidity
contract LoopOptimization {
    uint[] public data;
    
    // Bad - Length calculated each iteration
    function inefficientLoop() external view returns (uint sum) {
        for (uint i = 0; i < data.length; i++) {
            sum += data[i];
        }
    }
    
    // Good - Cache length
    function efficientLoop() external view returns (uint sum) {
        uint length = data.length;
        for (uint i = 0; i < length; i++) {
            sum += data[i];
        }
    }
    
    // Better - Use unchecked for index
    function optimalLoop() external view returns (uint sum) {
        uint length = data.length;
        for (uint i = 0; i < length;) {
            sum += data[i];
            unchecked { ++i; }
        }
    }
    
    // Best for known operations - Unroll loops
    function unrolledLoop(uint[4] memory arr) external pure returns (uint sum) {
        sum = arr[0] + arr[1] + arr[2] + arr[3];
    }
}
```

### Array Operations

```solidity
contract ArrayGasOptimization {
    uint[] public arr;
    
    // Expensive - Dynamic array in memory
    function badArray() external pure returns (uint[] memory) {
        uint[] memory temp = new uint[](100);
        for (uint i = 0; i < 100; i++) {
            temp[i] = i;
        }
        return temp;
    }
    
    // Cheaper - Fixed size when possible
    function goodArray() external pure returns (uint[100] memory) {
        uint[100] memory temp;
        for (uint i = 0; i < 100; i++) {
            temp[i] = i;
        }
        return temp;
    }
}
```

## Short-Circuiting and Conditionals

```solidity
contract ConditionalOptimization {
    // Order conditions by likelihood
    function checkConditions(uint x) external pure returns (bool) {
        // Bad - Expensive check first
        if (isPrime(x) && x > 0 && x < 100) {
            return true;
        }
        
        // Good - Cheap checks first
        if (x > 0 && x < 100 && isPrime(x)) {
            return true;
        }
        
        return false;
    }
    
    // Use short-circuit evaluation
    function shortCircuit(uint a, uint b) external pure returns (bool) {
        // If a == 0, expensiveCheck(b) never runs
        return a != 0 && expensiveCheck(b);
    }
    
    function isPrime(uint) internal pure returns (bool) {
        // Complex computation
        return true;
    }
    
    function expensiveCheck(uint) internal pure returns (bool) {
        // Complex computation
        return true;
    }
}
```

## Comparison: Pre/Post Increment

```solidity
contract IncrementComparison {
    // Post-increment (i++) - More expensive
    function postIncrement() external pure returns (uint) {
        uint i = 0;
        uint j = i++; // Creates temporary variable
        return j;     // Returns 0
    }
    
    // Pre-increment (++i) - Cheaper
    function preIncrement() external pure returns (uint) {
        uint i = 0;
        uint j = ++i; // No temporary variable
        return j;     // Returns 1
    }
}
```

| Operation | Gas Cost | Use Case |
|-----------|----------|----------|
| i++ | ~6 gas more | When old value needed |
| ++i | Baseline | Default choice |
| i += 1 | Same as ++i | Clarity |
| unchecked { ++i } | Cheapest | When overflow impossible |

## Function Optimization

### Visibility Modifiers Impact

```solidity
contract VisibilityGas {
    // External - Cheapest for external calls
    function externalFunc(uint[] calldata data) external {}
    
    // Public - More expensive, creates internal + external interface
    function publicFunc(uint[] memory data) public {}
    
    // Internal - Only callable internally
    function internalFunc(uint[] memory data) internal {}
    
    // Private - Same cost as internal, better optimization potential
    function privateFunc(uint[] memory data) private {}
}
```

### Function Modifiers vs Internal Functions

```solidity
contract ModifierVsInternal {
    uint public value;
    
    // Modifier - Inlines code (more deployment gas)
    modifier onlyPositive(uint x) {
        require(x > 0, "Must be positive");
        _;
    }
    
    function withModifier(uint x) external onlyPositive(x) {
        value = x;
    }
    
    // Internal function - Less deployment gas, slight runtime overhead
    function checkPositive(uint x) internal pure {
        require(x > 0, "Must be positive");
    }
    
    function withInternal(uint x) external {
        checkPositive(x);
        value = x;
    }
}
```

## Error Handling Optimization

```solidity
contract ErrorOptimization {
    // Expensive - Long revert string
    function badError(uint x) external pure {
        require(x > 0, "Value must be greater than zero for this function to work properly");
    }
    
    // Better - Short revert string
    function betterError(uint x) external pure {
        require(x > 0, "Value <= 0");
    }
    
    // Best - Custom errors (Solidity 0.8.4+)
    error ValueNotPositive();
    
    function bestError(uint x) external pure {
        if (x == 0) revert ValueNotPositive();
    }
    
    // Gas comparison
    // Long string: ~2500 gas on revert
    // Short string: ~2000 gas on revert
    // Custom error: ~1500 gas on revert
}
```

## Mathematical Operations

```solidity
contract MathOptimization {
    // Division and multiplication
    function inefficientMath(uint a, uint b, uint c) external pure returns (uint) {
        return a / b * c; // Potential precision loss
    }
    
    function efficientMath(uint a, uint b, uint c) external pure returns (uint) {
        return a * c / b; // Better precision
    }
    
    // Power of 2 operations
    function expensive(uint x) external pure returns (uint) {
        return x * 2;   // Uses MUL opcode
    }
    
    function cheap(uint x) external pure returns (uint) {
        return x << 1;  // Uses SHL opcode (3 gas vs 5 gas)
    }
    
    // Unchecked blocks for known safe operations
    function withoutUnchecked(uint x) external pure returns (uint) {
        uint result = x + 1; // Overflow check included
        return result;
    }
    
    function withUnchecked(uint x) external pure returns (uint) {
        unchecked {
            uint result = x + 1; // No overflow check
            return result;
        }
    }
}
```

## Mapping vs Array Trade-offs

| Aspect | Mapping | Array |
|--------|---------|-------|
| Gas for access | O(1) - ~2100 gas | O(n) - varies |
| Enumeration | Not possible | Possible |
| Deletion | Sets to default | Can remove element |
| Storage cost | Pay per non-zero entry | Pay for declared size |
| Existence check | Requires separate tracking | length property |

```solidity
contract MappingVsArray {
    // Mapping approach
    mapping(address => uint) public balances;
    mapping(address => bool) public hasBalance;
    
    function setBalance(address user, uint amount) external {
        balances[user] = amount;
        hasBalance[user] = amount > 0;
    }
    
    // Array approach
    address[] public users;
    uint[] public userBalances;
    
    function addUser(address user, uint balance) external {
        users.push(user);
        userBalances.push(balance);
    }
    
    function getBalance(address user) external view returns (uint) {
        for (uint i = 0; i < users.length; i++) {
            if (users[i] == user) {
                return userBalances[i];
            }
        }
        return 0;
    }
}
```

## Events vs Storage

```solidity
contract EventVsStorage {
    // Expensive - Storing historical data
    struct Transaction {
        address from;
        address to;
        uint amount;
        uint timestamp;
    }
    Transaction[] public transactions;
    
    function recordTransactionExpensive(address to, uint amount) external {
        transactions.push(Transaction(msg.sender, to, amount, block.timestamp));
    }
    
    // Cheap - Using events
    event TransactionRecorded(address indexed from, address indexed to, uint amount, uint timestamp);
    
    function recordTransactionCheap(address to, uint amount) external {
        emit TransactionRecorded(msg.sender, to, amount, block.timestamp);
    }
}
```

## Batch Operations

```solidity
contract BatchOperations {
    mapping(address => uint) public balances;
    
    // Expensive - Multiple transactions
    function singleTransfer(address to, uint amount) external {
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }
    
    // Efficient - Batch processing
    function batchTransfer(address[] calldata recipients, uint[] calldata amounts) external {
        require(recipients.length == amounts.length, "Length mismatch");
        
        uint totalAmount = 0;
        for (uint i = 0; i < recipients.length; i++) {
            totalAmount += amounts[i];
        }
        
        require(balances[msg.sender] >= totalAmount, "Insufficient balance");
        balances[msg.sender] -= totalAmount;
        
        for (uint i = 0; i < recipients.length; i++) {
            balances[recipients[i]] += amounts[i];
        }
    }
}
```

## Common Patterns and Anti-patterns

### Initialization Patterns

```solidity
contract InitializationPatterns {
    // Anti-pattern: Initializing to default
    uint public value = 0; // Wastes gas
    
    // Pattern: Let it default
    uint public value2; // Defaults to 0
    
    // Anti-pattern: Redundant initialization in constructor
    uint[] public arr;
    constructor() {
        arr = new uint[](0); // Unnecessary
    }
    
    // Pattern: Clone pattern for cheap deployment
    address public implementation;
    
    function clone() external returns (address) {
        bytes memory bytecode = abi.encodePacked(
            hex"3d602d80600a3d3981f3363d3d373d3d3d363d73",
            implementation,
            hex"5af43d82803e903d91602b57fd5bf3"
        );
        address addr;
        assembly {
            addr := create2(0, add(bytecode, 0x20), mload(bytecode), salt)
        }
        return addr;
    }
}
```

### Comparison with Other Optimizations

| Optimization Type | Gas Saved | Complexity | Risk |
|------------------|-----------|------------|------|
| Storage packing | High (5000-15000) | Low | Low |
| Calldata vs Memory | Medium (500-2000) | Low | None |
| Custom errors | Low (500-1000) | Low | None |
| Unchecked math | Low (50-200) | Medium | High |
| Assembly usage | High (varies) | High | High |
| Proxy patterns | Very High | High | Medium |

## Real-World Example: Optimized Token Contract

```solidity
contract OptimizedToken {
    // Packed storage
    mapping(address => uint256) private _balances;
    mapping(address => mapping(address => uint256)) private _allowances;
    
    // Events for cheap storage
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    
    // Custom errors
    error InsufficientBalance();
    error InsufficientAllowance();
    
    uint256 private _totalSupply;
    
    function transfer(address to, uint256 amount) external returns (bool) {
        address owner = msg.sender;
        _transfer(owner, to, amount);
        return true;
    }
    
    function _transfer(address from, address to, uint256 amount) private {
        uint256 fromBalance = _balances[from];
        if (fromBalance < amount) revert InsufficientBalance();
        
        unchecked {
            _balances[from] = fromBalance - amount;
            _balances[to] += amount;
        }
        
        emit Transfer(from, to, amount);
    }
    
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        address spender = msg.sender;
        uint256 currentAllowance = _allowances[from][spender];
        
        if (currentAllowance != type(uint256).max) {
            if (currentAllowance < amount) revert InsufficientAllowance();
            unchecked {
                _allowances[from][spender] = currentAllowance - amount;
            }
        }
        
        _transfer(from, to, amount);
        return true;
    }
}
```

## Advanced Techniques

### Assembly Optimization

```solidity
contract AssemblyOptimization {
    // Efficient address check
    function isContract(address account) external view returns (bool) {
        uint256 size;
        assembly {
            size := extcodesize(account)
        }
        return size > 0;
    }
    
    // Efficient memory operations
    function efficientCopy(bytes memory data) external pure returns (bytes memory) {
        bytes memory result = new bytes(data.length);
        assembly {
            let len := mload(data)
            let dataPtr := add(data, 0x20)
            let resultPtr := add(result, 0x20)
            
            for { let i := 0 } lt(i, len) { i := add(i, 0x20) } {
                mstore(add(resultPtr, i), mload(add(dataPtr, i)))
            }
        }
        return result;
    }
}
```

### Storage Slot Manipulation

```solidity
contract StorageSlotOptimization {
    // Direct slot access
    function getSlot(uint256 slot) external view returns (bytes32 value) {
        assembly {
            value := sload(slot)
        }
    }
    
    function setSlot(uint256 slot, bytes32 value) external {
        assembly {
            sstore(slot, value)
        }
    }
    
    // Efficient packed struct storage
    struct PackedData {
        uint128 a;
        uint64 b;
        uint64 c;
    }
    
    function packData(uint128 a, uint64 b, uint64 c) external pure returns (uint256) {
        return uint256(a) | (uint256(b) << 128) | (uint256(c) << 192);
    }
}
```

## Pros and Cons Summary

### Storage Optimization
**Pros:**
- Significant gas savings (5000-15000 per slot)
- Permanent optimization
- No runtime complexity

**Cons:**
- Reduced readability
- Harder maintenance
- Potential upgrade issues

### Loop Unrolling
**Pros:**
- Eliminates loop overhead
- Predictable gas costs
- Better for small, fixed iterations

**Cons:**
- Increased bytecode size
- Less flexible
- Higher deployment cost

### Assembly Usage
**Pros:**
- Maximum efficiency
- Direct EVM control
- Unique optimizations possible

**Cons:**
- High complexity
- Security risks
- Audit difficulty
- Maintenance burden

