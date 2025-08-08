[What is a Mint Account?](https://www.anchor-lang.com/docs/tokens/basics/create-mint#what-is-a-mint-account)
------------------------------------------------------------------------------------------------------------

A mint account is an account type in Solana's Token Programs that uniquely represents a token on the network and stores global metadata about the token.

```rust
/// Mint data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mint {
  /// Optional authority used to mint new tokens. The mint authority may only
  /// be provided during mint creation. If no mint authority is present
  /// then the mint has a fixed supply and no further tokens may be
  /// minted.
  pub mint_authority: COption<Pubkey>,
  /// Total supply of tokens.
  pub supply: u64,
  /// Number of base 10 digits to the right of the decimal place.
  pub decimals: u8,
  /// Is `true` if this structure has been initialized
  pub is_initialized: bool,
  /// Optional authority to freeze token accounts.
  pub freeze_authority: COption<Pubkey>,
}
```

Note that both the [Token Program](https://github.com/solana-program/token/blob/main/program/src/state.rs#L18-L32) and [Token Extension Program](https://github.com/solana-program/token-2022/blob/main/program/src/state.rs#L30-L43) have the same base implementation for the Mint account.

Every token on Solana is represented by a mint account where the address of the mint account acts as its unique identifier on the network.

For example, the USDC stablecoin on Solana is identified by the mint address `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`. This mint address serves as USDC's unique identifier across the entire Solana ecosystem. You can view details about this mint account on [Solana Explorer](https://explorer.solana.com/address/3emsAVdmGKERbHjmGfQ6oZ1e35dkf5iYcS6U4CPKFVaa).