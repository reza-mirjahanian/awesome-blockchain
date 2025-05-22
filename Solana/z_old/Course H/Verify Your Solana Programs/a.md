# Verifying Solana Programs: A Step-by-Step Guide

## Overview
- Program verification allows confirmation that on-chain programs match their source code
- Verified programs receive a "Program Source Verified" badge on:
  - Solana Explorer
  - Solscan
  - Solana FM

## Benefits of Verification
- Ensures transparency of program source code
- Improves program discoverability
- Provides access to repository URLs
- Enables users to find associated SDKs and contact developers

## Prerequisites
- **Required Tools:**
  - Docker
  - Cargo
  - Solana Verify CLI (`cargo install solana-verify`)
- **Repository Requirements:**
  - Public repository
  - Cargo.lock file in root directory

## Verification Process

### 1. Build Program
```bash
solana-verify build
```
- Uses Docker container
- Creates deterministic build
- Picks Solana version from cargo.toml

### 2. Deploy Program
```bash
solana program deploy
```
*Note: Multi-sig and Squads V3 deployment recommended for production*

### 3. Verify Program
```bash
solana-verify verify-from-repository <PROGRAM_ID> <REPOSITORY_PATH>
```
- Clones repository
- Builds in Docker container
- Compares hashes
- Creates PDA with build information

### 4. Remote Verification
```bash
solana-verify verify-from-repository --remote <PROGRAM_ID> <REPOSITORY_PATH>
```
- Sends build command to AASC API
- Compares hashes
- Grants verified badge upon success

## Important Notes
- Verification doesn't increase security
- Enables source code inspection
- Helps with program discovery
- Can include links to frontends and SDKs