**SIMD 191: Relax Transaction Constraints for Loading Failures**  
- **Proposer**: Andrew Fitzgerald (Anza)  
- **Purpose**:  
  - Allows transactions failing during processing to be **included in blocks** (with fees collected by validators).  
  - Prevents wasted validator compute on invalid transactions.  
- **Implementation**: Splits SIMD 83 into smaller, manageable updates.  

---

**Program Deployment Improvements**  
- **CLI Updates (Agave 2.2)**:  
  - New `--preflight-checks` flag to enable **stake-weighted transactions** (improves deployment success rates).  
  - **Workaround for non-staked nodes**:  
    - Manually set `--compute-unit` limits above baseline.  
    - Use `--max-signature-attempts 1000` to retry deployments.  
  - **Public RPCs**: ~10-minute deployment time for Anchor programs (faster with paid nodes via `--use-rpc`).  
- **Buffer Management**: Use `solana program show buffers` to recover stuck SOL from failed deployments.  

---

**Feature Gate: CPI Caller Restrictions Lifted**  
- **Commit**: Implements part of SIMD 163.  
- **Impact**:  
  - Removes redundant CPI deserialization checks.  
  - Reduces computational overhead for **cross-program interactions**.  

---

**Performance Optimizations**  
- **Program Cache Expansion**:  
  - **Commit**: Doubles RAM cache size for validators (reduces eviction frequency).  
  - **Benefit**: Faster reloading of frequently used programs.  
- **Executable Flag Removal**:  
  - **Feature Gate**: Replaces `is_executable` checks with **owner-based validation** (programs owned by loader are executable).  
  - **Impact**: Frees storage space by eliminating redundant boolean flags in accounts.  

---

**Web3.js v2 Release**  
- **Status**: Officially launching as **version 2** (tagged `@2.x`).  
- **Key Changes**:  
  - Smaller bundle size, improved performance.  
  - **Migration Guide**: Compare v1/v2 snippets in the [Solana Cookbook](https://solanacookbook.com).  
- **Testing**: Use `@2` tag to experiment and report issues.  

---

**Veos: Lightweight Solana Client**  
- **Purpose**: Streamline data indexing with **50% infrastructure cost reduction**.  
- **Features**:  
  - Enables custom indexers without full-node/RPC dependencies.  
  - Open-source framework for efficient on-chain data processing.  

---

**Stack Exchange Contributors (Week Highlights)**  
- **Top Contributors**:  
  - **Jimmy**: 160 points.  
  - **Aditya**: New entrant in top 10.  
  - **Hana (Anza Core Contributor)**, Nick Frosty, Jacob Cree.  
- **Community Impact**: Rapid response to influx of new developer questions.  

---

**Key Links**  
- **SIMD 191 Discussion**: [GitHub Issue](https://github.com/solana-foundation/simd)  
- **Web3.js v2 Cookbook**: [Code Comparison](https://solanacookbook.com/web3js-v2)  
- **Veos Announcement**: [Thread Link](https://twitter.com/veos_team)