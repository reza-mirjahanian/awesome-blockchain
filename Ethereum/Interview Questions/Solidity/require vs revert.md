# `require` vs `revert` vs `assert` in Solidity

Let me explain these three error-handling mechanisms in Solidity smart contracts:

## Comparison Table

| Feature | `require` | `revert` | `assert` |
|---------|-----------|----------|----------|
| **Purpose** | Validate inputs & conditions | Complex conditional logic | Internal errors & invariants |
| **Gas Refund** | ✅ Yes (unused gas) | ✅ Yes (unused gas) | ❌ No (consumes all gas)* |
| **Error Message** | ✅ Supports custom message | ✅ Supports custom message | ⚠️ Limited (Solidity ≥0.8.0) |
| **When to Use** | Input validation, preconditions | Complex if-else logic | Overflow checks, invariants |
| **Typical Usage** | User errors | User errors | Developer errors |
| **Opcode** | `REVERT` | `REVERT` | `INVALID` (pre-0.8.0) / `REVERT` (0.8.0+) |

*Note: Since Solidity 0.8.0, `assert` also uses `REVERT` and refunds gas, but conceptually it's still for invariants.

## Detailed Examples

### 1. `require` - Input Validation

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract RequireExample {
    mapping(address => uint256) public balances;
    
    function withdraw(uint256 amount) public {
        // Validate user input
        require(amount > 0, "Amount must be positive");
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        balances[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }
    
    function deposit() public payable {
        require(msg.value > 0, "Must send ETH");
        balances[msg.sender] += msg.value;
    }
}
```

### 2. `revert` - Complex Logic

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract RevertExample {
    enum Status { Pending, Active, Completed, Cancelled }
    
    Status public status;
    
    function processTransaction(uint256 amount, bool isUrgent) public {
        // Complex conditional logic
        if (amount == 0) {
            revert("Amount cannot be zero");
        }
        
        if (status == Status.Cancelled) {
            revert("Transaction cancelled");
        } else if (status == Status.Completed) {
            revert("Transaction already completed");
        } else if (!isUrgent && status == Status.Pending) {
            revert("Non-urgent transactions must wait");
        }
        
        // Process transaction...
        status = Status.Active;
    }
    
    // Custom errors (more gas efficient in Solidity 0.8.4+)
    error InvalidAmount(uint256 provided);
    error InvalidStatus(Status current);
    
    function processWithCustomError(uint256 amount) public {
        if (amount == 0) {
            revert InvalidAmount(amount);
        }
        
        if (status != Status.Pending) {
            revert InvalidStatus(status);
        }
    }
}
```

### 3. `assert` - Invariants & Internal Checks

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AssertExample {
    uint256 public totalSupply;
    mapping(address => uint256) public balances;
    
    function transfer(address to, uint256 amount) public {
        // User input validation with require
        require(to != address(0), "Invalid recipient");
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        uint256 previousBalance = balances[msg.sender] + balances[to];
        
        balances[msg.sender] -= amount;
        balances[to] += amount;
        
        // Internal invariant check - should NEVER fail
        assert(balances[msg.sender] + balances[to] == previousBalance);
    }
    
    function mint(address to, uint256 amount) public {
        require(to != address(0), "Invalid address");
        
        uint256 oldSupply = totalSupply;
        
        balances[to] += amount;
        totalSupply += amount;
        
        // Invariant: totalSupply should increase by exactly amount
        assert(totalSupply == oldSupply + amount);
    }
}
```

## When to Use Each

### Use `require` for:
- ✅ Validating user inputs
- ✅ Checking external conditions
- ✅ Validating return values from external calls
- ✅ Access control (e.g., `onlyOwner`)

```solidity
require(msg.sender == owner, "Not authorized");
require(block.timestamp > startTime, "Too early");
require(token.transfer(to, amount), "Transfer failed");
```

### Use `revert` for:
- ✅ Complex if-else conditions
- ✅ Multiple validation paths
- ✅ Custom error types (gas efficient)
- ✅ When you need more control flow

```solidity
if (condition1) {
    revert CustomError1();
} else if (condition2) {
    revert CustomError2();
}
```

### Use `assert` for:
- ✅ Checking invariants (things that should NEVER be false)
- ✅ Verifying internal consistency
- ✅ Detecting bugs in your code
- ✅ Mathematical overflow checks (pre-0.8.0)

```solidity
assert(totalDeposits >= totalWithdrawals);
assert(address(this).balance >= totalLiabilities);
```

## Gas Considerations

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract GasComparison {
    // Most gas efficient (since 0.8.4)
    error InsufficientBalance(uint256 available, uint256 required);
    
    function method1(uint256 amount) public pure {
        // String error - stores string in bytecode
        require(amount > 100, "Amount too small"); // ~24,000 gas
    }
    
    function method2(uint256 amount) public pure {
        // Custom error - only stores error signature
        if (amount <= 100) {
            revert InsufficientBalance(amount, 100); // ~22,000 gas
        }
    }
}
```

## Best Practices Summary

1. **`require`**: Your first choice for validation
2. **`revert`**: When you need complex logic or custom errors
3. **`assert`**: Only for conditions that should never fail (invariants)

**Pro Tip**: In Solidity ≥0.8.4, use custom errors with `revert` for better gas efficiency! 🚀