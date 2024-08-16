
## # Tackling Interchain Communication

- **Focus**: Making the user experience (UX) friendly for transferring and swapping tokens across different chains

### Goals and Integrations
- **Main Goal**: 
  - Enhance UX for transferring and swapping tokens
  - Integrate with various protocols and applications like Kepler, Leap, Stargaze, and IBC fund

### Key Learnings and Challenges
- **Interchain Protocols and Composability**: 
  - Simple in principle but detailed execution is complex
  - Issues often arise when details don’t align, leading to confusion and challenges in unsticking processes

### Specific Issues with IBC (Inter-Blockchain Communication)
1. **Channel Complexity**:
   - Multiple channels between the same chains can result in sending tokens over the wrong channel, rendering them useless
   - Preferred channels are unclear

2. **Limited Ecosystem**:
   - Most bridges are restricted to their ecosystems, which limits user flexibility

### Solutions Implemented
1. **Token Unwinding**:
   - Indexing IBC channel and token data across supported chains
   - Building a graph to map token routes for efficient transfer

2. **Channel Identification**:
   - Algorithm to determine the correct channel and token name on the destination chain

3. **Extending Reach Beyond Cosmos Ecosystem**:
   - Focus on composability with alternative bridges and interchain operations

### Specific Problems and Solutions with Different Bridges
1. **Axelar**:
   - Challenges with native tokens on EVM chains (e.g., wrapped ETH)
   - Confusing fee structures: fees can be on top of or deducted from the sent amount
   - **Solutions**: 
     - Use of composability primitives like contract calls post-transfer
     - Displaying fees consistently in the API

2. **CCTP**:
   - High cost and poor incentivization for relaying, especially on Ethereum
   - **Solutions**: 
     - External incentivization mechanisms

### Interoperability Roundup
- **Problems**:
  - Complex effort to identify correct tokens and channels
  - Relays need better incentives
  - Bridges should enhance composability

- **Tools Used**:
  - **Packa Middleware**: Trigger additional IBC transfers
  - **IBC Hooks**: Call a WASM contract to perform multiple actions in a single transaction

### Challenges with Middleware and Composability
- Middleware can be inconsistently supported across different chains
- Failures in middleware often result in silent errors, confusing users
- **Skip API Solutions**: 
  - Indexing composability solutions
  - Automatically using supported middlewares or prompting users for multiple transactions

### Drawbacks and Future Improvements
- **IBC Hooks**:
  - The main issue is that the relayer pays the gas fees for executions
  - Potential attack vectors and performance degradation

- **Universal Unwind Handshake**:
  - Challenges in always being one hop from the home chain due to increasing chain numbers

- **Extendability**:
  - Considering extensibility and pluggability to handle the increasing number of bridges and their integration

### Verification and User Perspective
- Regular verification with light clients to ensure channel accuracy
- User confusion in choosing the correct channel despite accurate verification



- **Key Takeaways**:
  - Emphasize the importance of composability and user-friendly design in interchain protocols
  - Continuous improvement and adaptation to handle emerging challenges in the blockchain ecosystem