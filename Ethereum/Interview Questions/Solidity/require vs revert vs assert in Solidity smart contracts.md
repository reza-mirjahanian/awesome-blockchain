**require in Solidity**

`require` is a convenience function for validating conditions, primarily used for input validation, external conditions, or expected errors that can occur due to user input or external factors. It reverts the transaction if the condition is false, refunding remaining gas (in Solidity 0.8+). It can optionally include a string message or custom error for better debugging.

- **Key Behaviors**:
  - Evaluates the condition; if false, reverts with an `Error(string)` (if message provided) or without data.
  - Arguments are always evaluated, even if the condition is true, which can lead to side effects.
  - In Solidity <0.8, it used the REVERT opcode; behavior is consistent in 0.8+ with gas refunds.
  - Overloads: `require(bool)`, `require(bool, string)`, `require(bool, error)` (for custom errors since 0.8.4).

- **When to Use**:
  - For checking user inputs, balances, allowances, or contract states that might fail legitimately.
  - Avoid for internal bugs; use assert instead.

- **Code Snippets**:
  - Basic usage:
    ```solidity
    function deposit(uint amount) public {
        require(amount > 0, "Amount must be positive"); // Reverts with Error(string)
        // Proceed with deposit
    }
    ```
  - Without message (empty revert):
    ```solidity
    function withdraw(uint amount) public {
        require(balance[msg.sender] >= amount); // Reverts without data
        balance[msg.sender] -= amount;
    }
    ```
  - With custom error (more gas-efficient):
    ```solidity
    error InsufficientBalance(uint requested, uint available);
    function withdraw(uint amount) public {
        if (balance[msg.sender] < amount) {
            revert InsufficientBalance(amount, balance[msg.sender]);
        }
        // Or using require: require(balance[msg.sender] >= amount, InsufficientBalance(amount, balance[msg.sender]));
    }
    ```
  - Edge case: Side effects in arguments (tricky, as function is called regardless):
    ```solidity
    function riskyRequire() public {
        uint x = 0;
        require(x > 0, incrementAndReturnError(x)); // incrementAndReturnError() is called even if x > 0 is true
    }
    function incrementAndReturnError(uint y) internal pure returns (string memory) {
        y++; // Side effect occurs unconditionally
        return "Error occurred";
    }
    ```
  - Edge case: Revert in constructor:
    ```solidity
    constructor(uint initialSupply) {
        require(initialSupply > 0, "Initial supply must be positive");
        totalSupply = initialSupply;
    }
    ```

**revert in Solidity**

`revert` explicitly triggers a revert, undoing state changes and refunding remaining gas. It's more flexible than `require` as it doesn't require a condition and is often used with custom errors for efficiency. It can be called anywhere to abort execution.

- **Key Behaviors**:
  - Can revert with `Error(string)`, custom error, or no data.
  - Since Solidity 0.8.4, custom errors via `revert CustomError(args)` are recommended for gas savings (cheaper than string messages).
  - Does not evaluate conditions automatically; use with `if` statements.
  - Generated automatically in cases like external calls failing or non-payable functions receiving Ether.

- **When to Use**:
  - For conditional reverts in complex logic.
  - With custom errors to pass structured data back to callers.
  - In place of `require` for more control.

- **Code Snippets**:
  - Basic revert:
    ```solidity
    function testRevert() public pure {
        revert("Explicit revert"); // Reverts with Error(string)
    }
    ```
  - Conditional revert (equivalent to require):
    ```solidity
    function deposit(uint amount) public {
        if (amount == 0) {
            revert("Amount must be positive");
        }
        // Proceed
    }
    ```
  - With custom error:
    ```solidity
    error InvalidAmount(uint amount);
    function deposit(uint amount) public {
        if (amount == 0) {
            revert InvalidAmount(amount);
        }
    }
    ```
  - Edge case: Revert in fallback function:
    ```solidity
    fallback() external payable {
        revert("No fallback allowed"); // Prevents unintended Ether receipt
    }
    ```
  - Edge case: Revert with no data:
    ```solidity
    function silentRevert() public pure {
        revert(); // Empty revert, no selector or data
    }
    ```
  - Use in try/catch (caller perspective):
    ```solidity
    try externalContract.someFunction() {
        // Success
    } catch Error(string memory reason) {
        // Handle revert with string
    } catch (bytes memory lowLevelData) {
        // Handle other reverts
    }
    ```

**assert in Solidity**

`assert` checks for conditions that should always be true in bug-free code, like invariants or internal states. If false, it triggers a `Panic(uint256)` error, reverting and refunding remaining gas (changed in Solidity 0.8+; previously consumed all gas).

- **Key Behaviors**:
  - Only takes a bool condition; no message or custom error.
  - Used for detecting compiler bugs or impossible states.
  - In Solidity <0.8, used invalid opcode (0xfe), consuming all gas.
  - In 0.8+, uses REVERT opcode like require/revert, but with Panic code to distinguish bugs.
  - Automatic Panics: Arithmetic overflow/underflow (outside unchecked{}), array out-of-bounds, etc.

- **When to Use**:
  - For internal consistency checks (e.g., after calculations).
  - Never for input validation, as it signals a contract bug.

