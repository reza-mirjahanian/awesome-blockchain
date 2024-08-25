Upgrade your app modules smoothly with custom in-place store migration logic.

The Cosmos SDK uses two methods to perform upgrades:

-   Exporting the entire application state to a JSON file using the `export` CLI command, making changes, and then starting a new binary with the changed JSON file as the genesis file.

-   Perform upgrades in place, which significantly decrease the upgrade time for chains with a larger state. Use the [Module Upgrade Guide](https://docs.cosmos.network/v0.50/build/building-modules/upgrade) to set up your application modules to take advantage of in-place upgrades.