Starting with **Cosmos SDK v0.50**, it is best practice to keep modules in their own repositories. This increases overall modularization of the code and simplifies the procedure for third-party reuse. For example, the module called `Checkers` would be in the `github.com/alice/checkers` repository, and could be used as such in the whole Go application; or you could use a `go.mod` redirect to keep it local, such as `replace (github.com/alice/checkers => ../checkers-module/)`.

Before v0.50, it used to be best practice to define a module in the `x/moduleName` folder. Using this method is still possible with v0.50. For example, the module called `Checkers` would go in `x/checkers`. If you look at the Cosmos SDK's base code, it also [defines its modules (opens new window)](https://github.com/cosmos/cosmos-sdk/tree/v0.45.4/x)in an `x/` folder.

Modules implement several elements:
