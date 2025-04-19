# Register as a Staking Operator

How to become a node operator is Byzantine ecosystem ?

Please note that registering as an operator is not compulsory - Byzantine vaults are fully permissionless, especially in the case of [Solo Staker Vaults](https://docs.byzantine.fi/byzantine-vaults/types-of-native-vaults/solo-staker-vaults). Registering as an operator will however allow Byzantine to suggest you as a verified operator during the onboarding of new clients and to count on you for specific client demands.

For new operators looking to run staking and/or restaking nodes for Byzantine, become part of the Byzantine ecosystem, access its liquidity, and earn fees, the following steps are required:

## 

[](https://docs.byzantine.fi/node-operators/register-as-a-staking-operator#id-1.-request-to-be-whitelisted)

1\. Request to be whitelisted

At the moment, the ByzanTeam is carefully selecting the Node Operator Partners that users will be able to select as operators. All applicants will go through a due diligence process.

You can reach out to us by filling this [form](https://docs.google.com/forms/d/e/1FAIpQLSeQVdPA-djKkUihM34Tp9e_0hKPcEyx40o2tEN4ImmmMg7KQw/viewform?usp=dialog).

## 

[](https://docs.byzantine.fi/node-operators/register-as-a-staking-operator#id-2.-join-our-staking-operator-registry)

2\. Join our Staking Operator Registry

After having successfully passed the DD process, you will be added to the Operator Registry. You will only need to provide a few pieces of information:



```
 function registerOperator(
    string calldata _name,
    address _admin,
    uint16 _operatorFee,
    address[] calldata _managers
 ) external onlyByzantine;
```

Details about the parameters:

- **\_name**: The name of the staking operator. Lower case, spaces replaced by points. Example: for _Node Operator Inc_, `_name` will be `node.operator.inc`
    
- **\_admin**: The address of the Administrator of the operator. He will be allowed to change the staking fee, add or remove managers.
    
- **\_operatorFee**: The fees of the staking rewards in basis points. The fee can be updated by the Administrator and cannot exceed 10%. To set a fee at 10% inputing 1000 will be expected, 5% it will be 500, 2.89% 289, ...
    
- **\_managers**: The addresses of the Validator Managers. Knowing that the admin can update the managers list, this array can be empty during the registration. Validator Managers are capable of adding new validators' deposit data to vaults for which the applicant was selected as Validator Manager.
    

## 

[](https://docs.byzantine.fi/node-operators/register-as-a-staking-operator#id-3.-complete-your-profile-and-start-validating)

3\. Complete your profile, and start validating

After having added Validator Managers and made sure your fee is up to date, you can run validators in the Byzantine infrastructure.