- **Code Snippets**:
  - Basic usage:
    ```solidity
    function add(uint a, uint b) public pure returns (uint) {
        uint c = a + b;
        assert(c >= a); // Checks for overflow (though automatic in 0.8+)
        return c;
    }
    ```
  - In unchecked block (edge case, to avoid auto-panic):
    ```solidity
    function uncheckedAdd(uint a, uint b) public pure returns (uint) {
        unchecked {
            uint c = a + b;
            assert(c >= a); // Manual check for overflow
            return c;
        }
    }
    ```
  - Edge case: Assert after complex logic:
    ```solidity
    function processArray(uint[] memory arr) public {
        // Complex processing
        assert(arr.length > 0); // Invariant: array should never be empty here
    }
    ```
  - Automatic Panic example (no explicit assert):
    ```solidity
    function divide(uint a) public pure returns (uint) {
        return a / 0; // Auto-Panic 0x12 (division by zero)
    }
    ```

**Comparisons Between require, revert, and assert**

| Aspect | require | revert | assert |
|--------|---------|--------|--------|
| **Primary Purpose** | Input/external validation | Explicit abort anywhere | Internal invariants/bugs |
| **Condition Required** | Yes (bool) | No (use with if) | Yes (bool) |
| **Error Type** | Error(string) or custom | Error(string), custom, or none | Panic(uint256) |
| **Message/Custom Error** | Optional (string or error) | Optional (string or error) | None |
| **Gas Behavior (0.8+)** | Refunds remaining | Refunds remaining | Refunds remaining |
| **Gas Behavior (<0.8)** | Refunds remaining | Refunds remaining | Consumes all |
| **Argument Evaluation** | Always (side effects possible) | Only if reached | Always |
| **Auto-Triggered** | Yes (e.g., non-payable Ether) | Yes (e.g., failed calls) | Yes (e.g., overflow) |
| **Static Analysis** | For expected failures | Flexible | For bug detection |
| **Similar Concepts** | If-else with revert (equivalent) | Throw in other langs | Debug asserts in C++ |

**Pros/Cons Table**

| Function | Pros | Cons |
|----------|------|------|
| **require** | Simple syntax for conditions; Built-in message support; Good for readability in modifiers | Arguments always evaluated (side effects); Less flexible without if |
| **revert** | Highly flexible; Gas-efficient with custom errors; No unnecessary evaluations | Requires manual if statements; More verbose for simple checks |
| **assert** | Signals critical bugs; Enables static analysis tools to flag issues; Auto-panics for common errors | No custom data; Should never trigger in production (indicates bug) |

**Tricky Parts and Explanations**

- **Gas Changes Across Versions**: Pre-0.8, assert consumed all gas to penalize bugs heavily. In 0.8+, all three refund gas using REVERT opcode, but assert uses Panic to differentiate from user errors. Tip: Always specify Solidity version; use assert sparingly in gas-sensitive code.
- **Side Effects in require/assert**: Arguments are evaluated unconditionally. Tricky example: `require(condition, expensiveFunction())` calls `expensiveFunction()` even if condition is true, wasting gas. Use if-revert instead: `if (!condition) revert ExpensiveError(expensiveFunction());`.
- **Custom Errors vs Strings**: Strings cost more gas (up to 200 gas + 32 per byte). Custom errors are cheaper, ABI-encoded, and can carry data. Tricky: Custom errors don't include selector in revert data if <4 bytes.
- **Bubbling and Low-Level Calls**: Exceptions bubble up unless try/catch. But call/delegatecall return false on revert, no bubble. Tricky: Check return values manually.
- **Panic Codes**: Use for debugging internals. Full table from official docs:

| Code | Description |
|------|-------------|
| 0x00 | Generic compiler panic |
| 0x01 | assert(false) |
| 0x11 | Arithmetic under/overflow |
| 0x12 | Division/modulo by zero |
| 0x21 | Enum conversion error |
| 0x22 | Incorrect storage byte array |
| 0x31 | Pop on empty array |
| 0x32 | Out-of-bounds array access |
| 0x41 | Memory allocation too large |
| 0x51 | Zero function pointer call |

- **Error Propagation**: Reverts forward error data from sub-calls if available, but low-level calls don't.

**Real-World Usage and Projects**

- **OpenZeppelin Libraries**: Heavily uses `require` in access control modifiers (e.g., `onlyOwner` in Ownable.sol: `require(msg.sender == owner, "Ownable: caller is not the owner");`). Assert for internal math checks in SafeMath (pre-0.8).
- **Uniswap V3**: Employs `require` for slippage checks, position validations (e.g., `require(amount0Desired > 0 || amount1Desired > 0, 'INSUFFICIENT_AMOUNT');`). Revert with custom errors in liquidity math for efficiency.
- **Aave Protocol**: Uses `require` for borrow/repay validations (e.g., reserve active checks). Assert in internal calculations to ensure no overflows.
- **Compound Finance**: Revert in error-prone operations like accrual calculations; custom errors for market-specific failures.
- **Real-World Tip**: In audits (e.g., by ConsenSys or Trail of Bits), misuse of assert for inputs is flagged as a vulnerability, as it hides bugs instead of handling errors.
- **Projects Example**: ERC20 standard implementations (e.g., in OpenZeppelin) use `require` for transfer allowances: `require(balanceOf[from] >= amount, "ERC20: transfer amount exceeds balance");`.

Next Steps: Custom Errors and Error Propagation in Solidity