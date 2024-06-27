### Cosmos SDK: Simple Examples Using Code Snippets

The Cosmos SDK is a powerful framework for building custom blockchains. This guide provides simple examples to help you understand and use the Cosmos SDK, utilizing both standard and third-party libraries.

#### Prerequisites
- Go installed on your machine
- Basic knowledge of Go programming language

### Setting Up Your Cosmos SDK Environment

1. **Install Go**:
   ```sh
   wget https://dl.google.com/go/go1.17.5.linux-amd64.tar.gz
   sudo tar -xvf go1.17.5.linux-amd64.tar.gz
   sudo mv go /usr/local
   export GOROOT=/usr/local/go
   export PATH=$GOPATH/bin:$GOROOT/bin:$PATH
   ```

2. **Install Cosmos SDK**:
   ```sh
   git clone https://github.com/cosmos/cosmos-sdk
   cd cosmos-sdk
   make install
   ```

3. **Initialize a New Blockchain**:
   ```sh
   starport scaffold chain github.com/username/myblockchain
   cd myblockchain
   starport chain serve
   ```

### Base Account Example

A base account is a typical user account in the Cosmos SDK. Here's how you can define and manage a base account.

1. **Define Base Account**:
   ```go
   import (
       "github.com/cosmos/cosmos-sdk/types"
       "github.com/cosmos/cosmos-sdk/x/auth/types"
   )

   func createBaseAccount(address string, pubKey types.PubKey) types.BaseAccount {
       addr, err := types.AccAddressFromBech32(address)
       if err != nil {
           panic(err)
       }

       return types.BaseAccount{
           Address:       addr,
           PubKey:        pubKey,
           AccountNumber: 0,
           Sequence:      0,
       }
   }
   ```

2. **Store Base Account**:
   ```go
   import (
       "github.com/cosmos/cosmos-sdk/codec"
       "github.com/cosmos/cosmos-sdk/store"
   )

   func storeBaseAccount(cdc *codec.Codec, storeKey store.StoreKey, account types.BaseAccount) {
       store := ctx.KVStore(storeKey)
       key := types.AddressStoreKey(account.Address)
       value := cdc.MustMarshalBinaryLengthPrefixed(account)
       store.Set(key, value)
   }
   ```

### Module Account Example

Module accounts are owned by the blockchain and have special permissions. Here's an example of defining and using a module account.

1. **Define Module Account**:
   ```go
   import (
       "github.com/cosmos/cosmos-sdk/x/auth/types"
   )

   func createModuleAccount(moduleName string) types.ModuleAccount {
       return types.ModuleAccount{
           BaseAccount: types.NewBaseAccount(types.NewModuleAddress(moduleName), nil, 0, 0),
           Name:        moduleName,
           Permissions: []string{"mint", "burn"},
       }
   }
   ```

2. **Store Module Account**:
   ```go
   import (
       "github.com/cosmos/cosmos-sdk/codec"
       "github.com/cosmos/cosmos-sdk/store"
   )

   func storeModuleAccount(cdc *codec.Codec, storeKey store.StoreKey, account types.ModuleAccount) {
       store := ctx.KVStore(storeKey)
       key := types.AddressStoreKey(account.Address)
       value := cdc.MustMarshalBinaryLengthPrefixed(account)
       store.Set(key, value)
   }
   ```

### Using Third-Party Libraries

To enhance the functionality of your blockchain, you might need to integrate third-party libraries. Here’s an example using a popular third-party library, `gRPC`.

1. **Install gRPC**:
   ```sh
   go get -u google.golang.org/grpc
   ```

2. **gRPC Server Setup**:
   ```go
   import (
       "context"
       "google.golang.org/grpc"
       "net"
       "log"
   )

   type server struct{}

   func (s *server) GetAccount(ctx context.Context, req *pb.GetAccountRequest) (*pb.GetAccountResponse, error) {
       // Implementation of account retrieval
       return &pb.GetAccountResponse{Account: "exampleAccount"}, nil
   }

   func main() {
       lis, err := net.Listen("tcp", ":50051")
       if err != nil {
           log.Fatalf("failed to listen: %v", err)
       }
       s := grpc.NewServer()
       pb.RegisterAccountServiceServer(s, &server{})
       if err := s.Serve(lis); err != nil {
           log.Fatalf("failed to serve: %v", err)
       }
   }
   ```

### Complete Example

Combining the above snippets, here's a simple example to create and store both a base account and a module account in the Cosmos SDK.

1. **Create and Store Accounts**:
   ```go
   package main

   import (
       "github.com/cosmos/cosmos-sdk/codec"
       "github.com/cosmos/cosmos-sdk/store"
       "github.com/cosmos/cosmos-sdk/types"
       "github.com/cosmos/cosmos-sdk/x/auth/types"
       "log"
   )

   func main() {
       cdc := codec.New()
       storeKey := store.NewKVStoreKey("accountStore")
       ctx := // initialize context

       // Create Base Account
       baseAccount := createBaseAccount("cosmos1abcd...", nil)
       storeBaseAccount(cdc, storeKey, baseAccount)

       // Create Module Account
       moduleAccount := createModuleAccount("gov")
       storeModuleAccount(cdc, storeKey, moduleAccount)

       log.Println("Accounts created and stored successfully")
   }
   ```

This guide should help you get started with the Cosmos SDK, understand the basics of account types, and use both standard and third-party libraries effectively.