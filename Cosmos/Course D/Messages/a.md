Messages are one of two primary objects handled by a module in the Cosmos SDK. The other primary object handled by modules is queries. While messages inform the state and have the potential to alter it, queries inspect the module state and are always read-only.

In the Cosmos SDK, a **transaction** contains **one or more messages**. Modules process the messages after the transaction is included in a block by the consensus layer.

transactions must be signed before a validator includes them in a block. Every message in a transaction must be signed by the addresses as specified by `GetSigners`.
The Cosmos SDK currently allows signing transactions with either `SIGN_MODE_DIRECT` or `SIGN_MODE_LEGACY_AMINO_JSON` methods.
When an account signs a message it signs an array of bytes. This array of bytes is the outcome of serializing the message. For the signature to be verifiable at a later date, this conversion needs to be deterministic. For this reason, you define a canonical bytes-representation of the message, typically with the parameters ordered alphabetically.