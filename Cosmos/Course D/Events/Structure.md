Two elements stand out in the previous:

-   A `type` to categorize the event at a high level. For example, the Cosmos SDK uses the `message` *type* to filter events by `Msg`.
-   A list of `attributes`, which are key-value pairs giving more information on the categorized event. For example, we can filter events by key-value pairs using `message.action={some_action}`, `message.module={some_module}` or `message.sender={a_sender}` for the `message` type.

![](https://ida.interchain.io/hi-tip-icon.svg)

Make sure to add `'` (single quotes) around each attribute value to parse the attribute values as strings.

Events, their type, and attributes are defined on a per-module basis in the module's `/types/events.go` file. Each module additionally documents its events under `spec/xx_events.md`.

Events are returned to the underlying consensus engine in response to the following ABCI messages:

-   `BeginBlock`
-   `EndBlock`
-   `CheckTx`
-   `DeliverTx`

Events are managed by an abstraction called the `EventManager`. Events are triggered from the module's Protobuf `Msg` service with `EventManager`. 