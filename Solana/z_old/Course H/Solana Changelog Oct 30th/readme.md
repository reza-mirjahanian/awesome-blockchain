# **Salana Change Log Breakdown**  

## **Commits & Updates**  
- **Remove Outdated Solana Ledger UD Binary**  
  - Focus on enabling **deterministic builds** via **NixOS** (small Linux subsystem).  
  - Current options: **Docker** or **NixOS**.  

- **Logging Functions Moved to Separate Crate**  
  - Cleanup effort by **John Chin**.  
  - New import path: `salana_message_sis_codes`.  

- **Agave Installer Pull Request**  
  - **Back up local key pairs** (e.g., `id.json`) before updating to avoid deletion.  
  - PR aims to prevent accidental removal of `~/.config/salana` directory.  

---

## **Resources & Tools**  

### **Project Old Faithful**  
- Stores **entire Solana ledger history on Filecoin**.  
- Enables running **RPC nodes** directly from Filecoin (slower than Bigtable but decentralized).  
- Supports historical RPC methods (e.g., `getSignaturesForAccount`).  

### **Steel SDK v2**  
- Native SDK by **Hard Head** with new program examples in the Solana repo.  
- **Missing IDL support** – contributors encouraged to add via tools like *Kodama*.  

### **Solana Explorer Updates**  
- **Token 22 Program Enhancements**:  
  - **Verified builds** displayed for on-chain programs (compares repo hash with on-chain hash).  
  - **Multisig support** (e.g., Squads V3 multisig program integration).  
- Open-source verification process via **aasc** and **Ellipsis Labs**:  
  - Build in Docker/NixOS → verify hash matches on-chain.  

---

## **SIMD 186: Transaction Data Size Specification**  
- Proposed by **Hana (Anza)** to standardize transaction size calculations.  
- Solves double-counting of accounts and ensures consistency across validator clients.  

---
