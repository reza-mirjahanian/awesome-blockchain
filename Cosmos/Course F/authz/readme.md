**Authorization (`x/authz`) Module in Cosmos SDK**

The `x/authz` module in the Cosmos SDK enables granting authorizations to perform actions on behalf of another account. It allows accounts to delegate permissions to other accounts to execute certain transactions, enhancing flexibility and security in account management.

---

## **Overview**

- **Purpose**: To delegate permissions from a granter to a grantee.
- **Benefits**:
  - Allows for account delegation without sharing private keys.
  - Granular control over which actions can be performed.
  - Time-bound and limited usage permissions.

---

## **Key Concepts**

### **Granter and Grantee**

- **Granter**: The account that grants permissions to another account.
- **Grantee**: The account that receives permissions to act on behalf of the granter.

### **Authorization Types**

- **Generic Authorization**: Allows execution of any message type specified.
- **Send Authorization**: Specific to sending coins (`MsgSend`).
- **Stake Authorization**: Controls staking operations like delegate, undelegate, and redelegate.

---

## **Authorization Types and Structures**

### **Generic Authorization**

- **Purpose**: Grant permission to execute any message type.
- **Structure**:

  ```go
  type GenericAuthorization struct {
      Msg string
  }
  ```

- **Usage**:

  ```go
  auth := authz.NewGenericAuthorization(sdk.MsgTypeURL(&banktypes.MsgSend{}))
  ```

### **Send Authorization**

- **Purpose**: Grant permission to send coins.
- **Structure**:

  ```go
  type SendAuthorization struct {
      SpendLimit sdk.Coins
  }
  ```

- **Usage**:

  ```go
  spendLimit := sdk.NewCoins(sdk.NewCoin("atom", sdk.NewInt(1000)))
  auth := banktypes.NewSendAuthorization(spendLimit)
  ```

### **Stake Authorization**

- **Purpose**: Grant permission for staking operations.
- **Structure**:

  ```go
  type StakeAuthorization struct {
      MaxTokens       *sdk.Coin
      AllowList       *StakeAuthorization_Validators
      DenyList        *StakeAuthorization_Validators
      AuthorizationType StakeAuthorization_AuthorizationType
  }
  ```

- **Authorization Types**:
  - `Delegate`
  - `Undelegate`
  - `Redelegate`

- **Usage**:

  ```go
  validators := &authzTypes.StakeAuthorization{
      AllowList: &authzTypes.StakeAuthorization_Validators{Validators: []string{validatorAddr}},
      MaxTokens: sdk.NewCoin("atom", sdk.NewInt(1000)),
      AuthorizationType: authzTypes.StakeAuthorization_DELEGATE,
  }
  ```

---

## **Granting an Authorization**

- **Message**: `MsgGrant`

- **Fields**:
  - `Granter`: The address granting the authorization.
  - `Grantee`: The address receiving the authorization.
  - `Grant`: Contains the `Authorization` and `Expiration`.

- **Example**:

  ```go
  msg := &authz.MsgGrant{
      Granter: granterAddr.String(),
      Grantee: granteeAddr.String(),
      Grant: authz.Grant{
          Authorization: auth,
          Expiration:    &expirationTime,
      },
  }
  ```

---

## **Revoking an Authorization**

- **Message**: `MsgRevoke`

- **Fields**:
  - `Granter`: The address that granted the authorization.
  - `Grantee`: The address that received the authorization.
  - `MsgTypeUrl`: The type URL of the message being revoked.

- **Example**:

  ```go
  msg := &authz.MsgRevoke{
      Granter:    granterAddr.String(),
      Grantee:    granteeAddr.String(),
      MsgTypeUrl: sdk.MsgTypeURL(&banktypes.MsgSend{}),
  }
  ```

---

## **Executing an Authorization**

- **Message**: `MsgExec`

- **Fields**:
  - `Grantee`: The address executing the authorization.
  - `Msgs`: The messages to be executed under the granted authorization.

