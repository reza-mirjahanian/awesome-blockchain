[Other features](https://www.anchor-lang.com/docs/testing/litesvm#other-features)
---------------------------------------------------------------------------------

Other things you can do with `litesvm` include:

-   Changing the max compute units and other compute budget behaviour using [`.with_compute_budget`](https://docs.rs/litesvm/latest/litesvm/struct.LiteSVM.html#method.with_compute_budget).
-   Disable transaction signature checking using [`.with_sigverify(false)`](https://docs.rs/litesvm/latest/litesvm/struct.LiteSVM.html#method.with_sigverify).
-   Find previous transactions using [`.get_transaction`](https://docs.rs/litesvm/latest/litesvm/struct.LiteSVM.html#method.get_transaction).

[When should I use `solana-test-validator`?](https://www.anchor-lang.com/docs/testing/litesvm#when-should-i-use-solana-test-validator)
--------------------------------------------------------------------------------------------------------------------------------------

While `litesvm` is faster and more convenient, it is also less like a real RPC node. So `solana-test-validator` is still useful when you need to call RPC methods that LiteSVM doesn't support, or when you want to test something that depends on real-life validator behaviour rather than just testing your program and client code.

In general though it is recommended to use `litesvm` wherever possible, as it will make your life much easier.



[Time travel](https://www.anchor-lang.com/docs/testing/litesvm#time-travel)
---------------------------------------------------------------------------

Many programs rely on the `Clock` sysvar: for example, a mint that doesn't become available until after a certain time. With `litesvm` you can dynamically overwrite the `Clock` sysvar using [`svm.set_sysvar::<Clock>()`](https://docs.rs/litesvm/latest/litesvm/struct.LiteSVM.html#method.set_sysvar) (or `.setClock` in TS, or `.set_clock` in Python). Here's an example using a program that panics if `clock.unix_timestamp` is greater than 100 (which is on January 1st 1970):