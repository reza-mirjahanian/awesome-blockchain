
The creation of an application-specific blockchain with the Cosmos SDK is largely a process of selecting, configuring, and integrating well-solved modules, also known as "composing modules". This greatly reduces the scope of original development required, as development becomes mostly focused on the truly novel aspects of the application.


The **Inter-Blockchain Communication Protocol**(IBC) is a common framework for exchanging information between blockchains. it enables seamless interaction between blockchains that want to **transfer value (token transfers)** and exchange information. It enables communication between applications that run on separate application-specific blockchains.



The application, consensus, and network layers are contained within the custom blockchain node that forms the foundation of the custom blockchain.

CometBFT passes confirmed transactions to the application layer through the Application Blockchain Interface (ABCI). The application layer must implement ABCI, which is a **socket protocol**. CometBFT is unconcerned with the interpretation of transactions, and the application layer can be unconcerned with propagation, broadcast, confirmation, network formation, and other lower-level concerns that CometBFT attends to (unless it wants to inspect such properties).

Developers are free to create blockchains in **any language that supports sockets since the ABCI** is a socket protocol, provided their application implements ABCI. ABCI defines the boundary between replication concerns and the application, which is a state machine.