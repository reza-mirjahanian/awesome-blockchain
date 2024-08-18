**How Queries are Handled by the CLI**

The CLI (Command Line Interface) handles queries by creating a `client.Context` object that stores all the necessary data to process a request. The `client.Context` is used to:

-   **Encode the query parameters**: The codec (encoder/decoder) is used to marshal the query parameters into a `[]byte` form.
-   **Retrieve the full-node**: The `client.Context` is used to retrieve the full-node (RPC Client) that the user is connected to.
-   **Write the output**: The `client.Context` contains a Writer to write the response when it is returned.

**Key Components of the client.Context**

-   **Codec**: The encoder/decoder used by the application.
-   **Account Decoder**: The account decoder from the auth module.
-   **RPC Client**: The CometBFT RPC Client (node) to which requests are relayed.
-   **Keyring**: A key manager used to sign transactions and handle other operations with keys.
-   **Output Writer**: A Writer used to output the response.
-   **Configurations**: The flags configured by the user for this command.

**Arguments and Route Creation**

-   **Parse the command**: The user's command is parsed to extract the arguments.
-   **Encode the arguments**: The codec is used to marshal the arguments into a `[]byte` form.
-   **Create the query**: The query is created using the encoded arguments.

**gRPC Query Client Creation**

-   **Create a query client**: The Cosmos SDK generates a query client from the Protobuf services.
-   **Make the query**: The query client is used to make the query to the full-node.

**Query Lifecycle**

-   **Encode the query**: The query is encoded into a `[]byte` form.
-   **Retrieve the full-node**: The `client.Context` is used to retrieve the full-node.
-   **Make the ABCI call**: The query is relayed to the full-node using the `ABCIQueryWithOptions()` function.
-   **Write the output**: The response is written using the `client.Context` Writer.


How Queries are Handled by the CLI
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

 let's dig into how the CLI prepares the query, and how the node handles it. The interactions from the users' perspective are a bit different, but the underlying functions are almost identical because they are implementations of the same command defined by the module developer. This step of processing happens within the CLI, gRPC, or REST server, and heavily involves a `client.Context`.

### Context[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#context "Direct link to Context")

The first thing that is created in the execution of a CLI command is a `client.Context`. A `client.Context` is an object that stores all the data needed to process a request on the user side. In particular, a `client.Context` stores the following:

