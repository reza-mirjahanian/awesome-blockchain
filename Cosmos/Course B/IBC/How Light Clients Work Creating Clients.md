
# How Light Clients Work: Creating Clients

**Introduction**
- The message server is where messages come in
- The first message is "create client"
  - Allows a relayer to create a client for any chain on the chain where the message is submitted

**Client Creation Process**
1. Unpack the message
2. Create the client
   - Generate a local unique identifier called a client ID
     - Not related to the chain ID
     - IBC has no knowledge of the actual chain on the other end of the client
   - IBC focuses on:
     - Keeping the client in sync
     - Ensuring every update is valid
     - Proving that messages sent over the client have been sent by verifying Merkle proofs

**Discussion on Connecting to Known Client IDs**
- Challenges:
  - Chicken and egg problem: How to get the initial information?
  - Similar to off-chain clients needing an initial root of trust
- No in-protocol way of getting the information built
  - Relayers handle it in a permissionless way
  - No assumptions made in the protocol about what the clients refer to on the other end
- Analogous to how the internet works
  - Base layer doesn't know if an IP address is truly what it claims to be
  - Additional layer (e.g., DNS) provides a root of trust for verification

**Information in the Create Client Message**
- Includes:
  1. Client State
  2. Initial Consensus State (acts as a root of trust)
     - Similar to a header
     - Contains next validators hash, timestamp, and height
- Client State Parameters:
  - Required for all clients:
    - Chain ID
    - Unbonding period
    - Latest height
    - Proof specs
  - User-configurable parameters:
    - Trust level
    - Trusting period
    - Allows choosing between efficiency and security

**Consensus State Details**
- Timestamp: Block timestamp
- Root: App hash (hash of the application state of the other blockchain)
  - Used to prove inclusion or exclusion using Merkle proofs
- Next Validators Hash: Hash of the validator set for the next block of the counterparty blockchain
  - Used for verifying the next header
  - Enables sequential light clients (block-by-block validation)
  - Bisecting light clients (more efficient, skips some headers)

**Validator Set Changes**
- When the validator set changes after creating the client, the validator hash changes accordingly

**Conclusion**
- The relayer has created the client
- Chain A has created a light client for Chain B in its state machine
- Chain B has created a light client for Chain A