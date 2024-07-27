
## Exchanging assets and messages 
The objective of the bridge component is to enable the exchange of both assets and messages (See Figure 2). On the one hand, the term asset refers to any digital rep resentation of value within the Ethereum blockchain. Ethereum supports two types of assets: Ether (ETH), which is the native cryptocurrency of the network, and Tokens, including ERC-20 and ERC-721 tokens. On the other hand, a message refers to com munication or interaction between two smart contracts, involving both data and a value (in ETH) transferred from the origin contract (the one initiating the message) to the specified destination contract. Within the zkEVM context, these messages entail theexe cution of a function **onMessageReceived** of some existing contract. This is what we call the messaging mechanism of the bridge



![alt text](image.png)

## The Bridge

The bridge is an infrastructure component that allows migration of assets and commu-nication between diﬀerent layers. From the point of view of the user, they should be able to transfer an asset from one network to another without changing its value or its functionality, as well as being able to send data payloads between networks (cross-chain messaging).
For sake of simplicity, we will start the bridge description by deﬁning exchanges be-tween L1 and an L2 but our intention is to be general, that is, to enable exchanges between multiple layers LX and LY. This is why we call this subsystem the LXLY bridge. For the explanations, we will use three layers denoted as LX, LY and LZ as represented in Figure 1.