- **Example**:

  ```go
  sendMsg := &banktypes.MsgSend{
      FromAddress: granterAddr.String(),
      ToAddress:   recipientAddr.String(),
      Amount:      sdk.NewCoins(sdk.NewCoin("atom", sdk.NewInt(500))),
  }

  msg := &authz.MsgExec{
      Grantee: granteeAddr.String(),
      Msgs:    []sdk.Msg{sendMsg},
  }
  ```

---

## **Working with Expirations**

- **Setting Expiration**:

  ```go
  expiration := time.Now().Add(24 * time.Hour)
  ```

- **Usage in Grant**:

  ```go
  grant := authz.Grant{
      Authorization: auth,
      Expiration:    &expiration,
  }
  ```

- **Note**: If `Expiration` is not set, authorization will not expire unless explicitly revoked.

---

## **Queries**

### **Query Grants**

- **Purpose**: Retrieve all grants between a granter and grantee.

- **Usage**:

  ```go
  res, err := queryClient.Grants(
      context.Background(),
      &authz.QueryGrantsRequest{
          Granter: granterAddr.String(),
          Grantee: granteeAddr.String(),
      },
  )
  ```

### **Query Granter Grants**

- **Purpose**: Retrieve all grants from a specific granter.

- **Usage**:

  ```go
  res, err := queryClient.GranterGrants(
      context.Background(),
      &authz.QueryGranterGrantsRequest{
          Granter: granterAddr.String(),
      },
  )
  ```

### **Query Grantee Grants**

- **Purpose**: Retrieve all grants for a specific grantee.

- **Usage**:

  ```go
  res, err := queryClient.GranteeGrants(
      context.Background(),
      &authz.QueryGranteeGrantsRequest{
          Grantee: granteeAddr.String(),
      },
  )
  ```

---

## **Use Cases**

### **Delegating Transaction Capabilities**

- **Scenario**: An organization wants to allow an operator to send transactions on its behalf without sharing the organization's private key.

- **Solution**: The organization (granter) grants a `GenericAuthorization` or `SendAuthorization` to the operator (grantee).

### **Staking Operations**

- **Scenario**: Delegate staking responsibilities to a third-party service.

- **Solution**: Use `StakeAuthorization` to allow the grantee to perform staking operations like delegation or undelegation within specified limits.

### **Time-Limited Permissions**

- **Scenario**: Provide temporary access to an account's capabilities.

- **Solution**: Set the `Expiration` field in the grant to define how long the authorization lasts.

---

## **Best Practices**

### **Granular Permissions**

- **Use Specific Authorizations**: Prefer `SendAuthorization` or `StakeAuthorization` over `GenericAuthorization` for more control.

### **Limit Spend Amounts**

- **Set `SpendLimit`**: Define maximum amounts in `SendAuthorization` and `StakeAuthorization` to prevent misuse.

  ```go
  spendLimit := sdk.NewCoins(sdk.NewCoin("atom", sdk.NewInt(1000)))
  ```

### **Validator Allow/Deny Lists**

- **Control Validator Access**: In `StakeAuthorization`, use `AllowList` or `DenyList` to specify which validators can be interacted with.

  ```go
  allowList := &authzTypes.StakeAuthorization_Validators{
      Validators: []string{validatorAddr1, validatorAddr2},
  }
  ```

### **Expiration Dates**

- **Set Appropriate Expirations**: Always set an `Expiration` date to limit the duration of the authorization.

### **Monitoring and Revocation**

- **Regularly Review Grants**: Use query methods to monitor active grants.
- **Revoke Unnecessary Grants**: Use `MsgRevoke` to remove outdated or unnecessary authorizations.

---

## **Tips and Tricks**

### **Combining Multiple Authorizations**

- **Execute Multiple Messages**: Include various messages in `MsgExec` as long as there are corresponding authorizations.

  ```go
  msgExec := &authz.MsgExec{
      Grantee: granteeAddr.String(),
      Msgs:    []sdk.Msg{msg1, msg2, msg3},
  }
  ```

### **Chain-Specific Message Types**

- **Use Correct `MsgTypeURL`**: Ensure the `MsgTypeUrl` in authorizations matches the message types used in the chain.

  ```go
  sdk.MsgTypeURL(&banktypes.MsgSend{})
  ```

