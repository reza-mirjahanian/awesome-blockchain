
# Integration 

how to integrate WebAssembly (Wasm) smart contracts into a custom Cosmos SDK application. It explains how to use the x/wasm module and provides step-by-step instructions for integration, along with examples and code snippets.

## Prerequisites and Integration Methods

Before integrating Wasm, ensure you're using a compatible Cosmos SDK version (see the table provided) and are running on an Intel/AMD64 CPU with either OSX or Linux.

The document describes two main approaches for integration:

1.  **As an external module:**  Import x/wasm and connect it to your app.go file. This is the simplest method, allowing you to use Wasm contracts alongside your existing modules.
    
2.  **Copied into your app:**  This method is recommended for specific cases like disabling key SDK modules or customizing core modules. Copy the x/wasm module from the latest release into your app, ensuring minimal changes to the module itself.
    

## Customizing Integration

To enhance the integration, the document explores ways to add custom hooks and extend the contract interface:

-   **Calling contracts from native code:**  Use the wasm.Keeper to interact with contracts from your native modules by sending messages or querying the contract.
    
-   **Extending the Contract Interface:**  Define custom messages and queries to expose native module functionalities to the contracts. These extensions can be used by any CosmWasm contract importing your library.
    
-   **Calling into the SDK:**  Introduce a new module (e.g., x/contracts) to handle custom bindings between your contracts and native modules. This module uses a  `CustomQuerier`  for custom queries and a  `CustomEncoder`  to convert custom messages into SDK messages.
    

## Wiring It All Together

The final step is to wire up your custom callbacks within your application:

-   **Integration Tests:**  Define the supported feature set and create test cases with a compiled contract to ensure the integration works correctly.
    
-   **App.go Configuration:**  Edit the default NewKeeper constructor to include your custom encoder and querier. This allows you to compile your chain and upload custom contracts.