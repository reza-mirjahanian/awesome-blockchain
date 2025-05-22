How do the tests know about the initialize function?
----------------------------------------------------

When Anchor builds a Solana program, it creates an IDL (Interface Definition Language).

This is stored in `target/idl/anchor_function_tutorial.json.` This file is called `anchor_function_tutorial.json` because `anchor_function_tutorial` is the name of the program. Note that anchor converted the dashes to underscores!

Let's open it.

```
`{
  "version": "0.1.0",
  "name": "anchor_function_tutorial",
  "instructions": [
    {
      "name": "boatyMcBoatface",
      "accounts": [],
      "args": []
    }
  ]
}
`
```

The list of "instructions" is the public facing functions that the program supports, roughly equivalent to the external and public functions on an Ethereum contract. **An IDL file in Solana plays a similar role as the ABI file in Solidity, specifying how to interact with the program's/contract's.**

> We saw earlier that our function didn't take any arguments, so that's why the `args` list is empty. We'll explain later what "accounts" is.

One thing that stands out: functions in Rust are snake\_cased, but Anchor formats them in JavaScript land as camelCased. This is to respect the conventions of the languages: Rust tends to use snake case, and JavaScript typically uses camel case.

This JSON file is how the "methods" object knows what functions to support.

When we run the test, we expect it to pass, which means the test is correctly calling the Solana program: