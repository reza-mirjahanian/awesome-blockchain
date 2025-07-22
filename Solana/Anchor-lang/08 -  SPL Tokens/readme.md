[What are Token Programs?](https://www.anchor-lang.com/docs/tokens#what-are-token-programs)
-------------------------------------------------------------------------------------------

On Solana, there are two main token programs (developed by [Anza](https://www.anza.xyz/), previously Solana Labs):

1.  [Token Program (Original)](https://github.com/solana-program/token)
    -   Basic token functionality (mint, transfer, etc.)
    -   Immutable and widely used
2.  [Token Extension Program (Token 2022)](https://github.com/solana-program/token-2022)
    -   Includes all original Token Program features
    -   Adds additional functionality through "extensions"
    -   Recommended for new tokens

-

[Invoking Token Programs in an Anchor Program](https://www.anchor-lang.com/docs/tokens#invoking-token-programs-in-an-anchor-program)
------------------------------------------------------------------------------------------------------------------------------------

The [`anchor-spl`](https://github.com/coral-xyz/anchor/tree/0e5285aecdf410fa0779b7cd09a47f235882c156/spl) crate simplifies the process of interacting with Solana's Token Programs in an Anchor program. This crate includes instructions and account types for both the original Token Program and the newer Token Extension Program (Token 2022).

Simply add the `anchor-spl` crate as a dependency to your program and add `"anchor-spl/idl-build"` to idl-build feature list in `Cargo.toml`. For a walkthrough of how to create an Anchor program locally, see the


```
cargo add anchor-spl
```