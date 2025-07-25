# Mollusk

Write tests for Solana programs in Rust using Mollusk.

[Mollusk](https://github.com/anza-xyz/mollusk) is a lightweight test harness for Solana programs. It provides a simple interface for testing Solana program executions in a minified Solana Virtual Machine (SVM) environment.

```
mollusk.process_and_validate_instruction(
    &instruction,   // <-- Instruction to test
    &accounts,      // <-- Account states
    &checks,        // <-- Checks to run on the instruction result
);
```

It does not create any semblance of a validator runtime, but instead provisions a program execution pipeline directly from lower-level SVM components.

In summary, the main processor - `process_instruction` - creates minified instances of Agave's program cache, transaction context, and invoke context. It uses these components to directly execute the provided program's ELF using the BPF Loader.

Because it does not use AccountsDB, Bank, or any other large Agave components, the harness is exceptionally fast. However, it does require the user to provide an explicit list of accounts to use, since it has nowhere to load them from.

The test environment can be further configured by adjusting the compute budget, feature set, or sysvars. These configurations are stored directly on the test harness (the `Mollusk` struct), but can be manipulated through a handful of helpers.

Four main API methods are offered:

* `process_instruction`: Process an instruction and return the result.
* `process_and_validate_instruction`: Process an instruction and perform a series of checks on the result, panicking if any checks fail.
* `process_instruction_chain`: Process a chain of instructions and return the result.
* `process_and_validate_instruction_chain`: Process a chain of instructions and perform a series of checks on each result, panicking if any checks fail.

## [Single Instructions](#single-instructions)

Both `process_instruction` and `process_and_validate_instruction` deal with single instructions. The former simply processes the instruction and returns the result, while the latter processes the instruction and then performs a series of checks on the result. In both cases, the result is also returned.

```rust
use {
    mollusk_svm::Mollusk,
    solana_sdk::{account::Account, instruction::{AccountMeta, Instruction}, pubkey::Pubkey},
};
 
let program_id = Pubkey::new_unique();
let key1 = Pubkey::new_unique();
let key2 = Pubkey::new_unique();
 
let instruction = Instruction::new_with_bytes(
    program_id,
    &[],
    vec![
        AccountMeta::new(key1, false),
        AccountMeta::new_readonly(key2, false),
    ],
);
 
let accounts = vec![
    (key1, Account::default()),
    (key2, Account::default()),
];
 
let mollusk = Mollusk::new(&program_id, "my_program");
 
// Execute the instruction and get the result.
let result = mollusk.process_instruction(&instruction, &accounts);
```

To apply checks via `process_and_validate_instruction`, developers can use the `Check` enum, which provides a set of common checks.

```rust
use {
    mollusk_svm::{Mollusk, result::Check},
    solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey
        system_instruction,
        system_program,
    },
};
 
let sender = Pubkey::new_unique();
let recipient = Pubkey::new_unique();
 
let base_lamports = 100_000_000u64;
let transfer_amount = 42_000u64;
 
let instruction = system_instruction::transfer(&sender, &recipient, transfer_amount);
let accounts = [
    (
        sender,
        Account::new(base_lamports, 0, &system_program::id()),
    ),
    (
        recipient,
        Account::new(base_lamports, 0, &system_program::id()),
    ),
];
let checks = vec![
    Check::success(),
    Check::compute_units(system_processor::DEFAULT_COMPUTE_UNITS),
    Check::account(&sender)
        .lamports(base_lamports - transfer_amount)
        .build(),
    Check::account(&recipient)
        .lamports(base_lamports + transfer_amount)
        .build(),
];
 
Mollusk::default().process_and_validate_instruction(
    &instruction,
    &accounts,
    &checks,
);
```

Note: `Mollusk::default()` will create a new `Mollusk` instance without adding any provided BPF programs. It will still contain a subset of the default builtin programs. For more builtin programs, you can add them yourself or use the `all-builtins` feature.

## [Instruction Chains](#instruction-chains)

Both `process_instruction_chain` and `process_and_validate_instruction_chain` deal with chains of instructions. The former processes each instruction in the chain and returns the final result, while the latter processes each instruction in the chain and then performs a series of checks on each result. In both cases, the final result is also returned.

```rust
use {
    mollusk_svm::Mollusk,
    solana_sdk::{account::Account, pubkey::Pubkey, system_instruction},
};
 
let mollusk = Mollusk::default();
 
let alice = Pubkey::new_unique();
let bob = Pubkey::new_unique();
let carol = Pubkey::new_unique();
let dave = Pubkey::new_unique();
 
let starting_lamports = 500_000_000;
 
let alice_to_bob = 100_000_000;
let bob_to_carol = 50_000_000;
let bob_to_dave = 50_000_000;
 
mollusk.process_instruction_chain(
    &[
        system_instruction::transfer(&alice, &bob, alice_to_bob),
        system_instruction::transfer(&bob, &carol, bob_to_carol),
        system_instruction::transfer(&bob, &dave, bob_to_dave),
    ],
    &[
        (alice, system_account_with_lamports(starting_lamports)),
        (bob, system_account_with_lamports(starting_lamports)),
        (carol, system_account_with_lamports(starting_lamports)),
        (dave, system_account_with_lamports(starting_lamports)),
    ],
);
```

Just like with `process_and_validate_instruction`, developers can use the `Check` enum to apply checks via `process_and_validate_instruction_chain`. Notice that `process_and_validate_instruction_chain` takes a slice of tuples, where each tuple contains an instruction and a slice of checks. This allows the developer to apply specific checks to each instruction in the chain. The result returned by the method is the final result of the last instruction in the chain.

```rust
use {
    mollusk_svm::{Mollusk, result::Check},
    solana_sdk::{account::Account, pubkey::Pubkey, system_instruction},
};
 
let mollusk = Mollusk::default();
 
let alice = Pubkey::new_unique();
let bob = Pubkey::new_unique();
let carol = Pubkey::new_unique();
let dave = Pubkey::new_unique();
 
let starting_lamports = 500_000_000;
 
let alice_to_bob = 100_000_000;
let bob_to_carol = 50_000_000;
let bob_to_dave = 50_000_000;
 
mollusk.process_and_validate_instruction_chain(
    &[
        (
            // 0: Alice to Bob
            &system_instruction::transfer(&alice, &bob, alice_to_bob),
            &[
                Check::success(),
                Check::account(&alice)
                    .lamports(starting_lamports - alice_to_bob) // Alice pays
                    .build(),
                Check::account(&bob)
                    .lamports(starting_lamports + alice_to_bob) // Bob receives
                    .build(),
                Check::account(&carol)
                    .lamports(starting_lamports) // Unchanged
                    .build(),
                Check::account(&dave)
                    .lamports(starting_lamports) // Unchanged
                    .build(),
            ],
        ),
        (
            // 1: Bob to Carol
            &system_instruction::transfer(&bob, &carol, bob_to_carol),
            &[
                Check::success(),
                Check::account(&alice)
                    .lamports(starting_lamports - alice_to_bob) // Unchanged
                    .build(),
                Check::account(&bob)
                    .lamports(starting_lamports + alice_to_bob - bob_to_carol) // Bob pays
                    .build(),
                Check::account(&carol)
                    .lamports(starting_lamports + bob_to_carol) // Carol receives
                    .build(),
                Check::account(&dave)
                    .lamports(starting_lamports) // Unchanged
                    .build(),
            ],
        ),
        (
            // 2: Bob to Dave
            &system_instruction::transfer(&bob, &dave, bob_to_dave),
            &[
                Check::success(),
                Check::account(&alice)
                    .lamports(starting_lamports - alice_to_bob) // Unchanged
                    .build(),
                Check::account(&bob)
                    .lamports(starting_lamports + alice_to_bob - bob_to_carol - bob_to_dave) // Bob pays
                    .build(),
                Check::account(&carol)
                    .lamports(starting_lamports + bob_to_carol) // Unchanged
                    .build(),
                Check::account(&dave)
                    .lamports(starting_lamports + bob_to_dave) // Dave receives
                    .build(),
            ],
        ),
    ],
    &[
        (alice, system_account_with_lamports(starting_lamports)),
        (bob, system_account_with_lamports(starting_lamports)),
        (carol, system_account_with_lamports(starting_lamports)),
        (dave, system_account_with_lamports(starting_lamports)),
    ],
);
```

It's important to understand that instruction chains _should not_ be considered equivalent to Solana transactions. Mollusk does not impose constraints on instruction chains, such as loaded account keys or size. Developers should recognize that instruction chains are primarily used for testing program execution.

## [Benchmarking Compute Units](#benchmarking-compute-units)

The Mollusk Compute Unit Bencher can be used to benchmark the compute unit usage of Solana programs. It provides a simple API for developers to write benchmarks for their programs, which can be checked while making changes to the program.

A markdown file is generated, which captures all of the compute unit benchmarks. If a benchmark has a previous value, the delta is also recorded. This can be useful for developers to check the implications of changes to the program on compute unit usage.

```rust
use {
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    mollusk_svm::Mollusk,
    /* ... */
};
 
// Optionally disable logging.
solana_logger::setup_with("");
 
/* Instruction & accounts setup ... */
 
let mollusk = Mollusk::new(&program_id, "my_program");
 
MolluskComputeUnitBencher::new(mollusk)
    .bench(("bench0", &instruction0, &accounts0))
    .bench(("bench1", &instruction1, &accounts1))
    .bench(("bench2", &instruction2, &accounts2))
    .bench(("bench3", &instruction3, &accounts3))
    .must_pass(true)
    .out_dir("../target/benches")
    .execute();
```

The `must_pass` argument can be provided to trigger a panic if any defined benchmark tests do not pass. `out_dir` specifies the directory where the markdown file will be written.

Developers can invoke this benchmark test with `cargo bench`. They may need to add a bench to the project's `Cargo.toml`.

```
[[bench]]
name = "compute_units"
harness = false
```

The markdown file will contain entries according to the defined benchmarks.

```
| Name   | CUs   | Delta  |
| ------ | ----- | ------ |
| bench0 | 450   | --     |
| bench1 | 579   | -129   |
| bench2 | 1,204 | +754   |
| bench3 | 2,811 | +2,361 |
```