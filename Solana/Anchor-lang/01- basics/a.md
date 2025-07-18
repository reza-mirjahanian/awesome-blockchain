The main macros found in an Anchor program include:

-   [`declare_id`](https://www.anchor-lang.com/docs/basics/program-structure#declare_id-macro): Specifies the program's on-chain address
-   [`#[program]`](https://www.anchor-lang.com/docs/basics/program-structure#program-attribute): Specifies the module containing the program's instruction logic
-   [`#[derive(Accounts)]`](https://www.anchor-lang.com/docs/basics/program-structure#deriveaccounts-macro): Applied to structs to indicate a list of accounts required by an instruction
-   [`#[account]`](https://www.anchor-lang.com/docs/basics/program-structure#account-attribute): Applied to structs to create custom account types for the program