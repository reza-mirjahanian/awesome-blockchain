Walkabout example
-----------------

We first create three connections (mempool, consensus and query) to the application (running `kvstore` locally in this case).

```
I[10-04|13:54:27.364] Starting multiAppConn                        module=proxy impl=multiAppConn
I[10-04|13:54:27.366] Starting localClient                         module=abci-client connection=query impl=localClient
I[10-04|13:54:27.366] Starting localClient                         module=abci-client connection=mempool impl=localClient
I[10-04|13:54:27.367] Starting localClient                         module=abci-client connection=consensus impl=localClient

```

Then CometBFT and the application perform a handshake.

```
I[10-04|13:54:27.367] ABCI Handshake                               module=consensus appHeight=90 appHash=E0FBAFBF6FCED8B9786DDFEB1A0D4FA2501BADAD
I[10-04|13:54:27.368] ABCI Replay Blocks                           module=consensus appHeight=90 storeHeight=90 stateHeight=90
I[10-04|13:54:27.368] Completed ABCI Handshake - CometBFT and App are synced module=consensus appHeight=90 appHash=E0FBAFBF6FCED8B9786DDFEB1A0D4FA2501BADAD

```

After that, we start a few more things like the event switch and reactors.

```
I[10-04|13:54:27.374] Starting EventSwitch                         module=types impl=EventSwitch
I[10-04|13:54:27.375] This node is a validator                     module=consensus
I[10-04|13:54:27.379] Starting Node                                module=main impl=Node
I[10-04|13:54:27.381] Local listener                               module=p2p ip=:: port=26656
I[10-04|13:54:30.386] Starting DefaultListener                     module=p2p impl=Listener(@10.0.2.15:26656)
I[10-04|13:54:30.387] Starting P2P Switch                          module=p2p impl="P2P Switch"
I[10-04|13:54:30.387] Starting MempoolReactor                      module=mempool impl=MempoolReactor
I[10-04|13:54:30.387] Starting BlockchainReactor                   module=blockchain impl=BlockchainReactor
I[10-04|13:54:30.387] Starting ConsensusReactor                    module=consensus impl=ConsensusReactor
I[10-04|13:54:30.387] ConsensusReactor                             module=consensus fastSync=false
I[10-04|13:54:30.387] Starting ConsensusState                      module=consensus impl=ConsensusState
I[10-04|13:54:30.387] Starting WAL                                 module=consensus wal=/home/vagrant/.cometbft/data/cs.wal/wal impl=WAL
I[10-04|13:54:30.388] Starting TimeoutTicker                       module=consensus impl=TimeoutTicker

```

Notice the second row where CometBFT reports that "This node is a validator". It also could be just an observer (regular node).

Next we replay all the messages from the WAL.

```
I[10-04|13:54:30.390] Catchup by replaying consensus messages      module=consensus height=91
I[10-04|13:54:30.390] Replay: New Step                             module=consensus height=91 round=0 step=RoundStepNewHeight
I[10-04|13:54:30.390] Replay: Done                                 module=consensus

```

"Started node" message signals that everything is ready for work.

```
I[10-04|13:54:30.391] Starting RPC HTTP server on tcp socket 0.0.0.0:26657 module=rpc-server
I[10-04|13:54:30.392] Started node                                 module=main nodeInfo="NodeInfo{id: DF22D7C92C91082324A1312F092AA1DA197FA598DBBFB6526E, moniker: anonymous, network: test-chain-3MNw2N [remote , listen 10.0.2.15:26656], version: 0.11.0-10f361fc ([wire_version=0.6.2 p2p_version=0.5.0 consensus_version=v1/0.2.2 rpc_version=0.7.0/3 tx_index=on rpc_addr=tcp://0.0.0.0:26657])}"

```

N