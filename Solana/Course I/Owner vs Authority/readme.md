Owner vs Authority
------------------

Only programs can write data to accounts --- specifically, only to accounts they own. A program cannot write data to arbitrary accounts.

Programs of course cannot spontaneously write data to accounts. They need to receive an instruction to do so from a wallet. However, programs will generally only accept write instructions for a particular account from a privileged wallet: the *authority*.

An owner of an account is a program. An authority is a wallet. An authority sends a transaction to a program and that program can write to the account.

All accounts in Solana have the following fields, which are mostly self-explanatory:

-   Public Key
-   lamport balance
-   owner
-   executable (a boolean flag)
-   rent\_epoch (can be ignored for rent-exempt accounts)
-   data

We can see these by running `solana account <our wallet address>` in the terminal (with the Solana validator running in the background):
![alt text](image.png)


Note something interesting: **we are not the owner of our wallet!** The address `111...111` is the **system program**.

Why does the system program own wallets, instead of wallets owning themselves?

**Only the owner of an account can modify the data in it.**

The implication is that we are not able to modify our balance directly. Only the system program can do that. To transfer SOL out of our account, we send a signed transaction to the system program. The system program verifies we own the private key to the account, and then it modifies the balance on our behalf.

This is a pattern you will frequently see in Solana: only the owner of the account can modify the data in the account. The program will modify the data in the account if it sees a valid signature from a predesignated address: an authority.

**An authority is an address from which a program will accept instructions if it sees a valid signature. An authority cannot modify an account directly. It needs to work through a program that owns the account it is trying to modify.**

![alt text](image-1.png)


