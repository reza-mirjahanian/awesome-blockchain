Zero Copy
=========

Learn how to use Anchor's zero-copy deserialization feature to handle large account data in Solana programs.

[Usage](https://www.anchor-lang.com/docs/features/zero-copy#usage)
------------------------------------------------------------------

Zero copy is a deserialization feature that allows programs to read account data directly from memory without copying it. This is particularly useful when working with large accounts.

To use zero-copy add the `bytemuck` crate to your dependencies. Add the `min_const_generics` feature to allow working with arrays of any size in your zero-copy types.

Cargo.toml

```
[dependencies]bytemuck = { version = "1.20.0", features = ["min_const_generics"] }anchor-lang = "0.31.1"
```


### [Define a Zero Copy Account](https://www.anchor-lang.com/docs/features/zero-copy#define-a-zero-copy-account)

To define an account type that uses zero-copy, annotate the struct with [`#[account(zero_copy)]`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/account/src/lib.rs#L417).

```Rust
#[account(zero_copy)]
pub struct Data {
    // 10240 bytes - 8 bytes account discriminator
    pub data: [u8; 10232],
}
```




### [Use AccountLoader for Zero Copy Accounts](https://www.anchor-lang.com/docs/features/zero-copy#use-accountloader-for-zero-copy-accounts)

To deserialize a zero-copy account, use [`AccountLoader<'info, T>`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/accounts/account_loader.rs#L95-L99), where `T` is the zero-copy account type defined with the `#[account(zero_copy)]` attribute.

For example:

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub zero_copy_account: AccountLoader<'info, Data>,
}
```