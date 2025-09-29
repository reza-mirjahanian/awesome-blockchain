Q61: What is 'delegatecall'?
A: Calls a function in another contract's context, preserving msg.sender and storage.

Q62: What is 'call'?
A: Low-level call to another contract, returns success bool and data.

Q63: What is the difference between 'call' and 'delegatecall'?
A: 'call' executes in target's context; 'delegatecall' in caller's.

Q64: What is 'staticcall'?
A: Read-only call, cannot modify state.

Q65: What is fallback function?
A: Special function called when no other matches, or for receiving Ether.

Q66: What is receive function?
A: Special payable function for receiving Ether without data.

Q75: What is keccak256?
A: Hash function, e.g., keccak256(abi.encodePacked(data)).

Q235: What is precompile?  
A: Special contracts at low addresses, e.g., ecrecover at 0x01.


Q226: Tricky: Calldata vs memory cost?
A: Calldata cheaper for large inputs.


Proxy patterns


Q228: Related: What is Viem?  
A: Type-safe alternative to ethers.js.

Q214: What is EIP-4844?  
A: Blob transactions for cheaper data.

Q208: Tricky: Randomness in Solidity?  
A: Not secure; use oracles like Chainlink VRF.

Q209: What is commit-reveal?  
A: Scheme for fair randomness.


Q198: What is ethers.js?  
A: Alternative to Web3.js, more modular.

Q199: What is Hardhat?  
A: Development environment for testing/deploying.

Q200: What is Foundry?  
A: Rust-based testing framework for Solidity.

Q201: What is forge?  
A: Foundry's CLI for building/testing.

Q202: What is a fuzz test?  
A: Random inputs to find bugs.

Q203: What is invariant testing?  
A: Properties that always hold, e.g., totalSupply == sum balances.

Q195: How to audit Solidity code?  
A: Use Slither, MythX, manual review.

Q191: Tricky: msg.value in delegatecall?  
A: 0, since no value forwarded.

Q188: What happens on uint256 overflow in unchecked?  
A: Wraps around modular arithmetic.


Q183: How to mitigate oracle issues?  
A: Use TWAP (Time-Weighted Average Price).


Q185: What is integer overflow edge case?  
A: In <0.8, uint256 max +1 =0; now reverts.

Q179: What is MEV?  
A: Miner/Maximal Extractable Value from tx ordering.

Q180: How to protect from MEV?  
A: Use Flashbots or private relays.

---

Q175: What is account abstraction?  
A: EIP-4337: UserOperations for custom validation.

Q176: How does ERC-4337 work?  
A: EntryPoint contract handles bundles, paymasters.

Q177: What is a paymaster?  
A: Pays gas for users in account abstraction.

Q178: What is a bundler?  
A: Relays UserOps to EntryPoint.

---

Q159: What is Yul?  
A: Intermediate language for Solidity, like assembly.

Q160: How to use inline assembly?  
A: assembly { let x := add(1, 2) }

Q161: What is mload?  
A: Loads 32 bytes from memory.

Q162: What is mstore?  
A: Stores 32 bytes to memory.

Q163: What is sload?  
A: Loads from storage slot.

Q164: What is sstore?  
A: Stores to storage slot.

Q165: What is a storage slot?  
A: 256-bit location in contract storage.

Q166: How are mappings stored?  
A: keccak256(key . slot) as storage key.

Q167: How are dynamic arrays stored?  
A: Length at slot, elements at keccak256(slot).

Q168: What is packing in storage?  
A: Combining small types into one slot to save gas.

Q169: Example of packing?  
A: uint128 a; uint128 b; // in one slot.

Q170: What is dirty storage write?  
A: Changing a slot costs more if it was zero.

Q171: Gas cost of sstore?  
A: 20,000 if new, 5,000 if dirty, refunds possible.

---
Q155: What is ABI?  
A: Application Binary Interface for encoding/decoding calls.

