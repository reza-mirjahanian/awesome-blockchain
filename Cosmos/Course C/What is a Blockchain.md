### What is a Blockchain?

Blockchain protocols define programs that hold a **state** and describe how to modify the state according to the received inputs. The **inputs** are called transactions.

The **consensus mechanism** ensures that a blockchain has a canonical transaction history. Blockchain transactions must be **deterministic**, meaning there is only one **correct interpretation**. The blockchain state is also deterministic. If you begin with the same genesis state and replicate all changes, you always achieve the same state.

A blockchain architecture can be **split into three layers**:

*   **The network layer**: tasked with discovering nodes and propagating transactions and consensus-related messages between single nodes.
*   **The consensus layer**: runs the consensus protocol between the single nodes of a peer-to-peer (P2P) network.
*   **The application layer**: running a state machine that defines the application's state and updates it with the processing of transactions implementing the network's consensus.

This layered model can be applied to blockchains generally