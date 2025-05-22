# Solana Changelog Overview

## SIMD 191: Transaction Constraints Update
- **Key Changes:**
  - Relaxed transaction constraints for loading failures
  - Allows failed transactions to be included in blocks
  - Enables validators to collect fees for processed transactions
  - Prevents compute waste and potential DoS attacks

## Program Deployment Improvements
### Current Challenges
- **Issues with Stake Connections:**
  - Problems with Helios and Triton
  - Need for SK pre-flight check
  - Non-stake weighted transactions

### New Features (CLI Version 2.2)
- SK pre-flight check option
- **Deployment Tips:**
  - Monitor current fees using gas tracker
  - Set compute units above minimum
  - Set max sign attempts to ~1000
  - Use `--use-rpc` flag for stake nodes

## Technical Improvements
### CPI Caller Restriction
- Feature gate for lifting restrictions
- Improved efficiency in CPI calls
- Reduced double deserialization

### Program Cache Enhancement
- **Updates:**
  - Doubled program cache size
  - Reduced program eviction frequency
  - 55x performance improvement
  - Better program caching efficiency

## Web3.js V2 Release
- **Features:**
  - Faster and smaller
  - New syntax and improvements
  - Available using @2 tag
  - Documentation updates in Solana Cookbook
  - Side-by-side V1 and V2 code examples

## Veos Client
- Fast, lightweight streaming client
- 50% reduction in infrastructure costs
- Enables personal indexer deployment
- Reduced dependency on full nodes

## Stack Exchange Champions
1. Jimmy (160 points)
2. Adita
3. Abishek
4. Russo
5. Hana (Anza core contributor)
6. John
7. Nick
8. Jacob