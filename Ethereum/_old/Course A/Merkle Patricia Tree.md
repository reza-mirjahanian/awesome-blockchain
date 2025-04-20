

# Merkle Patricia Tree

## Introduction
- Merkle Patricia Tree is a fairly complex data structure used in Ethereum
- It combines concepts from Patricia Trees (Radix Trees) and Merkle Trees

## Radix Tree (Patricia Tree)
- A tree data structure where each path from root to leaf represents a value
- Nodes have values and the leaf value is an aggregate of the node values along the path
- Example: Traversing paths like "t r i e" to get values like "tree", "trun", "toe", "token"

## Merkle Tree
- A tree of hashes where each node contains a hash value
- Leaf nodes contain hashed values, and non-leaf nodes contain hashes of their children's values
- Changing any leaf value will change the hash values of all nodes up to the root
- **Merkle Proof**: A way to prove that a certain value belongs to a Merkle Tree without needing the entire tree
  - Provide sibling hash values along the path from the leaf to the root
  - Allows verification by recalculating the root hash

## Merkle Patricia Tree in Ethereum
- Combines Radix Tree and Merkle Tree concepts
- Indexed by fixed-size byte values (e.g., machine words) represented as hexadecimal
- Each node is hashed, so changing a leaf value requires updating hashes up to the root
- Can be used as a key-value store (dictionary) where keys are hex strings and values are stored in leaves

### Node Types
1. **Branch Node**: Has more than one child
2. **Extension Node**: Has one value but can represent multiple levels
3. **Leaf Node**: Represents a node at the bottom of the tree

### Compression
- Ethereum uses node types to compress the tree since the set of values is usually much smaller than the possible key space
- Extension nodes can represent multiple levels with a single node

## Real-World Example: Uniswap Airdrop
- Uniswap built a Merkle Tree of all addresses that interacted with their platform
- Created a smart contract "Merkle Distributor" with the Merkle root and a bitmap of claimed tokens
- Users could claim tokens by providing a Merkle Proof and the contract would verify the proof before transferring tokens