### **Testing Authorizations**

- **Simulate Transactions**: Before executing, simulate transactions to ensure that the authorizations are correctly set up.

  ```go
  res, err := clientCtx.Simulate(txBytes)
  ```

### **Error Handling**

- **Check for Common Errors**:
  - **Unauthorized**: Occurs when the grantee lacks the necessary authorization.
  - **Expired Authorization**: Happens if the authorization has passed its expiration date.
  - **Exceeded Spend Limit**: Triggered when attempting to spend more than allowed.

### **Custom Authorizations**

- **Create Custom Authorization Types**: Implement `Authorization` interface for specific use cases.

  ```go
  type MyCustomAuthorization struct {
      // Custom fields
  }

  func (a *MyCustomAuthorization) MsgTypeURL() string         { /* implementation */ }
  func (a *MyCustomAuthorization) Accept(...) (AcceptResponse, error) { /* implementation */ }
  func (a *MyCustomAuthorization) ValidateBasic() error       { /* implementation */ }
  ```

---

## **Implementation Steps**

### **1. Define the Authorization**

- Choose the appropriate authorization type.
- Set the necessary limits and parameters.

### **2. Grant the Authorization**

- The granter creates and signs a `MsgGrant` transaction.
- Broadcast the transaction to the network.

### **3. Execute Authorized Transactions**

- The grantee creates a `MsgExec` containing the messages to execute.
- The grantee signs and broadcasts the transaction.

### **4. Revocation (Optional)**

- The granter can revoke the authorization at any time using `MsgRevoke`.

---

## **Code Examples**

### **Granting a Send Authorization**

```go
// Define the spend limit
spendLimit := sdk.NewCoins(sdk.NewCoin("atom", sdk.NewInt(1000)))

// Create the SendAuthorization
auth := banktypes.NewSendAuthorization(spendLimit)

// Set expiration time
expiration := time.Now().Add(7 * 24 * time.Hour)

// Create the MsgGrant message
msg := &authz.MsgGrant{
    Granter: granterAddr.String(),
    Grantee: granteeAddr.String(),
    Grant: authz.Grant{
        Authorization: auth,
        Expiration:    &expiration,
    },
}

// Sign and broadcast the transaction
```

### **Executing a Send Authorization**

```go
// Create the MsgSend message
msgSend := &banktypes.MsgSend{
    FromAddress: granterAddr.String(),
    ToAddress:   recipientAddr.String(),
    Amount:      sdk.NewCoins(sdk.NewCoin("atom", sdk.NewInt(500))),
}

// Create the MsgExec message
msgExec := &authz.MsgExec{
    Grantee: granteeAddr.String(),
    Msgs:    []sdk.Msg{msgSend},
}

// Sign and broadcast the transaction as the grantee
```

### **Revoking an Authorization**

```go
msgRevoke := &authz.MsgRevoke{
    Granter:    granterAddr.String(),
    Grantee:    granteeAddr.String(),
    MsgTypeUrl: sdk.MsgTypeURL(&banktypes.MsgSend{}),
}

// Granter signs and broadcasts the transaction
```

---

## **Custom Authorization Example**

### **Defining a Custom Authorization**

```go
type CustomAuthorization struct {
    CustomLimit int64
}

// Implement the Authorization interface
func (a *CustomAuthorization) MsgTypeURL() string {
    return sdk.MsgTypeURL(&customtypes.MsgCustom{})
}

func (a *CustomAuthorization) Accept(ctx context.Context, msg sdk.Msg) (authz.AcceptResponse, error) {
    // Custom logic to accept or reject the message
}

func (a *CustomAuthorization) ValidateBasic() error {
    // Basic validation logic
}
```

### **Using the Custom Authorization**

- **Grant**:

  ```go
  auth := &CustomAuthorization{CustomLimit: 10}
  // Proceed to grant as usual
  ```

- **Execute**:

  ```go
  msgCustom := &customtypes.MsgCustom{
      // Custom message fields
  }

  msgExec := &authz.MsgExec{
      Grantee: granteeAddr.String(),
      Msgs:    []sdk.Msg{msgCustom},
  }

  // Grantee signs and broadcasts the transaction
  ```

