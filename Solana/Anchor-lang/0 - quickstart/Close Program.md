SOL allocated to the on-chain program can be fully recovered by closing the program.


You can close a program by running the following command and specifying the program address found in `declare_id!()`:

```
solana program close [ProgramID]
```

For example:

```bash
solana program close 2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r
```

```
$ solana program close 2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r
Closed Program Id 2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r, 2.79511512 SOL reclaimed
```

Note that once a program is closed, the program ID cannot be reused to deploy a new program.