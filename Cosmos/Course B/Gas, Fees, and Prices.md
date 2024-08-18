### Understanding Gas

-   **Gas**: Refers to the amount of computational power required to execute a transaction.
    -   **Compute**: Gas is essentially a substitute term for compute; the more complex the transaction, the more gas is needed.
    -   **Standard Gas Value**: Starts around 100,000 and increases with the computational demands of the transaction.

### Gas Adjustment

-   **Definition**: An adjustment to the amount of gas to ensure successful transaction execution.
    -   **Example**: If a transaction typically requires 1 million gas, you can add a 30% adjustment (e.g., 1.3), allowing up to 1.3 million gas to be used.
    -   **Purpose**: Ensures the transaction covers unexpected computational costs, such as increased state size from multiple users interacting with a contract.

### Gas Fees vs. Gas Prices

-   **Gas Fees**:
    -   **Set Value**: The fixed amount of tokens you are willing to pay for the gas used in a transaction.
    -   **Example**: For a transaction requiring 1 million compute units, you might set a fee of 5,000 UT tokens.
-   **Gas Prices**:
    -   **Definition**: The cost per unit of gas, often used in combination with gas adjustments.
    -   **Calculation**: Total gas cost = Gas used x Gas price.
    -   **Example**: If the gas price is 0.001 UT and the transaction requires 1 million compute units, the total fee is calculated based on this price.

### Practical Example of Running a Transaction

1.  **Setting Up the Environment**:

    -   Start a local Juno node to process blocks.
    -   Use the command `Juno status 26657` to verify the node status.
2.  **Running a Transaction**:

    -   **Account Setup**: Create and view accounts using `Juno Keys list`.
    -   **Bank Send Command**: Transfer funds between accounts using the transaction subcommand, specifying the bank module and the send functionality.
    -   **Transaction Parameters**:
        -   **Amount**: Send 123 U Juno (where 1 U Juno is 1/1,000,000th of a Juno).
        -   **Fees**: If no fees are specified, the validator sets a default fee, calculated based on the gas used.
3.  **Handling Fees and Gas Calculations**:

    -   **Gas Fee Example**:
        -   Assume a transaction uses 250,000 gas and requires 500 U Juno.
        -   If the fee is set to 499 U Juno, the transaction fails due to insufficient fees.
        -   If the fee is set higher (e.g., 251 U Juno), the transaction succeeds, using only the required gas.
    -   **Auto Gas Adjustment**:
        -   Use the `gas-adjustment` command to automatically adjust gas by a set percentage (e.g., 1.3 for a 30% increase).
        -   The system recalculates the required fees based on the actual gas used.

### Gas Prices in Action

-   **Gas Prices Setting**:
    -   Set a minimum gas price to ensure transactions are processed.
    -   Example: Set gas price to 0.0025 U Juno.
-   **Public Nodes**:
    -   Public nodes might not always display the gas price; it's typically assumed to be a standard value unless otherwise specified.

### Final Thoughts on Gas Management

-   **Fee Calculation**:
    -   Multiply the gas price by the amount of gas used to determine the fee.
    -   Overestimating gas results in the excess being consumed, with no way to recoup overpaid fees.
-   **Tips**:
    -   Be cautious of entering incorrect amounts to avoid excessive fees.
    -   Utilize the different options to fine-tune gas and fee settings for optimal transaction execution.