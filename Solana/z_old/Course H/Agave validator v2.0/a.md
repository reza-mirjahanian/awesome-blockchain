**Agave 2.0 Mainnet Launch**  
- **Status**: Live on mainnet with **~95-96% stake adoption** (exceeding supermajority).  
- **Performance**: Faster than 1.18; validators on older versions fell behind, prompting early upgrades.  
- **Deprecated Calls**: Removed deprecated IPC calls (e.g., `getRecentBlockhashes`). Users must update immediately.  

---

**Agave 2.1 Changes**  
- **Deprecated Commands**:  
  - `cargo build-bpf` and `cargo test-bpf` (deprecated for 2 years) **officially removed**.  
  - Use **`cargo build-sbf`** and **`cargo test-sbf`** instead.  
- **New Entry Point**: `no_alloc` entry point reduces **20–30K CU per unique account** with minimal effort.  
- **Tools Version Declaration**:  
  - Specify platform tools version in `Cargo.toml` via package metadata.  
  - Configurable per program/workspace or via CLI args during builds.  

---

**Feature Gates & Protocol Improvements**  
- **Feature Gate Activations**:  
  - **1 feature per epoch** (e.g., increased CPI limits: **Max CPI Account Info = 128** (previously 32)).  
  - Track via [feature gate schedule](https://docs.solana.com/implemented-proposals/feature-gates).  
- **CU Cost Changes**:  
  - **8 CU per 32KB** for loaded accounts/programs.  
  - Optimize transactions using **`setMaxLoadedAccounts`** in compute budget instructions for better block prioritization.  

---

**Weekly Commits**  
- **Windows Support**:  
  - Fixes for native Windows builds (**no WSL required**).  
- **Validator Port Updates**:  
  - Minimum validator port requirement raised to **17** for QUIC-based voting.  
  - Votes now use **QUIC protocol** (improved speed/reliability vs. UDP).  

---

**Solana Program Library (SPL) Migration**  
- **Centralized Repository**:  
  - SPL programs (e.g., token, token-22, memo) moved to [Solana Program GitHub](https://github.com/solana-program).  
- **Web3.js v2 Compatibility**:  
  - Use auto-generated clients (via **Kodama**) from the new repository.  

---

**Web3.js v2 Update**  
- **Release Timeline**:  
  - Version 2 live now; **`latest` tag switches to v2 on December 16, 2024**.  
  - **Action Required**: Pin to `v1.x` or migrate existing projects to v2.  
- **Benefits**: Faster performance, improved tooling, and Kodama-generated IDLs.  

---

**Hackathon Winners**  
- **Hecaton (Largest Solana Hackathon)**:  
  - Winners announced (e.g., **Gaming Supersize**).  
  - Explore projects via [official winners video](https://solana.com/hecaton-winners).  

---

**Verified Builds**  
- **Program Verification**:  
  - Deployed programs can be verified against source code (collaboration by **Ellipsis Labs** and **Otter**).  
  - On-chain display of **GitHub URL, IDL, SDK, docs, and frontend links**.  
  - API available for public verification.  

---

**Stack Exchange Contributions**  
- **Community Support**:  
  - Core engineers and community members actively answer technical questions.  
  - **Encouraged**: Ask/answer questions to improve ecosystem knowledge.