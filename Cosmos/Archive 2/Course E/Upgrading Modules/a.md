[In-Place Store Migrations](https://docs.cosmos.network/v0.50/learn/advanced/upgrade) allow your modules to upgrade to new versions that include breaking changes. This document outlines how to build modules to take advantage of this functionality.


Consensus Version
-------------------------------------------------------------------------------------------------------------------------------------------

Successful upgrades of existing modules require each `AppModule` to implement the function `ConsensusVersion() uint64`.

-   The versions must be hard-coded by the module developer.
-   The initial version **must** be set to 1.

Consensus versions serve as state-breaking versions of app modules and must be incremented when the module introduces breaking changes.



Writing Migration Scripts
-------------------------------------------------------------------------------------------------------------------------------------------------------------------

To define the functionality that takes place during an upgrade, write a migration script and place the functions in a `migrations/` directory. For example, to write migration scripts for the bank module, place the functions in `x/bank/migrations/`. Use the recommended naming convention for these functions. For example, `v2bank` is the script that migrates the package `x/bank/migrations/v2`:

```
// Migrating bank module from version 1 to 2
func(m Migrator)Migrate1to2(ctx sdk.Context)error{
return v2bank.MigrateStore(ctx, m.keeper.storeKey)// v2bank is package `x/bank/migrations/v2`.
}

```

To see example code of changes that were implemented in a migration of balance keys, check out [migrateBalanceKeys](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/bank/migrations/v2/store.go#L55-L76). For context, this code introduced migrations of the bank store that updated addresses to be prefixed by their length in bytes as outlined in [ADR-028](https://docs.cosmos.network/v0.50/build/architecture/adr-028-public-key-addresses).