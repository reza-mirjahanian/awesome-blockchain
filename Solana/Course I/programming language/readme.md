- The primary Solana programming language is **Rust**, but C, C++, and even Python are supported.

- Rust is a compiled language. If you compile Rust on your computer, it will ultimately turn into **LLVM-IR** (low level virtual machine intermediate representation), and LLVM turns it into the bytecode that can run on your machine (x86, arm64, etc.).

- In Solana, the sequence looks like this: 
1) Compile Rust to LLVM-IR,
2) BPF (Berkeley Packet Filter) and store the bytecode on the blockchain. 
3) The validators JIT compile (just in time compile) the BPF to the instruction set compatible with their hardware, usually x86, but arm64 might be another common target.

- Linux has a notion of kernel space and a user space. If you want to do things like open a file or start another process, your executable needs to ask the operating system to do that for it. If you are filtering incoming internet packets, then that is a lot of jumps back and forth from userspace and kernel space. Imagine copying every incoming packet from kernel space to user space. That would create a lot of overhead. This is why BPF was invented. You can run executables inside the kernel space to avoid this jumping.