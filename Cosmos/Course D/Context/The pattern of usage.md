The usage pattern for context is as follows:

1.  Process receives a context `ctx` from its parent process, which provides information needed to perform the process.
2.  The `ctx.ms` is a branched store, meaning that a branch of the multistore is made so that the process can make changes to the state as it executes without changing the original `ctx.ms`. This is useful to protect the underlying multistore in case the changes need to be reverted at some point in the execution.
3.  The process may read and write from `ctx` as it is executing. It may call a subprocess and pass `ctx` to them as needed.
4.  When a subprocess returns, it checks the result for success or failure. In case of a failure, nothing needs to be done - the branch `ctx` is simply discarded. If it is successful, the changes made to the `CacheMultiStore` can be committed to the original ctx.ms via `Write()`.