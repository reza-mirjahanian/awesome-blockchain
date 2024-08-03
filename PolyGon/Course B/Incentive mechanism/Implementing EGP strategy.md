
## How to avert rejection of transactions

The first step is to ensure that users sign transactions with sufficient gas price, and thus ensure the transactions are included in the L2’s pool of transactions.

Polygon zkEVM’s strategy is to use pre-execution of transactions so as to estimate each transaction’s possible gas costs.

### Suggested gas prices

Due to fluctuations in the L1 gas price, the L2 network polls for L1 gas price every 5 seconds.

The polled L1 gas prices are used to determine the appropriate L2 gas price to suggest to users.

Since the L1 gas price is likely to change between when a user signs a transaction and when the transaction is pre-executed, the following parameters are put in place:

-   A  5-minute interval of several suggested gas prices, called  MinAllowedPriceInterval.
-   During the  MinAllowedPriceInterval, the user’s transactions can be accepted for pre-execution, provided the  SignedGasPrice  is higher than the lowest suggested gas price in the interval.
-   The lowest among the suggested gas prices is called  L2MinGasPrice.