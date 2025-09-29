### Basic Level Questions (1-50)


Q1: What is Solidity?  
A: Solidity is a statically-typed, contract-oriented programming language for developing smart contracts on the Ethereum Virtual Machine (EVM).

Q2: What is the file extension for Solidity files?  
A: .sol

Q3: What is a smart contract in Solidity?  
A: A self-executing contract with the terms directly written into code, deployed on the blockchain.

Q4: What is the current major version of Solidity as of 2025?  
A: 0.8.x (e.g., 0.8.26, but versions evolve; always check pragma).

Q5: How do you specify the Solidity version in a contract?  
A: Using pragma solidity ^0.8.0; (caret for compatible versions).

Q6: What is the purpose of the 'pragma' directive?  
A: It instructs the compiler on the Solidity version to use.

Q7: What is a contract in Solidity?  
A: A blueprint similar to a class, defining state variables, functions, and behavior.

Q8: How do you declare a simple contract?  
A: contract MyContract { }

Q9: What are state variables?  
A: Variables stored permanently on the blockchain, part of the contract's storage.

Q10: What is the default visibility of state variables?  
A: Internal (accessible within the contract and derived contracts).

Q11: What are the basic data types in Solidity?  
A: bool, int, uint, address, bytes, string.

Q12: What is the difference between int and uint?  
A: int is signed (can be negative), uint is unsigned (non-negative).

Q13: What is the size of uint256?  
A: 256 bits, commonly used for balances and large numbers.

Q14: What is an address type?  
A: A 20-byte value representing an Ethereum account or contract.

Q15: How do you declare a constant variable?  
A: uint constant MY_CONST = 42;

Q16: What is an immutable variable?  
A: Similar to constant but set at runtime (e.g., in constructor), e.g., uint immutable MY_IMMUT;

Q17: What is a function in Solidity?  
A: A reusable block of code that can read/write state or perform computations.

Q18: What is the syntax for a basic function?  
A: function myFunction() public { }

Q19: What does 'public' visibility mean for a function?  
A: Accessible from anywhere, including externally.

Q20: What is 'msg.sender'?  
A: The address of the account that called the current function.

Q21: What is 'msg.value'?  
A: The amount of Ether sent with the transaction.

Q22: What is a constructor?  
A: A special function called once **during contract deployment**, e.g., constructor() { }

Q23: What is an event in Solidity?  
A: A way to log data to the blockchain for off-chain listening.

Q24: How do you declare an event?  
A: event MyEvent(uint value);

Q25: How do you emit an event?  
A: emit MyEvent(42);

Q26: What is a modifier?  
A: A reusable code snippet that can wrap functions, e.g., modifier onlyOwner() { _; }

Q27: What does the **underscore** '_' mean in a modifier?  
A: Placeholder for the wrapped function's body.

Q28: What is 'require' used for?  
A: To validate conditions; reverts if false, e.g., require(condition, "Error");

Q29: What is 'revert'?  
A: Explicitly reverts the transaction with a custom error.

Q30: What is 'assert'?  
A: Checks for internal errors; panics if false (consumes all gas).

Q31: What is a mapping in Solidity?  
A: A key-value store, e.g., mapping(address => uint) balances;

Q32: What keys can a mapping use?  
A: Any built-in type except mappings or dynamic arrays.

Q33: How do you access a mapping value?  
A: balances[msg.sender]

Q34: What is an array in Solidity?  
A: A collection of elements, fixed or dynamic size.

Q35: How do you declare a fixed-size array?  
A: uint[5] myArray;

Q36: How do you declare a dynamic array?  
A: uint[] myArray;

Q37: What is 'push' for arrays?  
A: Adds an element to the end of a dynamic array.

Q38: What is 'pop' for arrays?  
A: Removes the last element from a dynamic array.

Q39: What is a struct in Solidity?  
A: A custom data type grouping variables, e.g., struct Person { string name; uint age; }

Q40: How do you create a struct instance?  
A: Person memory p = Person("Alice", 30);

Q41: What is 'memory' storage location?  
A: Temporary storage, not persisted on blockchain.

Q42: What is 'storage' storage location?  
A: Permanent storage on the blockchain.

Q43: What is 'calldata' storage location?  
A: Read-only, for function parameters from external calls.

Q44: What is the default storage for local variables?  
A: Memory.

Q45: What is Ether?  
A: The native cryptocurrency of Ethereum.

Q46: What units are used for Ether in Solidity?  
A: wei (smallest), gwei, ether (e.g., 1 ether = 10^18 wei).

