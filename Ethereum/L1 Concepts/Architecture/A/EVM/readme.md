EVM is the state transition function of the Ethereum state machine. It determines how Ethereum transitions into a new (world) state based on input (transactions) and current state.


In Ethereum, the world state is essentially a mapping of 20-byte addresses to account states.

![alt text](image.png)

Ethereum has two kinds of accounts:

-   **External account:** An account [controlled by an associated private key](https://epf.wiki/#/wiki/Cryptography/ecdsa) and empty EVM code.
-   **Contract account:** An account controlled by an associated non-empty EVM code. The EVM code as part of such an account is colloquially known as a *smart contract.*


EVM
--------------------------------------------

The virtual machine concept serves as an abstraction. Ethereum Virtual Machine (EVM) is a *specific* software implementation of this abstraction. The anatomy of the EVM is described below:

In computer architecture, a word refers to a fixed-size unit of data that the CPU can process at once. EVM has a word size of **32 bytes**.

![alt text](image-1.png)

*or clarity, the figure above simplifies the Ethereum state. The actual state includes additional elements like Message Frames and Transient Storage.*

[EVM bytecode](https://epf.wiki/#/wiki/EL/evm?id=evm-bytecode)
--------------------------------------------------------------

EVM bytecode is a representation of a program as a sequence of [**bytes** (8 bits).](https://en.wikipedia.org/wiki/Byte) Each byte within the bytecode is either:

-   an instruction known as **opcode**, or
-   input to an opcode known as **operand**.

![alt text](image-2.png)

For brevity, EVM bytecode is commonly expressed in hexadecimal notation:
![alt text](image-3.png)

To further enhance comprehension, opcodes have human-readable mnemonics. This simplified bytecode, called **EVM assembly**, is the lowest human-readable form of EVM code:
![alt text](image-4.png)

Identifying opcodes from operands is straightforward. Currently, only `PUSH*` opcodes have operands (this might change with [EOF](https://eips.ethereum.org/EIPS/eip-7569)). `PUSHX` defines operand length (X bytes after PUSH).

Select Opcodes used in this discussion:

| Opcode | Name | Description |
| --- |  --- |  --- |
| 60 | `PUSH1` | Push 1 byte on the stack |
| --- |  --- |  --- |
| 01 | `ADD` | Add the top 2 values of the stack |
| 02 | `MUL` | Multiply the top 2 values of the stack |
| 39 | `CODECOPY` | Copy code running in current environment to memory |
| 51 | `MLOAD` | Load word from memory |
| 52 | `MSTORE` | Store word to memory |
| 53 | `MSTORE8` | Store byte to memory |
| 59 | `MSIZE` | Get the byte size of the expanded memory |
| 54 | `SLOAD` | Load word from storage |
| 55 | `SSTORE` | Store word to storage |
| 56 | `JUMP` | Alter the program counter |
| 5B | `JUMPDEST` | Mark destination for jumps |
| f3 | `RETURN` | Halt execution returning output data |
| 35 | `CALLDATALOAD` | Copy 32 bytes from calldata to stack |
| 37 | `CALLDATACOPY` | Copy input data from calldata to memory |
| 80--8F | `DUP1--DUP16` | Duplicate Nth stack item to top |
| 90--9F | `SWAP1--SWAP16` | Swap top with N+1th stack item |

Refer [Appendix H of Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf) for a comprehensive list.