# **Salana Change Log Breakdown**  

---

## **Hackathon & Ecosystem Updates**  
- **Coliseum Radar Hackathon**:  
  - **1,359 project submissions** (largest crypto hackathon to date).  
  - **30% increase** in projects vs. previous hackathons.  
  - Directory of submissions available for review.  

- **a16z Report Highlights**:  
  - **Solana** saw the **largest increase in developer interest** among blockchains.  
  - Growing founder attraction → increased job opportunities in the ecosystem.  

---

## **SIMD 184: Block Writable Account Data Limits**  
- **Current Limit**: 100MB per account per block (no cluster-wide cap).  
- **Proposed Change**:  
  - **2GB cluster-wide cap** on total account data written per block.  
  - Prepares for **direct mapping** (simplifies writing account data).  
- **Goal**: Prevent excessive data writes with new features.  

---

## **Commits & Code Updates**  
- **Windows Support Fixes**:  
  - Agave tool suite now **functional on Windows** (commit by *Yow*).  
- **Scheduler Improvements**:  
  - Better handling of **address lookup table (DLT) expirations** (commit by *Andrew*).  
- **Database Optimization**:  
  - **2-3x efficiency boost** via batched transaction writes.  
- **Package Metadata Standardization** (John Chenway):  
  - Declare **program IDs in `Cargo.toml`** → simplifies dependency management/testing.  

---

## **Developer Resources**  
- **Pinocchio**:  
  - **Dependency-free Solana program framework**.  
  - Benefits: Faster builds, fewer dependency conflicts.  
  - Trade-offs: Differs from standard `solana-program` (e.g., pubkey handling).  
- **Solana Developer Bootcamp**:  
  - **19 hours of free content** (YouTube).  
  - Covers basics (counters) to advanced topics (stablecoins).  

