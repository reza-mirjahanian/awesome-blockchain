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