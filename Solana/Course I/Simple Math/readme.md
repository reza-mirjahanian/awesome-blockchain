### Convert this Solidity to Solana:

```Solidity
contract Day2 {

	event Result(uint256);
	event Who(string, address);
	
	function doSomeMath(uint256 a, uint256 b) public {
		uint256 result = a + b;
		emit Result(result);
	}

	function sayHelloToMe() public {
		emit Who("Hello World", msg.sender);
	}
}


```
---------

### Passing a string
Rust

```rust
pub fn initialize(ctx: Context<Initialize>,
                  a: u64,
                  b: u64,
                  message: String) -> Result<()> {
    msg!("You said {:?}", message);
    msg!("You sent {} and {}", a, b);
    Ok(())
}

// added this function
pub fn array(ctx: Context<Initialize>,
             arr: Vec<u64>) -> Result<()> {
    msg!("Your array {:?}", arr);
    Ok(())
}

```
TS
```ts
it("Is initialized!", async () => {
  // Add your test here.
  const tx = await program.methods.initialize(new anchor.BN(777), new anchor.BN(888), "hello").rpc();
  console.log("Your transaction signature", tx);
});

// added this test
it("Array test", async () => {
  const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888)]).rpc();
  console.log("Your transaction signature", tx);
});

```

Logs
```bash
Transaction executed in slot 368489:
  Signature: 3TBzE3NddEY8KREv1FSXnieoyT6G6iNxF1n4hJHCeeWhAsUward3MEKm9WJHV4PMjPxeN2jRSRC9Rq8FUKjXoBQR
  Status: Ok
  Log Messages:
    Program 8o3ehd3XnyDocd9hG1uz5trbmSRB7gaLaE9BCXDpEnMY invoke [1]
    Program log: Instruction: Initialize
    Program log: You said [777, 888]
    Program 8o3ehd3XnyDocd9hG1uz5trbmSRB7gaLaE9BCXDpEnMY consumed 1587 of 200000 compute units
    Program 8o3ehd3XnyDocd9hG1uz5trbmSRB7gaLaE9BCXDpEnMY success


```

### overflow-checks = true in Cargo.toml
