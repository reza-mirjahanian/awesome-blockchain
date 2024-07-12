**Ethereum equivalence**  means making no changes to the Ethereum architecture.

**EVM equivalence**  means changing some parts of the ethereum stack which are too complex to prove.

### Type 1 (fully Ethereum-equivalent)

Type 1 zkEVMs aim to be fully equivalent to Ethereum, allowing for perfect compatibility. However, because Ethereum was not designed to be ZK-friendly, the ZK proof process requires significant computation, resulting in larger prover time.

### Type 2 (fully EVM-equivalent)

Type 2 zkEVMs are EVM Equivalent, they look exactly like Ethereum from “within” but with some minor changes to make development easier and to make proof generation faster. While these modifications may affect some Ethereum applications, most should still be compatible with a Type 2 zkEVM rollup. However, the inefficiencies and ZK-unfriendliness of the EVM still slow down the proving process.

### Type 2.5 (EVM-equivalent, except for gas costs)

Type 2.5 zkEVMs increase gas costs of difficult-to-prove EVM operations to improve worst-case prover times. This may cause some developer tooling compatibility issues but is less risky than deeper EVM changes. Another alternative to manage resource constraints is to simply set hard limits on the number of times each operation can be called.This is easier to implement in circuits, but plays much less nicely with EVM security assumptions.

### Type 3 (almost EVM-equivalent)

Type 3 zkEVMs are almost equivalent to the EVM, with a few key differences. They are easier to build and have faster prover times, as it removes certain difficult-to-implement features, but this can lead to compatibility issues with some applications.

### Type 4 (high-level-language equivalent)

Type 4 systems use smart contract source code written in high-level languages (Solidity and Vyper) and compile it to a language specifically designed to be ZK-SNARK-friendly. To avoid overhead, some parts of EVM execution steps can be skipped from ZK proofing by starting directly from higher-level code. However, some applications may not be compatible with Type 4 zkEVMs.