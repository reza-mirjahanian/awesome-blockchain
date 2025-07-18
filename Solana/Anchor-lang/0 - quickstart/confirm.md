```rust
solana confirm -v [TxHash]
``` 
```rust
solana confirm -v 3TewJtiUz1EgtT88pLJHvKFzqrzDNuHVi8CfD2mWmHEBAaMfC5NAaHdmr19qQYfTiBace6XUmADvR4Qrhe8gH5uc
``` 


```rust
$ solana confirm -v 3TewJtiUz1EgtT88pLJHvKFzqrzDNuHVi8CfD2mWmHEBAaMfC5NAaHdmr19qQYfTiBace6XUmADvR4Qrhe8gH5uc
RPC URL: https://api.devnet.solana.com
Default Signer: Playground Wallet
Commitment: confirmed
 
Transaction executed in slot 308150984:
  Block Time: 2024-06-25T12:52:05-05:00
  Version: legacy
  Recent Blockhash: 7AnZvY37nMhCybTyVXJ1umcfHSZGbngnm4GZx6jNRTNH
  Signature 0: 3TewJtiUz1EgtT88pLJHvKFzqrzDNuHVi8CfD2mWmHEBAaMfC5NAaHdmr19qQYfTiBace6XUmADvR4Qrhe8gH5uc
  Signature 1: 3TrRbqeMYFCkjsxdPExxBkLAi9SB2pNUyg87ryBaTHzzYtGjbsAz9udfT9AkrjSo1ZjByJgJHBAdRVVTZv6B87PQ
  Account 0: srw- 3z9vL1zjN6qyAFHhHQdWYRTFAcy69pJydkZmSFBKHg1R (fee payer)
  Account 1: srw- c7yy8zdP8oeZ2ewbSb8WWY2yWjDpg3B43jk3478Nv7J
  Account 2: -r-- 11111111111111111111111111111111
  Account 3: -r-x 2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r
  Instruction 0
    Program:   2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r (3)
    Account 0: c7yy8zdP8oeZ2ewbSb8WWY2yWjDpg3B43jk3478Nv7J (1)
    Account 1: 3z9vL1zjN6qyAFHhHQdWYRTFAcy69pJydkZmSFBKHg1R (0)
    Account 2: 11111111111111111111111111111111 (2)
    Data: [175, 175, 109, 31, 13, 152, 155, 237, 42, 0, 0, 0, 0, 0, 0, 0]
  Status: Ok
    Fee: ◎0.00001
    Account 0 balance: ◎5.47001376 -> ◎5.46900152
    Account 1 balance: ◎0 -> ◎0.00100224
    Account 2 balance: ◎0.000000001
    Account 3 balance: ◎0.00139896
  Log Messages:
    Program 2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r invoke [1]
    Program log: Instruction: Initialize
    Program 11111111111111111111111111111111 invoke [2]
    Program 11111111111111111111111111111111 success
    Program log: Changed data to: 42!
    Program 2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r consumed 5661 of 200000 compute units
    Program 2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r success
 
Confirmed

``` 
