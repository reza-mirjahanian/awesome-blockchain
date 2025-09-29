

### **Solidity Senior Engineer Interview: The Gauntlet**

#### **Category 1: Solidity & EVM Fundamentals (The Bedrock)**
*These questions are table stakes. A senior should answer these fluently and without hesitation.*

1.  **What is the `view` and `pure` function modifier?**
    *   **Answer:** `view` promises no state modification. `pure` promises no state modification or reading. Both allow static calls and don't require gas if called externally.

2.  **Explain the `memory`, `calldata`, and `storage` keywords.**
    *   **Answer:** `memory` is temporary, volatile. `calldata` is non-modifiable, temporary function arguments. `storage` is persistent, on-chain state.

3.  **What is the difference between `block.timestamp` and `block.number`?**
    *   **Answer:** `block.timestamp` is the Unix epoch time of the block. `block.number` is the sequential block height.

4.  **What is the `msg.sender` and `msg.value`?**
    *   **Answer:** `msg.sender` is the immediate caller of the function. `msg.value` is the amount of native Ether (in wei) sent with the call.

5.  **What is a function selector?**
    *   **Answer:** The first 4 bytes of the keccak hash of the function signature, used to identify which function to call.

6.  **What is an Event and why is it important?**
    *   **Answer:** Logged data on-chain, stored cheaply and is queryable off-chain. Crucial for dApp frontends and tracking contract state.

7.  **What is the difference between `call`, `delegatecall`, and `staticcall`?**
    *   **Answer:** `call` executes code in another contract in its own context. `delegatecall` executes code in another contract in the context of the caller. `staticcall` is like `call` but reverts if state is modified.

8.  **What is the ABI?**
    *   **Answer:** Application Binary Interface. The standard way to interact with a contract from outside the blockchain, defining how to encode/decode data.

9.  **What is a contract's creation code?**
    *   **Answer:** The bytecode that, when executed, returns the contract's runtime bytecode to be stored on-chain.

10. **What is the difference between `transfer()` and `send()`?**
    *   **Answer:** Both send Ether. `send()` returns a `bool` on failure, `transfer()` reverts. `transfer()` is generally safer.

11. **What is a constructor? Can it be executed after deployment?**
    *   **Answer:** A special function run once during contract creation. No, it cannot be executed after deployment.

12. **What is the `receive()` and `fallback()` function?**
    *   **Answer:** `receive()` is called on plain Ether transfers. `fallback()` is called when no other function matches or if data is provided with a Ether transfer.

13. **What is `selfdestruct`? What are its security implications?**
    *   **Answer:** Deletes a contract, forcing its remaining Ether to a designated address. Can be a centralization risk and break assumptions if called maliciously.

14. **What is function visibility? List the types.**
    *   **Answer:** `public`, `private`, `internal`, `external`.

15. **What is state mutability? List the types.**
    *   **Answer:** `pure`, `view`, `payable`, non-payable.

16. **How do you handle integer overflows/underflows in Solidity <0.8 vs >=0.8?**
    *   **Answer:** In <0.8, use SafeMath library. In >=0.8, it's built-in and checked by default, causing a revert.

17. **What is `gasleft()`?**
    *   **Answer:** Returns the remaining gas for the current execution context.

18. **What is `tx.origin` and why should you avoid using it for authorization?**
    *   **Answer:** `tx.origin` is the original EOA that started the transaction. It can be phished; use `msg.sender` instead.

19. **What is a library in Solidity? How is it linked?**
    *   **Answer:** Deployed, reusable code. Can be `internal` (embedded in contract) or `external` (deployed once and called via `delegatecall`).

20. **What is an immutable variable?**
    *   **Answer:** A variable that can be set only once, in the constructor, and is gas-efficient to read.

21. **What is a constant variable?**
    *   **Answer:** A variable whose value is fixed at compile-time and is replaced in the bytecode.

22. **What is the difference between `bytes` and `string`?**
    *   **Answer:** `bytes` is a dynamic byte array. `string` is a dynamic UTF-8 encoded array. `bytes` is generally more gas-efficient for arbitrary data.

23. **What is `abi.encode`, `abi.encodePacked`, and `abi.encodeWithSignature`?**
    *   **Answer:** Different encoding methods. `encodePacked` is non-padded and can lead to hash collisions, so use with caution.

