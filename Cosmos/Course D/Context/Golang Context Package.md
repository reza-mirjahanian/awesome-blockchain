A context is an immutable data structure that carries request-scoped data across APIs and processes. Contexts are also designed to enable concurrency and to be used in Go routines.

![](https://ida.interchain.io/hi-info-icon.svg)

A basic context is defined in the [Golang Context Package ](https://pkg.go.dev/context).

Contexts are intended to be immutable: they should never be edited. The convention is to instead create a child context from its parent using a `With` function. The Golang Context Package documentation instructs developers to [explicitly pass a context `ctx` ](https://pkg.go.dev/context)as the first argument of a process.