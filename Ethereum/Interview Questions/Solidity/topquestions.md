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