Q47: How do you send Ether from a contract?  
A: payable(address).transfer(amount) or .send(amount).

Q48: What makes a function payable?  
A: The '**payable**' keyword, allowing it to receive Ether.

Q49: What is 'block.timestamp'?  
A: The current block's timestamp in seconds since Unix epoch.

Q50: What is 'block.number'?  
A: The current block's number.

### Intermediate Level Questions (51-150)

These questions build on basics, covering more complex syntax, patterns, and common practices.

Q51: What is inheritance in Solidity?  
A: Allowing a contract to extend another, using 'is', e.g., contract Child is Parent { }

Q52: What is multiple inheritance?  
A: Inheriting from multiple contracts, e.g., contract C is A, B { }

Q53: How does Solidity handle diamond inheritance?  
A: Uses C3 linearization to resolve method order.

Q54: What is an abstract contract?  
A: A contract with unimplemented functions, marked 'abstract contract'.

Q55: What is an interface?  
A: A contract with only function declarations, no implementation or state.

Q56: How do you implement an interface?  
A: contract MyContract is IMyInterface { } and provide bodies.

Q57: What is 'virtual' keyword?  
A: Marks a function that can be overridden in child contracts.

Q58: What is 'override' keyword?  
A: Used in child contracts to override a virtual function.

Q59: What is a library in Solidity?  
A: Reusable code without state, deployed separately or embedded.

Q60: How do you use a library?  
A: using MyLib for uint; then uintVar.myFunction();

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

Q67: How do you declare a fallback?  
A: fallback() external { }

Q68: What is 'selfdestruct'?  
A: Deprecated; use delete or transfer funds and mark as inactive.

Q69: What is gas in Ethereum?  
A: Unit measuring computational effort for transactions.

Q70: What is the gas limit?  
A: Maximum gas a transaction can consume before failing.

Q71: How do you optimize gas?  
A: Use immutables, pack variables, avoid loops.

Q72: What is an enum in Solidity?  
A: Custom type with named values, e.g., enum Status { Pending, Done }

Q73: How do you use an enum?  
A: Status s = Status.Pending;

Q74: What is a bytes32 type?  
A: Fixed 32-byte array, often for hashes.

Q75: What is keccak256?  
A: Hash function, e.g., keccak256(abi.encodePacked(data)).

Q76: What is abi.encode?  
A: Encodes data to ABI format for calls.

Q77: What is abi.decode?  
A: Decodes ABI-encoded data.

Q78: What is 'pure' function modifier?  
A: Does not read or write state, only computes.

Q79: What is 'view' function modifier?  
A: Reads state but does not write.

Q80: Difference between 'pure' and 'view'?  
A: 'pure' cannot read state; 'view' can read but not write.

Q81: What is 'private' visibility?  
A: Accessible only within the current contract.

Q82: What is 'internal' visibility?  
A: Accessible within current and derived contracts.

Q83: What is 'external' visibility?  
A: For functions, callable only from outside.

Q84: What is ERC-20?  
A: Standard interface for fungible tokens.

Q85: Key functions in ERC-20?  
A: totalSupply, balanceOf, transfer, approve, allowance, transferFrom.

Q86: What is ERC-721?  
A: Standard for non-fungible tokens (NFTs).

Q87: What is ownerOf in ERC-721?  
A: Returns owner of a token ID.

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

Q93: What is OpenZeppelin?  
A: Library of secure, audited contracts like Ownable, ERC20.

Q94: What is Ownable contract?  
A: Provides owner address and onlyOwner modifier.

Q95: What is Pausable?  
A: Allows pausing contract functions.

Q96: What is a proxy contract?  
A: For upgradability, delegates calls to implementation.

Q97: What is UUPS proxy?  
A: Universal Upgradeable Proxy Standard (ERC-1967).

Q98: What is transparent proxy?  
A: Proxy that handles upgrades transparently.

Q99: What is EIP-1559?  
A: Transaction pricing with base fee and tip.

Q100: How does EIP-1559 affect Solidity?  
A: Impacts gas estimation, but no direct syntax change.

Q101: What is block.gaslimit?  
A: Gas limit of the current block.

Q102: What is tx.origin?  
A: Original sender of the transaction (not reliable for auth).

Q103: Difference between msg.sender and tx.origin?  
A: msg.sender is immediate caller; tx.origin is transaction initiator.

Q104: Why avoid tx.origin?  
A: Vulnerable to phishing-like attacks.

