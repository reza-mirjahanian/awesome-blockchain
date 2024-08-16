### *How do you connect your chain to a non-Tendermint (non-CometBFT) chain?*

 The IBC connection is not limited to Tendermint-based chains. If another, non-Tendermint blockchain uses a fast-finality consensus algorithm, a connection can be established by adapting IBC to work with the non-Tendermint consensus mechanism.

If the other chain is a **probabilistic-finality chain**, a simple adaptation of IBC is not sufficient. A proxy chain called a **peg-zone** helps establish interoperability. Peg-zones are fast-finality blockchains which track chain states to establish finality. The peg-zone chain itself is IBC-compatible and acts as a **bridge** between the rest of the IBC network's chains and the probabilistic-finality chain.


A peg-zone implementation exists for Ethereum and is named the **[Gravity Bridge ](https://github.com/cosmos/gravity-bridge)**.