Q156: How to generate ABI?  
A: Compiled output from solc.

Q157: What is solc?  
A: Solidity compiler.

---

Q138: What is Chainlink?  
A: Oracle network for off-chain data.

Q139: How to use Chainlink in Solidity?  
A: Inherit VRFConsumerBase or AggregatorV3Interface.

Q140: What is VRF?  
A: Verifiable Random Function for randomness.

Q141: What is a keeper?  
A: Automated bot for upkeep, e.g., Chainlink Keepers.

Q142: What is Gelato?  
A: Automation network similar to keepers.

Q143: What is The Graph?  
A: Indexing protocol for querying blockchain data.

--

Q134: What is constant product formula?  
A: x * y = k for liquidity pools.

Q135: How to interact with Uniswap in Solidity?  
A: Call router functions like swapExactTokensForTokens.

Q136: What is slippage?  
A: Difference between expected and actual price due to trade size.

Q137: What is impermanent loss?  
A: LP value loss from price divergence.

---

Q124: What is domain separator in EIP-712?  
A: Prevents replay across chains/apps.

Q125: What is a nonce?  
A: Counter to prevent replay attacks.

Q126: What is AccessControl in OpenZeppelin?  
A: Role-based access with grant/revoke roles.

Q127: What is TimelockController?  
A: Delays execution for governance.

Q128: What is Governor contract?  
A: For DAO voting and proposals.

Q129: What is ERC-2612?  
A: Permit for gasless approvals via signatures.

Q130: How does permit work?  
A: approve via signature, using nonces.

Q119: What is Merkle tree?  
A: Hash tree for efficient proofs, used in airdrops.

Q120: How to use Merkle proof in Solidity?  
A: Verify leaf against root using keccak256.

Q121: What is ECDSA?  
A: Elliptic Curve Digital Signature Algorithm for signing.

---

Q113: What is a multicall?  
A: Pattern to batch multiple calls in one transaction.

Q114: What is flash loan?  
A: Borrow and repay in one transaction, e.g., via Aave.

Q115: How to implement flash loan in Solidity?  
A: Provide callback to borrower, ensure repayment.

Q116: What is ERC-1155?  
A: Multi-token standard for fungible and non-fungible.

Q117: What is safeTransferFrom?  
A: In ERC-721/1155, checks if recipient can handle tokens.

---

Q106: How to deploy a contract from Solidity?  
A: new MyContract(args);

Q107: What is CREATE opcode?  
A: EVM opcode for deploying contracts.

Q108: What is CREATE2?  
A: Deterministic deployment with salt for predictable addresses.

Q109: How to use CREATE2 in Solidity?  
A: new MyContract{salt: salt}(args);

Q110: What is address(this)?  
A: Current contract's address.

Q111: What is balance of an address?  
A: address.balance (in wei).

---

Q102: What is tx.origin?  
A: Original sender of the transaction (not reliable for auth).

Q103: Difference between msg.sender and tx.origin?  
A: msg.sender is immediate caller; tx.origin is transaction initiator.

Q104: Why avoid tx.origin?  
A: Vulnerable to phishing-like attacks.
---

Q96: What is a proxy contract?  
A: For upgradability, delegates calls to implementation.

Q97: What is UUPS proxy?  
A: Universal Upgradeable Proxy Standard (ERC-1967).

Q98: What is transparent proxy?  
A: Proxy that handles upgrades transparently.

---

Q88: What is SafeMath?  
A: Library to prevent overflow/underflow (built-in since 0.8.0).

Q89: How does Solidity 0.8+ handle overflows?  
A: Automatically checks and reverts on overflow.

Q90: What is unchecked block?  
A: { unchecked { i++; } } to disable overflow checks for gas savings.

Q91: What is a reentrancy attack?  
A: Attacker calls back into contract before state update.

Q92: How to prevent reentrancy?  
A: Use Checks-Effects-Interactions pattern or ReentrancyGuard.