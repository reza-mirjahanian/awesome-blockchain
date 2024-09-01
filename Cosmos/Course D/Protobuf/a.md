Protobuf is a data serialization method which developers use to describe message formats. There is a lot of internal communication within an interchain application, and Protobuf is central to how communication is done.


`.proto` files contain data structures called messages. The compiler `protoc` interprets the `.proto` file and generates source code in supported languages (C++, C#, Dart, Go, Java, and Python).

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/6-protobuf.html#working-with-protocol-buffers)Working with Protocol Buffers

First you must define a data structure in a `.proto` file. This is a normal text file with descriptive syntax. Data is represented as a message containing name-value pairs called fields.

Next, compile your Protobuf schema. `protoc` generates data access classes, with accessors for each field in your preferred language according to the command-line options. Accessors include serializing, deserializing, and parsing.

Types
------

The core of a Cosmos SDK application mainly consists of type definitions and constructor functions. Defined in `app.go`, the type definition of a custom application is simply a `struct` comprised of the following:

-   Reference to **`BaseApp`**: a reference to the `BaseApp` defines a custom application type embedding `BaseApp` for your application. The reference to `BaseApp` allows the custom application to inherit most of `BaseApp`'s core logic, such as ABCI methods and the routing logic.
-   List of **store keys**: each module in the Cosmos SDK uses a multistore to persist their part of the state. Access to such stores requires a list of keys that are declared in the type definition of the app.
-   List of each module's **keepers**: a keeper is an abstract piece in each module to handle the module's interaction with stores, specify references to other modules' keepers, and implement other core functionalities of the module. For cross-module interactions to work, all modules in the Cosmos SDK need to have their keepers declared in the application's type definition and exported as interfaces to other modules, so that the keeper's methods of one module can be called and accessed in other modules when authorized.
-   Reference to **codec**: defaulted to go-amino, the codec in your Cosmos SDK application can be substituted with other suitable encoding frameworks as long as they persist data stores in byte slices and are deterministic.
-   Reference to the **module manager**: a reference to an object containing a list of the application modules known as the module manager.