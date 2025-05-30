# Table of Contents

1. [Write the basic structure Validate Basic](#1-write-the-basic-structure-validate-basic)
2. [Find error in this code](#2-find-error-in-this-code)
3. [Write tests for this function](#3-write-tests-for-this-function)
4. [What is wrong with these requirements](#4-what-is-wrong-with-these-requirements)
5. [Command to upload and init a CosmWasm](#5-command-to-upload-and-init-a-cosmwasm)

## 1️⃣ Write the basic structure Validate Basic

```go
package types
import (
    errorsmod "cosmossdk.io/errors"
    "encoding/hex"    sdk "github.com/cosmos/cosmos-sdk/types"
    sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
    "net/url")
func (msg *MsgSwapByDenom) ValidateBasicReza() error {
    // Validate the creator address
    _, err := sdk.AccAddressFromBech32(msg.Creator) //  Or msg.Sender
    if err != nil {
       return errorsmod.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address: %v", err)
    }
    // Validate the denom (non-empty and valid format)
    if err := sdk.ValidateDenom(msg.Denom); err != nil {
       return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid denom: %v", err)
    }
    // Validate max supply
    if msg.MaxSupply {
       if !msg.MaxSupplyAmount.IsPositive() {
          return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "max supply amount must be positive when max supply is enabled")
       }
    } else {
       if !msg.MaxSupplyAmount.IsZero() {
          return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "max supply amount must be zero when max supply is disabled")
       }
    }
    if msg.URI != "" {
       // Basic URI validation (you might want to use a more robust URI parsing library for production)
       _, err := url.ParseRequestURI(msg.URI)
       if err != nil {
          return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "uri must be a valid URI format (e.g., http://, https://, ipfs://)")
       }
       if msg.URIHash == "" {
          return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "uri hash is required when uri is provided")
       }
       // Basic URI Hash validation (assuming it's expected to be a hex-encoded hash)
       if _, err := hex.DecodeString(msg.URIHash); err != nil {
          return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid uri hash format: %s. Expected hex-encoded string.", err)
       }
       // Example Hash length check (SHA256 - 32 bytes = 64 hex chars, SHA512 - 64 bytes = 128 hex chars)
       if len(msg.URIHash) != 64 && len(msg.URIHash) != 128 {
          return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid uri hash length: %d. Expected length for common hash algorithms (e.g., 64 or 128).", len(msg.URIHash))
       }
    } else {
       if msg.URIHash != "" {
          return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "uri hash should not be provided when uri is not set")
       }
    }
    return nil
}

```

![alt text](image.png)


--------------------------
## 2️⃣ Find error in this code

![alt text](image-1.png)

--------------------------
## 3️⃣ Write tests for this function

```go
package keeper

import (
	"context"

	errorsmod "cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	"github.com/reza/x/zigch/types"
)

func (k msgServer) UpdateParams(goCtx context.Context, req *types.MsgUpdateParams) (*types.MsgUpdateParamsResponse, error) {

	if k.authority != req.Authority {
		return nil, errorsmod.Wrapf(govtypes.ErrInvalidSigner, "invalid authority; expected %s, got %s", k.authority, req.Authority)
	}
	ctx := sdk.UnwrapSDKContext(goCtx)

	if err := k.SetParams(ctx, req.Params); err != nil {
		return nil, err
	}

	return &types.MsgUpdateParamsResponse{}, nil
}

```

```go
package keeper_test

import (
	"fmt"

	"cosmossdk.io/math"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	"github.com/reza/x/zigch/keeper"
	"github.com/reza/x/zigch/types"
)

func (suite *ZigchKeeperTestSuite) TestUpdateParams() {
	sender := authtypes.NewModuleAddress(govtypes.ModuleName)
	tests := []struct {
		name      string
		authority string
		params    types.Params
		expected  error
	}{
		{
			name:      "invalid authority",
			authority: "invalid_authority",
			params:    types.Params{},
			expected:  fmt.Errorf("invalid authority"),
		},
		{
			name:      "valid authority",
			authority: sender.String(),
			params: types.Params{
				LeverageEnabled: true,
				Reza:            100,
			},
			expected: nil,
		},
		{
			name: "empty authority",
			params: types.Params{
				LeverageEnabled: true,
				Reza:            100,
			},
			authority: "",
			expected:  fmt.Errorf("invalid authority"),
		},
	}

	for _, tt := range tests {
		suite.Run(tt.name, func() {
			suite.SetupTest()

			msgServer := keeper.NewMsgServerImpl(suite.app.ZigchKeeperTestSuite)

			msg := &types.MsgUpdateParams{
				Authority: tt.authority,
				Params:    &tt.params,
			}

			_, err := msgServer.UpdateParams(suite.ctx, msg)
			if tt.expected != nil {
				suite.Require().ErrorContains(err, tt.expected.Error())
			} else {
				suite.Require().NoError(err)
				storedParams := suite.app.ZigchKeeperTestSuite.GetParams(suite.ctx)
				suite.Require().Equal(tt.params, storedParams)
			}
		})
	}
}

```

--------------------------
## 4️⃣ What is wrong with these requirements

My brainstorming questions:

1.  Does maxPerAddress refer to the total amount across all requests or just the limit within a specific time window?

2.  For addresses that make many requests, should we consider adding page breaks or filters?

3.  How long must users wait between requests?

4.  How can we stop users from using multiple wallets to bypass limits?

5.  How should we handle situations where one or more users send many valid requests very quickly?

6.  Go programming language doesn't have a basic "uint" type - it only has uint64 and uint32.

7.  How long should we keep the log history? Should we delete older logs after a certain time to save storage space?

-----------------

## 5️⃣ Command to upload and init a CosmWasm

### Example 1 (JUNO)

##### A - Compile

```bash
# compile the wasm contract with stable toolchain
rustup default stable
# this will produce a wasm build in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
cargo wasm
# this runs unit tests with helpful backtraces
RUST_BACKTRACE=1 cargo unit-test
# auto-generate json schema
cargo schema
# create an optimized version
sudo docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer:0.11.4
```

![alt text](image-2.png)

##### B - Upload 

```bash
# Make sure to get Testnet tokens which can be requested from the faucet 
# Upload our increment smart contract to the testnet
RES=$(junod tx wasm store artifacts/increment.wasm --from JunoWallet $TXFLAG -y --output json -b block) CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
```
![alt text](image-3.png)
![alt text](image-4.png)

##### C - Interact  
```bash
# Instantiate the smart contract
INIT='{"count":100}'
junod tx wasm instantiate $CODE_ID "$INIT" --from JunoWallet --label "increment project label" $TXFLAG -y
# Save contract details for future use
CONTRACT=$(junod query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
echo $CONTRACT
# Increment the counter
INC='{"increment" :{}}'
junod tx wasm execute $CONTRACT "$INC" --amount 100ujunox --from JunoWallet $TXFLAG -y
# Query the current count
GET_COUNT='{"get_count": {}}'
junod query wasm contract-state smart $CONTRACT "$GET_COUNT" $NODE --output json
```

![alt text](image-5.png)

![alt text](image-6.png)

![alt text](image-7.png)

---
### Example 1 (NibiruChain)

##### A - Compile
https://github.com/NibiruChain/cw-nibiru

##### B - Upload 
```bash
# Execute command (Change your wallet name if needed)

nibid tx wasm store $HOME/cw-nibiru/artifacts-cw-plus/cw20_base.wasm --from wallet  \
--gas-adjustment 1.2 --gas auto  --fees 80000unibi  -y 

#Copy txhash and paste-search it in Explorer. In LOGS find code_id. Copy that and put it into variable with command (Put your Code_id instead of YOUR_CODE_ID):

id=YOUR_CODE_ID

# [OPTIONAL] Check Wasm contract 

nibid query wasm code-info $id

```
![alt text](image-8.png)
![alt text](image-9.png)
![alt text](image-10.png)
##### C - Interact  
```bash
#Setup Variable $init (Change name, symbol. Instead of YOUR_WALLET_ADDRESS write your wallet address)

init='{"name":"Reza_Mirjah","symbol":"OFFC","decimals":2,"initial_balances":[{"address":"YOUR_WALLET_ADDRESS","amount":"100000"}],"mint":{"minter":"YOUR_WALLET_ADDRESS"},"marketing":{}}'

#sending tx ( Change  label and wallet name if needed)

nibid tx wasm instantiate $id $init --from wallet --label "YOUR_LABEL_NAME" \
--gas-adjustment 1.2 --gas auto  --fees 100000unibi --no-admin -y


#Copy txhash and paste-search it in Explorer. In LOGS find Instantiate Contract Address. Copy that and put it into variable with command (Put your Contract Address instead of YOUR_CONTRACT_ADDRESS):

contract=YOUR_CONTRACT_ADDRESS

```

##### D - [OPTIONAL]  Check Token balance in your Wallet
  ```bash
  # Create Variable $BALANCE_QUERY (Set your wallet address instead of YOUR_WALLET_ADDRESS)

BALANCE_QUERY='{"balance": {"address": "YOUR_WALLET_ADDRESS"}}'

# Execute tx:

nibid query wasm contract-state smart $contract "$BALANCE_QUERY" --output json

# You should see the amount of tokens that you minted

  ```

  ##### E - Broadcasting {ExecuteContract}
  ```bash
  # Here we gonna send some tokens to another wallet (Receiver wallet)
# Create $transfer variable (Change RECEIVER_WALLET_ADRESS to your receiver wallet address)

transfer='{"transfer":{"recipient":"RECEIVER_WALLET_ADRESS","amount":"100"}}'

# Execute tx:

nibid tx wasm execute $contract $transfer --gas-adjustment 1.2 \
--gas auto --fees 10000unibi --from wallet -y

  ```

  ##### F - [OPTIONAL] Check Token balance in Receiver Wallet

  ```bash
  # Rewrite Variable $BALANCE_QUERY (Set receiver wallet address instead of RECEIVER_WALLET_ADDRESS)

BALANCE_QUERY='{"balance": {"address": "RECEIVER_WALLET_ADDRESS"}}'

# Execute tx:
nibid query wasm contract-state smart $contract "$BALANCE_QUERY" --output json

  ```