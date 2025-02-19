A keeper is a Cosmos SDK abstraction that manages access to the subset of the state defined by a module. All access to the module's data must go through the module's keeper.


A keeper can be thought of as the literal gatekeeper of a module's stores. Each store defined within a module (typically an IAVL store) comes with a `storeKey` which grants unlimited access. The module's keeper holds this `storeKey`, which should otherwise remain unexposed, and defines methods for reading and writing to any store.

When a module needs to interact with the state defined in another module, it does so by interacting with the methods of the other module's keeper. Developers control the interactions their module can have with other modules by defining methods and controlling access.

Format
-------

Keepers are generally defined in a `/keeper/keeper.go` file located in the module's folder. The type keeper of a module is named simply `keeper.go` by convention. It usually follows the following structure:

```
type Keeper struct {
    // Expected external keepers, if any

    // Store key(s)

    // codec
}

```

### Parameters

The following parameters are of importance concerning the type definitions of keepers in modules:

-   An expected `keeper` is a keeper external to a module that is required by the internal keeper of said module. External keepers are listed in the internal keeper's type definition as interfaces. These interfaces are themselves defined in an `expected_keepers.go` file in the root of the module's folder. Interfaces are used to reduce the number of dependencies and to facilitate the maintenance of the module in this context.
-   `storeKeys` grant access to any stores of the multistore managed by the module. They should always remain unexposed to external modules.
-   `cdc` is the codec used to marshal and unmarshal structs to and from \[\]byte. The `cdc` can be `codec.BinaryCodec`, `codec.JSONCodec`, or `codec.Codec` based on your requirements. Note that `code.Codec` includes the other interfaces. It can be either a proto or amino codec, as long as they implement these interfaces.

Each keeper has its own constructor function, which is called from the application's constructor function. This is where keepers are instantiated and where developers make sure to pass correct instances of the module's keepers to other modules that require them.


### Scope and best practices

Keepers primarily expose getter and setter methods for stores managed by their module. Methods should remain simple and strictly limited to getting or setting the requested value. Validity checks should already have been done with the `ValidateBasic()` method of the message and the `Msg` server before the keeper's methods are called.

The getter method will typically have the following signature:

```
func (k Keeper) Get(ctx sdk.Context, key string) (value returnType, found bool)
```
The setter method will typically have the following signature:

```
func (k Keeper) Set(ctx sdk.Context, key string, value valueType)
```

Keepers also should implement an iterator method with the following signature when appropriate:

```
func (k Keeper) GetAll(ctx sdk.Context) (list []returnType)
```

