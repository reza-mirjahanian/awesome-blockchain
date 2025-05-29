 # **Ethereum Virtual Machine (EVM) Deep Dive**

## **EVM Architecture Overview**

### **What is the Ethereum Virtual Machine?**

The **Ethereum Virtual Machine** is a *stack-based virtual machine* that executes smart contracts on the Ethereum blockchain. It provides a **sandboxed execution environment** where bytecode instructions are processed sequentially through a series of **opcodes**.

### **Key EVM Components**

- **Stack**: *Primary data structure* for temporary value storage
- **Memory**: *Volatile storage* for contract execution
- **Storage**: *Persistent state* maintained between transactions
- **Call Data**: *Input data* sent to contract functions
- **Program Counter**: *Instruction pointer* tracking execution position

## **Smart Contract Interaction Fundamentals**

### **Call Data Structure**

**Call data** contains the *instructions and parameters* for contract execution:

1. **Function Selector** (First 4 bytes)
   - Example: `2E 1a 7D 4D` 
   - Corresponds to function signature: `"withdraw uint256"`
   - Generated using **Keccak-256 hash** of function signature

2. **Function Parameters** (Remaining bytes)
   - *Encoded arguments* following ABI specification
   - Each parameter padded to **32 bytes**

```solidity
// Function call example
contract.withdraw(100);
// Call data: 2E1a7D4D + 0000000000000000000000000000000000000000000000000000000000000064
//            selector  + padded parameter (100 in hex = 0x64)
```

### **Transaction Components**

- **To Address**: *Target contract* address
- **Value**: *ETH amount* sent with transaction
- **Gas Limit**: *Maximum computational* resources allowed
- **Call Data**: *Function selector* and encoded parameters

## **EVM Opcodes Categories**

### **Arithmetic Operations**

**Basic Mathematical Operations:**
- `ADD`: *Addition* of two stack values
- `SUB`: *Subtraction* operation
- `MUL`: *Multiplication* of values
- `DIV`: *Integer division* (unsigned)
- `SDIV`: *Signed division* operation
- `MOD`: *Modulo operation* (remainder)
- `SMOD`: *Signed modulo* operation

**Advanced Arithmetic:**
- `ADDMOD`: *Modular addition* - `(a + b) % n`
- `MULMOD`: *Modular multiplication* - `(a * b) % n`
- `EXP`: *Exponentiation* operation
- `SIGNEXTEND`: *Sign extension* for two's complement

```assembly
PUSH1 0x05    ; Push 5 onto stack
PUSH1 0x03    ; Push 3 onto stack
ADD           ; Add: 5 + 3 = 8
PUSH1 0x0A    ; Push 10 onto stack
MOD           ; Modulo: 8 % 10 = 8
```

### **Comparison Operations**

**Relational Operators:**
- `LT`: *Less than* comparison (unsigned)
- `GT`: *Greater than* comparison (unsigned)
- `SLT`: *Signed less than*
- `SGT`: *Signed greater than*
- `EQ`: *Equality* check
- `ISZERO`: *Zero check* - returns 1 if zero, 0 otherwise

### **Bitwise Operations**

**Important Note:** EVM operations are *bitwise*, not logical!

**Core Bitwise Operations:**
- `AND`: *Bitwise AND* operation
- `OR`: *Bitwise OR* operation
- `XOR`: *Bitwise exclusive OR*
- `NOT`: *Bitwise NOT* (complement)
- `BYTE`: *Extract specific byte* from word

**Shift Operations:**
- `SHL`: *Shift left* (logical)
- `SHR`: *Shift right* (logical)
- `SAR`: *Arithmetic right shift* (preserves sign)

```assembly
PUSH1 0x0F    ; Push 15 (0x0F)
PUSH1 0x0F    ; Push 15 (0x0F)
AND           ; Bitwise AND: 0x0F & 0x0F = 0x0F
```

## **Stack Management**

### **Stack Architecture**

The EVM uses a **Last-In-First-Out (LIFO)** stack with specific limitations:

- **Maximum depth**: *1024 items*
- **Direct access**: *Only top 16 items*
- **Item size**: *256 bits* (32 bytes) each

### **Stack Operations**

**Push Operations:**
- `PUSH1` through `PUSH32`: *Push 1-32 bytes* onto stack
- Most common: `PUSH1` for *single byte values*

