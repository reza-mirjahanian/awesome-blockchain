The BaseApp type holds many important parameters for any Cosmos SDK based application.

baseapp/baseapp.go
```
type BaseApp struct{
// initialized on creation
	logger            log.Logger
	name              string// application name from abci.BlockInfo
	db                dbm.DB                      // common DB backend
	cms               storetypes.CommitMultiStore // Main (uncached) state
	qms               storetypes.MultiStore       // Optional alternative multistore for querying only.
	storeLoader       StoreLoader                 // function to handle store loading, may be overridden with SetStoreLoader()
	grpcQueryRouter   *GRPCQueryRouter            // router for redirecting gRPC query calls
	msgServiceRouter  *MsgServiceRouter           // router for redirecting Msg service messages
	interfaceRegistry codectypes.InterfaceRegistry
	txDecoder         sdk.TxDecoder // unmarshal []byte into sdk.Tx
	txEncoder         sdk.TxEncoder // marshal sdk.Tx into []byte
	mempool     mempool.Mempool // application side mempool
	anteHandler sdk.AnteHandler // ante handler for fee and auth
	postHandler sdk.PostHandler // post handler, optional, e.g. for tips
	initChainer        sdk.InitChainer                // ABCI InitChain handler
	beginBlocker       sdk.BeginBlocker               // (legacy ABCI) BeginBlock handler
	endBlocker         sdk.EndBlocker                 // (legacy ABCI) EndBlock handler
	processProposal    sdk.ProcessProposalHandler     // ABCI ProcessProposal handler
	prepareProposal    sdk.PrepareProposalHandler     // ABCI PrepareProposal
	extendVote         sdk.ExtendVoteHandler          // ABCI ExtendVote handler
	verifyVoteExt      sdk.VerifyVoteExtensionHandler // ABCI VerifyVoteExtension handler
	prepareCheckStater sdk.PrepareCheckStater         // logic to run during commit using the checkState
	precommiter        sdk.Precommiter                // logic to run during commit using the deliverState
	addrPeerFilter sdk.PeerFilter // filter peers by address and port
	idPeerFilter   sdk.PeerFilter // filter peers by node ID
	fauxMerkleMode bool// if true, IAVL MountStores uses MountStoresDB for simulation speed.
// manages snapshots, i.e. dumps of app state at certain intervals
	snapshotManager *snapshots.Manager
// volatile states:
//
// - checkState is set on InitChain and reset on Commit
// - finalizeBlockState is set on InitChain and FinalizeBlock and set to nil
// on Commit.
//
// - checkState: Used for CheckTx, which is set based on the previous block's
// state. This state is never committed.
//
// - prepareProposalState: Used for PrepareProposal, which is set based on the
// previous block's state. This state is never committed. In case of multiple
// consensus rounds, the state is always reset to the previous block's state.
//
// - voteExtensionState: Used for ExtendVote and VerifyVoteExtension, which is
// set based on the previous block's state. This state is never committed. In
// case of multiple rounds, the state is always reset to the previous block's
// state.
//
// - processProposalState: Used for ProcessProposal, which is set based on the
// the previous block's state. This state is never committed. In case of
// multiple rounds, the state is always reset to the previous block's state.
//
// - finalizeBlockState: Used for FinalizeBlock, which is set based on the
// previous block's state. This state is committed.
	checkState           *state
	prepareProposalState *state
	processProposalState *state
	voteExtensionState   *state
	finalizeBlockState   *state
// An inter-block write-through cache provided to the context during the ABCI
// FinalizeBlock call.
	interBlockCache storetypes.MultiStorePersistentCache
// paramStore is used to query for ABCI consensus parameters from an
// application parameter store.
	paramStore ParamStore
// The minimum gas prices a validator is willing to accept for processing a
// transaction. This is mainly used for DoS and spam prevention.
	minGasPrices sdk.DecCoins
// initialHeight is the initial height at which we start the BaseApp
	initialHeight int64
// flag for sealing options and parameters to a BaseApp
	sealed bool
// block height at which to halt the chain and gracefully shutdown
	haltHeight uint64
// minimum block time (in Unix seconds) at which to halt the chain and gracefully shutdown
	haltTime uint64
// minRetainBlocks defines the minimum block height offset from the current
// block being committed, such that all blocks past this offset are pruned
// from CometBFT. It is used as part of the process of determining the
// ResponseCommit.RetainHeight value during ABCI Commit. A value of 0 indicates
// that no blocks should be pruned.
//
// Note: CometBFT block pruning is dependant on this parameter in conjunction
// with the unbonding (safety threshold) period, state pruning and state sync
// snapshot parameters to determine the correct minimum value of
// ResponseCommit.RetainHeight.
	minRetainBlocks uint64
// application's version string
	version string
// application's protocol version that increments on every upgrade
// if BaseApp is passed to the upgrade keeper's NewKeeper method.
	appVersion uint64
// recovery handler for app.runTx method
	runTxRecoveryMiddleware recoveryMiddleware
// trace set will return full stack traces for errors in ABCI Log field
	trace bool
// indexEvents defines the set of events in the form {eventType}.{attributeKey},
// which informs CometBFT what to index. If empty, all events will be indexed.
	indexEvents map[string]struct{}
// streamingManager for managing instances and configuration of ABCIListener services
	streamingManager storetypes.StreamingManager
	chainID string
	cdc codec.Codec
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/baseapp.go#L58-L182)

Let us go through the most important components.

> **Note**: Not all parameters are described, only the most important ones. Refer to the type definition for the full list.


-   [`CommitMultiStore`](https://docs.cosmos.network/v0.50/learn/advanced/store#commitmultistore): This is the main store of the application, which holds the canonical state that is committed at the [end of each block](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#commit). This store is **not** cached, meaning it is not used to update the application's volatile (un-committed) states. The `CommitMultiStore` is a multi-store, meaning a store of stores. Each module of the application uses one or multiple `KVStores` in the multi-store to persist their subset of the state.