
## Anatomy[​](https://docs.near.org/build/smart-contracts/security/checklist#anatomy "Direct link to Anatomy")

1.  All private methods are decorated as  `private`.

## Environment[​](https://docs.near.org/build/smart-contracts/security/checklist#environment "Direct link to Environment")

2.  `predecessor`  and  `signer`  are used correctly through the entire contract.

## Storage[​](https://docs.near.org/build/smart-contracts/security/checklist#storage "Direct link to Storage")

3.  Each time the state grows it is ensured that there is enough balance to cover it
4.  All collections (i.e. Vector, Map, Tree, etc) have an unique id
5.  Check for underflow and overflow!. In rust, you can do this by simply adding the  `overflow-checks = true`  flag in your  `Cargo.toml`.

## Actions[​](https://docs.near.org/build/smart-contracts/security/checklist#actions "Direct link to Actions")

6.  When sending money, you leave enough in the contract to cover the storage cost
7.  If you are tracking user's fund, you  **deduct them before**  sending them back to the user.

## Callbacks[​](https://docs.near.org/build/smart-contracts/security/checklist#callbacks "Direct link to Callbacks")

8.  All private callbacks are marked as  `private`
9.  All cross-contract calls have a callback
10.  All callbacks check for errors and roll back the state if necessary
11.  All callbacks return money to the  `predecessor`  if necessary
12.  Callbacks are free of  `panic!`
13.  All the callbacks are given enough GAS to execute entirely
14.  The contract is not left in an exploitable state between a cross-contract call and its callback