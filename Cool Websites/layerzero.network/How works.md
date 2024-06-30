
![](https://layerzero.network/static/features/how-it-works/diagram.svg)

Send

![](https://layerzero.network/static/features/how-it-works/contract.svg)

Sender Contract

ONCHAIN / SOURCE

A user calls the ‘lzSend’ method inside the Sender OApp Contract, specifying a message, a destination LayerZero Endpoint, the destination OApp address, and other protocol handling options.

![](https://layerzero.network/static/features/how-it-works/endpoint.svg)

LayerZero Endpoint

ONCHAIN / SOURCE

The source Endpoint generates a packet based on the Sender OApp’s message data, assigning each packet a unique, sequentially increasing number (i.e. nonce).

![](https://layerzero.network/static/features/how-it-works/library.svg)

MessageLib Registry

ONCHAIN / SOURCE

The Endpoint encodes the packet using the OApp’s specified MessageLib to emit the message to the selected Security Stack and Executor, completing the send transaction with a PacketSent event.

Verify

![](https://layerzero.network/static/features/how-it-works/dvn.svg)

DVNs

OFFCHAIN

The configured DVNs, each using unique verification methods, independently verify the message. The destination MessageLib enforces that only the DVNs configured by the OApp can submit a verification.

Commit

![](https://layerzero.network/static/features/how-it-works/library.svg)

Message Library

ONCHAIN / DESTINATION

Once all of the DVNs in the OApp’s Security Stack have verified the message, the destination MessageLib marks the message as verifiable.

![](https://layerzero.network/static/features/how-it-works/executor.svg)

Executor

OFFCHAIN

The Executor commits this packet’s verification to the Endpoint, staging the packet for execution in the Destination Endpoint.

Receive

![](https://layerzero.network/static/features/how-it-works/endpoint.svg)

LayerZero Endpoint

ONCHAIN / DESTINATION

The Destination Endpoint enforces that the packet being delivered by the Executor matches the message verified by the DVNs.

![](https://layerzero.network/static/features/how-it-works/executor.svg)

Executor

OFFCHAIN

An Executor calls the ‘lzReceive’ function on the committed message to process the packet using the Receiver OApp's logic.

![](https://layerzero.network/static/features/how-it-works/contract.svg)

Receiver Contract

ONCHAIN / DESTINATION

The message is received by the Receiver OApp Contract on the destination chain.