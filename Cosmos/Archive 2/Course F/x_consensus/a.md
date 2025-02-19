# Cosmos SDK x/consensus Module Guide

## Core Components

### 1. Parameters
- **MaxExpectedTimePerBlock**: Duration between blocks
- **ValidatorMissedBlockBurst**: Max consecutive missed blocks
- **ValidatorMissedBlockWindow**: Window for tracking missed blocks

### 2. Key Functions

```go
// Set consensus params
SetConsensusParams(ctx sdk.Context, params *tmproto.ConsensusParams)

// Get consensus params
GetConsensusParams(ctx sdk.Context) *tmproto.ConsensusParams

// Update consensus params
UpdateConsensusParams(ctx sdk.Context, params *tmproto.ConsensusParams)
```

## Parameter Management

### 1. Block Parameters
```go
BlockParams {
    MaxBytes    int64
    MaxGas      int64
    TimeIotaMs  int64
}
```

### 2. Evidence Parameters
```go
EvidenceParams {
    MaxAgeNumBlocks    int64
    MaxAgeDuration     time.Duration
    MaxBytes          int64
}
```

### 3. Validator Parameters
```go
ValidatorParams {
    PubKeyTypes []string
}
```

## Key Features

### 1. Parameter Updates
- **Governance-based**: Changes via governance proposals
- **Immediate effect**: Updates apply after proposal passes
- **Safety checks**: Validates parameter ranges

### 2. Query Functions
```go
// Query current params
QueryParams(ctx sdk.Context) (params types.QueryParamsResponse)

// Query specific param subset
QuerySubspace(ctx sdk.Context, key []byte) []types.ParamChange
```

## Common Operations

### 1. Parameter Modification
```go
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
    k.paramstore.SetParamSet(ctx, &params)
}
```

### 2. Parameter Validation
```go
func ValidateParams(params types.Params) error {
    if params.MaxBytes <= 0 {
        return fmt.Errorf("max bytes must be positive: %d", params.MaxBytes)
    }
    return nil
}
```

## Integration Points

### 1. With Other Modules
- **Staking**: Validator set management
- **Governance**: Parameter change proposals
- **Evidence**: Evidence handling parameters

### 2. ABCI Integration
```go
func BeginBlocker(ctx sdk.Context, req abci.RequestBeginBlock) {
    // Update consensus params if needed
}
```

## Security Considerations

### 1. Parameter Bounds
- **MaxBytes**: Must be positive
- **MaxGas**: Must be positive
- **TimeIotaMs**: Minimum 1ms

### 2. Access Control
- Only governance can modify parameters
- Keeper methods require proper authorization

## Error Handling

```go
var (
    ErrInvalidConsensusParams = sdkerrors.Register(ModuleName, 1, "invalid consensus params")
    ErrInvalidMaxGas         = sdkerrors.Register(ModuleName, 2, "invalid max gas")
)
```

## Events

### 1. Parameter Change
```go
EventTypeParamChange = "param_change"
AttributeKeyParamName = "param_name"
AttributeKeyOldValue = "old_value"
AttributeKeyNewValue = "new_value"
```

## CLI Commands

```bash
# Query current params
simd query consensus params

# Submit param change proposal
simd tx gov submit-proposal param-change proposal.json
```

## Genesis Configuration

```json
{
  "consensus_params": {
    "block": {
      "max_bytes": "22020096",
      "max_gas": "-1"
    },
    "evidence": {
      "max_age_num_blocks": "100000",
      "max_age_duration": "172800000000000"
    },
    "validator": {
      "pub_key_types": [
        "ed25519"
      ]
    }
  }
}
```

## Best Practices

1. **Parameter Updates**
- Test changes on testnet first
- Use gradual parameter adjustments
- Document all parameter changes

2. **Monitoring**
- Track parameter change events
- Monitor validator performance
- Alert on unusual parameter values

3. **Maintenance**
- Regular parameter review
- Backup parameter values
- Document parameter rationale