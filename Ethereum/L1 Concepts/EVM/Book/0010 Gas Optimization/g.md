# Gas Optimization in Solidity Smart Contracts

## **Gas Fundamentals**

Gas is the computational unit that measures the amount of computational effort required to execute operations on the Ethereum network. Every operation costs gas, and users pay for gas in ETH.

**Gas Price Components:**
- **Base Fee**: Minimum fee per gas unit (burned)
- **Priority Fee**: Tip to miners/validators
- **Gas Limit**: Maximum gas willing to spend
- **Gas Used**: Actual gas consumed

```solidity
// Gas cost example
contract GasExample {
    uint256 public value;
    
    // Costs ~21,000 + 20,000 gas (SSTORE from 0 to non-zero)
    function setValue(uint256 _value) external {
        value = _value;
    }
    
    // Costs ~21,000 + 5,000 gas (SSTORE from non-zero to non-zero)
    function updateValue(uint256 _value) external {
        value = _value;
    }
}
```

## **Storage Optimization**

### **Storage Layout and Packing**

Solidity packs multiple variables into a single 32-byte storage slot when possible.

```solidity
// Inefficient - uses 3 storage slots
contract Inefficient {
    uint128 a;  // Slot 0
    uint256 b;  // Slot 1 (full slot)
    uint128 c;  // Slot 2
}

// Efficient - uses 2 storage slots
contract Efficient {
    uint128 a;  // Slot 0 (first 16 bytes)
    uint128 c;  // Slot 0 (last 16 bytes)
    uint256 b;  // Slot 1
}
```

### **Variable Packing Strategies**

```solidity
contract PackingOptimized {
    // Pack related variables together
    struct UserInfo {
        address user;      // 20 bytes
        uint96 balance;    // 12 bytes - fits in same slot
        bool isActive;     // 1 byte - fits in same slot
        uint8 level;       // 1 byte - fits in same slot
    }
    
    // Timestamp packing
    struct TimeData {
        uint32 createdAt;  // 4 bytes - sufficient until 2106
        uint32 updatedAt;  // 4 bytes
        uint32 expiresAt;  // 4 bytes
        address owner;     // 20 bytes - total 32 bytes (1 slot)
    }
}
```

### **Storage vs Memory vs Calldata**

```solidity
contract StorageTypes {
    uint256[] public numbers;
    
    // Most expensive - modifies storage
    function addToStorage(uint256[] calldata _numbers) external {
        for (uint256 i = 0; i < _numbers.length; i++) {
            numbers.push(_numbers[i]);
        }
    }
    
    // Cheaper - uses memory
    function processInMemory(uint256[] calldata _numbers) external pure returns (uint256) {
        uint256[] memory temp = new uint256[](_numbers.length);
        uint256 sum = 0;
        for (uint256 i = 0; i < _numbers.length; i++) {
            temp[i] = _numbers[i] * 2;
            sum += temp[i];
        }
        return sum;
    }
    
    // Cheapest - direct calldata access
    function processCalldata(uint256[] calldata _numbers) external pure returns (uint256) {
        uint256 sum = 0;
        for (uint256 i = 0; i < _numbers.length; i++) {
            sum += _numbers[i];
        }
        return sum;
    }
}
```

## **Loop Optimization**

### **Caching Array Length**

```solidity
contract LoopOptimization {
    uint256[] public items;
    
    // Inefficient - reads length on every iteration
    function inefficientLoop() external view returns (uint256) {
        uint256 sum = 0;
        for (uint256 i = 0; i < items.length; i++) {
            sum += items[i];
        }
        return sum;
    }
    
    // Efficient - caches length
    function efficientLoop() external view returns (uint256) {
        uint256 sum = 0;
        uint256 length = items.length;
        for (uint256 i = 0; i < length; i++) {
            sum += items[i];
        }
        return sum;
    }
    
    // Most efficient - unchecked increment
    function ultraEfficientLoop() external view returns (uint256) {
        uint256 sum = 0;
        uint256 length = items.length;
        for (uint256 i = 0; i < length;) {
            sum += items[i];
            unchecked { ++i; }
        }
        return sum;
    }
}
```

## **Function Optimization**

### **Function Visibility and Mutability**

```solidity
contract FunctionOptimization {
    uint256 private _value;
    
    // More expensive - can modify state
    function getValue() public view returns (uint256) {
        return _value;
    }
    
    // Cheaper - explicitly pure
    function calculate(uint256 a, uint256 b) public pure returns (uint256) {
        return a + b;
    }
    
    // Cheapest - external with calldata
    function processData(bytes calldata data) external pure returns (bytes32) {
        return keccak256(data);
    }
    
    // Internal functions save gas when called multiple times
    function _internalCalculation(uint256 x) internal pure returns (uint256) {
        return x * x + 2 * x + 1;
    }
}
```

