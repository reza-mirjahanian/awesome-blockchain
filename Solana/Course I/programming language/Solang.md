

---

#### **Overview of SoLong Compiler**
- **Purpose:**  
  - Targets different blockchain platforms.  
  - Current targets: Substrate, EOS, Sawtooth.  
  - Potential future targets: LLVM EVM back-end as it stabilizes.  

- **Language and Tooling:**  
  - Written in **Rust**.  
  - Utilizes Rust Enums for representing syntax trees and control flow graphs.  
  - Leverages LLVM for optimization and WASM file generation.  

---

#### **Key Features**
- **Grammar and Parser:**  
  - Automatically generates grammar from AST (Abstract Syntax Tree).  
  - Recent additions: `try-catch` functionality (added with ~10 lines of code).  

- **Blockchain-Specific Features:**  
  - Substrate-specific:  
    - Address type defaults to 256 bits but can vary.  
    - Overloaded constructors.  
    - Different ABI (Application Binary Interface) encoding compared to Ethereum.  
    - Debugging support via the `print` function (available only in development chains).  

- **String Handling:**  
  - ABI encoding for strings:  
    - Contains length followed by bytes of the string.  
  - Features include string concatenation.  

---

#### **History of SoLong**
- **March:** Prototype developed with limited Solidity grammar and LLVM integration.  
- **December:** Awarded a grant by Web3 Foundation to complete language support for Substrate.  
  - **Grant Milestones:**  
    - Divided into 10 milestones.  
    - 5 milestones completed; remaining milestones in progress.  

- **Goal:** Achieve feature-complete language support by September.  
  - Focus is on correctness over optimization.  

---

#### **Compiler Design**
- **Stages:**  
  1. **Parsing:**  
     - LALR grammar.  
     - Custom lexer to handle pragma statements.  

  2. **Symbol Resolution:**  
     - Resolves symbols, generates warnings/errors.  

  3. **AST to Control Flow Graph:**  
     - Bulk of language support implemented here.  

  4. **Standard Library:**  
     - Written in C, compiled into LLVM IR via Clang.  
     - Provides heap support (`malloc`, `realloc`, etc.).  
     - Enables features like string comparison and concatenation.  

  5. **LLVM Integration:**  
     - Links standard library into one LLVM codebase.  
     - Uses global dead-code elimination for unused functionality.  

---

#### **Compilation Process**
- **Steps:**  
  - Generate control flow graph geared towards LLVM.  
  - Handle target-specific requirements:  
    - WASM-specific arithmetic functions (e.g., 256-bit arithmetic).  
    - ABI encoder/decoder for Ethereum and Substrate.  

- **Substrate ABI Metadata Files:**  
  - Contains function constructors, events, compiler version, and WASM hash.  
  - Ensures ABI matches the WASM file.  

---

#### **Mentorship Program**
- Hosted by Hyperledger (similar to Google Summer of Code).  
- Current focus: Language server for IDEs.  
  - Features: Syntax highlighting, error/warning detection, identifier information.  

---

#### **Future Ideas**
- **LLVM Advantages:**  
  - Foreign function interface for Solidity to call external C/Rust code.  
  - Easier integration of cryptographic functions written in C.  

- **Desired Features:**  
  - Push/pop operations for memory arrays.  
  - Improved data structures (e.g., hash maps, linked lists, sets, trees).  
  - Type systems similar to TypeScript.  
  - Enhanced string processing for debugging.  

---

#### **Challenges**
- **Trade-offs:**  
  - Substrate-specific features may differ from Solidity due to platform constraints.  
  - Example: Memory-related features might give misleading impressions about cost.  

---

#### **Final Remarks**
- Interest in LLVM EVM back-end stability and performance benchmarks.  
- Open to collaboration and feedback from the Solidity team.