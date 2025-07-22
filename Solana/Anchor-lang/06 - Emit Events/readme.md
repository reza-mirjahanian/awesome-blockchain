Anchor provides two macros for emitting events in your programs:

1.  `emit!()` \- Emits events directly to program logs. This is the simpler, though program logs may be truncated by data providers in some cases
2.  `emit_cpi!()` \- Emits events through a Cross Program Invocation (CPI) by including the event data in the instruction data.

The `emit_cpi()` approach was introduced an alternative to program logs, which can sometimes be truncated by data providers. While CPI instruction data is less likely to be truncated, this approach does incur **additional compute costs** from the Cross Program Invocation.

For more robust solutions for events, consider geyser gRPC services by [Triton](https://docs.triton.one/project-yellowstone/dragons-mouth-grpc-subscriptions) or [Helius](https://docs.helius.dev/data-streaming/geyser-yellowstone).

### [`emit`](https://www.anchor-lang.com/docs/features/events#emit)

The [`emit!()`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/event/src/lib.rs#L101-L109) macro provides a way to emit events through program logs. When called, it:

1.  Uses the [`sol_log_data()`](https://github.com/anza-xyz/agave/blob/c2b350023ba849d1b33142592264aaa51fcb7f1e/sdk/program/src/log.rs#L115-L124) syscall to write the data to program logs
2.  Encodes the event data as a [base64 string](https://github.com/anza-xyz/agave/blob/c2b350023ba849d1b33142592264aaa51fcb7f1e/program-runtime/src/stable_log.rs#L46-L61) prefixed with `Program Data:`

To receive emitted events in your client application, use the [`addEventListener()`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/ts/packages/anchor/src/program/event.ts#L74-L123) method. This method automatically [parses and decodes](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/ts/packages/anchor/src/program/event.ts#L232-L253) event data from the program logs.


```rust
#[program]
pub mod event {
    use super::*;
 
    pub fn emit_event(_ctx: Context<EmitEvent>, input: String) -> Result<()> {
        emit!(CustomEvent { message: input });
        Ok(())
    }
}
 
``` 

```ts
const program = anchor.workspace.Event as Program<Event>;
 
  it("Emits custom event", async () => {
    // Set up listener before sending transaction
    const listenerId = program.addEventListener("customEvent", event => {
      // Do something with the event data
      console.log("Event Data:", event);
    });

``` 

### [`emit_cpi`](https://www.anchor-lang.com/docs/features/events#emit_cpi)

The [`emit_cpi!()`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/event/src/lib.rs#L155-L195) macro emits events through Cross Program Invocations (CPIs) to the program itself. The event data is encoded and included in the CPI's instruction data (instead of program logs).

To emit events through CPIs, you need to enable the `event-cpi` feature in your program's `Cargo.toml`:

Cargo.toml

```
[dependencies]anchor-lang = { version = "0.31.1", features = ["event-cpi"] }
```
