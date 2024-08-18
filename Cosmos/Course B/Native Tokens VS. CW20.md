**Token Types in Cosmos**

* **Native Tokens**: These are the native tokens of the Cosmos network, which are easy to use and interact with.
* **CW20 Tokens**: These are smart contract-based tokens that are similar to ERC20 tokens, but with a more complex architecture.

**Native Tokens**

* **Token Factory Module**: This module allows developers to create their own native tokens with a specific namespace.
* **Denomination**: Native tokens have a denomination name, which is used to identify the token.
* **Unsigned Integers**: Native tokens use unsigned integers to represent the amount of tokens.
* **Micro (u) Token**: The micro token is a unit of the native token that is 10^6 times smaller than the native token.

**Benefits of Native Tokens**

* **Easy to Use**: Native tokens are easy to use and interact with, as they are integrated into the CLI.
* **Native Support**: Native tokens have native support in the CLI, making it easy to send and receive tokens.
* **Airdropping**: Native tokens can be easily airdropped to users, as they are saved natively within the state.
* **Contract Interaction**: Native tokens can be used to interact with contracts, making it easy to perform actions on the blockchain.

**CW20 Tokens**

* **Smart Contract-Based**: CW20 tokens are smart contract-based tokens that are similar to ERC20 tokens.
* **Contract Address**: CW20 tokens have a contract address that is used to interact with the token.
* **Database of Token Ownership**: CW20 tokens have a database of token ownership that is stored within the contract.

**Benefits of CW20 Tokens**

* **Flexibility**: CW20 tokens are flexible and can be used to create complex token economies.
* **Smart Contract-Based**: CW20 tokens are smart contract-based, making it easy to create custom logic and rules for the token.

**Migration from CW20 to Native Token**

* **Migration Contract**: The migration contract is a smart contract that allows users to migrate from a CW20 token to a native token.
* **One-to-One Bidirectional Change**: The migration contract allows for a one-to-one bidirectional change between the CW20 token and the native token.

**Querying CW20 Tokens**

* **Querying Contract Address**: To query a CW20 token, you need to know the contract address of the token.
* **Querying Token Info**: You can query the token info of a CW20 token using the `token_info` query.
* **Querying User Tokens**: You can query the user tokens of a CW20 token using the `balance` query.

**Smart Contract-Based Token**

* **Smart Contract**: A smart contract is a self-executing contract with the terms of the agreement written directly into lines of code.
* **Contract Address**: A smart contract has a contract address that is used to interact with the contract.
* **Database of Token Ownership**: A smart contract has a database of token ownership that is stored within the contract.

**Token Logo and Marketing Info**

* **Logo**: The logo of a CW20 token can be queried using the `logo` query.
* **Marketing Info**: The marketing info of a CW20 token can be queried using the `marketing_info` query.

**Getting All Accounts and Allowances**

* **Get All Accounts**: You can get all accounts that hold a CW20 token using the `accounts` query.
* **Get All Allowances**: You can get all allowances of a CW20 token using the `allowances` query.

--------------------------------------
=========================

Cosmos Token Types
1. Native Tokens
Characteristics

Ease of use for developers and users
Queried through the banks module
Represented as integers with denomination names
Utilize the Token Factory module

Benefits

CLI-friendly: Easy to send tokens via command line
Simplified airdropping: State can be exported from nodes
Efficient queries and transactions
Enhanced contract interactions: Funds can be pinned during execution

Token Denominations

u: micro (10^6)
n: nano (10^9)
a: ato (10^18, used by Ethereum)

2. CW20 Tokens
Characteristics

Smart contract-based, similar to ERC20 tokens
Require CosmWasm (km wasm) to be enabled on the chain
Stored as a contract address in a database

Limitations

More complex to interact with compared to native tokens
Require knowledge of contract address for queries and executions
Harder to index without querying the contract every block

Token Factory Module

Allows developers to create namespace tokens
Prevents name collisions
Examples:

Joel's token
Personal tokens
Web app-specific tokens



Token Amounts and Conversion

Represented as unsigned integers
Conversion needed between developer and user-facing amounts
Example:

