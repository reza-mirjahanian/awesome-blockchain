**Gas Optimization in Solidity Smart Contracts**

---

### **1. Storage Optimization**
Storage operations are the most expensive (20,000–50,000 gas for writes).  
**Tips:**  
- **Pack variables into structs** using `uint128`/`address` instead of separate slots.  
- **Use mappings instead of arrays** for dynamic data.  
- **Initialize arrays** with `new T[](length)` to avoid repeated `push()`.  
- **Delete unused storage** to reclaim 15,000 gas via refunds.  

**Code Example:**  
```solidity
struct PackedData {
    uint128 a;
    uint128 b;
    address c;
} // Occupies 1 storage slot (32 bytes)
```

**Pros/Cons Table:**  
| Pros | Cons |  
|------|------|  
| Reduces storage slots used | Requires careful type alignment |  
| Lowers gas for writes/reads | Harder to read/maintain |  

**Gas Costs:**  
- `SSTORE`: 20,000 (write), 5,000 (reset), 15,000 refund (delete)  
- `SLOAD`: 800 gas  

---

### **2. Computation Optimization**  
Avoid expensive operations:  
- **Bitwise operations** over arithmetic (e.g., `x << 1` vs `x * 2`).  
- **Precompute values** off-chain (e.g., hash arguments).  
- **Avoid unbounded loops**: Use `O(1)` logic.  

**Code Example:**  
```solidity
function square(uint x) external pure returns (uint) {
    return x * x; // Cost: 10 gas (MUL)
}
function squareShift(uint x) external pure returns (uint) {
    return x << 1; // Cost: 3 gas (SHL)
}
```

**Complexity Comparison:**  
| Operation | Gas Cost | Time Complexity |  
|----------|-----------|------------------|  
| Division (`÷`) | 50 gas | O(1) |  
| Bit Shifting (`<<`) | 3 gas | O(1) |  

---

### **3. Memory vs. Storage**  
- **Memory** (temporary) is cheaper than **storage** (persistent).  
- Use `calldata` for external function args (cheaper than memory).  

**Code Example:**  
```solidity
function sumCalldata(uint[] calldata arr) external pure returns (uint) {
    uint total = 0; // Stored in memory
    for (uint i = 0; i < arr.length; i++) total += arr[i];
    return total;
}
```

---

### **4. Function Optimizations**  
- **Use `external` functions** for public calls.  
- **Inline small functions** to avoid call overhead.  
- **Use `pure`/`view`** to signal no state changes.  

**Code Example:**  
```solidity
function add(uint a, uint b) internal pure returns (uint) {
    return a + b; // Inlined
}
```

---

### **5. Events**  
- **Indexed parameters** reduce off-chain search costs.  
- Emit fewer events; prefer batched logs.  

**Code Example:**  
```solidity
event Transfer(address indexed from, address indexed to, uint amount);
```

---

### **6. External Calls**  
- Use `call`/`delegatecall` for low-level interactions.  
- **Batch transactions** to reduce overhead.  

**Code Example:**  
```solidity
(bool success, ) = recipient.call{value: 1 ether}("");
require(success, "Transfer failed");
```

**Pros/Cons:**  
| Pros | Cons |  
|------|------|  
| Cheaper than contract calls | Less safe (no type checks) |  

---

### **7. Deployment Optimization**  
- **Enable Solidity Optimizer**: `--optimize --optimize-runs`.  
- **Use minimal proxies** (e.g., EIP-1167).  

**Compiler Flag:**  
```bash
solc --optimize --optimize-runs 200 contract.sol
```

---

### **8. Edge Cases**  
- **Block gas limit**: ~30M gas per block.  
- **Reentrancy**: Gas limits on `.call` (2300 for `transfer`).  
- **Out-of-gas (OOG)**: Complex loops may hit limits.  

**Code Example:**  
```solidity
function badLoop() public {
    for (uint i = 0; i < 1e20; i++) {} // OOG error
}
```

---

### **Real-World Examples**  
- **Uniswap V3**: Uses bit-packing for liquidity positions.  
- **OpenZeppelin**: Gas-efficient `SafeMath` library.  

---

### **Comparison with Similar Concepts**  
| Technique | Trade-off | Use Case |  
|----------|-----------|----------|  
| Layer 2 (e.g., Optimism) | Lower on-chain gas | High-throughput apps |  
| Off-chain computation | Trust assumptions | Complex logic |  

---

### **Next Steps Suggestion**  
**Advanced Topic:** **EIP-1559 Gas Fee Market Analysis** – Understand how base fees and priority tips affect transaction costs in post-London Upgrade networks. Study miner extractable value (MEV) implications and strategies for efficient fee bidding.