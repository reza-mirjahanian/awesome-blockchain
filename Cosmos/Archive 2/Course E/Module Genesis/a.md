Modules generally handle a subset of the state and, as such, they need to define the related subset of the genesis file as well as methods to initialize, verify and export it.

Type Definition
-------------------------------------------------------------------------------------------------------------------------------------

The subset of the genesis state defined from a given module is generally defined in a `genesis.proto` file ([more info](https://docs.cosmos.network/v0.50/learn/advanced/encoding#gogoproto) on how to define protobuf messages). The struct defining the module's subset of the genesis state is usually called `GenesisState` and contains all the module-related values that need to be initialized during the genesis process.

See an example of `GenesisState` protobuf message definition from the `auth` module:

proto/cosmos/auth/v1beta1/genesis.proto
```
syntax="proto3";
package cosmos.auth.v1beta1;
import"google/protobuf/any.proto";
import"gogoproto/gogo.proto";
import"cosmos/auth/v1beta1/auth.proto";
import"amino/amino.proto";
option go_package ="github.com/cosmos/cosmos-sdk/x/auth/types";
// GenesisState defines the auth module's genesis state.
messageGenesisState{
// params defines all the parameters of the module.
Params params =1[(gogoproto.nullable)=false,(amino.dont_omitempty)=true];
// accounts are the accounts present at genesis.
repeatedgoogle.protobuf.Any accounts =2;
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/proto/cosmos/auth/v1beta1/genesis.proto)


### `InitGenesis`[​](https://docs.cosmos.network/v0.50/build/building-modules/genesis#initgenesis "Direct link to initgenesis")

The `InitGenesis` method is executed during when the application is first started. Given a `GenesisState`, it initializes the subset of the state managed by the module by using the module's [`keeper`](https://docs.cosmos.network/v0.50/build/building-modules/keeper) setter function on each parameter within the `GenesisState`.

The [module manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#manager) of the application is responsible for calling the `InitGenesis` method of each of the application's modules in order. This order is set by the application developer via the manager's `SetOrderGenesisMethod`, which is called in the [application's constructor function](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#constructor-function).

See an example of `InitGenesis` from the `auth` module:

x/auth/keeper/genesis.go


### `ExportGenesis`

The `ExportGenesis` method is executed whenever an export of the state is made. It takes the latest known version of the subset of the state managed by the module and creates a new `GenesisState` out of it. This is mainly used when the chain needs to be upgraded via a hard fork.


### GenesisTxHandler

`GenesisTxHandler` is a way for modules to submit state transitions prior to the first block. This is used by `x/genutil` to submit the genesis transactions for the validators to be added to staking.

core/genesis/txhandler.go
```
// TxHandler is an interface that modules can implement to provide genesis state transitions
type TxHandler interface{
ExecuteGenesisTx([]byte)error
}
```