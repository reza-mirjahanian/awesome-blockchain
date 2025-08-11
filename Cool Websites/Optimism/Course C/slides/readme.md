


# Optimism's Fault Proof Alpha System Overview

## Introduction
- The session discusses the high-level components of the OP Stack's fault proof Alpha
- Core components of the proving mechanism are complete
- The system is not yet production-ready but can successfully dispute claims about L2 state

## Key Components

### 1. Dispute Game Primitive
- A simple state machine initialized with a 32-byte root claim
- Can exist in three states: in progress, Challenger wins, or Defender wins

### 2. Dispute Game Protocol
- Built on top of the dispute game primitive
- Facilitated by the dispute game Factory contract
- Allows registration and creation of various dispute game implementations

### 3. Fault Dispute Game
- First contribution to the protocol
- Allows bisection over an execution trace from a fault proof program
- Modeled as a DAG to prevent attack vectors
- Generic over the VM to support multiple fault proof programs and emulators

### 4. Game Visualization
- Demonstrated using a mock "Alphabet VM" for testing
- Illustrates the bisection process and game moves

### 5. Canon
- On-chain MIPS instruction emulator
- Consists of:
  - Binary for emulating the OP program and generating witness data
  - Smart contracts (MIPS thread context and pre-image Oracle)

### 6. OP Program
- Fault proof program that verifies L2 output from L1 inputs
- Designed to run deterministically

### 7. OP Challenger
- Off-chain challenge agent
- Listens to outputs in the L2 output Oracle
- Initiates dispute games when faults are detected

## System Flow
1. OP Challenger detects fault
2. Canon runs over disputed blocks
3. Dispute game is created
4. Bisection occurs
5. On-chain instruction execution via MIPS thread context
6. Game resolution

