
## Celestia: The First Modular Blockchain Network

### Overview

**Celestia** is a pioneering modular blockchain network renowned for its:

- Customizability
- Scalability
- Sovereign Roll-Ups (a feature Ethereum cannot offer)

This innovation opens up new possibilities in blockchain technology.


### Understanding Celestia

This video aims to simplify the complex technology behind Celestia through diagrams and animations.

#### Key Concepts

1. **Monolithic vs. Modular Blockchains**:
   - **Monolithic Blockchains**: Validators handle all layers (execution, consensus, data availability, and settlement).
   - **Modular Blockchains**: Layers are separated, allowing developers to choose their technologies.

2. **Celestia's Focus**:
   - Consensus
   - Data Availability

3. **Customizability**:
   - **Execution Environments**: EVM (Ethereum Virtual Machine), SVM (Solana Virtual Machine), Cosmos SDK, etc.
   - **Settlement Layers**: Options include Ethereum, Neutron, Evmos, Dimensions, or a custom roll-up.

#### Core Features

1. **Consensus**:
   - Agreement on transaction ordering.

2. **Data Availability**:
   - Ensuring transaction data is available for verification.

### Data Availability in Celestia

- **Roll-Ups**: Can dump their data onto Celestia and select their own execution and settlement layers.
- **Data Availability Sampling**: Ensures data integrity by randomly sampling transactions from a block.
  - **Erasure Coding**: Protects information by adding redundancy, allowing data recovery even if parts are missing.

### Detailed Breakdown

1. **Monolithic vs. Modular Blockchains**:
   - **Monolithic**: All layers handled by validators.
   - **Modular**: Layers are separated, offering more flexibility.

2. **Celestia's Unique Features**:
   - **Consensus and Data Availability**: Core functions for ordering transactions and ensuring data accessibility.

3. **Data Availability Sampling**:
   - **Light Nodes**: Randomly sample transactions to check data integrity.
   - **Erasure Coding**: Adds redundancy, allowing data recovery even if some data is missing.

### Advantages of Celestia

- **Scalability**: Maintains low audit costs while increasing computational capacity.
- **Security**: Higher security with more light nodes participating in data availability sampling.

### How Data Availability Sampling Works

1. **Erasure Encoding**:
   - Data is encoded horizontally and vertically into rows and columns.
   - Allows for data recovery even if some parts are missing.

2. **Light Nodes**:
   - Check for correct Erasure encoding.
   - Verify the presence of transaction data.

### Conclusion

Celestia introduces a new paradigm in blockchain technology by focusing on modularity, consensus, and data availability. Its innovative approach, including data availability sampling and Erasure coding, ensures scalability and security, making it a significant advancement in the blockchain space.