https://github.com/Jac0xb/lighthouse

Lighthouse is a Solana program that provides assertion instructions that can be added to transactions. If a bad actor spoofs simulation results, there is overspending during the transaction, or an oracle account is in an undesired state, the assertion will fail, and the transaction will also fail. Lighthouse makes it simple to append assertion instructions to existing transactions without needing to write new Solana programs.

Lighthouse is an open source, public utility Solana program with an emphasis on security (multisig, verifiable build, non-upgradable releases, etc coming soon), composability (program-agnostic use cases), and community (open source, assist in integration with open source projects, incentivize contributions).

Example Usecases
Solana at its core is a decentralized database (accounts) and assertion/mutation of that permissioned data (programs). Programs generally make assertions about account state before allowing data to be mutated, one of Anchor's innovations for preventing "footguns and attacks" was that it has common account state assertions baked into the framework. Currently performing even trivial assertions such as account balance checks requires one to deploy a Solana program. Lighthouse at a high-level seeks to be an composable way to make assertions on onchain state and the instruction-level delta of these state changes without the need to deploy additional Solana programs.

Guardrail Example: A wallet simulates that a token account changes balance from 100 to 90 for a transaction. It appends a Lighthouse assertion instruction to the transaction which says the token account balance must be 90 at the end of the transaction (the assertion instruction is placed at the end of the transaction).
```