24. **What is `keccak256` used for?**
    *   **Answer:** The Keccak-256 hash function, used for creating unique identifiers, commitments, and verifying data.

25. **What is `assert()` vs `require()`?**
    *   **Answer:** `require()` is for validating inputs and conditions, refunds unused gas. `assert()` is for checking internal errors, consumes all gas on failure. Post-Solidity 0.8, `assert` uses the `Panic` error.

26. **What is `revert()`?**
    *   **Answer:** Aborts execution and reverts state changes. Can include a custom error message or custom error type.

27. **What is a Custom Error and why is it better than a string?**
    *   **Answer:** Defined with `error MyError();`. It's more gas-efficient because it uses only 4 bytes of selector instead of a full string.

28. **What is the difference between `block.chainid` and `address(this).codehash`?**
    *   **Answer:** `block.chainid` identifies the network (e.g., Mainnet=1). `address(this).codehash` is the hash of the contract's runtime code.

29. **What is `type(X).name` and `type(X).creationCode`?**
    *   **Answer:** Returns the name of contract `X` and its creation bytecode, respectively.

30. **What is `unchecked` and when would you use it?**
    *   **Answer:** A block that disables automatic overflow checks for gas optimization, to be used only where arithmetic is guaranteed to be safe.

---

#### **Category 2: Advanced Solidity Concepts & Patterns (The Architect)**
*This is where we separate the seniors. They should explain the "why" behind the patterns.*

31. **Explain the Checks-Effects-Interactions pattern. Why is it critical?**
    *   **Answer:** A pattern to prevent reentrancy: 1) Check all conditions, 2) Update your state, 3) Interact with other contracts. It prevents state changes after an external call.

32. **What is reentrancy? Give an example and how to prevent it.**
    *   **Answer:** An attack where a malicious contract calls back into the original function before its first invocation is finished. Prevent with Checks-Effects-Interactions or reentrancy guards.

33. **What is a reentrancy guard? How do you implement it?**
    *   **Answer:** A modifier that uses a boolean lock to prevent a function from being called recursively. OpenZeppelin provides a standard implementation.

34. **Explain the Proxy Pattern and how `delegatecall` enables it.**
    *   **Answer:** Uses a Proxy contract (holding state) that `delegatecall`s to a Logic contract (holding code). This allows for upgrading the logic while preserving state and address.

35. **What is the "Storage Collision" problem in upgradeable contracts?**
    *   **Answer:** If the storage layout of the Logic contract changes incompatibly with the Proxy, the Proxy will read/write to the wrong storage slots, corrupting data.

36. **How do you safely change storage layout in an upgradeable contract?**
    *   **Answer:** Only append new variables. Never remove or change the order of existing state variables. Use gaps (e.g., `__gap`) in base contracts for future-proofing.

37. **What are the different types of Proxies? (Transparent vs UUPS)**
    *   **Answer:** **Transparent:** Upgrade logic is in the Proxy admin. **UUPS:** Upgrade logic is in the Implementation contract itself. UUPS is more gas-efficient but riskier if the implementation lacks upgrade function.

38. **What is a Beacon Proxy?**
    *   **Answer:** Many Proxies point to a single "Beacon" contract that holds the implementation address. Updating the Beacon updates all proxies at once.

39. **Explain the Diamond Pattern (EIP-2535).**
    *   **Answer:** A multi-facet proxy that allows adding/replacing/removing any number of functions from a single contract address, solving code size limits.

40. **What is an Initializer function and why is it used?**
    *   **Answer:** Replaces the constructor in upgradeable contracts, called manually after deployment to set up initial state.

41. **What is the "Constructor" problem in Proxies?**
    *   **Answer:** The constructor of the Logic contract only runs once during its deployment, not when the Proxy uses it via `delegatecall`. Hence, the Proxy's state is not initialized by it.

42. **Explain the Pull over Push pattern for payments.**
    *   **Answer:** Instead of "pushing" Ether to users (which can fail due to gas limits or reverts), let users "pull" their owed Ether themselves. This shifts the gas burden and fault tolerance to the user.

43. **What is a Merkle Tree and how is it used in Solidity (e.g., for airdrops)?**
    *   **Answer:** A cryptographic data structure for efficient verification of large sets. For airdrops, you store a Merkle root on-chain, and users provide a Merkle proof to claim their tokens, saving massive gas.

