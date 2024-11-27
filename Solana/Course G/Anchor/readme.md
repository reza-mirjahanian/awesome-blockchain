
Before you begin, ensure you have the following installed:

-   Rust: The programming language for building Solana programs.
-   Solana CLI: Command-line tool for Solana development.
-   Anchor CLI: Command-line tool for the Anchor framework.


start a local Solana validator by running the `solana-test-validator` command.

 1. In a separate terminal, run the tests against the local cluster. Use the `--skip-local-validator` flag to skip starting the local validator since it's already running. `anchor test --skip-local-validator`




```
[programs.localnet]
my_program = "3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg"
 
[registry]
url = "https://api.apr.dev"
 
[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"
 
[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

```

To deploy your program to **devnet**, change the cluster value to Devnet. Note that this requires your wallet to have enough SOL on Devnet to cover deployment cost.

```
-cluster = "Localnet"
+cluster = "Devnet"
#
[provider]
cluster = "Devnet"
wallet = "~/.config/solana/id.json"

```

Now when you run anchor deploy, your program will be deployed to the devnet cluster. The anchor test command will also use the cluster specified in the Anchor.toml file.


To update a program, simply make changes to your program's code and run the anchor build command to generated an updated .so file.


```
anchor build

anchor deploy
```


To reclaim the SOL allocated to a program account, you can close your Solana program.

To close a program, use the  `solana program close <PROGRAM_ID>`  command. For example:

    solana program close 3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg --bypass-warning

```

.
├── .anchor
│   └── program-logs
├── app
├── migrations
├── programs
│   └── [project-name]
│       └── src
│           ├── lib.rs
│           ├── Cargo.toml
│           └── Xargo.toml
├── target
│   ├── deploy
│   │   └── [project-name]-keypair.json
│   ├── idl
│   │   └── [project-name].json
│   └── types
│       └── [project-name].ts
├── tests
│   └── [project-name].ts
├── Anchor.toml
├── Cargo.toml
└── package.json
```

