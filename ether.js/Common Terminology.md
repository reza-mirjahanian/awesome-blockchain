### Some Common Terminology

To begin, it is useful to have a basic understanding of the types of objects available and what they are responsible for, at a high level.

#### Provider

A  [Provider](https://docs.ethers.org/v6/api/providers/#Provider)  is a read-only connection to the blockchain, which allows querying the blockchain state, such as account, block or transaction details, querying event logs or evaluating read-only code using call.

If you are coming from **Web3.js,** you are used to a  Provider  offering both read and write access. In Ethers, all write operations are further abstracted into another Object, the  Signer.

#### Signer

A  [Signer](https://docs.ethers.org/v6/api/providers/#Signer)  wraps all operations that interact with an account. An account generally has a private key located  somewhere, which can be used to sign a variety of types of payloads.

The private key may be located in memory (using a  [Wallet](https://docs.ethers.org/v6/api/wallet/#Wallet)) or protected via some IPC layer, such as MetaMask which proxies interaction from a website to a browser plug-in, which keeps the private key out of the reach of the website and only permits interaction after requesting permission from the user and receiving authorization.

#### Transaction

To make any state changes to the blockchain, a transaction is required, which requires a fee to be paid, where the fee covers the associated costs with executing the transaction (such as reading the disk and performing maths) and storing the updated information.

If a transaction reverts, a fee must still be paid, since the validator still had to expend resources to try running the transaction to determine that it reverted and the details of its failure are still be recorded.

Transactions include sending ether from one user to another, deploying a  Contract  or executing a state-changing operation against a  Contract.

#### Contract

A  [Contract](https://docs.ethers.org/v6/api/contract/#Contract)  is a program that has been deployed to the blockchain, which includes some code and has allocated storage which it can read from and write to.

It may be read from when it is connected to a  [Provider](https://docs.ethers.org/v6/api/providers/#Provider)  or state-changing operations can be called when connected to a  [Signer](https://docs.ethers.org/v6/api/providers/#Signer).

#### Receipt

Once a  Transaction  has been submitted to the blockchain, it is placed in the memory pool (mempool) until a validator decides to include it.

A transaction's changes are only made once it has been included in the blockchain, at which time a receipt is available, which includes details about the transaction, such as which block it was included in, the actual fee paid, gas used, all the events that it emitted and whether it was successful or reverted.

---------------
### Connecting to Ethereum

This very first thing needed to begin interacting with the blockchain is connecting to it using a  [Provider](https://docs.ethers.org/v6/api/providers/#Provider).

#### MetaMask (and other injected providers)

The quickest and easiest way to experiment and begin developing on Ethereum is to use  [MetaMask](https://metamask.io/), which is a browser extension that injects objects into the  window, providing:

-   read-only access to the Ethereum network (a  [Provider](https://docs.ethers.org/v6/api/providers/#Provider))
-   authenticated write access backed by a private key (a  [Signer](https://docs.ethers.org/v6/api/providers/#Signer))

When requesting access to the authenticated methods, such as sending a transaction or even requesting the private key address, MetaMask will show a pop-up to the user asking for permission.

```javascript
let signer = null;
let provider;
if (window.ethereum == null) {
   // If MetaMask is not installed, we use the default provider,
   // which is backed by a variety of third-party services (such
   // as INFURA). They do not have private keys installed,
   // so they only have read-only access
   console.log("MetaMask not installed; using read-only defaults")
   provider = ethers.getDefaultProvider()
} else {
   // Connect to the MetaMask EIP-1193 object. This is a standard
   // protocol that allows Ethers access to make all read-only
   // requests through MetaMask.
   provider = new ethers.BrowserProvider(window.ethereum)
   // It also provides an opportunity to request access to write
   // operations, which will be performed by the private key
   // that MetaMask manages for the user.
   signer = await provider.getSigner();
}
```

#### Custom RPC Backend

If you are running your own Ethereum node (e.g.  [Geth](https://geth.ethereum.org/)) or using a custom third-party service (e.g.  [INFURA](https://infura.io/)), you can use the  [JsonRpcProvider](https://docs.ethers.org/v6/api/providers/jsonrpc/#JsonRpcProvider)  directly, which communicates using the  [link-jsonrpc](https://github.com/ethereum/wiki/wiki/JSON-RPC)  protocol.

When using your own Ethereum node or a developer-base blockchain, such as Hardhat or Ganache, you can get access to the accounts with  JsonRpcProvider-getSigner.

```javascript
// If no %%url%% is provided, it connects to the default
// http://localhost:8545, which most nodes use.
provider = new ethers.JsonRpcProvider(url)

// Get write access as an account by getting the signer
signer = await provider.getSigner()
```