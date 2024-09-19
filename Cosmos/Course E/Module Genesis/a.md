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