### **Short-Circuit Evaluation**

```solidity
contract ShortCircuit {
    mapping(address => bool) public authorized;
    uint256 public expensiveValue;
    
    function _expensiveOperation() internal view returns (bool) {
        // Simulate expensive operation
        return expensiveValue > 1000;
    }
    
    // Optimized - check cheaper condition first
    function isAuthorizedAndValid(address user) external view returns (bool) {
        return authorized[user] && _expensiveOperation();
    }
}
```

## **Data Structure Optimization**

### **Mappings vs Arrays**

```solidity
contract DataStructures {
    // Efficient for random access
    mapping(uint256 => uint256) public valueMap;
    
    // Efficient for iteration
    uint256[] public valueArray;
    
    // Hybrid approach for best of both worlds
    mapping(uint256 => uint256) public indexMap;
    uint256[] public keys;
    
    function addValue(uint256 key, uint256 value) external {
        if (indexMap[key] == 0) {
            keys.push(key);
            indexMap[key] = keys.length;
        }
        valueMap[key] = value;
    }
}
```

### **Struct Optimization**

```solidity
contract StructOptimization {
    // Inefficient struct - 4 storage slots
    struct BadStruct {
        uint256 id;        // Slot 0
        bool active;       // Slot 1
        uint256 amount;    // Slot 2
        address owner;     // Slot 3
    }
    
    // Optimized struct - 2 storage slots
    struct GoodStruct {
        uint256 id;        // Slot 0
        uint256 amount;    // Slot 1
        address owner;     // Slot 2 (20 bytes)
        bool active;       // Slot 2 (1 byte)
    }
    
    // Ultra-optimized with bit manipulation
    struct UltraStruct {
        uint256 packed;    // Single slot containing all data
    }
    
    function packData(uint256 id, uint256 amount, address owner, bool active) 
        external pure returns (uint256) {
        return uint256(uint160(owner)) | 
               (active ? 1 << 160 : 0) |
               (amount << 168) |
               (id << 200);
    }
}
```

## **Advanced Optimization Techniques**

### **Assembly Optimization**

```solidity
contract AssemblyOptimization {
    // Standard implementation
    function standardSum(uint256[] calldata arr) external pure returns (uint256) {
        uint256 sum = 0;
        for (uint256 i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        return sum;
    }
    
    // Assembly optimized version
    function assemblySum(uint256[] calldata arr) external pure returns (uint256 sum) {
        assembly {
            let len := arr.length
            let data := arr.offset
            for { let i := 0 } lt(i, len) { i := add(i, 1) } {
                sum := add(sum, calldataload(add(data, mul(i, 0x20))))
            }
        }
    }
    
    // Efficient memory copying
    function efficientMemcpy(bytes calldata source) external pure returns (bytes memory) {
        bytes memory result = new bytes(source.length);
        assembly {
            calldatacopy(add(result, 0x20), source.offset, source.length)
        }
        return result;
    }
}
```

### **Bit Manipulation Optimization**

```solidity
contract BitOptimization {
    // Store multiple booleans in single uint256
    uint256 private _flags;
    
    function setFlag(uint256 index, bool value) external {
        if (value) {
            _flags |= (1 << index);
        } else {
            _flags &= ~(1 << index);
        }
    }
    
    function getFlag(uint256 index) external view returns (bool) {
        return (_flags >> index) & 1 == 1;
    }
    
    // Efficient power of 2 check
    function isPowerOfTwo(uint256 x) external pure returns (bool) {
        return x != 0 && (x & (x - 1)) == 0;
    }
}
```

## **Gas Optimization Patterns**

### **Lazy Evaluation**

```solidity
contract LazyEvaluation {
    mapping(address => uint256) public balances;
    mapping(address => uint256) public lastUpdate;
    uint256 public constant DAILY_RATE = 100;
    
    // Only calculate when needed
    function getBalance(address user) external view returns (uint256) {
        uint256 baseBalance = balances[user];
        uint256 timeElapsed = block.timestamp - lastUpdate[user];
        return baseBalance + (timeElapsed * DAILY_RATE) / 86400;
    }
    
    // Update only when necessary
    function updateBalance(address user) external {
        balances[user] = getBalance(user);
        lastUpdate[user] = block.timestamp;
    }
}
```

### **Batch Operations**

