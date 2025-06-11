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


EVM Data Locations
==========================================================================

The EVM has four main places to store data during execution:

-   **Stack**
-   **Memory**
-   **Storage**
-   **Calldata**

Let's explore each of these data stores more in depth.

[Stack](https://epf.wiki/#/wiki/EL/evm?id=stack)
------------------------------------------------

Stack is a simple data structure with two operations: **PUSH** and **POP**. Push adds an item to top of the stack, while pop removes the top-most item. Stack operates on Last-In-First-Out (LIFO) principle - the last element added is the first removed. If you try to pop from an empty stack, a **stack underflow error** occurs.

Since the stack is where most opcodes operate, it is responsible for holding the values used to read from and write to **memory** and **storage**, which we'll detail later.

The primary utility of the stack by the EVM is to store intermediate values in computations and to supply arguments to opcodes.


### Program counter

Recall that the bytecode is a flat array of bytes with each opcode being a 1 byte. The EVM needs a way to track what is the next byte (opcode) to execute in the bytecode array. This is where the EVM **program counter** comes in. It will keep track of the next opcode's offset, which is the location in the byte array of the next instruction to execute on the stack.

In the example above, the values on the left of the assembly code represent the byte offset (starting at 0) of each opcode within the bytecode:

| Bytecode | Assembly | Length of Instruction in bytes | Offset in hex |
| --- |  --- |  --- |  --- |
| 60 06 | PUSH1 06 | 2 | 00 |
| --- |  --- |  --- |  --- |
| 60 07 | PUSH1 07 | 2 | 02 |
| 01 | ADD | 1 | 04 |

Notice how the table above doesn't include offset 01. This is because the operand 06 takes position of offset 01, and the same concept applies for operand 07 taking position of offset 03.

Essentially, the **program counter** ensures the EVM knows the position of each next instruction to execute and when to stop executing as illustrated in the example below.

![alt text](image-5.png)

Memory
--------------------------------------------------

EVM memory is a byte array of 22562256 (or [practically infinite](https://www.talkcrypto.org/blog/2019/04/08/all-you-need-to-know-about-2256/)) bytes. All locations in memory are well-defined initially as zero.

![alt text](image-6.png)


Calldata
------------------------------------------------------

The **calldata** is read-only input data passed to the EVM via message call instructions or from a transaction and is stored as a sequence of bytes that are accessible via specific opcodes.

### [Reading from calldata](https://epf.wiki/#/wiki/EL/evm?id=reading-from-calldata)


Storage
----------------------------------------------------

Storage is designed as a **word-addressed word array**. Unlike memory, storage is associated with an Ethereum account and is **persisted** across transactions as part of the world state. It can be thought of as a key-value **database** associated with the smart contract, which is why it contains the contract's "state" variables. Storage size is fixed at 2^256 slots, 32 bytes each.
The calldata for the current environment can be accessed using either:

-   `CALLDATALOAD` opcode which reads 32 bytes from a desired offset onto the stack, [learn more](https://veridelisi.medium.com/learn-evm-opcodes-v-a59dc7cbf9c9).
-   or, using `CALLDATACOPY` to copy a portion of calldata to memory.


![alt text](image-7.png)

### [EVM](https://epf.wiki/#/wiki/EL/evm?id=evm-1)

The resources below has been categorized into different sections based on different EVM learning stages.

#### [Basics of EVM](https://epf.wiki/#/wiki/EL/evm?id=basics-of-evm)

-   🎥 Whiteboard Crypto, ["EVM: An animated non-technical introduction."](https://youtu.be/sTOcqS4msoU)
-   📝 Vasa, [Getting Deep Into EVM: How Ethereum Works Backstage](https://medium.com/swlh/getting-deep-into-evm-how-ethereum-works-backstage-ab6ad9c0d0bf)
-   📝 Zaryab Afser, [The ABCs of Ethereum Virtual Machine](https://www.decipherclub.com/the-abcs-of-ethereum-virtual-machine/)
-   📝 Preethi, [EVM Tweet Thread](https://twitter.com/iam_preethi/status/1483459717670309895)
-   📝 Decipher Club, [EVM learning resources based on your level of expertise](https://www.decipherclub.com/evm-learning-resources/)

#### [Understanding EVM Architecture & Core Components](https://epf.wiki/#/wiki/EL/evm?id=understanding-evm-architecture-amp-core-components)

-   📝 Gavin Wood, ["Ethereum Yellow Paper."](https://ethereum.github.io/yellowpaper/paper.pdf)
-   📝 Ethereum Book, [Chapter 13, Ethereum Book](https://cypherpunks-core.github.io/ethereumbook/13evm.html?ref=decipherclub.com)
-   📘 Andreas M. Antonopoulos & Gavin Wood, ["Mastering Ethereum."](https://github.com/ethereumbook/ethereumbook)
-   🎥 Jordan McKinney, ["Ethereum Explained: The EVM."](https://www.youtube.com/watch?v=kCswGz9naZg)
-   📝 LeftAsExercise, ["Smart contracts and the Ethereum virtual machine."](https://leftasexercise.com/2021/08/08/q-smart-contracts-and-the-ethereum-virtual-machine/) - [archived](https://web.archive.org/web/20230324200211/https://leftasexercise.com/2021/08/08/q-smart-contracts-and-the-ethereum-virtual-machine/)
-   📝 Femboy Capital, ["A Playdate with the EVM."](https://femboy.capital/evm-pt1) - [archived](https://web.archive.org/web/20221001113802/https://femboy.capital/evm-pt1)
-   🎥 Alex, [EVM - Some Assembly Required](https://www.youtube.com/watch?v=yxgU80jdwL0)

#### [Deep-Dive into EVM](https://epf.wiki/#/wiki/EL/evm?id=deep-dive-into-evm)

-   📝 Takenobu Tani, [EVM illustrated](https://github.com/takenobu-hs/ethereum-evm-illustrated)
-   📝 Shafu, ["EVM from scratch."](https://evm-from-scratch.xyz/)
-   📝 NOXX, ["3 part series: EVM Deep Dives - The Path to Shadowy Super Coder."](https://noxx.substack.com/p/evm-deep-dives-the-path-to-shadowy) - [archived](https://web.archive.org/web/20240106034644/https://noxx.substack.com/p/evm-deep-dives-the-path-to-shadowy)
-   📝 OpenZeppelin, ["6 part series: Deconstructing a Solidity."](https://blog.openzeppelin.com/deconstructing-a-solidity-contract-part-i-introduction-832efd2d7737) - [archived](https://web.archive.org/web/20240121025651/https://blog.openzeppelin.com/deconstructing-a-solidity-contract-part-i-introduction-832efd2d7737)
-   📝 TrustLook, ["Understand EVM bytecode."](https://blog.trustlook.com/understand-evm-bytecode-part-1/) - [archived](https://web.archive.org/web/20230603080857/https://blog.trustlook.com/understand-evm-bytecode-part-1/)
-   📝 Degatchi, ["A Low-Level Guide To Solidity's Storage Management."](https://degatchi.com/articles/low_level_guide_to_soliditys_storage_management) - [archived](https://web.archive.org/web/20231202105650/https://degatchi.com/articles/low_level_guide_to_soliditys_storage_management/)
-   📝 Zaryab Afser, ["Journey of smart contracts from Solidity to Bytecode"](https://www.decipherclub.com/ethereum-virtual-machine-article-series/)
-   🎥 Ethereum Engineering Group, [EVM: From Solidity to byte code, memory and storage](https://www.youtube.com/watch?v=RxL_1AfV7N4&t=2s)
-   📝 Trust Chain, [7 part series about how Solidity uses EVM under the hood.](https://trustchain.medium.com/reversing-and-debugging-evm-smart-contracts-392fdadef32d)
-   [Learn EVM Opcodes](https://veridelisi.medium.com/learn-evm-opcodes-v-a59dc7cbf9c9) - [archived](https://web.archive.org/web/20240806231824/https://veridelisi.medium.com/learn-evm-opcodes-v-a59dc7cbf9c9)
-   [More on EVM Storage](https://medium.com/coinmonks/solidity-storage-how-does-it-work-8354afde3eb) - [archived](https://web.archive.org/web/20230808231549/https://medium.com/coinmonks/solidity-storage-how-does-it-work-8354afde3eb)
-   [Storage, Memory, and Stack Overview](https://ethereum.stackexchange.com/questions/23720/usage-of-memory-storage-and-stack-areas-in-evm) - [archived](https://web.archive.org/web/20240529150647/https://ethereum.stackexchange.com/questions/23720/usage-of-memory-storage-and-stack-areas-in-evm)
-   [Calldata](https://learnevm.com/chapters/fn/calldata) - [archived](https://web.archive.org/web/20250306133755/https://learnevm.com/chapters/fn/calldata)

### [Tools & EVM Puzzles](https://epf.wiki/#/wiki/EL/evm?id=tools-amp-evm-puzzles)

-   🧮 smlXL, ["evm.codes: Opcode reference and interactive playground."](https://www.evm.codes/)
-   🧮 smlXL, ["evm.storage: Interactive storage explorer."](https://www.evm.storage/)
-   🧮 Ethervm, [Low level reference for EVM opcodes](https://ethervm.io/)
-   🎥 Austin Griffith, ["ETH.BUILD."](https://www.youtube.com/watch?v=30pa790tIIA&list=PLJz1HruEnenCXH7KW7wBCEBnBLOVkiqIi)
-   💻 Franco Victorio, ["EVM puzzles."](https://github.com/fvictorio/evm-puzzles)
-   💻 Dalton Sweeney, ["More EVM puzzles."](https://github.com/daltyboy11/more-evm-puzzles)
-   💻 Zaryab Afser, ["Decipher EVM puzzles."](https://www.decipherclub.com/decipher-evm-puzzles-game/)

[Implementations](https://epf.wiki/#/wiki/EL/evm?id=implementations)
--------------------------------------------------------------------

-   💻 Solidity: Brock Elmore, ["solvm: EVM implemented in solidity."](https://github.com/brockelmore/solvm)
-   💻 Go: [Geth](https://github.com/ethereum/go-ethereum)
-   💻 C++: [EVMONE](https://github.com/ethereum/evmone)
-   💻 Python: [py-evm](https://github.com/ethereum/py-evm)
-   💻 Rust: [revm](https://github.com/bluealloy/revm)
-   💻 Js/CSS: Riley, ["The Ethereum Virtual Machine."](https://github.com/jtriley-eth/the-ethereum-virtual-machine)

### [EVM based programming languages](https://epf.wiki/#/wiki/EL/evm?id=evm-based-programming-languages)

-   🗄 [Solidity](https://soliditylang.org/)
-   🗄 [Huff](https://github.com/huff-language/)
-   🗄 [Vyper](https://docs.vyperlang.org/en/stable/)
-   🗄 [Fe](https://fe-lang.org/)