**Stack Manipulation:**
- `POP`: *Remove top item* from stack
- `DUP1` through `DUP16`: *Duplicate stack items* (1st to 16th from top)
- `SWAP1` through `SWAP16`: *Swap top item* with nth item

### **Stack Too Deep Error**

The *infamous "stack too deep" error* occurs when:
- Function has **more than 16 local variables**
- Compiler cannot access variables beyond *16th stack position*

**Solutions:**
1. **Reduce local variables**
2. **Use structs** to group variables
3. **Split complex functions**
4. **Use memory** for temporary storage

```solidity
// Problematic function (too many variables)
function complexFunction() external {
    uint256 var1 = 1;
    uint256 var2 = 2;
    // ... 15+ more variables
    uint256 var17 = 17; // Causes "stack too deep"
}

// Solution: Use struct
struct Variables {
    uint256 var1;
    uint256 var2;
    // ... group related variables
}
```

## **Memory and Storage Operations**

### **Memory Operations**

**Memory Opcodes:**
- `MLOAD`: *Load 32 bytes* from memory
- `MSTORE`: *Store 32 bytes* to memory
- `MSTORE8`: *Store single byte* to memory
- `MSIZE`: *Get memory size* in bytes

### **Storage Operations**

**Persistent Storage:**
- `SLOAD`: *Load value* from storage slot
- `SSTORE`: *Store value* to storage slot
- **Gas costs**: Storage operations are *expensive*

### **Memory Initialization Pattern**

Most Solidity contracts start with **memory pointer setup**:

```assembly
PUSH1 0x60    ; Push 96 (0x60)
PUSH1 0x40    ; Push 64 (0x40)
MSTORE        ; Store 0x60 at memory position 0x40
              ; Sets "free memory pointer"
```

## **Call Data Operations**

### **Call Data Opcodes**

- `CALLDATALOAD`: *Load 32 bytes* from call data at offset
- `CALLDATASIZE`: *Get total size* of call data
- `CALLDATACOPY`: *Copy call data* to memory

### **Function Selector Extraction**

```assembly
PUSH1 0x00        ; Push offset 0
CALLDATALOAD      ; Load first 32 bytes of call data
PUSH1 0xE0        ; Push 224 (32-4)*8 bits
SHR               ; Shift right to get first 4 bytes
                  ; Result: function selector
```

## **Contract Interaction Opcodes**

### **External Call Operations**

**CALL Opcode:**
- **Purpose**: *Message call* to another account
- **Parameters**: Gas, address, value, input data
- **Returns**: Success flag and return data

```assembly
PUSH1 0x00    ; retSize (return data size)
PUSH1 0x00    ; retOffset (return data offset)
PUSH1 0x00    ; argSize (argument size)
PUSH1 0x00    ; argOffset (argument offset)
PUSH1 0x00    ; value (ETH to send)
PUSH20 0x...  ; address (target contract)
PUSH2 0x8FC   ; gas (gas limit)
CALL          ; Execute call
```

**DELEGATECALL:**
- **Purpose**: *Execute code* in current contract's context
- **Key feature**: Preserves `msg.sender` and `msg.value`
- **Use case**: **Proxy patterns** and upgradeable contracts

**STATICCALL:**
- **Purpose**: *Read-only* external calls
- **Guarantee**: Cannot modify state
- **Use case**: **View functions** and safe queries

### **Contract Creation**

**CREATE Opcode:**
- **Purpose**: *Deploy new contract*
- **Address**: Determined by `keccak256(creator_address, nonce)`

**CREATE2 Opcode:**
- **Purpose**: *Deploy with deterministic address*
- **Address**: `keccak256(0xFF, creator, salt, init_code_hash)`
- **Use case**: **Vanity address mining**

```solidity
// CREATE2 example for vanity addresses
function deployContract(bytes32 salt) external {
    address predictedAddress = address(uint160(uint256(keccak256(
        abi.encodePacked(
            hex"ff",
            address(this),
            salt,
            keccak256(bytecode)
        )
    ))));
    
    // Mine for desired address pattern
    require(predictedAddress & 0xFFFF == 0x1337, "Not vanity address");
}
```

## **Advanced EVM Concepts**

### **Cryptographic Operations**

