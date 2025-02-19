### InitChain[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#initchain "Direct link to InitChain")

The [`InitChain` ABCI message](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_basic_concepts.md#method-overview) is sent from the underlying CometBFT engine when the chain is first started. It is mainly used to **initialize** parameters and state like:

-   [Consensus Parameters](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_app_requirements.md#consensus-parameters) via `setConsensusParams`.
-   [`checkState` and `finalizeBlockState`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#state-updates) via `setState`.
-   The [block gas meter](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#block-gas-meter), with infinite gas to process genesis transactions.

Finally, the `InitChain(req abci.RequestInitChain)` method of `BaseApp` calls the [`initChainer()`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#initchainer) of the application in order to initialize the main state of the application from the `genesis file` and, if defined, call the [`InitGenesis`](https://docs.cosmos.network/v0.50/build/building-modules/genesis#initgenesis) function of each of the application's modules.

### FinalizeBlock[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#finalizeblock "Direct link to FinalizeBlock")

The [`FinalizeBlock` ABCI message](https://github.com/cometbft/cometbft/blob/v0.38.x/spec/abci/abci++_basic_concepts.md#method-overview) is sent from the underlying CometBFT engine when a block proposal created by the correct proposer is received. The previous `BeginBlock, DeliverTx and Endblock` calls are private methods on the BaseApp struct.

baseapp/abci.go
```
func(app *BaseApp)FinalizeBlock(req *abci.RequestFinalizeBlock)(*abci.ResponseFinalizeBlock,error){

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/abci.go#L623)

#### PreBlock[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#preblock "Direct link to PreBlock")

-   Run the application's [`preBlocker()`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#preblocker), which mainly runs the [`PreBlocker()`](https://docs.cosmos.network/v0.50/build/building-modules/preblock#preblock) method of each of the modules.

#### BeginBlock[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#beginblock "Direct link to BeginBlock")

-   Initialize [`finalizeBlockState`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#state-updates) with the latest header using the `req abci.RequestFinalizeBlock` passed as parameter via the `setState` function.

    baseapp/baseapp.go
    ```
    func(app *BaseApp)beginBlock(req *abci.RequestFinalizeBlock) sdk.BeginBlock {
    var(
    		resp sdk.BeginBlock
    		err  error
    )
    if app.beginBlocker !=nil{
    		resp, err = app.beginBlocker(app.finalizeBlockState.ctx)
    if err !=nil{
    panic(err)
    }
    // append BeginBlock attributes to all events in the EndBlock response
    for i, event :=range resp.Events {
    			resp.Events[i].Attributes =append(
    				event.Attributes,
    				abci.EventAttribute{Key:"mode", Value:"BeginBlock"},
    )
    }
    		resp.Events = sdk.MarkEventsToIndex(resp.Events, app.indexEvents)
    }
    return resp
    }

    ```

    [See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/baseapp.go#L682-L706)

    This function also resets the [main gas meter](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#main-gas-meter).

-   Initialize the [block gas meter](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#block-gas-meter) with the `maxGas` limit. The `gas` consumed within the block cannot go above `maxGas`. This parameter is defined in the application's consensus parameters.

-   Run the application's [`beginBlocker()`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#beginblocker-and-endblocker), which mainly runs the [`BeginBlocker()`](https://docs.cosmos.network/v0.50/build/building-modules/beginblock-endblock#beginblock) method of each of the modules.

-   Set the [`VoteInfos`](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_methods.md#voteinfo) of the application, i.e. the list of validators whose *precommit* for the previous block was included by the proposer of the current block. This information is carried into the [`Context`](https://docs.cosmos.network/v0.50/learn/advanced/context) so that it can be used during transaction execution and EndBlock.

#### Transaction Execution[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#transaction-execution "Direct link to Transaction Execution")

When the underlying consensus engine receives a block proposal, each transaction in the block needs to be processed by the application. To that end, the underlying consensus engine sends the transactions in FinalizeBlock message to the application for each transaction in a sequential order.

Before the first transaction of a given block is processed, a [volatile state](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#state-updates) called `finalizeBlockState` is initialized during FinalizeBlock. This state is updated each time a transaction is processed via `FinalizeBlock`, and committed to the [main state](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#main-state) when the block is [committed](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#commit), after what it is set to `nil`.

baseapp/baseapp.go
```
package baseapp
import(
"context"
"fmt"
"sort"
"strconv"
	errorsmod "cosmossdk.io/errors"
"cosmossdk.io/log"
"github.com/cockroachdb/errors"
	abci "github.com/cometbft/cometbft/abci/types"
"github.com/cometbft/cometbft/crypto/tmhash"
	cmtproto "github.com/cometbft/cometbft/proto/tendermint/types"
	dbm "github.com/cosmos/cosmos-db"
"github.com/cosmos/gogoproto/proto"
"golang.org/x/exp/maps"
	protov2 "google.golang.org/protobuf/proto"
"cosmossdk.io/store"
	storemetrics "cosmossdk.io/store/metrics"
"cosmossdk.io/store/snapshots"
	storetypes "cosmossdk.io/store/types"
"github.com/cosmos/cosmos-sdk/codec"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	servertypes "github.com/cosmos/cosmos-sdk/server/types"
"github.com/cosmos/cosmos-sdk/telemetry"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
"github.com/cosmos/cosmos-sdk/types/mempool"
)
type(
	execMode uint8
// StoreLoader defines a customizable function to control how we load the
// CommitMultiStore from disk. This is useful for state migration, when
// loading a datastore written with an older version of the software. In
// particular, if a module changed the substore key name (or removed a substore)
// between two versions of the software.
	StoreLoader func(ms storetypes.CommitMultiStore)error
)
const(
	execModeCheck           execMode =iota// Check a transaction
	execModeReCheck                         // Recheck a (pending) transaction after a commit
	execModeSimulate                        // Simulate a transaction
	execModePrepareProposal                 // Prepare a block proposal
	execModeProcessProposal                 // Process a block proposal
	execModeVoteExtension                   // Extend or verify a pre-commit vote
	execModeFinalize                        // Finalize a block proposal
)
var_ servertypes.ABCI =(*BaseApp)(nil)
// BaseApp reflects the ABCI application implementation.
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
// NewBaseApp returns a reference to an initialized BaseApp. It accepts a
// variadic number of option functions, which act on the BaseApp to set
// configuration choices.
funcNewBaseApp(
	name string, logger log.Logger, db dbm.DB, txDecoder sdk.TxDecoder, options ...func(*BaseApp),
)*BaseApp {
	app :=&BaseApp{
		logger:           logger,
		name:             name,
		db:               db,
		cms:              store.NewCommitMultiStore(db, logger, storemetrics.NewNoOpMetrics()),// by default we use a no-op metric gather in store
		storeLoader:      DefaultStoreLoader,
		grpcQueryRouter:NewGRPCQueryRouter(),
		msgServiceRouter:NewMsgServiceRouter(),
		txDecoder:        txDecoder,
		fauxMerkleMode:false,
}
for_, option :=range options {
option(app)
}
if app.mempool ==nil{
		app.SetMempool(mempool.NoOpMempool{})
}
	abciProposalHandler :=NewDefaultProposalHandler(app.mempool, app)
if app.prepareProposal ==nil{
		app.SetPrepareProposal(abciProposalHandler.PrepareProposalHandler())
}
if app.processProposal ==nil{
		app.SetProcessProposal(abciProposalHandler.ProcessProposalHandler())
}
if app.extendVote ==nil{
		app.SetExtendVoteHandler(NoOpExtendVote())
}
if app.verifyVoteExt ==nil{
		app.SetVerifyVoteExtensionHandler(NoOpVerifyVoteExtensionHandler())
}
if app.interBlockCache !=nil{
		app.cms.SetInterBlockCache(app.interBlockCache)
}
	app.runTxRecoveryMiddleware =newDefaultRecoveryMiddleware()
// Initialize with an empty interface registry to avoid nil pointer dereference.
// Unless SetInterfaceRegistry is called with an interface registry with proper address codecs base app will panic.
	app.cdc = codec.NewProtoCodec(codectypes.NewInterfaceRegistry())
return app
}
// Name returns the name of the BaseApp.
func(app *BaseApp)Name()string{
return app.name
}
// AppVersion returns the application's protocol version.
func(app *BaseApp)AppVersion()uint64{
return app.appVersion
}
// Version returns the application's version string.
func(app *BaseApp)Version()string{
return app.version
}
// Logger returns the logger of the BaseApp.
func(app *BaseApp)Logger() log.Logger {
return app.logger
}
// Trace returns the boolean value for logging error stack traces.
func(app *BaseApp)Trace()bool{
return app.trace
}
// MsgServiceRouter returns the MsgServiceRouter of a BaseApp.
func(app *BaseApp)MsgServiceRouter()*MsgServiceRouter {return app.msgServiceRouter }
// SetMsgServiceRouter sets the MsgServiceRouter of a BaseApp.
func(app *BaseApp)SetMsgServiceRouter(msgServiceRouter *MsgServiceRouter){
	app.msgServiceRouter = msgServiceRouter
}
// MountStores mounts all IAVL or DB stores to the provided keys in the BaseApp
// multistore.
func(app *BaseApp)MountStores(keys ...storetypes.StoreKey){
for_, key :=range keys {
switch key.(type){
case*storetypes.KVStoreKey:
if!app.fauxMerkleMode {
				app.MountStore(key, storetypes.StoreTypeIAVL)
}else{
// StoreTypeDB doesn't do anything upon commit, and it doesn't
// retain history, but it's useful for faster simulation.
				app.MountStore(key, storetypes.StoreTypeDB)
}
case*storetypes.TransientStoreKey:
			app.MountStore(key, storetypes.StoreTypeTransient)
case*storetypes.MemoryStoreKey:
			app.MountStore(key, storetypes.StoreTypeMemory)
default:
panic(fmt.Sprintf("Unrecognized store key type :%T", key))
}
}
}
// MountKVStores mounts all IAVL or DB stores to the provided keys in the
// BaseApp multistore.
func(app *BaseApp)MountKVStores(keys map[string]*storetypes.KVStoreKey){
for_, key :=range keys {
if!app.fauxMerkleMode {
			app.MountStore(key, storetypes.StoreTypeIAVL)
}else{
// StoreTypeDB doesn't do anything upon commit, and it doesn't
// retain history, but it's useful for faster simulation.
			app.MountStore(key, storetypes.StoreTypeDB)
}
}
}
// MountTransientStores mounts all transient stores to the provided keys in
// the BaseApp multistore.
func(app *BaseApp)MountTransientStores(keys map[string]*storetypes.TransientStoreKey){
for_, key :=range keys {
		app.MountStore(key, storetypes.StoreTypeTransient)
}
}
// MountMemoryStores mounts all in-memory KVStores with the BaseApp's internal
// commit multi-store.
func(app *BaseApp)MountMemoryStores(keys map[string]*storetypes.MemoryStoreKey){
	skeys := maps.Keys(keys)
	sort.Strings(skeys)
for_, key :=range skeys {
		memKey := keys[key]
		app.MountStore(memKey, storetypes.StoreTypeMemory)
}
}
// MountStore mounts a store to the provided key in the BaseApp multistore,
// using the default DB.
func(app *BaseApp)MountStore(key storetypes.StoreKey, typ storetypes.StoreType){
	app.cms.MountStoreWithDB(key, typ,nil)
}
// LoadLatestVersion loads the latest application version. It will panic if
// called more than once on a running BaseApp.
func(app *BaseApp)LoadLatestVersion()error{
	err := app.storeLoader(app.cms)
if err !=nil{
return fmt.Errorf("failed to load latest version: %w", err)
}
return app.Init()
}
// DefaultStoreLoader will be used by default and loads the latest version
funcDefaultStoreLoader(ms storetypes.CommitMultiStore)error{
return ms.LoadLatestVersion()
}
// CommitMultiStore returns the root multi-store.
// App constructor can use this to access the `cms`.
// UNSAFE: must not be used during the abci life cycle.
func(app *BaseApp)CommitMultiStore() storetypes.CommitMultiStore {
return app.cms
}
// SnapshotManager returns the snapshot manager.
// application use this to register extra extension snapshotters.
func(app *BaseApp)SnapshotManager()*snapshots.Manager {
return app.snapshotManager
}
// LoadVersion loads the BaseApp application version. It will panic if called
// more than once on a running baseapp.
func(app *BaseApp)LoadVersion(version int64)error{
	app.logger.Info("NOTICE: this could take a long time to migrate IAVL store to fastnode if you enable Fast Node.\n")
	err := app.cms.LoadVersion(version)
if err !=nil{
return fmt.Errorf("failed to load version %d: %w", version, err)
}
return app.Init()
}
// LastCommitID returns the last CommitID of the multistore.
func(app *BaseApp)LastCommitID() storetypes.CommitID {
return app.cms.LastCommitID()
}
// LastBlockHeight returns the last committed block height.
func(app *BaseApp)LastBlockHeight()int64{
return app.cms.LastCommitID().Version
}
// ChainID returns the chainID of the app.
func(app *BaseApp)ChainID()string{
return app.chainID
}
// AnteHandler returns the AnteHandler of the app.
func(app *BaseApp)AnteHandler() sdk.AnteHandler {
return app.anteHandler
}
// Init initializes the app. It seals the app, preventing any
// further modifications. In addition, it validates the app against
// the earlier provided settings. Returns an error if validation fails.
// nil otherwise. Panics if the app is already sealed.
func(app *BaseApp)Init()error{
if app.sealed {
panic("cannot call initFromMainStore: baseapp already sealed")
}
	emptyHeader := cmtproto.Header{ChainID: app.chainID}
// needed for the export command which inits from store but never calls initchain
	app.setState(execModeCheck, emptyHeader)
	app.Seal()
if app.cms ==nil{
return errors.New("commit multi-store must not be nil")
}
return app.cms.GetPruning().Validate()
}
func(app *BaseApp)setMinGasPrices(gasPrices sdk.DecCoins){
	app.minGasPrices = gasPrices
}
func(app *BaseApp)setHaltHeight(haltHeight uint64){
	app.haltHeight = haltHeight
}
func(app *BaseApp)setHaltTime(haltTime uint64){
	app.haltTime = haltTime
}
func(app *BaseApp)setMinRetainBlocks(minRetainBlocks uint64){
	app.minRetainBlocks = minRetainBlocks
}
func(app *BaseApp)setInterBlockCache(cache storetypes.MultiStorePersistentCache){
	app.interBlockCache = cache
}
func(app *BaseApp)setTrace(trace bool){
	app.trace = trace
}
func(app *BaseApp)setIndexEvents(ie []string){
	app.indexEvents =make(map[string]struct{})
for_, e :=range ie {
		app.indexEvents[e]=struct{}{}
}
}
// Seal seals a BaseApp. It prohibits any further modifications to a BaseApp.
func(app *BaseApp)Seal(){ app.sealed =true}
// IsSealed returns true if the BaseApp is sealed and false otherwise.
func(app *BaseApp)IsSealed()bool{return app.sealed }
// setState sets the BaseApp's state for the corresponding mode with a branched
// multi-store (i.e. a CacheMultiStore) and a new Context with the same
// multi-store branch, and provided header.
func(app *BaseApp)setState(mode execMode, header cmtproto.Header){
	ms := app.cms.CacheMultiStore()
	baseState :=&state{
		ms:  ms,
		ctx: sdk.NewContext(ms, header,false, app.logger).WithStreamingManager(app.streamingManager),
}
switch mode {
case execModeCheck:
		baseState.ctx = baseState.ctx.WithIsCheckTx(true).WithMinGasPrices(app.minGasPrices)
		app.checkState = baseState
case execModePrepareProposal:
		app.prepareProposalState = baseState
case execModeProcessProposal:
		app.processProposalState = baseState
case execModeVoteExtension:
		app.voteExtensionState = baseState
case execModeFinalize:
		app.finalizeBlockState = baseState
default:
panic(fmt.Sprintf("invalid runTxMode for setState: %d", mode))
}
}
// GetFinalizeBlockStateCtx returns the Context associated with the FinalizeBlock
// state. This Context can be used to write data derived from processing vote
// extensions to application state during ProcessProposal.
//
// NOTE:
// - Do NOT use or write to state using this Context unless you intend for
// that state to be committed.
// - Do NOT use or write to state using this Context on the first block.
func(app *BaseApp)GetFinalizeBlockStateCtx() sdk.Context {
return app.finalizeBlockState.ctx
}
// SetCircuitBreaker sets the circuit breaker for the BaseApp.
// The circuit breaker is checked on every message execution to verify if a transaction should be executed or not.
func(app *BaseApp)SetCircuitBreaker(cb CircuitBreaker){
if app.msgServiceRouter ==nil{
panic("cannot set circuit breaker with no msg service router set")
}
	app.msgServiceRouter.SetCircuit(cb)
}
// GetConsensusParams returns the current consensus parameters from the BaseApp's
// ParamStore. If the BaseApp has no ParamStore defined, nil is returned.
func(app *BaseApp)GetConsensusParams(ctx sdk.Context) cmtproto.ConsensusParams {
if app.paramStore ==nil{
return cmtproto.ConsensusParams{}
}
	cp, err := app.paramStore.Get(ctx)
if err !=nil{
panic(fmt.Errorf("consensus key is nil: %w", err))
}
return cp
}
// StoreConsensusParams sets the consensus parameters to the BaseApp's param
// store.
//
// NOTE: We're explicitly not storing the CometBFT app_version in the param store.
// It's stored instead in the x/upgrade store, with its own bump logic.
func(app *BaseApp)StoreConsensusParams(ctx sdk.Context, cp cmtproto.ConsensusParams)error{
if app.paramStore ==nil{
panic("cannot store consensus params with no params store set")
}
return app.paramStore.Set(ctx, cp)
}
// AddRunTxRecoveryHandler adds custom app.runTx method panic handlers.
func(app *BaseApp)AddRunTxRecoveryHandler(handlers ...RecoveryHandler){
for_, h :=range handlers {
		app.runTxRecoveryMiddleware =newRecoveryMiddleware(h, app.runTxRecoveryMiddleware)
}
}
// GetMaximumBlockGas gets the maximum gas from the consensus params. It panics
// if maximum block gas is less than negative one and returns zero if negative
// one.
func(app *BaseApp)GetMaximumBlockGas(ctx sdk.Context)uint64{
	cp := app.GetConsensusParams(ctx)
if cp.Block ==nil{
return0
}
	maxGas := cp.Block.MaxGas
switch{
case maxGas <-1:
panic(fmt.Sprintf("invalid maximum block gas: %d", maxGas))
case maxGas ==-1:
return0
default:
returnuint64(maxGas)
}
}
func(app *BaseApp)validateFinalizeBlockHeight(req *abci.RequestFinalizeBlock)error{
if req.Height <1{
return fmt.Errorf("invalid height: %d", req.Height)
}
	lastBlockHeight := app.LastBlockHeight()
// expectedHeight holds the expected height to validate
var expectedHeight int64
if lastBlockHeight ==0&& app.initialHeight >1{
// In this case, we're validating the first block of the chain, i.e no
// previous commit. The height we're expecting is the initial height.
		expectedHeight = app.initialHeight
}else{
// This case can mean two things:
//
// - Either there was already a previous commit in the store, in which
// case we increment the version from there.
// - Or there was no previous commit, in which case we start at version 1.
		expectedHeight = lastBlockHeight +1
}
if req.Height != expectedHeight {
return fmt.Errorf("invalid height: %d; expected: %d", req.Height, expectedHeight)
}
returnnil
}
// validateBasicTxMsgs executes basic validator calls for messages.
funcvalidateBasicTxMsgs(msgs []sdk.Msg)error{
iflen(msgs)==0{
return errorsmod.Wrap(sdkerrors.ErrInvalidRequest,"must contain at least one message")
}
for_, msg :=range msgs {
		m, ok := msg.(sdk.HasValidateBasic)
if!ok {
continue
}
if err := m.ValidateBasic(); err !=nil{
return err
}
}
returnnil
}
func(app *BaseApp)getState(mode execMode)*state {
switch mode {
case execModeFinalize:
return app.finalizeBlockState
case execModePrepareProposal:
return app.prepareProposalState
case execModeProcessProposal:
return app.processProposalState
default:
return app.checkState
}
}
func(app *BaseApp)getBlockGasMeter(ctx sdk.Context) storetypes.GasMeter {
if maxGas := app.GetMaximumBlockGas(ctx); maxGas >0{
return storetypes.NewGasMeter(maxGas)
}
return storetypes.NewInfiniteGasMeter()
}
// retrieve the context for the tx w/ txBytes and other memoized values.
func(app *BaseApp)getContextForTx(mode execMode, txBytes []byte) sdk.Context {
	modeState := app.getState(mode)
if modeState ==nil{
panic(fmt.Sprintf("state is nil for mode %v", mode))
}
	ctx := modeState.ctx.
WithTxBytes(txBytes)
// WithVoteInfos(app.voteInfos) // TODO: identify if this is needed
	ctx = ctx.WithConsensusParams(app.GetConsensusParams(ctx))
if mode == execModeReCheck {
		ctx = ctx.WithIsReCheckTx(true)
}
if mode == execModeSimulate {
		ctx,_= ctx.CacheContext()
}
return ctx
}
// cacheTxContext returns a new context based off of the provided context with
// a branched multi-store.
func(app *BaseApp)cacheTxContext(ctx sdk.Context, txBytes []byte)(sdk.Context, storetypes.CacheMultiStore){
	ms := ctx.MultiStore()
// TODO: https://github.com/cosmos/cosmos-sdk/issues/2824
	msCache := ms.CacheMultiStore()
if msCache.TracingEnabled(){
		msCache = msCache.SetTracingContext(
			storetypes.TraceContext(
map[string]interface{}{
"txHash": fmt.Sprintf("%X", tmhash.Sum(txBytes)),
},
),
).(storetypes.CacheMultiStore)
}
return ctx.WithMultiStore(msCache), msCache
}
func(app *BaseApp)beginBlock(req *abci.RequestFinalizeBlock) sdk.BeginBlock {
var(
		resp sdk.BeginBlock
		err  error
)
if app.beginBlocker !=nil{
		resp, err = app.beginBlocker(app.finalizeBlockState.ctx)
if err !=nil{
panic(err)
}
// append BeginBlock attributes to all events in the EndBlock response
for i, event :=range resp.Events {
			resp.Events[i].Attributes =append(
				event.Attributes,
				abci.EventAttribute{Key:"mode", Value:"BeginBlock"},
)
}
		resp.Events = sdk.MarkEventsToIndex(resp.Events, app.indexEvents)
}
return resp
}
func(app *BaseApp)deliverTx(tx []byte)*abci.ExecTxResult {
	gInfo := sdk.GasInfo{}
	resultStr :="successful"
var resp *abci.ExecTxResult
deferfunc(){
		telemetry.IncrCounter(1,"tx","count")
		telemetry.IncrCounter(1,"tx", resultStr)
		telemetry.SetGauge(float32(gInfo.GasUsed),"tx","gas","used")
		telemetry.SetGauge(float32(gInfo.GasWanted),"tx","gas","wanted")
}()
	gInfo, result, anteEvents, err := app.runTx(execModeFinalize, tx)
if err !=nil{
		resultStr ="failed"
		resp = sdkerrors.ResponseExecTxResultWithEvents(
			err,
			gInfo.GasWanted,
			gInfo.GasUsed,
			sdk.MarkEventsToIndex(anteEvents, app.indexEvents),
			app.trace,
)
return resp
}
	resp =&abci.ExecTxResult{
		GasWanted:int64(gInfo.GasWanted),
		GasUsed:int64(gInfo.GasUsed),
		Log:       result.Log,
		Data:      result.Data,
		Events:    sdk.MarkEventsToIndex(result.Events, app.indexEvents),
}
return resp
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/baseapp.go#LL708-L743)

Transaction execution within `FinalizeBlock` performs the **exact same steps as `CheckTx`**, with a little caveat at step 3 and the addition of a fifth step:

1.  The `AnteHandler` does **not** check that the transaction's `gas-prices` is sufficient. That is because the `min-gas-prices` value `gas-prices` is checked against is local to the node, and therefore what is enough for one full-node might not be for another. This means that the proposer can potentially include transactions for free, although they are not incentivised to do so, as they earn a bonus on the total fee of the block they propose.
2.  For each `sdk.Msg` in the transaction, route to the appropriate module's Protobuf [`Msg` service](https://docs.cosmos.network/v0.50/build/building-modules/msg-services). Additional *stateful* checks are performed, and the branched multistore held in `finalizeBlockState`'s `context` is updated by the module's `keeper`. If the `Msg` service returns successfully, the branched multistore held in `context` is written to `finalizeBlockState` `CacheMultiStore`.

During the additional fifth step outlined in (2), each read/write to the store increases the value of `GasConsumed`. You can find the default cost of each operation:

store/types/gas.go
```

// KVGasConfig returns a default gas config for KVStores.
funcKVGasConfig() GasConfig {
return GasConfig{
		HasCost:1000,
		DeleteCost:1000,
		ReadCostFlat:1000,
		ReadCostPerByte:3,
		WriteCostFlat:2000,
		WriteCostPerByte:30,
		IterNextCostFlat:30,
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/store/types/gas.go#L230-L241)

At any point, if `GasConsumed > GasWanted`, the function returns with `Code != 0` and the execution fails.

Each transactions returns a response to the underlying consensus engine of type [`abci.ExecTxResult`](https://github.com/cometbft/cometbft/blob/v0.38.0-rc1/spec/abci/abci%2B%2B_methods.md#exectxresult). The response contains:

-   `Code (uint32)`: Response Code. `0` if successful.
-   `Data ([]byte)`: Result bytes, if any.
-   `Log (string):` The output of the application's logger. May be non-deterministic.
-   `Info (string):` Additional information. May be non-deterministic.
-   `GasWanted (int64)`: Amount of gas requested for transaction. It is provided by users when they generate the transaction.
-   `GasUsed (int64)`: Amount of gas consumed by transaction. During transaction execution, this value is computed by multiplying the standard cost of a transaction byte by the size of the raw transaction, and by adding gas each time a read/write to the store occurs.
-   `Events ([]cmn.KVPair)`: Key-Value tags for filtering and indexing transactions (eg. by account). See [`event`s](https://docs.cosmos.network/v0.50/learn/advanced/events) for more.
-   `Codespace (string)`: Namespace for the Code.

#### EndBlock[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#endblock "Direct link to EndBlock")

EndBlock is run after transaction execution completes. It allows developers to have logic be executed at the end of each block. In the Cosmos SDK, the bulk EndBlock() method is to run the application's EndBlocker(), which mainly runs the EndBlocker() method of each of the application's modules.

baseapp/baseapp.go
```
func(app *BaseApp)endBlock(ctx context.Context)(sdk.EndBlock,error){
var endblock sdk.EndBlock
if app.endBlocker !=nil{
		eb, err := app.endBlocker(app.finalizeBlockState.ctx)
if err !=nil{
panic(err)
}
// append EndBlock attributes to all events in the EndBlock response
for i, event :=range eb.Events {
			eb.Events[i].Attributes =append(
				event.Attributes,
				abci.EventAttribute{Key:"mode", Value:"EndBlock"},
)
}
		eb.Events = sdk.MarkEventsToIndex(eb.Events, app.indexEvents)
		endblock = eb
}
return endblock,nil
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/baseapp.go#L747-L769)

### Commit[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#commit "Direct link to Commit")

The [`Commit` ABCI message](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_basic_concepts.md#method-overview) is sent from the underlying CometBFT engine after the full-node has received *precommits* from 2/3+ of validators (weighted by voting power). On the `BaseApp` end, the `Commit(res abci.ResponseCommit)` function is implemented to commit all the valid state transitions that occurred during `FinalizeBlock` and to reset state for the next block.

To commit state-transitions, the `Commit` function calls the `Write()` function on `finalizeBlockState.ms`, where `finalizeBlockState.ms` is a branched multistore of the main store `app.cms`. Then, the `Commit` function sets `checkState` to the latest header (obtained from `finalizeBlockState.ctx.BlockHeader`) and `finalizeBlockState` to `nil`.

Finally, `Commit` returns the hash of the commitment of `app.cms` back to the underlying consensus engine. This hash is used as a reference in the header of the next block.

### Info[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#info "Direct link to Info")

The [`Info` ABCI message](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_basic_concepts.md#info-methods) is a simple query from the underlying consensus engine, notably used to sync the latter with the application during a handshake that happens on startup. When called, the `Info(res abci.ResponseInfo)` function from `BaseApp` will return the application's name, version and the hash of the last commit of `app.cms`.

### Query[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#query "Direct link to Query")

The [`Query` ABCI message](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_basic_concepts.md#info-methods) is used to serve queries received from the underlying consensus engine, including queries received via RPC like CometBFT RPC. It used to be the main entrypoint to build interfaces with the application, but with the introduction of [gRPC queries](https://docs.cosmos.network/v0.50/build/building-modules/query-services) in Cosmos SDK v0.40, its usage is more limited. The application must respect a few rules when implementing the `Query` method, which are outlined [here](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_app_requirements.md#query).

Each CometBFT `query` comes with a `path`, which is a `string` which denotes what to query. If the `path` matches a gRPC fully-qualified service method, then `BaseApp` will defer the query to the `grpcQueryRouter` and let it handle it like explained [above](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#grpc-query-router). Otherwise, the `path` represents a query that is not (yet) handled by the gRPC router. `BaseApp` splits the `path` string with the `/` delimiter. By convention, the first element of the split string (`split[0]`) contains the category of `query` (`app`, `p2p`, `store` or `custom` ). The `BaseApp` implementation of the `Query(req abci.RequestQuery)` method is a simple dispatcher serving these 4 main categories of queries:

-   Application-related queries like querying the application's version, which are served via the `handleQueryApp` method.
-   Direct queries to the multistore, which are served by the `handlerQueryStore` method. These direct queries are different from custom queries which go through `app.queryRouter`, and are mainly used by third-party service provider like block explorers.
-   P2P queries, which are served via the `handleQueryP2P` method. These queries return either `app.addrPeerFilter` or `app.ipPeerFilter` that contain the list of peers filtered by address or IP respectively. These lists are first initialized via `options` in `BaseApp`'s [constructor](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#constructor).

### ExtendVote[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#extendvote "Direct link to ExtendVote")

`ExtendVote` allows an application to extend a pre-commit vote with arbitrary data. This process does NOT have to be deterministic and the data returned can be unique to the validator process.

In the Cosmos-SDK this is implemented as a NoOp:

baseapp/abci\_utils.go
```
// NoOpExtendVote defines a no-op ExtendVote handler. It will always return an
// empty byte slice as the vote extension.
funcNoOpExtendVote() sdk.ExtendVoteHandler {
returnfunc(_ sdk.Context,_*abci.RequestExtendVote)(*abci.ResponseExtendVote,error){
return&abci.ResponseExtendVote{VoteExtension:[]byte{}},nil
}
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/abci_utils.go#L274-L281)

### VerifyVoteExtension[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#verifyvoteextension "Direct link to VerifyVoteExtension")

`VerifyVoteExtension` allows an application to verify that the data returned by `ExtendVote` is valid. This process MUST be deterministic. Moreover, the value of ResponseVerifyVoteExtension.status MUST exclusively depend on the parameters passed in the call to RequestVerifyVoteExtension, and the last committed Application state.

In the Cosmos-SDK this is implemented as a NoOp:

baseapp/abci\_utils.go
```
// NoOpVerifyVoteExtensionHandler defines a no-op VerifyVoteExtension handler. It
// will always return an ACCEPT status with no error.
funcNoOpVerifyVoteExtensionHandler() sdk.VerifyVoteExtensionHandler {
returnfunc(_ sdk.Context,_*abci.RequestVerifyVoteExtension)(*abci.ResponseVerifyVoteExtension,error){
return&abci.ResponseVerifyVoteExtension{Status: abci.ResponseVerifyVoteExtension_ACCEPT},nil
}
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/abci_utils.go#L282-L288)