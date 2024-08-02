The terms **state machine** and **virtual machine** are used interchangeably in this documentation.


## Basics of EVM


 In addition to recording transactions, the EVM can store and execute smart contracts. These smart contracts are low-level codes that can perform a variety of tasks and operations on the network.

The EVM is therefore the computational engine of the Ethereum blockchain responsible for **smart contract deployment** and **execution**.

---
The EVM is categorized as a **quasi–Turing-complete** state machine because it can handle all execution processes except that, due to the gas limit set in every smart contract, it is limited to computations with a finite number of steps.

At any given point in time, the current state of the Ethereum blockchain is defined by a collection of the blockchain data. An **Ethereum state** therefore includes **account balances**, **smart contract code**, **smart contract storage**, and other information relevant to the operation of the network.

Since Ethereum is a distributed digital ledger, its state is maintained by each of the network’s **full-nodes**.


Key features of EVM
--------------------------------------------

In terms of how it operates, the EVM is described as; deterministic, sandboxed, and stack-based.

**Deterministic**: For any given input, it always produces the exact same output. This feature is critical for ensuring dependability and predictability of smart contract execution, as well as enabling reliable verification of execution.

**Sandboxed**: Transactions processed by smart contracts run in an environment that is isolated from the rest of the system, making it impossible for transactions to access or modify data outside this environment. This contributes towards network security by preventing unauthorized access to sensitive data.

**Stack-based:** It employs a _last-in-first-out_ (LIFO) memory data structure for processing operations, with data being pushed onto a stack and popped off as needed