**KECCAK256 (SHA3):**
- **Also known as**: SHA3 in some contexts
- **Purpose**: *Primary hashing function* in Ethereum
- **Usage**: Function selectors, storage keys, merkle trees

### **Environment Information**

**Transaction Context:**
- `ADDRESS`: *Current contract* address
- `BALANCE`: *Account balance* query
- `ORIGIN`: *Transaction originator* (EOA)
- `CALLER`: *Immediate caller* address
- `CALLVALUE`: *ETH amount* sent with call
- `GASPRICE`: *Gas price* of transaction

### **Code Inspection Operations**

**External Code Operations:**
- `EXTCODESIZE`: *Get code size* of external account
- `EXTCODECOPY`: *Copy external code* to memory
- `EXTCODEHASH`: *Get code hash* of account

### **Creative Storage Patterns**

**SSTORE2 Pattern:**
Using `EXTCODECOPY` for *cheap data storage*:

1. **Deploy contract** with data as bytecode
2. **Use EXTCODECOPY** to retrieve data later
3. **Cost advantage**: Code storage cheaper than SSTORE

```solidity
// Store data as contract bytecode
function storeData(bytes memory data) external returns (address) {
    bytes memory bytecode = abi.encodePacked(
        hex"00",  // Invalid opcode prevents execution
        data
    );
    
    address dataContract;
    assembly {
        dataContract := create2(0, add(bytecode, 0x20), mload(bytecode), salt)
    }
    return dataContract;
}

// Retrieve stored data
function readData(address dataContract) external view returns (bytes memory) {
    uint256 size = dataContract.code.length - 1; // Exclude invalid opcode
    bytes memory data = new bytes(size);
    
    assembly {
        extcodecopy(dataContract, add(data, 0x20), 1, size)
    }
    return data;
}
```

## **Return Data Handling**

### **Return Data Opcodes**

- `RETURNDATASIZE`: *Size of return data* from last external call
- `RETURNDATACOPY`: *Copy return data* to memory

### **Creative Optimization Techniques**

**Zero Value Trick:**
Before `PUSH0` opcode was available, developers used:

```assembly
RETURNDATASIZE    ; Returns 0 if no external call made
                  ; Cheaper than PUSH1 0x00
```

This technique exploits the fact that `RETURNDATASIZE` returns **zero** when no external call has been made in the current execution context.

## **EVM Development Tools**

### **EVM Playground**

**EVM.codes** provides:
- **Interactive opcode testing**
- **Step-by-step execution** visualization
- **Stack and memory** inspection
- **Gas cost** analysis

### **Bytecode Analysis Tools**

**Heimdall** specializes in:
- **Bytecode to Solidity** decompilation
- **Control flow graph** generation
- **Function signature** recovery
- **Storage layout** analysis

### **Assembly Integration**

Solidity supports **inline assembly** for direct EVM programming:

```solidity
function efficientOperation() external pure returns (uint256 result) {
    assembly {
        // Direct EVM opcodes
        let a := 0x42
        let b := 0x24
        result := add(a, b)
    }
}
```

## **Gas Optimization Strategies**

### **Opcode Gas Costs**

**Low Cost Operations** (3 gas):
- Arithmetic: `ADD`, `SUB`, `MUL`
- Bitwise: `AND`, `OR`, `XOR`
- Comparison: `LT`, `GT`, `EQ`

**Medium Cost Operations** (5-8 gas):
- `DIV`, `MOD`, `SIGNEXTEND`
- `BYTE`, `SHL`, `SHR`

**High Cost Operations**:
- `KECCAK256`: *30 gas + 6 per word*
- `SSTORE`: *20,000 gas* (new) / *5,000 gas* (update)
- External calls: *2,300+ gas*

### **Optimization Techniques**

1. **Bit manipulation** instead of division/multiplication by powers of 2
2. **Pack variables** to minimize storage slots
3. **Use events** instead of storage for historical data
4. **Batch operations** to reduce transaction overhead

```solidity
// Optimized bit operations
function divideByEight(uint256 value) external pure returns (uint256) {
    return value >> 3;  // Shift right by 3 (2^3 = 8)
}

function multiplyByFour(uint256 value) external pure returns (uint256) {
    return value << 2;  // Shift left by 2 (2^2 = 4)
}
```