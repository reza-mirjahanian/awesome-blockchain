The [Anchor framework](https://www.anchor-lang.com/) uses [Rust macros](https://doc.rust-lang.org/book/ch19-06-macros.html) to reduce boilerplate code and simplify the implementation of common security checks required for writing Solana programs.

The main macros found in an Anchor program include:

-   [`declare_id`](https://solana.com/docs/programs/anchor/program-structure#declare-id-macro): Specifies the program's on-chain address
-   [`#[program]`](https://solana.com/docs/programs/anchor/program-structure#program-macro): Specifies the module containing the program's instruction logic
-   [`#[derive(Accounts)]`](https://solana.com/docs/programs/anchor/program-structure#derive-accounts-macro): Applied to structs to indicate a list of accounts required by an instruction
-   [`#[account]`](https://solana.com/docs/programs/anchor/program-structure#account-macro): Applied to structs to create custom account types for the program