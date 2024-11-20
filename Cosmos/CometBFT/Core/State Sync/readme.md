State Sync
==========

With block sync a node is downloading all of the data of an application from genesis and verifying it. With state sync your node will download data related to the head or near the head of the chain and verify the data. This leads to drastically shorter times for joining a network.

Using State Sync
----------------

State sync will continuously work in the background to supply nodes with chunked data when bootstrapping.

> NOTE: Before trying to use state sync, see if the application you are operating a node for supports it.

Under the state sync section in `config.toml` you will find multiple settings that need to be configured in order for your node to use state sync.

Lets breakdown the settings:

-   `enable`: Enable is to inform the node that you will be using state sync to bootstrap your node.
-   `rpc_servers`: RPC servers are needed because state sync utilizes the light client for verification.
    -   2 servers are required, more is always helpful.
-   `temp_dir`: Temporary directory is store the chunks in the machines local storage, If nothing is set it will create a directory in `/tmp`