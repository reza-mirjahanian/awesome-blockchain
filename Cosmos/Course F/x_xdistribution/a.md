# Cosmos SDK x/distribution Module Guide

## Core Components

### 1. State Objects
```go
// Fee Pool
type FeePool struct {
    CommunityPool sdk.DecCoins
}

// Validator Distribution Info
type ValidatorDistInfo struct {
    OperatorAddress     string
    SelfBondRewards    sdk.DecCoins
    ValidatorCommission sdk.DecCoins
}

// Delegator Distribution Info
type DelegatorDistInfo struct {
    Rewards sdk.DecCoins
}
```

## Parameters

### 1. Distribution Parameters
```go
type Params struct {
    CommunityTax        sdk.Dec
    BaseProposerReward  sdk.Dec
    BonusProposerReward sdk.Dec
    WithdrawAddrEnabled bool
}
```

## Key Functions

### 1. Reward Distribution
```go
// Distribute rewards to validators
func (k Keeper) AllocateTokens(
    ctx sdk.Context,
    totalPreviousPower int64,
    bondedVotes []abci.VoteInfo,
)

// Withdraw rewards
func (k Keeper) WithdrawDelegatorReward(
    ctx sdk.Context,
    delAddr sdk.AccAddress,
    valAddr sdk.ValAddress,
)
```

### 2. Commission Management
```go
// Set validator commission
func (k Keeper) SetValidatorCommission(
    ctx sdk.Context,
    valAddr sdk.ValAddress,
    commission sdk.DecCoins,
)

// Withdraw validator commission
func (k Keeper) WithdrawValidatorCommission(
    ctx sdk.Context,
    valAddr sdk.ValAddress,
)
```

## Distribution Flow

### 1. Block Rewards
1. **Collection**
   - Transaction fees
   - Inflation rewards
   - Other rewards

2. **Distribution**
   - Proposer reward
   - Validator commission
   - Delegator rewards
   - Community pool

### 2. Reward Calculation
```go
ProposerReward = BasePropserReward + (BonusProposerReward * PrecommitPower)
ValidatorReward = (1 - CommunityTax - ProposerReward) * ValidatorPower/TotalPower
DelegatorReward = ValidatorReward * DelegatorShares/TotalShares
```

## Query Functions

### 1. Common Queries
```go
// Query params
QueryParams(ctx sdk.Context)

// Query community pool
QueryCommunityPool(ctx sdk.Context)

// Query validator rewards
QueryValidatorOutstandingRewards(ctx sdk.Context, valAddr sdk.ValAddress)

// Query delegator rewards
QueryDelegationRewards(ctx sdk.Context, delAddr sdk.AccAddress, valAddr sdk.ValAddress)
```

## CLI Commands

```bash
# Query distribution params
simd query distribution params

# Query community pool
simd query distribution community-pool

# Withdraw rewards
simd tx distribution withdraw-rewards [validator-addr] --from [delegator-addr]

# Withdraw all rewards
simd tx distribution withdraw-all-rewards --from [delegator-addr]

# Withdraw validator commission
simd tx distribution withdraw-validator-commission [validator-addr]
```

## Events

### 1. Event Types
```go
const (
    EventTypeSetWithdrawAddress     = "set_withdraw_address"
    EventTypeRewards               = "rewards"
    EventTypeCommission            = "commission"
    EventTypeWithdrawRewards      = "withdraw_rewards"
    EventTypeWithdrawCommission   = "withdraw_commission"
    EventTypeProposerReward       = "proposer_reward"
)
```

### 2. Event Attributes
```go
const (
    AttributeKeyWithdrawAddress = "withdraw_address"
    AttributeKeyValidator       = "validator"
    AttributeKeyDelegator      = "delegator"
    AttributeKeyAmount         = "amount"
)
```

## Error Handling

```go
var (
    ErrEmptyDelegatorAddr        = sdkerrors.Register(ModuleName, 1, "delegator address is empty")
    ErrEmptyWithdrawAddr         = sdkerrors.Register(ModuleName, 2, "withdraw address is empty")
    ErrEmptyValidatorAddr        = sdkerrors.Register(ModuleName, 3, "validator address is empty")
    ErrNoValidatorCommission     = sdkerrors.Register(ModuleName, 4, "no validator commission")
    ErrNoValidatorRewards        = sdkerrors.Register(ModuleName, 5, "no validator rewards")
)
```

## Genesis State

```go
type GenesisState struct {
    Params                          Params
    FeePool                         FeePool
    DelegatorWithdrawInfos         []DelegatorWithdrawInfo
    PreviousProposer               sdk.ConsAddress
    OutstandingRewards             []ValidatorOutstandingRewardsRecord
    ValidatorAccumulatedCommissions []ValidatorAccumulatedCommissionRecord
    ValidatorHistoricalRewards     []ValidatorHistoricalRewardsRecord
    ValidatorCurrentRewards        []ValidatorCurrentRewardsRecord
    DelegatorStartingInfos         []DelegatorStartingInfoRecord
    ValidatorSlashEvents           []ValidatorSlashEventRecord
}
```

## Best Practices

### 1. Reward Management
- Regular withdrawal of rewards
- Monitor commission rates
- Track reward distribution

### 2. Security Considerations
- Validate withdrawal addresses
- Check reward amounts
- Monitor community pool

### 3. Performance Optimization
- Batch withdrawals
- Regular commission collection
- Efficient query usage

### 4. Monitoring
- Track reward distribution
- Monitor commission rates
- Alert on large withdrawals

## Integration Points

### 1. With Other Modules
- **Staking**: Validator and delegation management
- **Bank**: Token transfers
- **Gov**: Parameter updates
- **Auth**: Account management

### 2. Hooks
```go
type DistributionHooks interface {
    AfterValidatorCreated(ctx sdk.Context, valAddr sdk.ValAddress)
    BeforeValidatorModified(ctx sdk.Context, valAddr sdk.ValAddress)
    AfterValidatorRemoved(ctx sdk.Context, valAddr sdk.ValAddress)
    AfterValidatorBonded(ctx sdk.Context, valAddr sdk.ValAddress)
    AfterValidatorBeginUnbonding(ctx sdk.Context, valAddr sdk.ValAddress)
}
```