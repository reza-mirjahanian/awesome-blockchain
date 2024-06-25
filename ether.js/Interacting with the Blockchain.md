
### Interacting with the Blockchain
Querying State

Once you have a  [Provider](https://docs.ethers.org/v6/api/providers/#Provider), you have a read-only connection to the data on the blockchain. This can be used to query the current account state, fetch historic logs, look up contract code and so on.

```javascript
// Look up the current block number (i.e. height)
await provider.getBlockNumber()
// 20116823

// Get the current balance of an account (by address or ENS name)
balance = await provider.getBalance("ethers.eth")
// 4085267032476673080n

// Since the balance is in wei, you may wish to display it
// in ether instead.
formatEther(balance)
// '4.08526703247667308'

// Get the next nonce required to send a transaction
await provider.getTransactionCount("ethers.eth")
// 2
```

Sending Transactions

To write to the blockchain you require access to a private key which controls some account. In most cases, those private keys are not accessible directly to your code, and instead you make requests via a  [Signer](https://docs.ethers.org/v6/api/providers/#Signer), which dispatches the request to a service (such as  [MetaMask](https://metamask.io/)) which provides strictly gated access and requires feedback to the user to approve or reject operations.

```javascript
// When sending a transaction, the value is in wei, so parseEther
// converts ether to wei.
tx = await signer.sendTransaction({
  to: "ethers.eth",
  value: parseEther("1.0")
});

// Often you may wish to wait until the transaction is mined
receipt = await tx.wait();
```

----------
#### Contracts

A  Contract  is a meta-class, which means that its definition is derived at run-time, based on the ABI it is passed, which then determined what methods and properties are available on it.

Application Binary Interface (ABI)

Since all operations that occur on the blockchain must be encoded as binary data, we need a concise way to define how to convert between common objects (like strings and numbers) and its binary representation, as well as encode the ways to call and interpret the Contract.

For any method, event or error you wish to use, you must include a  [Fragment](https://docs.ethers.org/v6/api/abi/abi-coder/#Fragment)  to inform Ethers how it should encode the request and decode the result.

Any methods or events that are not needed can be safely excluded.

There are several common formats available to describe an ABI. The Solidity compiler usually dumps a JSON representation but when typing an ABI by hand it is often easier (and more readable) to use the **human-readable ABI**, which is just the Solidity signature.
```javascript
abi = [
  "function decimals() view returns (string)",
  "function symbol() view returns (string)",
  "function balanceOf(address addr) view returns (uint)"
]

// Create a contract
contract = new Contract("dai.tokens.ethers.eth", abi, provider)
```

Read-only methods (i.e.  view  and  pure)

A read-only method is one which cannot change the state of the blockchain, but often provide a simple interface to get important data about a Contract.

```javascript
// The contract ABI (fragments we care about)
abi = [
  "function decimals() view returns (uint8)",
  "function symbol() view returns (string)",
  "function balanceOf(address a) view returns (uint)"
]

// Create a contract; connected to a Provider, so it may
// only access read-only methods (like view and pure)
contract = new Contract("dai.tokens.ethers.eth", abi, provider)

// The symbol name for the token
sym = await contract.symbol()
// 'DAI'

// The number of decimals the token uses
decimals = await contract.decimals()
// 18n

// Read the token balance for an account
balance = await contract.balanceOf("ethers.eth")
// 4000000000000000000000n

// Format the balance for humans, such as in a UI
formatUnits(balance, decimals)
// '4000.0'
```

---------
State-changing Methods

change state on an ERC-20 contract
```javascript
abi = [
  "function transfer(address to, uint amount)"
]

// Connected to a Signer; can make state changing transactions,
// which will cost the account ether
contract = new Contract("dai.tokens.ethers.eth", abi, signer)

// Send 1 DAI
amount = parseUnits("1.0", 18);

// Send the transaction
tx = await contract.transfer("ethers.eth", amount)

// Currently the transaction has been sent to the mempool,
// but has not yet been included. So, we...

// ...wait for the transaction to be included.
await tx.wait()
```

forcing a call (simulation) of a state-changing method


```javascript
abi = [
  "function transfer(address to, uint amount) returns (bool)"
]

// Connected to a Provider since we only require read access
contract = new Contract("dai.tokens.ethers.eth", abi, provider)

amount = parseUnits("1.0", 18)

// There are many limitations to using a static call, but can
// often be useful to preflight a transaction.
await contract.transfer.staticCall("ethers.eth", amount)
// true

// We can also simulate the transaction as another account
other = new VoidSigner("0x643aA0A61eADCC9Cc202D1915D942d35D005400C")
contractAsOther = contract.connect(other.connect(provider))
await contractAsOther.transfer.staticCall("ethers.eth", amount)
```
 `staticCall` in Ether.js (using the `call` method) is a powerful feature for reading data from smart contracts without incurring gas fees. It’s an essential tool for developers who need to query blockchain data efficiently.

------

#### Listening to Events

When adding event listeners for a named event, the event parameters are destructed for the listener.

There is always one additional parameter passed to a listener, which is an  [EventPayload](https://docs.ethers.org/v6/api/utils/events/#EventPayload), which includes more information about the event including the filter and a method to remove that listener.

```javascript
abi = [
  "event Transfer(address indexed from, address indexed to, uint amount)"
]

// Create a contract; connected to a Provider, so it may
// only access read-only methods (like view and pure)
contract = new Contract("dai.tokens.ethers.eth", abi, provider)

// Begin listening for any Transfer event
contract.on("Transfer", (from, to, _amount, event) => {
  const amount = formatEther(_amount, 18)
  console.log(`${ from } => ${ to }: ${ amount }`);

  // The `event.log` has the entire EventLog

  // Optionally, stop listening
  event.removeListener();
});

// Same as above
contract.on(contract.filters.Transfer, (from, to, amount, event) => {
  // See above
})

// Listen for any Transfer to "ethers.eth"
filter = contract.filters.Transfer("ethers.eth")
contract.on(filter, (from, to, amount, event) => {
  // `to` will always be equal to the address of "ethers.eth"
});

// Listen for any event, whether it is present in the ABI
// or not. Since unknown events can be picked up, the
// parameters are not destructed.
contract.on("*", (event) => {
  // The `event.log` has the entire EventLog
});
```
### Query Historic Events

When querying within a large range of blocks, some backends may be prohibitively slow, may return an error or may truncate the results without any indication. This is at the discretion of each backend.

```javascript
abi = [
  "event Transfer(address indexed from, address indexed to, uint amount)"
]

// Create a contract; connected to a Provider, so it may
// only access read-only methods (like view and pure)
contract = new Contract("dai.tokens.ethers.eth", abi, provider)

// Query the last 100 blocks for any transfer
filter = contract.filters.Transfer
events = await contract.queryFilter(filter, -100)

// The events are a normal Array
events.length
// 119

// The first matching event
events[0]
// EventLog {
//   address: '0x6B175474E89094C44Da98b954EedeAC495271d0F',
//   args: Result(3) [
//     '0xa9A76fBbAe04A6236Fe83F4ABebbE268d2246c72',
//     '0x1418c3bF144D692366787c3475e4Acf582a74E01',
//     13200000000000000000000n
//   ],
//   blockHash: '0xb40dcb612d81f94396993303e320af304fe48bfd775f40a7ac64d7678a36a13e',
//   blockNumber: 20116747,
//   data: '0x0000000000000000000000000000000000000000000002cb92cc8f6714400000',
//   fragment: EventFragment { ... },
//   index: 89,
//   interface: Interface { ... },
//   provider: InfuraProvider { ... },
//   removed: false,
//   topics: [
//     '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef',
//     '0x000000000000000000000000a9a76fbbae04a6236fe83f4abebbe268d2246c72',
//     '0x0000000000000000000000001418c3bf144d692366787c3475e4acf582a74e01'
//   ],
//   transactionHash: '0xd0ca611ccbfa9d7c9bdf485052d59317d4258eb632c26331b32afe0b31c6c7d2',
//   transactionIndex: 55
// }

// Query all time for any transfer to ethers.eth
filter = contract.filters.Transfer("ethers.eth")
events = await contract.queryFilter(filter)

// The first matching event
events[0]
// undefined
```


-------------------
### Signing Messages

A private key can do a lot more than just sign a transaction to authorize it. It can also be used to sign other forms of data, which are then able to be validated for other purposes.

For example, signing  a message  can be used to prove ownership of an account which a website could use to authenticate a user and log them in.

```javascript
// Our signer; Signing messages does not require a Provider
signer = new Wallet(id("test"))
// Wallet {
//   address: '0xC08B5542D177ac6686946920409741463a15dDdB',
//   provider: null
// }

message = "sign into ethers.org?"

// Signing the message
sig = await signer.signMessage(message);
// '0xefc6e1d2f21bb22b1013d05ecf1f06fd73cdcb34388111e4deec58605f3667061783be1297d8e3bee955d5b583bac7b26789b4a4c12042d59799ca75d98d23a51c'

// Validating a message; notice the address matches the signer
verifyMessage(message, sig)
// '0xC08B5542D177ac6686946920409741463a15dDdB'
```