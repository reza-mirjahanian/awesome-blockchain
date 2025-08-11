Batches need to be sequenced and validated before they can become a part of the L2 virtual state.

The trusted sequencer successfully adds a batch to a sequence of batches using the L1 `PolygonZkEVM.sol` contract's `sequencedBatches` mapping, which is basically a storage structure that holds the queue of sequences defining the virtual state.

```
// SequenceBatchNum --> SequencedBatchData
mapping(uint64 => SequencedBatchData) public sequencedBatches;

```

Batches must be a part of the array of batches to be sequenced in order to be sequenced. The trusted sequencer invokes the `PolygonZkEVM.sol` contract, which employs its `sequenceBatches` mapping, which accepts an array of batches to be sequenced as an argument. Please see the code snippet provided below.

```
function sequenceBatches (
 BatchData[] memory batches
) public ifNotEmergencyState onlyTrustedSequencer

```

The below figure shows the logic structure of a sequence of batches


Batch data minimal storage
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Since storage operations on L1 are very costly in terms of gas consumption, it is essential to use them as sparingly as possible. To accomplish this, storage slots (or mapping entries) are used solely to store a sequence commitment.

Each mapping entry commits two batch indices,

-   Last batch of the previous sequence as value of `SequencedBatchData` struct,
-   Last batch of the current sequence as mapping key,

along with the accumulated hash of the last batch in the current sequence and a timestamp.

It is important to note that only the accumulated hash of the last batch in the sequence is saved; all others are computed on the fly in order to obtain the last one.

As previously stated, the hash digest becomes a commitment of the entire batch chain. Batch indices also commit useful information like the number of batches in the sequence and their position in the batch chain. The timestamp anchors the sequence to a specific point in time.

The data availability of the L2 transactions is guaranteed because the data of each batch can be recovered from the calldata of the sequencing transaction, which is not part of the contract storage but is part of the L1 state.

Finally a `SequenceBatches` event emits.

```
`eventSequenceBatches(uint64indexednumBatch)
`
```

Once the batches are successfully sequenced in L1, all zkEVM nodes can sync their local L2 state by fetching the data directly from the L1 `PolygonZkEVM.sol` contract, without having to rely on the trusted sequencer alone. This is how the L2 virtual state is reached.


### `inputSnark`

`inputSnark` is a 256-bits unique cryptographic representative of a specific L2 state transition, which is used as public argument. It is computed as `sha256 mod % _RFIELD` hash of a bytes string called `snarkHashBytes` (modulo operator is needed due to math primitives used in SNARKs).

`snarkHashBytes` array is computed by a smart contract's function called `getInputSnarkBytes` and it is an ABI-encoded packed string of the following values:

-   `msg.sender`: Address of trusted aggregator.
-   `oldStateRoot`: L2 state root that represents the L2 state before the state transition that wants to be proven.
-   `oldAccInputHash`: Accumulated hash of the last batch aggregated.
-   `initNumBatch`: Index of the last batch aggregated.
-   `chainID`: Unique chain identifier.
-   `newStateRoot`: L2 state root that represents the L2 state after the state transition that is being proved.
-   `newAccInputHash`: Accumulated hash of the last batch in the sequence that is being aggregated.
-   `newLocalExitRoot`: Root of the bridge's L2 exit Merkle tree at the end of sequence execution.
-   `finalNewBatch`: Number of the final batch in the execution range.

`inputSnark` represents all the L2 transactions of a specific L2 state transition, executed in a specific order, in a specific L2 (`chainID`), and proved by a specific trusted aggregator (`msg.sender`). The `trustedVerifyBatches` function not only verifies the validity of the zero-knowledge proof, but it also checks that the value of `inputSnark` corresponds to an L2 state transition that is pending to be aggregated.

If the internal call to `_verifyAndRewardBatches` returns `true`, it means that the sequence of batches is verified successfully, and then the `newStateRoot` argument is added to the `batchNumToStateRoot` mapping. The index of the last batch in the sequence is used as the key for the entry.

Finally a `TrustedVerifyBatches` event is emitted.

```
`event TrustedVerifyBatches (
  uint64 indexed numBatch,
  bytes32 stateRoot,
  address indexed aggregator
);
`
```

Once the batches have been successfully aggregated in L1, all zkEVM nodes can validate their local L2 state by retrieving and validating consolidated roots directly from the L1 consensus contract (`PolygonZkEVM.sol`). As a result, the L2 consolidated state has been reached.