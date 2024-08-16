
###  Cosmos SDK Account Types

The Cosmos SDK has two main account types: **Base Account** and **Module Account**. Here's a detailed look at each type:

#### Base Account
- **Purpose**: Used by the average user.
- **Components**:
  - **Address**: The human-readable wallet.
  - **Public Key**: Stored when a transaction is submitted.
  - **Signature**: Saved when a transaction is made.
  - **Account Number**: Incremented by one for each new base account or module account.
  - **Sequence**: A number that increases with each transaction made by the account. Starts at zero upon account creation and increments with every transaction.

#### Module Account
- **Purpose**: Owned by the blockchain itself rather than an individual user.
- **Components**:
  - **No Public Key**: As the chain owns the account, no public key is needed.
  - **Derived Wallet**: The account wallet is derived from the account's name. For example, the string `gov` would generate a specific wallet address.
  - **Base Account Wrapping**: The base account is wrapped into the module account.
  - **Permissions**: Additional permissions can be set, such as the ability to mint or burn tokens and perform other specified functions.

#### Detailed Account Information

##### Base Account Example
- **Account Number**: Indicates the creation time of the account.
- **Wallet Address**: Unique identifier for the account.
- **Public Key**: Set after the account performs a transaction; if the account hasn't transacted, the public key is null but the account can still receive funds.
- **Sequence**: Reflects the number of transactions made. For instance, an account with a sequence of one has completed one transaction, and the public key is set.

##### Module Account Example
- **Example**: Transfer module of the IBC keeper.
- **Derived Address**: Converted from a string (e.g., `gov`).
- **Account Number**: Specific to the module.
- **Permissions**: Includes actions like minting and burning tokens, specific to how IBC operates.

#### Vesting Accounts
- **Continuous Vesting Account**: An example of a wrapped base account with additional fields.
  - **Delegated Free Vesting**: Indicates the vesting details.
  - **Vesting End Date**: When the vesting period concludes.
  - **Original Vesting Amount**: Initial amount set for vesting.
  - **Vesting Start Date**: When the vesting period begins.

### Summary
- **Base Accounts** are typical user accounts with address, public key, signature, account number, and sequence.
- **Module Accounts** are owned by the blockchain, derive wallets from names, and include additional permissions.
- **Vesting Accounts** are specialized base accounts with extra fields for managing vesting schedules and amounts.