Q105: What is a factory contract?  
A: Contract that deploys other contracts.

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

Q112: What is type(address).balance?  
A: No, it's address.balance directly.

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

Q118: What is onERC721Received?  
A: Hook for contracts receiving NFTs.

Q119: What is Merkle tree?  
A: Hash tree for efficient proofs, used in airdrops.

Q120: How to use Merkle proof in Solidity?  
A: Verify leaf against root using keccak256.

Q121: What is ECDSA?  
A: Elliptic Curve Digital Signature Algorithm for signing.

Q122: How to recover signer in Solidity?  
A: ecrecover(hash, v, r, s);

Q123: What is EIP-712?  
A: Typed structured data hashing and signing.

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

Q131: What is a wrapped token?  
A: e.g., WETH, ERC-20 wrapper for Ether.

Q132: How to implement WETH?  
A: deposit() payable to mint, withdraw() to burn and send Ether.

Q133: What is Uniswap?  
A: DEX using AMM (Automated Market Maker).

Q134: What is constant product formula?  
A: x * y = k for liquidity pools.

Q135: How to interact with Uniswap in Solidity?  
A: Call router functions like swapExactTokensForTokens.

Q136: What is slippage?  
A: Difference between expected and actual price due to trade size.

Q137: What is impermanent loss?  
A: LP value loss from price divergence.

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

Q144: What is a subgraph?  
A: Schema for indexing events.

Q145: What is IPFS?  
A: InterPlanetary File System for decentralized storage.

Q146: How to store data on IPFS in Solidity?  
A: Off-chain; contract stores CID (Content ID).

Q147: What is ENS?  
A: Ethereum Name Service for human-readable addresses.

Q148: How to resolve ENS in Solidity?  
A: Use ENS resolver interface.

Q149: What is a DAO?  
A: Decentralized Autonomous Organization.

Q150: How to build a simple DAO in Solidity?  
A: Use voting, proposals, and execution.

### Advanced Level Questions (151-250)

These questions delve into edge cases, optimizations, security, real-world integrations, and low-level concepts.

Q151: What is EVM?  
A: Ethereum Virtual Machine, runtime for smart contracts.

Q152: What is bytecode?  
A: Compiled machine code executed by EVM.

Q153: What is runtime bytecode?  
A: Code after deployment, excluding constructor.

Q154: What is init code?  
A: Constructor logic in deployment bytecode.

Q155: What is ABI?  
A: Application Binary Interface for encoding/decoding calls.

Q156: How to generate ABI?  
A: Compiled output from solc.

Q157: What is solc?  
A: Solidity compiler.

Q158: What optimizations does solc --optimize do?  
A: Reduces bytecode size and gas via peephole optimizer.

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

Q172: What is a refund?  
A: Gas returned for clearing storage.

Q173: What is warm/cold access?  
A: EIP-2929: cold sload costs more first time.

Q174: What is EIP-3074?  
A: AUTH and AUTHCALL for sponsored tx (not implemented).

Q175: What is account abstraction?  
A: EIP-4337: UserOperations for custom validation.

Q176: How does ERC-4337 work?  
A: EntryPoint contract handles bundles, paymasters.

Q177: What is a paymaster?  
A: Pays gas for users in account abstraction.

Q178: What is a bundler?  
A: Relays UserOps to EntryPoint.

Q179: What is MEV?  
A: Miner/Maximal Extractable Value from tx ordering.

Q180: How to protect from MEV?  
A: Use Flashbots or private relays.

Q181: What is sandwich attack?  
A: Frontrun and backrun a trade for profit.

Q182: What is oracle manipulation?  
A: Flash loan to skew price feeds.

Q183: How to mitigate oracle issues?  
A: Use TWAP (Time-Weighted Average Price).

Q184: What is governance attack?  
A: 51% token control to drain treasury.

Q185: What is integer overflow edge case?  
A: In <0.8, uint256 max +1 =0; now reverts.

Q186: What is division rounding?  
A: Integer division truncates down, e.g., 5/2=2.

Q187: Tricky: What is 2**256 -1?  
A: Max uint256 value.

Q188: What happens on uint256 overflow in unchecked?  
A: Wraps around modular arithmetic.

Q189: Edge case: Empty bytes calldata?  
A: Triggers receive() if payable, else fallback().

Q190: What if no receive/fallback?  
A: Reverts on Ether send.

Q191: Tricky: msg.value in delegatecall?  
A: 0, since no value forwarded.

Q192: What is reentrancy in fallback?  
A: Possible if fallback calls back.