44. **How would you implement a commit-reveal scheme?**
    *   **Answer:** Users submit a hash of their data (commit phase). Later, they submit the actual data, which is hashed and checked against the commit (reveal phase). Prevents front-running for certain actions.

45. **What is EIP-712? What problem does it solve?**
    *   **Answer:** A standard for typed structured data signing. It allows users to sign off-chain messages (like meta-transactions) that are human-readable in their wallet, improving security and UX.

46. **What are meta-transactions and ERC-2771?**
    *   **Answer:** Meta-transactions allow a user to sign a transaction that is relayed by a third party (relayer) who pays the gas. ERC-2771 is a standard for securely handling them.

47. **Explain the concept of "gasless" transactions.**
    *   **Answer:** Achieved via meta-transactions or ERC-4337 (Account Abstraction), where the user either doesn't pay gas in the native token or a sponsor pays for it.

48. **What is ERC-4337 (Account Abstraction)?**
    *   **Answer:** Allows smart contracts to be the primary wallet (account) for users, enabling social recovery, gas sponsorship, batched operations, and custom security logic.

49. **What is a Flash Loan? How would you implement a simple one?**
    *   **Answer:** A loan that must be borrowed and repaid in the same transaction. The contract checks the balance at the start and end, requiring the final balance to be >= starting balance + fee.

50. **What is a Sandwich Attack and how can it be mitigated?**
    *   **Answer:** A MEV attack where a bot front-runs a user's large trade (buying before them) and back-runs it (selling after them). Mitigation: use DEXs with private mempools, lower trade sizes, or use slippage protection.

51. **Explain the "DAO" and "MasterChef" staking contract patterns.**
    *   **Answer:** **DAO:** Voting power based on token balance. **MasterChef:** Staking LP tokens to earn a reward token, with emission rates and multipliers.

52. **How do you safely generate random numbers on-chain?**
    *   **Answer:** You can't get true randomness. Use a commit-reveal from oracles (like Chainlink VRF), or use future block hashes, understanding the miner's influence.

53. **What is a Time Lock? Why is it a critical security feature for governance?**
    *   **Answer:** A delay between a proposal's execution and when it is queued. Gives users time to exit if they disagree with a malicious governance decision.

54. **What is a Multi-signature wallet?**
    *   **Answer:** A wallet that requires M-of-N approved signatures to execute a transaction.

55. **Explain the difference between an EOA and a Contract Account.**
    *   **Answer:** **EOA:** Externally Owned Account, controlled by a private key, can initiate transactions. **CA:** Controlled by code, can only act when triggered by a transaction from an EOA.

56. **What is CREATE2? What is its "magic"?**
    *   **Answer:** An opcode that allows pre-computing a contract's address *before* it is deployed, based on the sender's address, a salt, and the creation code.

57. **How can you use CREATE2 for counterfactual deployments?**
    *   **Answer:** You can build systems where users can interact with a contract address that doesn't exist yet, and it only gets deployed on-chain when absolutely necessary, saving gas.

58. **What is a "Singleton" factory?**
    *   **Answer:** A factory that uses CREATE2 to ensure only one instance of a contract exists per user/salt, useful for creating deterministic user-specific wallets.

59. **What is ERC-1167?**
    *   **Answer:** The standard for Minimal Proxy contracts. They are cheap clones that delegate all calls to a fixed implementation contract.

60. **How would you design a gas-efficient NFT mint?**
    *   **Answer:** Use ERC-721A, pack multiple NFTs in one transaction, use Merkle trees for allowlists (instead of storing a list on-chain), and optimize metadata.

61. **What is ERC-721A and how does it save gas?**
    *   **Answer:** It allows minting multiple NFTs for the cost of minting one, by only updating the owner balance once and storing consecutive token IDs efficiently.

62. **Explain the concept of "ERC-20 permit".**
    *   **Answer:** EIP-2612 allows a user to approve a spender via a signed message (off-chain) instead of an on-chain transaction, saving gas and improving UX.

63. **What is a "Governor" contract?**
    *   **Answer:** A standard (e.g., OpenZeppelin) for implementing on-chain governance with voting, quorum, and timelocks.

64. **What is a "Vesting" contract?**
    *   **Answer:** A contract that locks tokens and releases them linearly over a set period (cliff and duration).

