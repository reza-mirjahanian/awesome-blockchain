If you are a Whitelisted Node Operator, the Vault creator will be able to select you as the Validator Manager for his vault. The managers then have the power to register new validators to the vault, **having their withdrawal address pointing to the the vault itself.**

## 

[](https://docs.byzantine.fi/node-operators/staking#register-validators-for-staking)

Register Validators for Staking

To register validators, you must provide the following information:

Copy

```
function registerValidator(
    uint256 maxEB, 
    bytes32 depositDataRoot, 
    bytes calldata pubkey, 
    bytes calldata signature
) external onlyManagers;
```

Details about the parameters :

- **maxEB**: The maximum effective balance of the validator. In other word, it is the maximum amount of ETH that the validator can receive (from 32 ETH to 2048 ETH).
    
- **depositDataRoot**: The deposit data root
    
- **pubkey**: The validator's public key
    
- **signature**: The validator's signature
    

## 

[](https://docs.byzantine.fi/node-operators/staking#how-is-stake-allocated-to-registered-validators)

How is stake allocated to registered validators?

First come first serve - like a waterfall. Registered validators are filled up to their chosen maximum effective balance (maxEB), in order of joining. The first validator is filled, then the second, etc.

Accrued staking rewards compound onto validator balance and thereby get automatically staked, up to the limit of that validator's maxEB.

**Sweeping ETH**

If maxEB is reached and additional rewards are accrued, these rewards will flow to the associated ByzVault every 5-7 days.

In case of a voluntary exit, the staking oracle is monitoring voluntary exits to update ByzVault validator lists and prevent unexpected behavior.