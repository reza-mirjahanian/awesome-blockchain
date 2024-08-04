![
    
](image.png)
The data streamer is another component of the zkEVM infrastructure through which external nodes can access raw block data.

Any trustless node connected to the zkEVM can be up-to-date with the current L2 state by using the data streamer instead of using the JSON RPC.

Any archive node that needs to receive all information about blocks and their transactions with minimal delay can use the data streamer.

The usual procedure when working with the JSON RPC is that, for example;

-   The user's external node makes requests for information to the zkNode,
-   The zkNode fetches this information from the internal database, and
-   The zkNode relays it back to the requesting node.

Developer experience has shown that although the JSON RPC is perfect for small queries, it is not an efficient way to obtain huge amounts of data often required for fetching the entire L2 state.

A data streamer is best suited for the purpose of serving raw block data to external nodes that need to keep an up-to-date L2 state, irrespective of the required amount of data.


With this approach the zkNode does not serve processed data via the JSON-RPC API to downstream nodes, but just "fast streams" L2 data (which includes; batch data, block header data and transactions data) using a protocol with a low overhead.

Instead of using existing streaming protocols, such as MQTT from IoT, the Polygon zkEVM team has developed a new custom-ware protocol, tailor-made for the zkEVM.