Upgrade your app modules smoothly with custom in-place store migration logic.

The Cosmos SDK uses two methods to perform upgrades:

-   Exporting the entire application state to a JSON file using the `export` CLI command, making changes, and then starting a new binary with the changed JSON file as the genesis file.

-   Perform upgrades in place, which significantly decrease the upgrade time for chains with a larger state. Use the [Module Upgrade Guide](https://docs.cosmos.network/v0.50/build/building-modules/upgrade) to set up your application modules to take advantage of in-place upgrades.


Tracking Module Versions[​](https://docs.cosmos.network/v0.50/learn/advanced/upgrade#tracking-module-versions "Direct link to Tracking Module Versions")
--------------------------------------------------------------------------------------------------------------------------------------------------------

Each module gets assigned a consensus version by the module developer. The consensus version serves as the breaking change version of the module. The Cosmos SDK keeps track of all module consensus versions in the x/upgrade `VersionMap` store. During an upgrade, the difference between the old `VersionMap` stored in state and the new `VersionMap` is calculated by the Cosmos SDK. For each identified difference, the module-specific migrations are run and the respective consensus version of each upgraded module is incremented