1 token (user-facing) = 1,000,000 u-tokens (developer-side)



Migration from CW20 to Native Tokens

Possible using Cosmos Contracts Token Factory
Allows for one-to-one bidirectional change
Documentation available for upload and store keys

CW20 Contract Examples
Queries

Contract Info

Admin
Store code ID
Creator
Label
IBC ports IDs (if applicable)


Token Info

Decimals
Name
Symbol
Total supply


User Tokens

Can query both user and smart contract balances
Balance calculation: divide by 10^6 for human-readable form



Additional Features

Smart contracts can hold CW20 token balances
Multiple query types available (logo, marketing info, accounts, allowances, etc.)


=======================

---------------------




### **Cosmos SDK Token Types Overview**

In the Cosmos ecosystem, there are two main types of tokens:

- **Native Tokens**
- **CW20 Tokens**

---

#### **1. Native Tokens**

- **Description**:
  - Native tokens are integral to the Cosmos SDK and are used extensively for their ease of use by both developers and users.

- **Example**:
  - **Juno Network**:
    - Querying for all native tokens via the bank module.
    - Tokens are stored as unsigned integers with a denomination name.

- **Features**:
  - **Token Factory Module**:
    - Allows developers to create namespace tokens for specific applications.
    - Prevents namespace collisions, enabling unique token creation even with similar names.
  - **No Floating Point Numbers**:
    - All token amounts are represented as integers.
    - Supports microdenominations (e.g., micro, nano, atto) for smaller units of tokens.
    - **Microdenominations**:
      - **uToken**: Represents micro (10^6 power).
      - **nToken**: Represents nano (10^9 power).
      - **aToken**: Represents atto (10^18 power), similar to Ethereum.
  
- **Benefits**:
  - **Ease of Use**:
    - Integrated within the CLI (Command Line Interface) for straightforward token transfers.
    - Simplifies airdrops and state exports, making it easier to manage tokens on the network.
  - **Enhanced Security**:
    - Transactions and interactions with contracts are natively secured within the state.
    - Supports transaction pinning to ensure proper fund transfers or return funds in case of errors.

---

#### **2. CW20 Tokens**

- **Description**:
  - CW20 tokens are similar to ERC20 tokens on Ethereum. They are smart contract-based and require specific contract interactions.

- **Example**:
  - **Raccoon Token**:
    - The token contract is identified by a unique contract address.
    - User token ownership is tracked within this contract.

- **Features**:
  - **Smart Contract Dependency**:
    - Requires the `kmWasm` module enabled for smart contract interactions.
    - Interactions involve executing smart contracts via JSON and require knowledge of the contract address.
  - **Complexity**:
    - Not as straightforward as native tokens for transactions.
    - Requires additional steps for queries and transfers.

- **Drawbacks**:
  - **Limited Indexing**:
    - Not as easily indexed as native tokens, requiring queries for each block and tracking all transactions.
  - **Migration**:
    - There is a provision to migrate CW20 tokens to native tokens using the Token Factory module.
    - The migration process involves a one-to-one bidirectional change to a new native token denomination.

---

### **Token Interaction Examples**

- **Native Tokens**:
  - Simple CLI command for token transfer: `bank send`.
  - Tokens can be sent, received, and managed easily through native Cosmos CLI commands.

- **CW20 Tokens**:
  - Requires smart contract interaction through `wasm execute` commands.
  - The contract address is crucial for executing and querying tokens.

---

### **CW20 Token Querying and Interaction**

- **Querying Token Information**:
  - **Contract Address**: Used for querying token details like decimals, name, symbol, and total supply.
  - **Smart Contract Balances**:
    - Smart contracts can hold CW20 token balances.
    - User balances within smart contracts are represented in microdenominations and need conversion for human-readable formats.

- **User Interaction**:
  - **MintScan UI**: 
    - Allows users to view contracts and token amounts but requires contract address knowledge.
  - **Repository Resources**:
    - CW20 tokens are defined in the `kmWasm CW+` repository, providing specs and guidelines for token creation and management.