Q193: Real-world: DAO hack 2016?  
A: Reentrancy drained funds.

Q194: What is Parity multisig bug?  
A: Library selfdestruct killed wallets.

Q195: How to audit Solidity code?  
A: Use Slither, MythX, manual review.

Q196: What is formal verification?  
A: Mathematically prove properties, e.g., with KEVM.

Q197: Related tech: What is Web3.js?  
A: JS library for Ethereum interaction.

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

Q204: Real-world: DeFi protocol?  
A: Compound for lending/borrowing.

Q205: How does Compound work?  
A: cTokens accrue interest over time.

Q206: What is liquidation?  
A: Seize collateral if undercollateralized.

Q207: Edge case: Timestamp manipulation?  
A: Miners can skew by ~15s.

Q208: Tricky: Randomness in Solidity?  
A: Not secure; use oracles like Chainlink VRF.

Q209: What is commit-reveal?  
A: Scheme for fair randomness.

Q210: Related: Layer 2 solutions?  
A: Optimism, Arbitrum for scalability.

Q211: What is rollup?  
A: Batches tx off-chain, posts data on L1.

Q212: What is zk-rollup?  
A: Uses zero-knowledge proofs for validity.

Q213: How does Solidity differ on L2?  
A: Mostly compatible, but gas costs vary.

Q214: What is EIP-4844?  
A: Blob transactions for cheaper data.

Q215: Advanced: What is extcodesize?  
A: Checks if address has code (for contracts).

Q216: Tricky: extcodesize during construction?  
A: 0, since code not yet deployed.

Q217: How to detect contract creation?  
A: Check tx.origin == msg.sender.

Q218: What is a minimal proxy?  
A: Cheap clone via EIP-1167, delegates to implementation.

Q219: How to implement clone factory?  
A: Use assembly to deploy minimal bytecode.

Q220: Real-world: Upgradable NFT?  
A: Use proxy with storage separation.

Q221: Edge case: Storage collision in proxy?  
A: If implementation changes slots.

Q222: How to avoid: Use unstructured storage or eternal storage.

Q223: What is EIP-1967?  
A: Storage slots for proxy admin/implementation.

Q224: Advanced: What is PUSH0 opcode?  
A: EIP-3855, pushes 0 cheaply (Shanghai upgrade).

Q225: What is transient storage?  
A: EIP-1153, per-tx storage.

Q226: Tricky: Calldata vs memory cost?  
A: Calldata cheaper for large inputs.

Q227: Optimize: Use calldata for strings/bytes.  
A: Yes, to save gas.

Q228: Related: What is Viem?  
A: Type-safe alternative to ethers.js.

Q229: What is Alchemy?  
A: Node provider with APIs.

Q230: What is Infura?  
A: Ethereum API gateway.

Q231: Real-world: Bridge contracts?  
A: Cross-chain transfers, e.g., Wormhole.

Q232: Security: Front-running?  
A: Mitigate with commit-reveal or submarines.

Q233: What is a submarine send?  
A: Hide tx until mined.

Q234: Advanced: SIMD in EVM?  
A: Not supported; EVM is stack-based.

Q235: What is precompile?  
A: Special contracts at low addresses, e.g., ecrecover at 0x01.

Q236: Edge case: Call to precompile fails?  
A: If wrong input, returns 0 or reverts.

Q237: Tricky: Gas stipend in call?  
A: Forward all but 1/64 with .call{gas: gas}.

Q238: What is EIP-150?  
A: Adjusted gas forwarding to 63/64.

Q239: Real-world: Aave flash loan fee?  
A: 0.09% premium.

Q240: What is curve finance?  
A: DEX for stablecoins with low slippage.

Q241: How does Curve pool work?  
A: Bonding curve formula for stables.

Q242: Advanced: Verifying Merkle proof in assembly?  
A: Efficient hash computations.

Q243: Edge case: Zero address transfer?  
A: Burns tokens if not handled.

Q244: Tricky: msg.sender in constructor?  
A: Deployer address.

Q245: What is codecopy?  
A: Assembly: Copies runtime code.

Q246: Related: Solana vs Solidity?  
A: Solana uses Rust/BPF, parallel execution.

Q247: What is Move language?  
A: For Sui/Aptos, resource-oriented.

Q248: Real-world: ENS wildcard resolution?  
A: Subdomains via resolvers.

Q249: Security: Denial of service?  
A: Loops over unbounded data.

Q250: Optimize: Short-circuit conditions?  
A: Yes, require(a || b) evaluates left first.