An invariant is a property of the application that should always be true. In the context of the Cosmos SDK, an `Invariant` is a function that checks for a particular invariant. These functions are useful to detect bugs early on and act upon them to limit their potential consequences (e.g. by halting the chain). They are also useful in the development process of the application to detect bugs via simulations.

Implementing `Invariant`
------------------------------------------------------------------------------------------------------------------------------------------------------------------

An `Invariant` is a function that checks for a particular invariant within a module. Module `Invariant`s must follow the `Invariant` type:

types/invariant.go
```
type Invariant func(ctx Context)(string,bool)

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/types/invariant.go#L9)

------------------------

In practice, each module implements `Invariant`s in a `keeper/invariants.go` file within the module's folder. The standard is to implement one `Invariant` function per logical grouping of invariants with the following model:

```
// Example for an Invariant that checks balance-related invariants
funcBalanceInvariants(k Keeper) sdk.Invariant {
returnfunc(ctx context.Context)(string,bool){
// Implement checks for balance-related invariants
}
}

```

Additionally, module developers should generally implement an `AllInvariants` function that runs all the `Invariant`s functions of the module:

```
// AllInvariants runs all invariants of the module.
// In this example, the module implements two Invariants: BalanceInvariants and DepositsInvariants
funcAllInvariants(k Keeper) sdk.Invariant {
returnfunc(ctx context.Context)(string,bool){
        res, stop :=BalanceInvariants(k)(ctx)
if stop {
return res, stop
}
returnDepositsInvariant(k)(ctx)
}
}

```

Finally, module developers need to implement the `RegisterInvariants` method as part of the [`AppModule` interface](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#appmodule). Indeed, the `RegisterInvariants` method of the module, implemented in the `module/module.go` file, typically only defers the call to a `RegisterInvariants` method implemented in the `keeper/invariants.go` file. The `RegisterInvariants` method registers a route for each `Invariant` function in the [`InvariantRegistry`](https://docs.cosmos.network/v0.50/build/building-modules/invariants#invariant-registry):

x/staking/keeper/invariants.go
```
// RegisterInvariants registers all staking invariants
funcRegisterInvariants(ir sdk.InvariantRegistry, k *Keeper){
	ir.RegisterRoute(types.ModuleName,"module-accounts",
ModuleAccountInvariants(k))
	ir.RegisterRoute(types.ModuleName,"nonnegative-power",
NonNegativePowerInvariant(k))
	ir.RegisterRoute(types.ModuleName,"positive-delegation",
PositiveDelegationInvariant(k))
	ir.RegisterRoute(types.ModuleName,"delegator-shares",
DelegatorSharesInvariant(k))
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/staking/keeper/invariants.go#L12-L22)

For more, see an example of [`Invariant`s implementation from the `staking` module](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/staking/keeper/invariants.go).


Invariant Registry
-------------------------------------------------------------------------------------------------------------------------------------------------

The `InvariantRegistry` is a registry where the `Invariant`s of all the modules of an application are registered. There is only one `InvariantRegistry` per **application**, meaning module developers need not implement their own `InvariantRegistry` when building a module. **All module developers need to do is to register their modules' invariants in the `InvariantRegistry`, as explained in the section above**. The rest of this section gives more information on the `InvariantRegistry` itself, and does not contain anything directly relevant to module developers.

At its core, the `InvariantRegistry` is defined in the Cosmos SDK as an interface:

types/invariant.go
```
// expected interface for registering invariants
type InvariantRegistry interface{
RegisterRoute(moduleName, route string, invar Invariant)
}
```