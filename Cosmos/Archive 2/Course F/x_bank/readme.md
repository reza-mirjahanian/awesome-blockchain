**`x/bank` Module in Cosmos SDK**

The `x/bank` module is a core module in the Cosmos SDK that handles the movement and management of coins (tokens) within the blockchain. It provides functionalities for transferring coins between accounts, querying balances, and managing supply.

---

## **Key Concepts**

### **Coins and Denominations**

- **Coin**: A struct representing a token with a denomination (`Denom`) and an amount (`Amount`).
  
  ```go
  type Coin struct {
      Denom  string
      Amount Int
  }
  ```
  
- **Denomination**: A unique identifier for a type of token (e.g., `"atom"`, `"uatom"`).

- **Coins**: A collection (slice) of `Coin`.

  ```go
  type Coins []Coin
  ```

**Tip**: Use `sdk.NewCoin` and `sdk.NewCoins` to create instances safely.

---

## **Core Components**

### **Keeper**

- The `Keeper` is the core component that provides APIs for interacting with the `x/bank` module.

- **Key Functions**:
  - `SendCoins`
  - `InputOutputCoins`
  - `GetBalance`
  - `SetBalance`
  - `MintCoins`
  - `BurnCoins`
  - `GetSupply`

**Example**:

```go
func NewKeeper(
    cdc codec.BinaryCodec,
    key sdk.StoreKey,
    ak types.AccountKeeper,
    blockedAddrs map[string]bool,
) Keeper
```

**Tip**: When creating a custom module, you can inject the `bank.Keeper` to interact with balances.

---

### **Messages**

- **MsgSend**: Transfers coins from one account to another.

  ```go
  type MsgSend struct {
      FromAddress string
      ToAddress   string
      Amount      sdk.Coins
  }
  ```

- **MsgMultiSend**: Allows batch transfers in a single transaction.

  ```go
  type MsgMultiSend struct {
      Inputs  []Input
      Outputs []Output
  }
  ```

**Tip**: Use `MsgMultiSend` to optimize gas fees when sending coins to multiple recipients.

---

### **Input and Output**

- **Input**: Defines an address and the coins to be deducted.

  ```go
  type Input struct {
      Address string
      Coins   sdk.Coins
  }
  ```

- **Output**: Defines an address and the coins to be added.

  ```go
  type Output struct {
      Address string
      Coins   sdk.Coins
  }
  ```

**Tip**: Ensure that the total input amount equals the total output amount in `MsgMultiSend`.

---

## **Supply Management**

### **Total Supply**

- **GetTotalSupply**: Retrieves the total supply of all coins in the network.

  ```go
  func (k Keeper) GetSupply(ctx sdk.Context) exported.SupplyI
  ```

- **GetDenomSupply**: Retrieves the supply of a specific denomination.

  ```go
  func (k BaseKeeper) GetSupply(ctx sdk.Context, denom string) sdk.Coin
  ```

**Tip**: Use supply queries to monitor inflation or deflation in your network.

---

### **Minting and Burning Coins**

- **MintCoins**: Increases the supply of a specific module account.

  ```go
  func (k Keeper) MintCoins(ctx sdk.Context, moduleName string, amt sdk.Coins) error
  ```

- **BurnCoins**: Decreases the supply from a module account.

  ```go
  func (k Keeper) BurnCoins(ctx sdk.Context, moduleName string, amt sdk.Coins) error
  ```

**Usage Example**:

```go
// Minting coins to 'mint' module account
err := bankKeeper.MintCoins(ctx, types.MinterModuleName, sdk.NewCoins(newCoin))

// Burning coins from 'distribution' module account
err := bankKeeper.BurnCoins(ctx, types.ModuleName, sdk.NewCoins(burnCoin))
```

**Tip**: Only module accounts can mint or burn coins. Ensure the module has the necessary permissions.

---

## **Blocked Addresses**

- **Purpose**: Prevent certain addresses from receiving tokens (e.g., module accounts not meant to hold external funds).

- **Setup**:

  ```go
  blockedAddrs := make(map[string]bool)
  blockedAddrs[authtypes.NewModuleAddress("distribution").String()] = true
  ```

**Tip**: Always block module accounts that should not handle user funds to prevent accidental transfers.

---

## **Module Accounts and Permissions**

- **Module Accounts**: Special accounts used by modules to hold and manage coins.

- **Permissions**:

  - **Minter**: Can mint new coins.

  - **Burner**: Can burn coins.

  - **Staking**: Used for staking operations.

  - **Example**:

    ```go
    func NewModuleAccount(moduleName string, perms ...string) *ModuleAccount
    ```

**Tip**: Define appropriate permissions to restrict module account capabilities and enhance security.

---

## **Genesis State**

- Defines the initial state of the `x/bank` module at chain genesis.

- **Structure**:

  ```go
  type GenesisState struct {
      Params   Params
      Balances []Balance
      Supply   sdk.Coins
      DenomMetadata []Metadata
  }
  ```

