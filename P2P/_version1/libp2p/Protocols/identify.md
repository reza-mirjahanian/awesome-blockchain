Overview
--------

There are two variations of the identify protocol, `identify` and `identify/push`.

### `identify`

The `identify` protocol has the protocol id `/ipfs/id/1.0.0`, and it is used to query remote peers for their information.

The protocol works by opening a stream to the remote peer you want to query, using `/ipfs/id/1.0.0` as the protocol id string. The peer being identified responds by returning an `Identify` message and closes the stream.

### `identify/push`

The `identify/push` protocol has the protocol id `/ipfs/id/push/1.0.0`, and it is used to inform known peers about changes that occur at runtime.

When a peer's basic information changes, for example, because they've obtained a new public listen address, they can use `identify/push` to inform others about the new information.

The push variant works by opening a stream to each remote peer you want to update, using `/ipfs/id/push/1.0.0` as the protocol id string. When the remote peer accepts the stream, the local peer will send an `Identify` message and close the stream.

Upon receiving the pushed `Identify` message, the remote peer should update their local metadata repository with the information from the message. Note that missing fields should be ignored, as peers may choose to send partial updates containing only the fields whose values have changed.