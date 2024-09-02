`EventManager` tracks a list of events for the entire execution flow of a transaction, or `BeginBlock`/`EndBlock`. `EventManager` implements a simple wrapper around a slice of event objects, which can be emitted from and provide useful methods. The most used method for Cosmos SDK module and application developers is `EmitEvent`.

Module developers should handle event emission via `EventManager#EmitEvent` in each message handler and in each `BeginBlock` or `EndBlock` handler accessed via the `Context`. Event emission generally follows this pattern:

```
func (em *EventManager) EmitEvent(event Event) {
    em.events = em.events.AppendEvent(event)
}

```

Each module's handler function should also set a new `EventManager` to the context to isolate emitted events per message:

```
func NewHandler(keeper Keeper) sdk.Handler {
    return func(ctx sdk.Context, msg sdk.Msg) (*sdk.Result, error) {
        ctx = ctx.WithEventManager(sdk.NewEventManager())
        switch msg := msg.(type) {
            // event types
        }
    ...
    }
}

```