- **Balance**:

  ```go
  type Balance struct {
      Address string
      Coins   sdk.Coins
  }
  ```

**Tip**: Ensure that the sum of all account balances matches the total supply.

---

## **Parameters (`Params`)**

- **Key Parameters**:

  - `SendEnabled`: Controls whether sending is enabled for specific denominations.

  - **Example**:

    ```go
    type Params struct {
        SendEnabled []*SendEnabled
        DefaultSendEnabled bool
    }
    ```

- **SendEnabled**:

  ```go
  type SendEnabled struct {
      Denom   string
      Enabled bool
  }
  ```

**Tip**: Use `SendEnabled` to freeze specific tokens or control their transferability.

---

## **Metadata**

- **Purpose**: Provides additional information about a token denomination, such as display units and descriptions.

- **Structure**:

  ```go
  type Metadata struct {
      Description string
      DenomUnits  []*DenomUnit
      Base        string
      Display     string
  }
  ```

- **DenomUnit**:

  ```go
  type DenomUnit struct {
      Denom   string
      Exponent uint32
      Aliases  []string
  }
  ```

**Usage Example**:

```go
metadata := banktypes.Metadata{
    Description: "The native staking token of the Cosmos Hub.",
    Base:        "uatom",
    Display:     "atom",
    DenomUnits: []*banktypes.DenomUnit{
        {
            Denom:    "uatom",
            Exponent: 0,
            Aliases:  []string{"microatom"},
        },
        {
            Denom:    "milliatom",
            Exponent: 3,
            Aliases:  []string{"milliatom"},
        },
        {
            Denom:    "atom",
            Exponent: 6,
            Aliases:  []string{},
        },
    },
}
```

**Tip**: Define metadata for your tokens to improve client interactions and display.

---

## **Queries**

### **Query Balance**

- **Purpose**: Get the balance of an account for a specific denomination.

- **Usage**:

  ```go
  res, err := queryClient.Balance(
      context.Background(),
      &banktypes.QueryBalanceRequest{
          Address: address,
          Denom:   denom,
      },
  )
  ```

### **Query All Balances**

- **Purpose**: Get all balances for an account.

- **Usage**:

  ```go
  res, err := queryClient.AllBalances(
      context.Background(),
      &banktypes.QueryAllBalancesRequest{
          Address:    address,
          Pagination: &query.PageRequest{Limit: 100},
      },
  )
  ```

**Tip**: Use pagination when querying accounts with many denominations.

---

### **Query Total Supply**

- **Purpose**: Retrieve the total supply of all denominations.

- **Usage**:

  ```go
  res, err := queryClient.TotalSupply(
      context.Background(),
      &banktypes.QueryTotalSupplyRequest{
          Pagination: &query.PageRequest{Limit: 100},
      },
  )
  ```

### **Query Supply of Specific Denomination**

- **Purpose**: Get the total supply for a specific denomination.

- **Usage**:

  ```go
  res, err := queryClient.SupplyOf(
      context.Background(),
      &banktypes.QuerySupplyOfRequest{
          Denom: denom,
      },
  )
  ```

**Tip**: Regularly monitor the total supply for economic analysis and auditing.

---

### **Query Denomination Metadata**

- **Purpose**: Retrieve metadata for denominations.

- **Usage**:

  ```go
  res, err := queryClient.DenomMetadata(
      context.Background(),
      &banktypes.QueryDenomMetadataRequest{
          Denom: denom,
      },
  )
  ```

---

## **Best Practices**

### **Input Validation**

- **Verify Addresses**: Always validate addresses before processing transactions.

  ```go
  if _, err := sdk.AccAddressFromBech32(address); err != nil {
      // Handle error
  }
  ```

### **Error Handling**

- **Handle Insufficient Funds**: Check account balances before attempting to send coins.

  ```go
  balance := bankKeeper.GetBalance(ctx, addr, denom)
  if balance.IsLT(amount) {
      // Handle insufficient funds
  }
  ```

**Tip**: Provide clear error messages to end-users on transaction failures.

---

### **Security Considerations**

- **Prevent Unauthorized Minting/Burning**: Only allow authorized modules to mint or burn coins.

- **Example**:

  ```go
  func (k Keeper) MintCoins(ctx sdk.Context, moduleName string, amt sdk.Coins) error {
      if !k.modulesPermissions[moduleName].HasPermission(types.Minter) {
          return errors.Wrapf(types.ErrUnauthorized, "module %s cannot mint coins", moduleName)
      }
      // Proceed with minting
  }
  ```

### **Optimizations**

- **Batch Transactions**: Use `MsgMultiSend` to reduce gas costs when sending to multiple recipients.

- **Keep Denominations Consistent**: Limit the number of different denominations to reduce state bloat and simplify client interfaces.

---

## **Transaction Examples**

### **Single Send Transaction**

