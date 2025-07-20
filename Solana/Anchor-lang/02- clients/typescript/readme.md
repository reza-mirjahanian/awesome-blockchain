

#### Section: Client Libraries > TypeScript
- **Content**: 
  - The section is part of the broader "Client Libraries" documentation, with a focus on TypeScript.
  - It introduces Anchor’s TypeScript client library (`@coral-xyz/anchor`) for interacting with Solana programs.
- **Key Points**:
  - Simplifies client-side interaction with Solana programs.
  - Compatible only with the legacy version (v1) of `@solana/web3.js` and `@solana/spl-token`, not version 2.

#### Subsection: Client Program
- **Content**:
  - Describes how to create a `Program` instance using the program's Interface Description Language (IDL) file and an `AnchorProvider`.
  - An `AnchorProvider` combines:
    - **Connection**: A connection to a Solana cluster (e.g., localhost, devnet, mainnet).
    - **Wallet**: An optional default wallet to pay for and sign transactions.
  - Notes that when integrating with a frontend using the Solana wallet adapter, you need to set up the `AnchorProvider` and `Program` manually.
- **Key Points**:
  - The `Program` instance is central to interacting with a Solana program.
  - The IDL file defines the program’s structure and is required to instantiate a `Program`.
  - The `AnchorProvider` abstracts the connection and wallet, simplifying transaction handling.

#### Subsection: Example Folder Structure
- **Content**:
  - Provides an example folder structure for a TypeScript client interacting with an Anchor program:
    ```
    .
    ├── idls
    │   └── example.json
    ├── src
    │   └── example.ts
    └── package.json
    ```
  - **Explanation of Files**:
    - `idls/example.json`: The IDL file representing the program's structure.
    - `src/example.ts`: The TypeScript script to interact with the program.
    - `package.json`: Includes dependencies like `@coral-xyz/anchor`, `@solana/web3.js`, etc.
- **Key Points**:
  - The IDL file must be placed in the `/idls` folder.
  - The `example.ts` file contains the logic to interact with the program.
  - Proper project organization is critical for TypeScript clients.

#### Subsection: Example Code (Tabs)
- **Content**:
  - The page includes a tabbed interface with two files: `example.json` (IDL) and `example.ts` (TypeScript script).
  
##### Tab: example.json (IDL File)
- **Content**:
  - Shows a sample IDL file for a program named `example`:
    ```json
    {
      "version": "0.1.0",
      "name": "example",
      "instructions": [
        {
          "name": "initialize",
          "accounts": [
            {
              "name": "counter",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "authority",
              "isMut": false,
              "isSigner": true
            },
            {
              "name": "systemProgram",
              "isMut": false,
              "isSigner": false
            }
          ],
          "args": [
            {
              "name": "start",
              "type": "u64"
            }
          ]
        },
        {
          "name": "increment",
          "accounts": [
            {
              "name": "counter",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "authority",
              "isMut": false,
              "isSigner": true
            }
          ],
          "args": []
        }
      ],
      "accounts": [
        {
          "name": "Counter",
          "type": {
            "kind": "struct",
            "fields": [
              {
                "name": "authority",
                "type": "publicKey"
              },
              {
                "name": "count",
                "type": "u64"
              }
            ]
          }
        }
      ]
    }
    ```
- **Key Points**:
  - The IDL defines the program’s version (`0.1.0`) and name (`example`).
  - **Instructions**:
    - `initialize`: Initializes a counter with a `start` value, requiring `counter`, `authority`, and `systemProgram` accounts.
    - `increment`: Increments the counter, requiring `counter` and `authority` accounts.
  - **Accounts**:
    - `Counter`: A struct with `authority` (publicKey) and `count` (u64) fields.
  - Account properties:
    - `isMut`: Indicates if the account is mutable.
    - `isSigner`: Indicates if the account must sign the transaction.

##### Tab: example.ts (TypeScript Script)
- **Content**:
  - Provides a TypeScript script to interact with the `example` program:
    ```typescript
    import * as anchor from "@coral-xyz/anchor";
    import { Program } from "@coral-xyz/anchor";
    import { Example } from "../target/types/example";
    import { assert } from "chai";

    describe("example", () => {
      // Configure the client to use the local cluster
      anchor.setProvider(anchor.AnchorProvider.env());

      // Generate keypair to use as address for the counter account
      const counter = anchor.web3.Keypair.generate();

      const provider = anchor.getProvider();
      const program = anchor.workspace.Example as Program<Example>;

      it("Initialize counter", async () => {
        await program.methods
          .initialize(new anchor.BN(1234))
          .accounts({
            counter: counter.publicKey,
            authority: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([counter])
          .rpc();

        const counterAccount = await program.account.counter.fetch(
          counter.publicKey
        );

        assert.equal(
          counterAccount.authority.toBase58(),
          provider.wallet.publicKey.toBase58()
        );
        assert.equal(counterAccount.count.toNumber(), 1234);
      });

      it("Increment counter", async () => {
        await program.methods
          .increment()
          .accounts({
            counter: counter.publicKey,
            authority: provider.wallet.publicKey,
          })
          .rpc();

        const counterAccount = await program.account.counter.fetch(
          counter.publicKey
        );

        assert.equal(
 counterAccount.authority.toBase58(),
          provider.wallet.publicKey.toBase58()
        );
        assert.equal(counterAccount.count.toNumber(), 1235);
      });
    });
    ```