65. **How do you handle decimal math in Solidity?**
    *   **Answer:** Use fixed-point arithmetic libraries (like PRBMath or ABDK) or scale all numbers by a factor (e.g., 1e18) to use integers.

66. **What is the difference between `balance` and `totalSupply` in an ERC-20?**
    *   **Answer:** `balance` is the amount a specific address holds. `totalSupply` is the sum of all balances.

67. **What is an "Inflation Attack" on a staking contract?**
    *   **Answer:** An attacker donates a large amount of rewards to a pool just before a user deposits, diluting the user's share and potentially locking their funds.

68. **How can you prevent Inflation Attacks?**
    *   **Answer:** Use a virtual shares/supply system (like ERC-4626) or sync the reward distribution before major deposits.

69. **What is ERC-4626?**
    *   **Answer:** The Tokenized Vault Standard, which standardizes yield-bearing vaults, making them composable and secure.

70. **What is a "Slashing" condition?**
    *   **Answer:** A penalty mechanism in Proof-of-Stake systems where a validator's staked funds can be destroyed for malicious behavior.

---

#### **Category 3: Security, Vulnerabilities & Gas Optimization (The Hacker)**
*A senior must have a security-first mindset. They should be able to dissect and prevent known attacks.*

71. **What are the most critical categories in the SWC Registry?**
    *   **Answer:** Reentrancy, Access Control, Arithmetic Issues, Unchecked Return Values, Denial of Service, Bad Randomness, Front-running, etc.

72. **Explain an "Access Control" vulnerability.**
    *   **Answer:** When a sensitive function lacks a proper permission check (e.g., `onlyOwner`), allowing any user to call it.

73. **What is an "Unchecked Call Return Value" vulnerability?**
    *   **Answer:** When a low-level `call` is made and its success is not checked, leading to silent failures (e.g., `send()` instead of `transfer()`).

74. **What is a "Denial of Service" (DoS) by block gas limit?**
    *   **Answer:** When an operation (e.g., looping over an unbounded array) consumes more gas than the block limit, making it impossible to execute.

75. **How can you prevent DoS by block gas limit?**
    *   **Answer:** Use pagination, allow users to handle their own claims (pull over push), and avoid unbounded loops.

76. **What is "Front-running" and how is it possible?**
    *   **Answer:** Miners/bots see pending transactions in the mempool and can pay a higher gas fee to have their transaction included before the original one.

77. **What is "Timestamp Dependence"?**
    *   **Answer:** Using `block.timestamp` for critical logic, as miners have a slight influence on it (can vary by ~15 seconds).

78. **What is "Transaction-Ordering Dependence" (TOD)?**
    *   **Answer:** The outcome of a transaction depends on the order it's mined relative to others, a broader form of front-running.

79. **What is a "Phishing" attack via `tx.origin`?**
    *   **Answer:** A malicious contract tricks a user into calling it, and the malicious contract then calls a victim contract that uses `tx.origin` for auth. The victim contract sees the user's EOA as `tx.origin` and approves the action.

80. **What is a "Floating Pragma" and why is it dangerous?**
    *   **Answer:** Using `^0.8.0` instead of a fixed version `0.8.21`. It can lead to unexpected compilation/deployment with a newer, potentially buggy compiler.

81. **What is "Shadowing" of state variables?**
    *   **Answer:** When a function parameter or local variable has the same name as a state variable, which can lead to confusion and bugs. The compiler now prevents this.

82. **What is a "Signature Replay Attack"?**
    *   **Answer:** Using the same signed message on different chains or after a contract upgrade. Prevent by including `chainid` and `address(this)` in the signed data.

83. **What is "Delegatecall to Untrusted Callee"?**
    *   **Answer:** Allowing a `delegatecall` to a user-supplied address lets that contract execute any code in the context of the caller, leading to complete compromise.

84. **What is an "Entropy Illusion"?**
    *   **Answer:** Mistaking `blockhash`, `timestamp`, etc., for good sources of randomness when they are manipulable by miners.

85. **What is "Implicit Visibility" and its risk?**
    *   **Answer:** In older Solidity, functions without a visibility specifier defaulted to `public`. This was a major source of vulnerabilities. Newer compilers force explicit visibility.

