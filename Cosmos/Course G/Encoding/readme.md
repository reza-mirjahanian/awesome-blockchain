FAQ[​](https://docs.cosmos.network/v0.52/learn/advanced/encoding#faq "Direct link to FAQ")
------------------------------------------------------------------------------------------

### How to create modules using protobuf encoding[​](https://docs.cosmos.network/v0.52/learn/advanced/encoding#how-to-create-modules-using-protobuf-encoding "Direct link to How to create modules using protobuf encoding")

#### Defining module types[​](https://docs.cosmos.network/v0.52/learn/advanced/encoding#defining-module-types "Direct link to Defining module types")

Protobuf types can be defined to encode:

-   state
-   [`Msg`s](https://docs.cosmos.network/v0.52/build/building-modules/messages-and-queries#messages)
-   [Query services](https://docs.cosmos.network/v0.52/build/building-modules/query-services)
-   [genesis](https://docs.cosmos.network/v0.52/build/building-modules/genesis)

#### Naming and conventions[​](https://docs.cosmos.network/v0.52/learn/advanced/encoding#naming-and-conventions "Direct link to Naming and conventions")

We encourage developers to follow industry guidelines: [Protocol Buffers style guide](https://protobuf.dev/programming-guides/style/) and [Buf](https://buf.build/docs/style-guide), see more details in [ADR 023](https://docs.cosmos.network/v0.52/architecture/adr-023-protobuf-naming.md)



### **Encoding in Cosmos SDK**

### **Core Concepts**

- **Definition:**  
  Encoding in Cosmos SDK refers to the processes of serializing and deserializing data for state storage, transaction processing, and communication between nodes or clients.

- **Purpose of Encoding:**  
  - **Persistence:** Efficiently store data in the blockchain state (key-value stores).  
  - **Interoperability:** Enable communication between modules, nodes, and external systems like wallets or explorers.  
  - **Security:** Ensure deterministic and verifiable encoding/decoding to maintain consensus.

---

### **Supported Encoding Formats**

1. **Protobuf (Protocol Buffers):**
   - Default encoding format in Cosmos SDK.
   - Compact, efficient, and widely adopted.
   - Used for transactions, state serialization, and gRPC communication.

2. **Amino:**
   - Legacy encoding format.
   - Still used for backward compatibility in certain scenarios (e.g., Tendermint).

3. **JSON:**
   - Human-readable format.
   - Used for API responses, CLI inputs, and exporting/importing state.

---

### **Protobuf in Cosmos SDK**

- **Why Protobuf?**
  - Efficient serialization.
  - Backward/forward compatibility.
  - Compact binary representation reduces network bandwidth and storage usage.

- **Protobuf Message Definition:**  
  Define your data structures in `.proto` files.  
  Example:
  ```proto
  syntax = "proto3";

  package mymodule;

  message MyMessage {
    string sender = 1;
    string receiver = 2;
    string amount = 3;
  }
  ```

- **Compiling Protobuf:**  
  Use `protoc` to generate Go code for your module:
  ```bash
  protoc --go_out=. --go-grpc_out=. types.proto
  ```

- **Registering Protobuf Messages:**
  - Use `proto.RegisterType` to register your Protobuf messages.  
  Example:
  ```go
  func RegisterCodec(cdc *codec.LegacyAmino) {
      cdc.RegisterConcrete(&MyMessage{}, "mymodule/MyMessage", nil)
  }

  func RegisterInterfaces(registry codectypes.InterfaceRegistry) {
      registry.RegisterImplementations((*sdk.Msg)(nil), &MyMessage{})
  }
  ```

- **Protobuf for gRPC Services:**  
  Define your gRPC services in `.proto` files:
  ```proto
  service MyModuleService {
    rpc MyFunction(MyRequest) returns (MyResponse);
  }
  ```

---

### **Amino (Legacy Encoding)**

- **Features:**  
  - Simpler format but less efficient than Protobuf.  
  - Still used for Tendermint consensus messages (e.g., blocks, votes).

- **Registering Amino Types:**  
  Modules must register their types with the `LegacyAmino` codec.  
  Example:
  ```go
  var amino = codec.NewLegacyAmino()

  func RegisterCodec(cdc *codec.LegacyAmino) {
      cdc.RegisterConcrete(MyMessage{}, "mymodule/MyMessage", nil)
  }
  ```

---

### **JSON Encoding**

- **Usage:**
  - Commonly used for APIs, CLI inputs, and genesis files.
  - More human-readable but less efficient than Protobuf or Amino.

- **Example of JSON Representation:**
  ```json
  {
      "sender": "cosmos1...",
      "receiver": "cosmos1...",
      "amount": "1000"
  }
  ```

- **Encoding/Decoding in Go:**
  ```go
  data, err := json.Marshal(myStruct)
  if err != nil {
      log.Fatal(err)
  }

  var myStruct MyStruct
  err = json.Unmarshal(data, &myStruct)
  if err != nil {
      log.Fatal(err)
  }
  ```

---

### **Codec Abstractions**

- **Codec:**  
  Handles all encoding and decoding tasks. The Cosmos SDK uses three main codecs:
  1. **Amino (LegacyAmino):** For backward compatibility.
  2. **Protobuf Codec:** For modern serialization.
  3. **JSON Codec:** For API and CLI tasks.

- **Code Example: Initializing a Codec**
  ```go
  func MakeCodec() *codec.ProtoCodec {
      registry := codectypes.NewInterfaceRegistry()
      return codec.NewProtoCodec(registry)
  }
  ```

---

### **Interface Encoding with `Any`**

- **Why `Any`?**
  - Protobuf uses the `google.protobuf.Any` type to handle polymorphism (e.g., encoding interfaces).  
  - This enables storing multiple types under a single interface.

- **Packing and Unpacking `Any`:**
  ```go
  // Packing
  any, err := codectypes.NewAnyWithValue(msg)
  if err != nil {
      log.Fatal(err)
  }

  // Unpacking
  var myMsg MyMessage
  err = any.UnmarshalTo(&myMsg)
  if err != nil {
      log.Fatal(err)
  }
  ```

---

### **Custom Encoding**

- **Use Cases:**
  - When you need optimized serialization or a custom format for specific requirements.

- **Example: Implementing Custom Encoding:**
  ```go
  func (m MyMessage) Marshal() ([]byte, error) {
      return json.Marshal(m)
  }

  func (m *MyMessage) Unmarshal(data []byte) error {
      return json.Unmarshal(data, m)
  }
  ```

---

### **Registering Interfaces and Types**

- **Interface Registry:**  
  Modules must register their types and interfaces for Protobuf encoding.  
  Example:
  ```go
  func RegisterInterfaces(registry codectypes.InterfaceRegistry) {
      registry.RegisterInterface(
          "cosmos.Msg",
          (*sdk.Msg)(nil),
          &MyMessage{},
      )
  }
  ```

- **Concrete Type Registration:**  
  Example:
  ```go
  cdc.RegisterConcrete(&MyMessage{}, "mymodule/MyMessage", nil)
  ```

---

### **Comparing Encoding Formats**

| Feature                | **Protobuf**              | **Amino**                 | **JSON**                   |
|------------------------|---------------------------|---------------------------|----------------------------|
| **Efficiency**         | High                     | Medium                    | Low                        |
| **Human-Readable**     | No                       | Partially                 | Yes                        |
| **Deterministic**      | Yes                      | Yes                       | No                         |
| **Use Case**           | Transactions, State      | Legacy State, Consensus   | APIs, CLI, Export/Import   |

---

### **Testing Encoding**

- **Ensure Compatibility:**
  - Test Protobuf and Amino encoding/decoding to maintain backward compatibility.
  ```go
  func TestEncoding(t *testing.T) {
      cdc := MakeCodec()
      original := MyMessage{Sender: "sender", Receiver: "receiver", Amount: "100"}
      
      // Protobuf Encoding
      bz, err := cdc.Marshal(&original)
      require.NoError(t, err)
      
      var decoded MyMessage
      err = cdc.Unmarshal(bz, &decoded)
      require.NoError(t, err)
      require.Equal(t, original, decoded)
  }
  ```

- **Protobuf Determinism Test:**
  - Ensure serialized data is consistent across different runs:
  ```go
  data1, _ := proto.Marshal(msg1)
  data2, _ := proto.Marshal(msg1)
  require.Equal(t, data1, data2) // Must be identical
  ```

---

### **Tips for Efficient Encoding**

1. **Use Protobuf for Transactions:**  
   Default format for most performant encoding in Cosmos SDK.

2. **Avoid Overuse of Amino:**  
   Use Amino only when backward compatibility is required.

3. **Leverage InterfaceRegistry:**  
   Register all interfaces and types properly to avoid runtime issues.

4. **Optimize Protobuf Messages:**  
   Use field numbers wisely and prefer compact field types (e.g., `uint32` over `string` for small numbers).

5. **Validate Encoding Outputs:**  
   Always test encoding/decoding logic, especially for custom implementations.

6. **Keep JSON for User-Facing Interactions:**  
   Use JSON for APIs and CLI commands to enhance usability.

---

### **Summary Table**

| **Component**           | **Purpose**                                                |
|-------------------------|-----------------------------------------------------------|
| **Protobuf**            | Default serialization for transactions and state.         |
| **Amino**               | Legacy format for backward compatibility.                 |
| **JSON**                | Used for APIs, CLI inputs, and state export/import.       |
| **Interface Registry**  | Enables polymorphic encoding with `Any`.                  |
| **Custom Encoding**     | Implement optimized formats for specialized use cases.    |

---

### **Practical Example: Encoding a Custom Message**

```proto
syntax = "proto3";

package mymodule;

message MyMessage {
  string sender = 1;
  string receiver = 2;
  uint64 amount = 3;
}
```

- **Protobuf Usage in Go:**
  ```go
  func HandleMessage(ctx sdk.Context, msg *MyMessage) (*sdk.Result, error) {
      // Encoding to Protobuf
      bz, err := proto.Marshal(msg)
      if err != nil {
          return nil, err
      }

      // Decoding from Protobuf
      var decodedMsg MyMessage
      err = proto.Unmarshal(bz, &decodedMsg)
      if err != nil {
          return nil, err
      }

      return &sdk.Result{Data: bz}, nil
  }
  ```