- **Key Points**:
  - **Imports**:
    - `@coral-xyz/anchor`: Provides Anchor’s TypeScript utilities (`Program`, `AnchorProvider`, etc.).
    - `Example` type: Imported from `../target/types/example`, generated by Anchor for type safety.
    - `chai`: Used for assertions in tests.
  - **Setup**:
    - Configures the client to use the local Solana cluster with `anchor.setProvider(anchor.AnchorProvider.env())`.
    - Generates a keypair for the `counter` account.
    - Retrieves the provider and initializes the `program` as a `Program<Example>` instance.
  - **Test: Initialize Counter**:
    - Calls the `initialize` instruction with a starting value of `1234` (using `anchor.BN` for large integers).
    - Specifies accounts: `counter`, `authority`, and `systemProgram`.
    - Uses `signers` to include the `counter` keypair for signing.
    - Sends the transaction with `.rpc()`.
    - Fetches the `counter` account and asserts:
      - The `authority` matches the provider’s wallet public key.
      - The `count` is `1234`.
  - **Test: Increment Counter**:
    - Calls the `increment` instruction.
    - Specifies accounts: `counter` and `authority`.
    - Sends the transaction with `.rpc()`.
    - Fetches the `counter` account and asserts:
      - The `authority` matches the provider’s wallet public key.
      - The `count` is `1235` (incremented from `1234`).

#### Subsection: Alternative Program Instance Creation
- **Content**:
  - Describes an alternative way to create a `Program` instance without a default wallet:
    ```typescript
    const connection = new anchor.web3.Connection("http://127.0.0.1:8899");
    const idl = JSON.parse(
      require("fs").readFileSync("./idls/example.json", "utf8")
    );
    const program = new anchor.Program(idl, new anchor.web3.PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"), { connection });
    ```
- **Key Points**:
  - Creates a `Connection` to a local Solana cluster.
  - Loads the IDL file manually using the `fs` module.
  - Instantiates a `Program` with:
    - The parsed IDL.
    - The program’s public key.
    - A configuration object specifying the `connection`.
  - This approach is useful for read-only operations (e.g., fetching accounts or building instructions without signing).

#### Subsection: TypeScript Type Definitions
- **Content**:
  - Mentions that TypeScript type definitions are generated in the `target/types` folder (e.g., `target/types/example.ts`).
  - Example of the generated `idlType.ts`:
    ```typescript
    export type Example = {
      "version": "0.1.0",
      "name": "example",
      "instructions": [
        {
          "name": "initialize",
          "accounts": [
            {
              "name": "counter",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "authority",
              "isMut": false,
              "isSigner": true
            },
            {
              "name": "systemProgram",
              "isMut": false,
              "isSigner": false
            }
          ],
          "args": [
            {
              "name": "start",
              "type": "u64"
            }
          ]
        },
        {
          "name": "increment",
          "accounts": [
            {
              "name": "counter",
              "isMut": true,
              "isSigner": false
            },
            {
              "name": "authority",
              "isMut": false,
              "isSigner": true
            }
          ],
          "args": []
        }
      ],
      "accounts": [
        {
          "name": "Counter",
          "type": {
            "kind": "struct",
            "fields": [
              {
                "name": "authority",
                "type": "publicKey"
              },
              {
                "name": "count",
                "type": "u64"
              }
            ]
          }
        }
      ]
    };

    export const IDL: Example = {
      "version": "0.1.0",
      "name": "example",
      ...
    };
    ```
- **Key Points**:
  - The `idlType.ts` file mirrors the IDL structure but is typed for TypeScript.
  - It defines the `Example` type and exports a constant `IDL` for use in the program.
  - Generated automatically by Anchor during the build process.

#### Subsection: Integration with Solana Wallet Adapter
- **Content**:
  - Provides an example of setting up the `AnchorProvider` and `Program` for a frontend using the Solana wallet adapter:
    ```typescript
    import * as anchor from "@coral-xyz/anchor";
    import { Program } from "@coral-xyz/anchor";
    import { Example } from "../target/types/example";
    import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";

    export function MyApp() {
      const wallet = useAnchorWallet();
      const { connection } = useConnection();

      if (!wallet) {
        return <div>Please connect your wallet</div>;
      }

      const provider = new anchor.AnchorProvider(
        connection,
        wallet,
        anchor.AnchorProvider.defaultOptions()
      );

      const program = new Program<Example>(
        anchor.workspace.Example.IDL,
        new anchor.web3.PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"),
        provider
      );

      const handleClick = async () => {
        const counter = anchor.web3.Keypair.generate();

        await program.methods
          .initialize(new anchor.BN(1234))
          .accounts({
            counter: counter.publicKey,
            authority: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([counter])
          .rpc();
      };

      return (
        <div>
          <button onClick={handleClick}>Run</button>
        </div>
      );
    }
    ```
- **Key Points**:
  - Uses `@solana/wallet-adapter-react` for wallet integration.
  - Checks if a wallet is connected before proceeding.
  - Creates an `AnchorProvider` with:
    - The `connection` from `useConnection`.
    - The `wallet` from `useAnchorWallet`.
    - Default options from `anchor.AnchorProvider.defaultOptions()`.
  - Instantiates a `Program` with the IDL, program ID, and provider.
  - Defines a `handleClick` function to call the `initialize` instruction, similar to the test script.
  - Renders a button to trigger the instruction execution.

---

### Additional Notes
- **Dependencies**:
  - The page assumes familiarity with dependencies like `@coral-xyz/anchor`, `@solana/web3.js`, and `@solana/wallet-adapter-react`.
- **Type Safety**:
  - Anchor’s TypeScript client emphasizes type safety through generated types in `target/types`.
- **Context**:
  - The examples focus on a simple `Counter` program, demonstrating initialization and increment operations.
- **References**:
  - The page links to related documentation (e.g., Rust client, IDL, Anchor CLI) but does not include external citations or further reading.[](https://www.anchor-lang.com/docs/clients/typescript)