```solidity
contract BatchOperations {
    mapping(address => uint256) public balances;
    
    // Inefficient - multiple transactions
    function singleTransfer(address to, uint256 amount) external {
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }
    
    // Efficient - batch processing
    function batchTransfer(address[] calldata recipients, uint256[] calldata amounts) 
        external {
        require(recipients.length == amounts.length, "Length mismatch");
        
        uint256 totalAmount = 0;
        uint256 length = recipients.length;
        
        // Calculate total first
        for (uint256 i = 0; i < length;) {
            totalAmount += amounts[i];
            unchecked { ++i; }
        }
        
        balances[msg.sender] -= totalAmount;
        
        // Process transfers
        for (uint256 i = 0; i < length;) {
            balances[recipients[i]] += amounts[i];
            unchecked { ++i; }
        }
    }
}
```

## **Gas Cost Comparison Tables**

### **Operation Costs**

| Operation | Gas Cost | Notes |
|-----------|----------|-------|
| SLOAD (read storage) | 2,100 | Cold storage access |
| SSTORE (write storage) | 20,000 | Zero to non-zero |
| SSTORE (update) | 5,000 | Non-zero to non-zero |
| CALL | 2,600 | Base cost |
| CREATE | 32,000 | Contract creation |
| LOG0 | 375 | Base log cost |
| KECCAK256 | 30 + 6/word | Hash function |

### **Data Type Efficiency**

| Type | Storage Slots | Gas Efficiency | Use Case |
|------|--------------|----------------|----------|
| uint256 | 1 | High | Large numbers |
| uint128 | 0.5 | Very High | Medium numbers, packing |
| uint64 | 0.25 | Highest | Timestamps, small amounts |
| address | 0.625 | High | Ethereum addresses |
| bool | 0.03125 | Variable | Flags (pack multiple) |

## **Common Gas Traps**

### **Dynamic Arrays**

```solidity
contract GasTraps {
    uint256[] public numbers;
    
    // Expensive - grows array
    function addNumber(uint256 num) external {
        numbers.push(num);  // 20,000+ gas for new slot
    }
    
    // Very expensive - deletes from middle
    function removeAt(uint256 index) external {
        for (uint256 i = index; i < numbers.length - 1; i++) {
            numbers[i] = numbers[i + 1];
        }
        numbers.pop();
    }
    
    // Efficient - swap and pop
    function efficientRemove(uint256 index) external {
        numbers[index] = numbers[numbers.length - 1];
        numbers.pop();
    }
}
```

### **String Concatenation**

```solidity
contract StringOptimization {
    // Expensive - creates new string each time
    function inefficientConcat(string[] calldata parts) 
        external pure returns (string memory) {
        string memory result = "";
        for (uint256 i = 0; i < parts.length; i++) {
            result = string(abi.encodePacked(result, parts[i]));
        }
        return result;
    }
    
    // Efficient - single allocation
    function efficientConcat(string[] calldata parts) 
        external pure returns (string memory) {
        return string(abi.encodePacked(parts[0], parts[1], parts[2]));
    }
}
```

## **Measurement and Profiling**

### **Gas Profiling Contract**

```solidity
contract GasProfiler {
    event GasUsed(string operation, uint256 gasUsed);
    
    modifier measureGas(string memory operation) {
        uint256 startGas = gasleft();
        _;
        uint256 gasUsed = startGas - gasleft();
        emit GasUsed(operation, gasUsed);
    }
    
    function testFunction() external measureGas("testFunction") {
        // Function implementation
    }
}
```

## **Optimization Strategies by Use Case**

### **DeFi Protocols**

```solidity
contract DeFiOptimization {
    // Pack price and timestamp
    struct PriceData {
        uint128 price;      // Sufficient for most tokens
        uint32 timestamp;   // Fits until 2106
        uint96 volume;      // Remaining space
    }
    
    mapping(address => PriceData) public prices;
    
    // Batch price updates
    function updatePrices(
        address[] calldata tokens,
        uint128[] calldata newPrices
    ) external {
        uint256 currentTime = block.timestamp;
        uint256 length = tokens.length;
        
        for (uint256 i = 0; i < length;) {
            prices[tokens[i]] = PriceData({
                price: newPrices[i],
                timestamp: uint32(currentTime),
                volume: 0
            });
            unchecked { ++i; }
        }
    }
}
```

### **NFT Contracts**

```solidity
contract NFTOptimization {
    // Packed token data
    struct TokenData {
        address owner;      // 20 bytes
        uint32 mintTime;    // 4 bytes
        uint8 rarity;       // 1 byte
        bool locked;        // 1 byte
    }
    
    mapping(uint256 => TokenData) public tokens;
    mapping(address => uint256) public balances;
    
    // Efficient batch minting
    function batchMint(address to, uint256 quantity) external {
        uint256 currentId = totalSupply();
        uint256 currentTime = block.timestamp;
        
        balances[to] += quantity;
        
        for (uint256 i = 0; i < quantity;) {
            tokens[currentId + i] = TokenData({
                owner: to,
                mintTime: uint32(currentTime),
                rarity: 1,
                locked: false
            });
            unchecked { ++i; }
        }
    }
    
    function totalSupply() public view returns (uint256) {
        // Implementation depends on contract
        return 0;
    }
}
```

