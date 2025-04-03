Does Solana use solidity?
-------------------------

Yes, it is possible to write Solana applications in Solidity but somewhat experimental. The [solang solidity compiler](https://solang.readthedocs.io/en/latest/) was built to support compiling Solidity to BPF.


The Solana client programming language
--------------------------------------

The [Solana client](https://github.com/solana-labs/solana) itself, the program that runs on nodes that powers the blockchain, as opposed to programs (smart contracts), is written in Rust. There is currently an in-progress re-implementation of the Solana client, [firedancer](https://jumpcrypto.com/firedancer/) by Jump Crypto, which is written entirely in C.


Rust is migrating BPF to SBF.
-----------------------------

As of October 2022, Solana began migrating from BPF to SBF (Solana binary format). As of the time of writing, the documentation on this change is quite sparse, but this won't affect most developers. If your build tool is configured to compile to BPF, you will get a deprecation warning, but everything will still run. Just change your build flags.