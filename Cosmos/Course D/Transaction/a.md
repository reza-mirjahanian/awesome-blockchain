Transaction process from an end-user perspective
------------------------------------------------

**Decide** on the messages to put into the transaction. This is normally done with the assistance of a wallet or application and a user interface.

**Generate** the transaction using the Cosmos SDK's `TxBuilder`. `TxBuilder` is the preferred way to generate a transaction.


**Sign** the transaction. Transactions must be signed before a validator includes them in a block

**Broadcast** the signed transaction using one of the available interfaces so that it eventually reaches validators.


--------------