## **Pros and Cons of Optimization Techniques**

### **Storage Packing**

| Pros | Cons |
|------|------|
| Significant gas savings | Increased code complexity |
| Reduced storage costs | Potential overflow risks |
| Better cache efficiency | Harder to debug |

### **Assembly Usage**

| Pros | Cons |
|------|------|
| Maximum gas efficiency | High complexity |
| Fine-grained control | Security risks |
| Bypass Solidity limitations | Reduced readability |

### **Batch Operations**

| Pros | Cons |
|------|------|
| Amortized gas costs | Higher single transaction cost |
| Better UX for multiple ops | Complexity in error handling |
| Reduced network congestion | Potential for partial failures |

## **Big O Complexity Analysis**

### **Common Patterns**

```solidity
contract ComplexityAnalysis {
    mapping(uint256 => uint256) public values;
    uint256[] public array;
    
    // O(1) - Constant time
    function constantTime(uint256 key) external view returns (uint256) {
        return values[key];
    }
    
    // O(n) - Linear time
    function linearTime() external view returns (uint256) {
        uint256 sum = 0;
        for (uint256 i = 0; i < array.length; i++) {
            sum += array[i];
        }
        return sum;
    }
    
    // O(n²) - Quadratic time (avoid!)
    function quadraticTime() external view returns (uint256) {
        uint256 sum = 0;
        for (uint256 i = 0; i < array.length; i++) {
            for (uint256 j = 0; j < array.length; j++) {
                sum += array[i] * array[j];
            }
        }
        return sum;
    }
}
```

## **Real-World Optimization Example**

```solidity
contract OptimizedERC20 {
    string public constant name = "OptimizedToken";
    string public constant symbol = "OPT";
    uint8 public constant decimals = 18;
    uint256 public totalSupply;
    
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    
    // Optimized transfer with unchecked arithmetic
    function transfer(address to, uint256 amount) external returns (bool) {
        address from = msg.sender;
        
        // Save gas by using unchecked block
        unchecked {
            require(balanceOf[from] >= amount, "Insufficient balance");
            balanceOf[from] -= amount;
            balanceOf[to] += amount;
        }
        
        emit Transfer(from, to, amount);
        return true;
    }
    
    // Batch transfer optimization
    function batchTransfer(
        address[] calldata recipients,
        uint256[] calldata amounts
    ) external returns (bool) {
        address from = msg.sender;
        uint256 length = recipients.length;
        require(length == amounts.length, "Length mismatch");
        
        uint256 totalAmount = 0;
        
        // Calculate total amount first
        for (uint256 i = 0; i < length;) {
            totalAmount += amounts[i];
            unchecked { ++i; }
        }
        
        require(balanceOf[from] >= totalAmount, "Insufficient balance");
        
        unchecked {
            balanceOf[from] -= totalAmount;
            
            // Process transfers
            for (uint256 i = 0; i < length;) {
                address to = recipients[i];
                uint256 amount = amounts[i];
                balanceOf[to] += amount;
                emit Transfer(from, to, amount);
                ++i;
            }
        }
        
        return true;
    }
}
```

## **Testing Gas Optimization**

```solidity
// Test contract for gas measurement
contract GasTest {
    function testGasUsage() external {
        uint256 startGas = gasleft();
        
        // Your optimized function here
        
        uint256 gasUsed = startGas - gasleft();
        // Log or return gasUsed for analysis
    }
}
```

## **Tricky Edge Cases**

### **Storage Collision**

```solidity
// Be careful with storage layout changes
contract StorageCollision {
    uint256 public value1;
    // Adding variables here breaks storage layout
    uint256 public value2;
    
    // Safer approach: use gaps for future variables
    uint256[50] private __gap;
}
```

### **Optimizer Pitfalls**

```solidity
contract OptimizerPitfalls {
    // Optimizer might not optimize this loop
    function unoptimizableLoop(uint256 n) external pure returns (uint256) {
        uint256 result = 0;
        for (uint256 i = 0; i < n; i++) {
            result += i * i;
        }
        return result;
    }
    
    // Manual optimization often needed
    function manuallyOptimized(uint256 n) external pure returns (uint256) {
        return n * (n - 1) * (2 * n - 1) / 6;
    }
}
```

**Next Steps Suggestion:**
**Advanced Solidity Assembly and EVM Internals** - Dive deep into inline assembly, understanding EVM opcodes, memory management, and creating highly optimized contracts that directly manipulate the EVM stack and memory layout for maximum gas efficiency.