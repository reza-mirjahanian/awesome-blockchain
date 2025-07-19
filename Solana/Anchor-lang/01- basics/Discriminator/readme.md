### [Account Discriminator](#account-discriminator)

An account discriminator in an Anchor program refers to an 8 byte identifier unique to each account type. You can find the implementation of the account discriminator [here](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/account/src/lib.rs#L111-L128).

The discriminator is the first 8 bytes of the SHA256 hash of the string `account:<AccountName>`. This discriminator is stored as the first 8 bytes of account data when an account is created.

When creating an account in an Anchor program, 8 bytes must be allocated for the discriminator.

```

#[account(init, payer = signer, space = 8 + 8)]
pub new_account: Account<'info, NewAccount>,
```

The **discriminator** is used during the following two scenarios:

* Initialization: When an account is created, the discriminator is set as the first 8 bytes of the account's data.
* Deserialization: When account data is deserialized, the first 8 bytes of account data is checked against the discriminator of the expected account type.

If there's a mismatch, it indicates that the client has provided an unexpected account. This mechanism serves as an account validation check in Anchor programs.