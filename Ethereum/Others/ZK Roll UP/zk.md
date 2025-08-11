


**Introduction to ZK-VMs and ZKPs**

**What is a ZK-VM?**
To understand ZK-VMs, we first need to understand ZKPs (Zero-Knowledge Proofs):

- **ZKP**: Proof of correct execution of a circuit
  - Circuit: Specific function compiled into gates and wires
  - Uses a proving system (e.g., STARK, SNARK)

**ZK-VM Definition**
- Generalized circuit
- Implements a virtual machine as a circuit
- Proves correct execution of the virtual machine
- Inputs: Bytecode (instead of specific function inputs)

**Tradeoffs: ZK-VMs vs. Simple Circuits**

| Advantages of ZK-VMs | Disadvantages of ZK-VMs |
|----------------------|--------------------------|
| One circuit, many programs | Requires powerful hardware (not suitable for constrained devices) |
| Can prove program failures | Overhead for small programs |
| Extensive optimization possible | |
| Better for developer experience | |

**Additional Benefits of ZK-VMs**
- Simpler to audit (one circuit vs. many)
- Gas metering possible
- Improved developer experience (errors, try-catch blocks, dynamic loops, jump statements)

**ZK-EVM vs. ZK-VM**
- ZK-EVM: Specific type of ZK-VM implementing the EVM instruction set
- Limitations of ZK-EVMs:
  1. Can't add new features/opcodes (for type-1 ZK-EVMs)
  2. High computational requirements
  3. Not ZK-optimized (uses functions like Keccak, RLP encoding)

**Types of Non-EVM ZK-VMs**

1. **StarkWare**
   - STARK-based VM
   - Proves correct execution of Cairo code
   - Optimized for compute and recursive aggregation

2. **Aztec**
   - Two types of execution:
     a. Public execution: ZK-VM
     b. Private execution: Circuits
   - Focus on speed and privacy

3. **Polygon Miden**
   - STARK-based VM
   - Targets client-side proving
   - Optimized for heavy compute and privacy

**Benefits of Non-EVM ZK-VMs**

1. **Unlimited runtime with client-side proving**
   - No gas concept
   - Network can succinctly verify proofs

2. **Native account abstraction**
   - Programs can define authentication and account structure

3. **Privacy**
   - Different state models (e.g., UTXO)
   - More efficient privacy primitives

**Conclusion**
Non-EVM ZK-VMs offer flexibility to combine features for exciting applications beyond what the EVM can support.