---

## **Understanding AcceptResponse**

When implementing custom authorizations or analyzing the execution flow, the `AcceptResponse` struct is crucial.

### **AcceptResponse Fields**

- `Accept`: Boolean indicating if the authorization permits the message.
- `Delete`: Boolean indicating if the authorization should be deleted after execution.
- `Updated`: (Optional) Updated authorization with new limits or parameters.

### **Example Usage in `Accept` Method**

```go
func (a *CustomAuthorization) Accept(ctx context.Context, msg sdk.Msg) (authz.AcceptResponse, error) {
    // Check if msg meets criteria
    if meetsCriteria {
        // Update or consume limits if necessary
        a.CustomLimit -= 1

        return authz.AcceptResponse{
            Accept:  true,
            Delete:  a.CustomLimit == 0,
            Updated: a,
        }, nil
    }

    return authz.AcceptResponse{
        Accept: false,
    }, nil
}
```

---

## **Integration with Other Modules**

- **Bank Module**: Use `SendAuthorization` to control sending coins.
- **Staking Module**: `StakeAuthorization` integrates with staking messages.
- **Custom Modules**: Implement custom authorizations for module-specific messages.

---

## **Client and CLI Usage**

### **Granting Authorization via CLI**

```bash
cosmoscli tx authz grant [grantee] [authorization_type] [flags]
```

- **Example**:

  ```bash
  cosmoscli tx authz grant cosmos1grantee ... --msg-type /cosmos.bank.v1beta1.MsgSend --spend-limit 1000atom
  ```

### **Executing Authorization via CLI**

```bash
cosmoscli tx authz exec [file_with_msgs] --from [grantee] [flags]
```

- **Example**:

  ```bash
  cosmoscli tx authz exec tx.json --from cosmos1grantee
  ```

### **Revoking Authorization via CLI**

```bash
cosmoscli tx authz revoke [grantee] [msg_type] --from [granter] [flags]
```

- **Example**:

  ```bash
  cosmoscli tx authz revoke cosmos1grantee /cosmos.bank.v1beta1.MsgSend --from cosmos1granter
  ```

---

## **Advanced Topics**

### **Pagination in Queries**

- **Handling Large Result Sets**: Use `pagination` parameters in query requests.

- **Example**:

  ```go
  res, err := queryClient.GranterGrants(
      context.Background(),
      &authz.QueryGranterGrantsRequest{
          Granter: granterAddr.String(),
          Pagination: &query.PageRequest{
              Limit:  10,
              Offset: 0,
          },
      },
  )
  ```

### **Authorization Encoding and Decoding**

- **Serialization**: Authorizations are serialized using protocol buffers.

- **Unpacking Any Types**:

  ```go
  var auth authz.Authorization
  err := anyAuthorization.GetCachedValue().(authz.Authorization)
  ```

### **Expiring Grants**

- **Automatic Expiration**: Expired grants are not automatically removed.

- **Cleanup Process**: Implement a periodic cleanup or rely on the execution flow to handle expired grants.

---

## **Common Pitfalls**

### **Mismatched `MsgTypeURL`**

- **Issue**: Using incorrect `MsgTypeURL` leads to authorization failures.

- **Resolution**: Always verify that the `MsgTypeURL` matches the message being authorized.

### **Exceeding Spend Limits**

- **Issue**: Grantee attempts to spend more than the allowed limit.

- **Resolution**: Ensure the transaction amount is within the authorized spend limit.

### **Unbounded Authorizations**

- **Risk**: Granting authorizations without limits or expiration can lead to misuse.

- **Mitigation**: Always define `SpendLimit` and `Expiration`.

---

## **Performance Considerations**

### **State Management**

- **Impact**: Each grant adds to the state size.

- **Best Practices**:

  - Revoke unnecessary grants.
  - Set appropriate expirations.
  - Limit grants to essential permissions.

### **Transaction Overhead**

- **MsgExec Overhead**: Executing via `MsgExec` adds extra computational steps.

- **Optimization**: Bundle multiple messages into a single `MsgExec` when possible.

---