86. **What is "Inheritance Order" confusion?**
    *   **Answer:** The order of multiple inheritance in Solidity matters due to C3 Linearization. Wrong order can lead to the wrong function being called.

87. **What is a "Frozen Ether" vulnerability?**
    *   **Answer:** When Ether gets stuck in a contract that has no way to withdraw it (no `receive`, `fallback`, or `withdraw` function).

88. **How can you make a contract "receive" Ether?**
    *   **Answer:** By having a `receive() external payable` or a `fallback() external payable` function.

89. **What is a "Gas Griefing" attack?**
    *   **Answer:** When a malicious actor forces the victim's transaction to use more gas than necessary, e.g., by causing a loop to iterate more times.

90. **What are some common gas optimization techniques?**
    *   **Answer:** Use `uint256` (EVM word size), pack structs, use `immutable`/`constant`, use `external` over `public`, use Custom Errors, use `unchecked` where safe, cache state variables in memory.

91. **Why is using `uint256` more gas-efficient than `uint8` for state variables?**
    *   **Answer:** The EVM operates on 256-bit words. Using smaller types requires extra operations to pack/unpack them, costing more gas.

92. **How does "packing" structs save gas?**
    *   **Answer:** Multiple variables that fit into a single 256-bit slot are stored together, reducing the number of expensive SSTORE operations.

93. **When should you use `calldata` instead of `memory`?**
    *   **Answer:** For `external` function parameters (arrays, structs). `calldata` is cheaper as it reads directly from the transaction data.

94. **What is the gas cost difference between a `memory` array and a `storage` array?**
    *   **Answer:** `memory` arrays are expensive to expand (O(n)), while `storage` arrays have high base costs (SSTORE can be 20,000 gas).

95. **How does using an ECDSA signature save gas over on-chain storage?**
    *   **Answer:** A signature is ~65 bytes passed as `calldata`, which is much cheaper than storing a `mapping` of used nonces or permissions on-chain.

96. **What is the gas impact of using a modifier?**
    *   **Answer:** The modifier's code is inlined, so it adds its gas cost to every function that uses it.

97. **Why is early gas refund (by clearing storage) no longer possible post-EIP-3529?**
    *   **Answer:** EIP-3529 reduced the refund for clearing storage, making "gas refunds" a less viable optimization strategy.

98. **What is the difference in gas cost between the first and subsequent SSTORE for a slot?**
    *   **Answer:** First write (from zero to non-zero) costs ~20,000 gas. Subsequent writes cost ~5,000 gas (if non-zero to non-zero) or yield a refund (if non-zero to zero, post-EIP-3529).

99. **How can you reduce gas costs for users during minting?**
    *   **Answer:** Use ERC-721A, off-chain allowlists (Merkle proofs), and deploy on L2s.

100. **What is the gas cost of a failed transaction?**
    *   **Answer:** The user pays for all gas consumed up until the point of revert, with no refund.

---

#### **Category 4: Testing, Debugging & Tooling (The Craftsman)**
*A senior engineer ships robust code. This requires mastery of the development lifecycle.*

101. **What is the difference between Hardhat and Foundry?**
    *   **Answer:** **Hardhat:** JavaScript/TypeScript based, flexible, large plugin ecosystem. **Foundry:** Rust/Solidity based, extremely fast, built-in fuzzing, direct Solidity scripting.

102. **What is a mainnet fork and why is it useful for testing?**
    *   **Answer:** Spins up a local testnet that mirrors the state of mainnet. Allows testing against live protocols (e.g., Uniswap, Aave) without real funds.

103. **How do you debug a failed transaction?**
    *   **Answer:** Use Tenderly, Hardhat Console, Foundry's trace functions, or Etherscan's debugger to step through the execution.

104. **What is `console.log` in Hardhat?**
    *   **Answer:** A built-in method for printing debug information from Solidity to the Hardhat console.

105. **What is `vm.prank` in Foundry?**
    *   **Answer:** A cheatcode that sets `msg.sender` for the next external call, useful for testing access control.

106. **What is a "fuzz" test?**
    *   **Answer:** A test that runs with a large number of random inputs to find edge cases that manual testing might miss. Foundry has excellent built-in fuzzing.

107. **What is an "invariant" test?**
    *   **Answer:** A fuzz test that checks a system's "invariant" (a property that should always hold, e.g., `totalSupply == sum of all balances`) across many random state transitions.

