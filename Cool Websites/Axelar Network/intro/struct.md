###  bridge the blockchain ecosystems and enable applications to communicate frictionlessly 

 Validators collectively run a byzantine consensus protocol and run the protocols
facilitating cross-chain requests. Anyone can join the network, participate, and use it. The underlying
network is optimized for high safety and liveness requirements unique for cross-chain requests. Axelar
network also includes a protocol suite and APIs. The **core protocols are**:

- **Cross-Chain Gateway Protocol (CGP**). This protocol is analogous to Border Gateway Protocol on
the Internet. This protocol is used to connect multiple autonomous blockchain ecosystems and is
responsible for routing across them. Blockchains do not need to "speak any custom language", their
platform developers do not need to make any custom changes on their chains, and their chains can be
plugged into the global network easily.

- **Cross-Chain Transfer Protocol (CTP)**. This protocol is analogous to application-level protocols File
Transfer, Hypertext Transfer Protocols on the Internet. It is an application-level protocol stack that
sits on top of routing protocols (such as CGP and other routing technologies). Application developers
can connect their dapps on any chain to perform cross-chain requests. Users can use the CTP protocol
to interact with applications on any chain using simple API calls analogous to HTTP GET/POST
requests. Developers can lock, unlock, and transfer assets between any two addresses across any
blockchain platforms, execute cross-chain application triggers (e.g., an dapps on chain A, can update its state if some other application on chain B satisfies some search criteria (interest rate > X)), and
perform general cross-chain requests between apps across chains (a smart contract on chain A can call
to update a state of a smart contract on chain B). This protocol enables the composability of programs
across blockchain ecosystems.

--------------------

Axelar network offers the following **advantages**:

- For blockchain platform builders: Ability to easily plug-in their blockchains to all other blockchain
ecosystems. Only a **threshold account** needs to be set up on the chain to plug into the network.
- For dapps builders: Application builders can host their dapps anywhere, lock, unlock, transfer assets,
and communicate with applications on any other chain via CTP API.
- For users: Users can interact with all applications across the ecosystem directly from their wallets.
