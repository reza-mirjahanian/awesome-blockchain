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

```
TS
```ts
it("Is initialized!", async () => {
  // Add your test here.
  const tx = await program.methods
    .initialize(
       new anchor.BN(777), new anchor.BN(888), "hello").rpc();
    console.log("Your transaction signature", tx);
});

```