https://docs.junonetwork.io/developer-guides/miscellaneous/conversions

https://github.com/scrtlabs/bech32.scrtlabs.com

https://www.mintscan.io/


Token Conversion
----------------

### **Understanding Micro Tokens**

-   **Example**: 1 Juno = 1 billion ujo
-   **Usage**: The backend handles tokens in micro denominations, while the frontend displays them in a human-readable format (with decimals).

### **Conversion Between Address Types**

-   **Account Types**:
    -   **Base Accounts**: Standard user wallets (e.g., Cosmos one).
    -   **Module Accounts**: Special accounts for modules or contracts.
    -   **Validator Operator (Valer) Addresses**: Used by validators running nodes (e.g., Cosmos valer one).
-   **Consensus Address**: Specific to validator consensus; difficult to convert between different address types without proper tools.

* * * *

Tools for Conversion
--------------------

### **BECH32 Encoding and Conversion**

-   **Usage**: Convert between different address formats using BECH32 encoding.
-   **Tools**:
    -   **CMJS Encoding Library**: Handles conversions between different wallet addresses (e.g., valer to base addresses).
    -   **Mintscan**: Helps verify address mappings between different account types.

### **CLI Tools**

-   **Debug Commands**: Available within Juno or other Cosmos SDK chains to convert between address formats.
    -   **Command Example**: `Beck 32 convert` allows conversion between different address prefixes (e.g., Cosmos).
    -   **Output**: Provides a corresponding address in the desired format.

### **Cross-Network Conversions**

-   **Standard Cosmos SDK Chains**: Conversion is possible across chains using the 118 coin type (e.g., Juno, Cosmos Hub, Osmosis).
-   **EVM-Based Chains**: Conversion requires user-provided public keys or mnemonics due to different coin types (e.g., Secret Network, Terra).

* * * *

Advanced Conversions
--------------------

### **Hex to BECH32 Conversion**

-   **Hex Address Conversion**:
    -   **Utility**: Convert hex addresses to BECH32 format to identify linked addresses or account types.
    -   **Application**: Used in scenarios where only hex addresses are available, often found in REST API signatures.

### **Public Key to Validator Address**

-   **Mapping Process**:
    -   **Public Key**: Available through APIs like LCD (Light Client Daemon).
    -   **Conversion**: Public keys can be converted to validator addresses using specific encoding methods (e.g., SHA-256 hashes).
    -   **Tools**: Cosmos directory can help find nodes that provide these endpoints.

### **Additional Debugging Tools**

-   **Juno Binary**: Offers tools for debugging and conversion across different networks without requiring a chain upgrade.
-   **Easily Extendable**: Other networks can integrate similar debugging tools to streamline conversion processes.