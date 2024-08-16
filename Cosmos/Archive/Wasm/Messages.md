
# Messages

Messages are how you interact with a CosmWasm smart contract. If you look inside most contracts, there will be a  `msg.rs`  file that defines the messages.

An instantiate message is usually different in that it is defined separately in  `msg.rs`  as  `InstantiateMsg`  and then handled by a  `instantiate`  fn in the main  `contract.rs`.

The examples we are using here are very simple, however if you are confused about what arguments can be passed, you can look in the contract's  `schema`  folder. In here you will see at least two relevant files:

-   `instantiate_msg.json`  - the expected shape and and types for the instantiate message
-   `execute_msg.json`  - the expected shape and types for each of the messages that the contract can use to execute an action

https://github.com/InterWasm/cw-contracts/blob/main/contracts/nameservice/src/msg.rs#L13





## Propagation of context between contracts[​](https://docs.cosmwasm.com/docs/smart-contracts/message/submessage#propagation-of-context-between-contracts "Direct link to heading")

To stop reentrancy attacks, CosmWasm does not allow context to be stored in the contract memory. There are two ways to propagate state between contracts:

1.  All  `events`  returned by the submessage can be read from the  `Reply`  message
2.  Storing a temporary state using  `cw_storage_plus::Item`  and loading it in the reply handler

# Submessages

Messages are used to interact with SDK modules or other CW smart contracts. Since messages are executed in a set-and-forget manner, you will not get a response on whether the call was successful or not. Getting the result of your call can be very useful in the following cases:

1.  Instantiating a new contract and getting the contract address
2.  Executing an action and asserting that the result was successful (e.g. Making sure that certain tokens are transferred to your contract)
3.  Handling the error from your cross-contract call instead of rolling back the transaction

To get the result of the message sent from your smart contract, you will need to dispatch a submessage.



## Examples[​](https://docs.cosmwasm.com/docs/smart-contracts/message/submessage#examples "Direct link to heading")

1.  [Handling of contract instantiate reply](https://github.com/terraswap/terraswap/blob/main/contracts/terraswap_pair/src/contract.rs)  (Terraswap)
2.  [Parsing of contract execution replies](https://github.com/larry0x/astrozap/blob/master/contracts/astrozap/src/contract.rs)  (larry0x)