```go
msg := &banktypes.MsgSend{
    FromAddress: fromAddr.String(),
    ToAddress:   toAddr.String(),
    Amount:      sdk.NewCoins(sdk.NewCoin("uatom", sdk.NewInt(1000))),
}

// Sign and broadcast the transaction
```

### **MultiSend Transaction**

```go
inputs := []banktypes.Input{
    {
        Address: fromAddr.String(),
        Coins:   sdk.NewCoins(sdk.NewCoin("uatom", sdk.NewInt(3000))),
    },
}

outputs := []banktypes.Output{
    {
        Address: toAddr1.String(),
        Coins:   sdk.NewCoins(sdk.NewCoin("uatom", sdk.NewInt(1000))),
    },
    {
        Address: toAddr2.String(),
        Coins:   sdk.NewCoins(sdk.NewCoin("uatom", sdk.NewInt(2000))),
    },
}

msg := &banktypes.MsgMultiSend{
    Inputs:  inputs,
    Outputs: outputs,
}

// Sign and broadcast the transaction
```

**Tip**: Ensure that the total input amount equals the total output amount to avoid transaction rejection.

---

## **CLI Usage Examples**

### **Sending Coins**

```bash
cosmoscli tx bank send [from_address] [to_address] [amount] [flags]
```

**Example**:

```bash
cosmoscli tx bank send cosmos1... cosmos1... 1000uatom --from mykey
```

### **Querying Balance**

```bash
cosmoscli query bank balances [address] [flags]
```

**Example**:

```bash
cosmoscli query bank balances cosmos1...
```

---

## **Integration with Other Modules**

- **Auth Module (`x/auth`)**: User accounts and balances are linked, so proper handling of accounts is essential.

- **Distribution Module (`x/distribution`)**: Manages rewards and commissions, interacting with bank to transfer coins.

- **Staking Module (`x/staking`)**: Handles delegation and undelegation, which affect balances.

**Tip**: Understand how `x/bank` interacts with other modules to avoid unintended side effects.

---

## **Tips and Tricks**

### **Custom Denominations**

- **Creating Custom Tokens**: Define new denominations by simply using a unique `Denom` string.

- **Avoid Conflicts**: Ensure that custom denominations do not conflict with existing ones.

### **Testing Transfers**

- **Use Simulations**: Test transactions in a simulated environment using the SDK's simulation tools.

- **Mock Keepers**: Implement mock keepers for unit testing module logic without dependencies.

---

### **Handling Decimals**

- **Use Exponents**: Represent token subdivisions using the `Exponent` in `DenomUnit`.

- **Client Display**: Convert base units to display units for user interfaces.

**Example**:

- 1 `atom` = 1,000,000 `uatom`

---

### **Implementing Custom Logic**

- **Custom Messages**: Create new message types that interact with `x/bank` for specialized functionality.

- **Middleware**: Implement middleware to intercept and modify bank transactions if necessary.

---

## **Advanced Topics**

### **Fee Grants**

- **Module**: `x/feegrant`

- **Purpose**: Allows accounts to pay fees on behalf of others.

- **Example**:

  ```go
  // Grant an allowance
  allowance := &feegrant.BasicAllowance{
      SpendLimit: sdk.NewCoins(sdk.NewCoin("uatom", sdk.NewInt(1000))),
  }
  err := feegrantKeeper.GrantAllowance(ctx, granterAddr, granteeAddr, allowance)
  ```

**Tip**: Combine `x/bank` with `x/feegrant` to enable sponsored transactions.

---

### **Vesting Accounts**

- **Context**: Vesting accounts hold coins that are locked and unlocked over time.

- **Impact on Bank Transfers**: Attempting to send locked coins will result in an error.

- **Example**:

  ```go
  // Check if account has sufficient spendable balance
  spendableCoins := bankKeeper.SpendableCoins(ctx, addr)
  ```

**Tip**: Always consider vesting schedules when calculating available balances.

---

### **IBC Transfers**

- **Module**: `ibc-transfer`

- **Purpose**: Allows transferring tokens across different blockchains.

- **Interaction with Bank**: Tokens received via IBC are tracked in `x/bank` with prefixed denominations (e.g., `ibc/...`).

---

## **Common Pitfalls**

### **Insufficient Fees**

- **Issue**: Transactions fail if not enough fees are provided.

- **Solution**: Estimate gas and set appropriate fees before broadcasting.

### **Unauthorized Minting/Burning**

- **Issue**: Unauthorized attempts to mint or burn coins.

- **Solution**: Ensure only authorized module accounts with correct permissions perform these actions.

### **Invalid Denominations**

- **Issue**: Using invalid or improperly formatted denominations.

- **Solution**: Adhere to denomination validation rules (lowercase letters, digits, '/', '.', and '-'; length 2-128).

---



---

**Note**: Always refer to the latest Cosmos SDK documentation and codebase for updates and detailed explanations.