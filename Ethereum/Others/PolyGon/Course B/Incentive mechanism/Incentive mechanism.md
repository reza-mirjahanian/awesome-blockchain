Incentive mechanism
===================

In order to ensure the system's sustainability, actors must be compensated for correctly performing their roles and giving the protocol finality.

Unless otherwise specified, the measures and rules presented here apply to cases in which the Sequencer and Aggregator roles are decentralised (i.e., when there are no trusted sequencer and no trusted aggregator).

L2 transaction fees and sequencing fees
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The **native currency used in L2 is `Bridged Ether`**, which originates from L1. This is the currency that is used to pay L2 transaction fees. **It can be transferred at a 1:1 exchange ratio from L1 to L2 and vice versa**.

The **Sequencer** earns the transaction fees paid by L2 users for submitting transactions, and thus gets paid directly in `Bridged Ether`. The amount of fees paid depends on the gas price, which is set by users based on how much they are willing to pay for the execution of their transactions.

To incentivize the **Aggregator** for each batch sequenced, the Sequencer must lock a number of MATIC tokens in the L1 `PolygonZkEVM.sol` Contract proportional to the number of batches in the sequence. The number of MATIC tokens **locked per batch** sequenced is saved in the variable `batchFee`.

![alt text](image.png)


The net Ether value earned by the Sequencer for sequencing a batch sequence is represented by the following expression:

![alt text](image-1.png)


-   `totalL2TxGasFees` is the total sum of fees gathered from all L2 transactions included in the sequence of batches,
-   `L1SeqTxGasFee` is the Sequencing transaction gas fee paid in L1,
-   `batchFee` is the storage variable in **PolygonZkEVM.sol** contract,
-   `nBatches` is the number of batches in the sequence,
-   `MATIC/ETH` is the price of MATIC token expressed in ETH.



Aggregation reward
-------------------------------------------------------------------------------------------------------------------------------------------------

The Aggregator also needs compensation for correctly fulfilling its role.

The **number of MATIC tokens earned** by the Aggregator each time it aggregates a sequence, denoted by `batchReward`, is determined by the **total contract MATIC balance** and the **number of batches aggregated**.

The MATIC earned per batch aggregated is calculated by the L1 `PolygonZkEVM.sol` contract prior to sequence aggregation using the following expression:

![alt text](image-2.png)

where:

-   `L1AggTxGasFee` is the Aggregation transaction gas fee paid in L1,
-   `batchReward` is the quantity of MATIC earned per batch aggregated,
-   `nBatches` is the number of batches in the sequence,
-   `MATIC/ETH` is the price of MATIC token expressed in ETH.



## Variable batchFee re-adjustments

The  `batchFee`  is automatically adjusted with every aggregation of a sequence by an independent Aggregator.

This happens when the trusted aggregator isn’t working properly and the  `batchFee`  variable needs to be changed to encourage aggregation. Further information on the trusted aggregator’s inactivity or malfunctioning is provided in upcoming sections.

An internal method called  `_updateBatchFee`, is used to adjust  `batchFee`  storage variable.

`function _updateBatchFee(uint64 newLastVerifiedBatch) internal` 

The admin defines two storage variables that are used to  **tune the fee adjustment function**:

-   `veryBatchTimeTarget`, which is  **the targeted time of the verification of a batch**, so the  `batchFee`  variable is updated to achieve this target, and
-   `multiplierBatchFee`, which is the batch fee multiplier, with 3 decimals ranging from 1000 to 1024.

The function  `_updateBatchFee`  first determines how many of the aggregated batches are late. That is, those who are in the sequence but have not yet been aggregated.

Second, how much time has passed, as indicated by  `veryBatchTimeTarget`.

The  `diffBatches`  variable represents the difference between late batches and those below the target, and its value is limited by a constant called  `MAX BATCH MULTIPLIER`, which is set to 12.