108. **What is Slither?**
    *   **Answer:** A static analysis framework for Solidity that detects vulnerabilities.

109. **What is Mythril?**
    *   **Answer:** A security analysis tool for EVM bytecode that uses symbolic execution.

110. **How do you measure test coverage?**
    *   **Answer:** Hardhat has a coverage plugin. Foundry has built-in coverage reporting with `forge coverage`.

111. **What is a "Scribble" tool?**
    *   **Answer:** A tool that translates high-level specifications into Solidity code comments (annotations) that can be checked by fuzzers.

112. **What is Echidna?**
    *   **Answer:** A fuzzer/property-based tester for Ethereum smart contracts, excellent for invariant testing.

113. **How do you deploy a contract with Hardhat?**
    *   **Answer:** Write a deployment script using the `ethers` library and run it with `npx hardhat run`.

114. **How do you deploy a contract with Foundry?**
    *   **Answer:** Write a Solidity script and run it with `forge script`.

115. **What is `dapptools`?**
    *   **Answer:** A suite of command-line tools for DApp development, similar in philosophy to Foundry (Unix-style).

116. **What is the role of a Verifier like Sourcify?**
    *   **Answer:** It verifies that the deployed bytecode matches the published source code, enabling transparency and allowing tools like Etherscan to display the code.

117. **How do you handle private keys and environment variables securely?**
    *   **Answer:** Use `.env` files (added to `.gitignore`) and never commit them. Use hardware wallets for production keys.

118. **What is a "Gas Snapshot" in Foundry?**
    *   **Answer:** `forge snapshot` generates a file showing the gas cost of each test, helping to track gas regressions.

119. **How do you simulate a mainnet transaction locally?**
    *   **Answer:** Use `eth_call` RPC on a forked network, or use Foundry's `cast call`.

120. **What is "Etherscan Verification"?**
    *   **Answer:** The process of uploading your source code to Etherscan so users can read and interact with it.

---

#### **Category 5: Real-World Scenarios & System Design (The Founder)**
*This is the final exam. It tests the ability to synthesize all knowledge into a coherent, secure, and scalable system.*

121. **Design a decentralized auction (English/Dutch). What are the key considerations?**
    *   **Answer:** Bidding logic, preventing sniping (extend auction), handling refunds for outbid users, pull-over-push for withdrawals, access control.

122. **How would you design a multi-chain bridge for an ERC-20 token?**
    *   **Answer:** Use a lock-and-mint (lock on chain A, mint on chain B) or burn-and-mint model. Security is paramount: use a trusted oracle/validator set or an optimistic challenge period.

123. **Design a vesting contract for team tokens. What features are needed?**
    *   **Answer:** Cliff period, linear vesting, revocable (for founders), beneficiary address, ability to claim periodically.

124. **You need to upgrade a live contract that holds $100M in TVL. What is your step-by-step process?**
    *   **Answer:** 1) Extensive testing on fork. 2) Security audit. 3) Governance proposal. 4) Timelock queue. 5) Communicate with users. 6) Execute upgrade during low activity. 7) Post-upgrade verification.

125. **A user reports a potential bug in your live contract. What do you do?**
    *   **Answer:** 1) Reproduce the issue on a fork. 2) Assess severity and exploitability. 3) If critical, pause the contract (if possible) or prepare an emergency fix. 4) Be transparent with the community.

126. **How would you design a system to prevent Sybil attacks for an airdrop?**
    *   **Answer:** Use off-chain analysis (e.g., Nansen), proof-of-humanity (BrightID), or require a minimum historical transaction volume/gas spent.

127. **Design a DEX. How do you handle price, liquidity, and swaps?**
    *   **Answer:** Constant Product Formula (x*y=k), LP tokens, fee structure, flash loan resistance, and oracle security (TWAP).

128. **Your contract is at the 24KB size limit. How do you reduce it?**
    *   **Answer:** Use a Proxy pattern, move logic to external libraries, use modifiers sparingly, shorten error names, use Solidity optimizer.

129. **Gas costs are too high for your users. What are your optimization strategies?**
    *   **Answer:** Audit gas usage, implement all gas optimizations, consider moving to an L2 (Arbitrum, Optimism, Base), or implement meta-transactions.