-   **Codec**: The [encoder/decoder](https://docs.cosmos.network/v0.50/learn/advanced/encoding) used by the application, used to marshal the parameters and query before making the CometBFT RPC request and unmarshal the returned response into a JSON object. The default codec used by the CLI is Protobuf.
-   **Account Decoder**: The account decoder from the [`auth`](https://docs.cosmos.network/v0.50/build/modules/auth) module, which translates `[]byte`s into accounts.
-   **RPC Client**: The CometBFT RPC Client, or node, to which requests are relayed.
-   **Keyring**: A \[Key Manager\]../beginner/03-accounts.md#keyring) used to sign transactions and handle other operations with keys.
-   **Output Writer**: A [Writer](https://pkg.go.dev/io/#Writer) used to output the response.
-   **Configurations**: The flags configured by the user for this command, including `--height`, specifying the height of the blockchain to query, and `--indent`, which indicates to add an indent to the JSON response.

The `client.Context` also contains various functions such as `Query()`, which retrieves the RPC Client and makes an ABCI call to relay a query to a full-node.

client/context.go
```
// Context implements a typical context created in SDK modules for transaction
// handling and queries.
type Context struct{
	FromAddress       sdk.AccAddress
	Client            CometRPC
	GRPCClient        *grpc.ClientConn
	ChainID           string
	Codec             codec.Codec
	InterfaceRegistry codectypes.InterfaceRegistry
	Input             io.Reader
	Keyring           keyring.Keyring
	KeyringOptions    []keyring.Option
	Output            io.Writer
	OutputFormat      string
	Height            int64
	HomeDir           string
	KeyringDir        string
	From              string
	BroadcastMode     string
	FromName          string
	SignModeStr       string
	UseLedger         bool
	Simulate          bool
	GenerateOnly      bool
	Offline           bool
	SkipConfirm       bool
	TxConfig          TxConfig
	AccountRetriever  AccountRetriever
	NodeURI           string
	FeePayer          sdk.AccAddress
	FeeGranter        sdk.AccAddress
	Viper             *viper.Viper
	LedgerHasProtobuf bool
	PreprocessTxHook  PreprocessTxFn
// IsAux is true when the signer is an auxiliary signer (e.g. the tipper).
	IsAux bool
// TODO: Deprecated (remove).
	LegacyAmino *codec.LegacyAmino
// CmdContext is the context.Context from the Cobra command.
	CmdContext context.Context
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/client/context.go#L25-L68)

The `client.Context`'s primary role is to store data used during interactions with the end-user and provide methods to interact with this data - it is used before and after the query is processed by the full-node. Specifically, in handling `MyQuery`, the `client.Context` is utilized to encode the query parameters, retrieve the full-node, and write the output. Prior to being relayed to a full-node, the query needs to be encoded into a `[]byte` form, as full-nodes are application-agnostic and do not understand specific types. The full-node (RPC Client) itself is retrieved using the `client.Context`, which knows which node the user CLI is connected to. The query is relayed to this full-node to be processed. Finally, the `client.Context` contains a `Writer` to write output when the response is returned. These steps are further described in later sections.

### Arguments and Route Creation[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#arguments-and-route-creation "Direct link to Arguments and Route Creation")

At this point in the lifecycle, the user has created a CLI command with all of the data they wish to include in their query. A `client.Context` exists to assist in the rest of the `MyQuery`'s journey. Now, the next step is to parse the command or request, extract the arguments, and encode everything. These steps all happen on the user side within the interface they are interacting with.

#### Encoding[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#encoding "Direct link to Encoding")

In our case (querying an address's delegations), `MyQuery` contains an [address](https://docs.cosmos.network/v0.50/learn/beginner/accounts#addresses) `delegatorAddress` as its only argument. However, the request can only contain `[]byte`s, as it is ultimately relayed to a consensus engine (e.g. CometBFT) of a full-node that has no inherent knowledge of the application types. Thus, the `codec` of `client.Context` is used to marshal the address.

Here is what the code looks like for the CLI command:

x/staking/client/cli/query.go
```
_, err = ac.StringToBytes(args[0])
if err !=nil{
return err
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/staking/client/cli/query.go#L315-L318)

#### gRPC Query Client Creation[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#grpc-query-client-creation "Direct link to gRPC Query Client Creation")

The Cosmos SDK leverages code generated from Protobuf services to make queries. The `staking` module's `MyQuery` service generates a `queryClient`, which the CLI uses to make queries. Here is the relevant code:

x/staking/client/cli/query.go
```
		RunE:func(cmd *cobra.Command, args []string)error{
			clientCtx, err := client.GetClientQueryContext(cmd)
if err !=nil{
return err
}
			queryClient := types.NewQueryClient(clientCtx)
_, err = ac.StringToBytes(args[0])
if err !=nil{
return err
}
			pageReq, err := client.ReadPageRequest(cmd.Flags())
if err !=nil{
return err
}
			params :=&types.QueryDelegatorDelegationsRequest{
				DelegatorAddr: args[0],
				Pagination:    pageReq,
}
			res, err := queryClient.DelegatorDelegations(cmd.Context(), params)
if err !=nil{
return err
}
return clientCtx.PrintProto(res)
},
}
	flags.AddQueryFlagsToCmd(cmd)
	flags.AddPaginationFlagsToCmd(cmd,"delegations")
return cmd
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/staking/client/cli/query.go#L308-L343)

Under the hood, the `client.Context` has a `Query()` function used to retrieve the pre-configured node and relay a query to it; the function takes the query fully-qualified service method name as path (in our case: `/cosmos.staking.v1beta1.Query/Delegations`), and arguments as parameters. It first retrieves the RPC Client (called the [**node**](https://docs.cosmos.network/v0.50/learn/advanced/node)) configured by the user to relay this query to, and creates the `ABCIQueryOptions` (parameters formatted for the ABCI call). The node is then used to make the ABCI call, `ABCIQueryWithOptions()`.

Here is what the code looks like:

client/query.go
```
func(ctx Context)queryABCI(req abci.RequestQuery)(abci.ResponseQuery,error){
	node, err := ctx.GetNode()
if err !=nil{
return abci.ResponseQuery{}, err
}
var queryHeight int64
if req.Height !=0{
		queryHeight = req.Height
}else{
// fallback on the context height
		queryHeight = ctx.Height
}
	opts := rpcclient.ABCIQueryOptions{
		Height: queryHeight,
		Prove:  req.Prove,
}
	result, err := node.ABCIQueryWithOptions(context.Background(), req.Path, req.Data, opts)
if err !=nil{
return abci.ResponseQuery{}, err
}
if!result.Response.IsOK(){
return abci.ResponseQuery{},sdkErrorToGRPCError(result.Response)
}
// data from trusted node or subspace query doesn't need verification
if!opts.Prove ||!isQueryStoreWithProof(req.Path){
return result.Response,nil
}
return result.Response,nil
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/client/query.go#L79-L113)