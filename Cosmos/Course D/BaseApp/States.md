`BaseApp` provides **three primary states**. Two are volatile and one is persistent:

-   The persistent **main** state is the canonical state of the application.
-   The volatile states `checkState` and `deliverState` are used to handle transitions between main states during commits.

There is one single `CommitMultiStore`, referred to as the main state or root state. `BaseApp` derives the two volatile states using a mechanism called branching from this main state which is performed by the `CacheWrap` function.