130. **How do you ensure the randomness for your NFT mint is fair and cannot be manipulated?**
    *   **Answer:** Use a proven Oracle solution like Chainlink VRF. Avoid using on-chain data alone.

131. **You suspect a front-running bot is targeting your contract. How can you mitigate it?**
    *   **Answer:** Use a commit-reveal scheme, a private mempool (like Flashbots), or increase the required slippage.

132. **Design a governance system for a DAO.**
    *   **Answer:** Governance token, voting contract (e.g., OZ Governor), timelock executor, proposal lifecycle (submit, vote, queue, execute), quorum and voting delay settings.

133. **How would you design a lending/borrowing protocol like Aave?**
    *   **Answer:** Interest rate models, over-collateralization, liquidations, health factor, price oracles, and aTokens for interest accrual.

134. **What is an "Oracle" and what are the security risks?**
    *   **Answer:** A service that provides external data to the blockchain. Risks: data manipulation, oracle downtime, using a single point of failure.

135. **Why would you use Chainlink over a custom oracle?**
    *   **Answer:** Chainlink is decentralized, battle-tested, and has a strong security model, reducing the risk of price manipulation.

136. **How do you handle a situation where a private key for a contract owner is compromised?**
    *   **Answer:** If the contract has a timelock, create an emergency proposal to revoke permissions. If it has an upgrade mechanism, upgrade to a new implementation without the compromised owner. If not, it may be irrecoverable.

137. **What is "Composability" and why is it a double-edged sword?**
    *   **Answer:** The ability for contracts to interact seamlessly. It's powerful for innovation but creates complex dependencies and risk of contagion if a core protocol fails.

138. **Explain "Money Legos".**
    *   **Answer:** The concept of DeFi protocols being composable, like legos, to build complex financial products on top of each other (e.g., deposit DAI to Aave to get aDAI, then use aDAI as collateral elsewhere).

139. **What is "Layer 2" and why is it important?**
    *   **Answer:** A scaling solution that executes transactions off-chain and posts proofs/data back to Layer 1 (Ethereum). It reduces gas costs and increases throughput dramatically.

140. **What are the different types of L2s?**
    *   **Answer:** Rollups (ZK-Rollups, Optimistic Rollups) and Sidechains/Validiums (Polygon PoS, zkEVM).

141. **What is the difference between an Optimistic Rollup and a ZK-Rollup?**
    *   **Answer:** **Optimistic:** Assumes transactions are valid, has a challenge period for fraud proofs. **ZK:** Uses validity proofs (ZK-SNARKs/STARKs) for every batch, offering instant finality.

142. **How does developing for an L2 differ from L1?**
    *   **Answer:** Generally the same (EVM-compatible), but must account for different gas costs, block times, and pre-deploy addresses (e.g., for L1->L2 messaging).

143. **What is "The Graph" and when would you use it?**
    *   **Answer:** A decentralized protocol for indexing and querying blockchain data. Use it when your dApp needs complex, efficient queries of historical event data.

144. **What is IPFS and how is it used with NFTs?**
    *   **Answer:** A distributed file system. NFT metadata and images are often stored on IPFS to ensure permanence and decentralization, as opposed to a centralized server.

145. **What is Arweave?**
    *   **Answer:** A protocol for permanent, decentralized data storage. You pay once to store data forever.

146. **How do you handle a hard fork as a developer?**
    *   **Answer:** Update client software, test contracts on the new fork, ensure compatibility with any EIP changes, and communicate with users.

147. **What is EIP-1559 and how did it change transaction fees?**
    *   **Answer:** Introduced a base fee (which is burned) and a priority fee (tip to miner). Made gas prices more predictable.

148. **What is the "Merge" and how did it impact contract developers?**
    *   **Answer:** The transition from Proof-of-Work to Proof-of-Stake. It had minimal impact on the EVM layer, so most contracts worked unchanged.

149. **What is "Account Abstraction" (ERC-4337) and how does it change user experience?**
    *   **Answer:** Allows smart contracts to be wallets. Enables social recovery, session keys, gas sponsorship, and batched transactions.

150. **If you could change one thing about the EVM or Solidity, what would it be and why?**
    *   **Answer:** (Open-ended) Example: "Native reentrancy protection at the EVM level to eliminate a whole class of bugs."

