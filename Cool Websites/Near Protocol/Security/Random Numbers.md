
# Random Numbers

When writing smart contracts in NEAR you have access to a  `random seed`  that enables you to create random numbers/strings within your contract.

This  `random seed`  is  **deterministic and verifiable**: it comes from the validator that produced the block signing the previous block-hash with their private key.

The way the random seed is created implies two things:

-   Only the validator mining the transaction  **can predict**  which random number will come out.  **No one else**  could predict it because nobody knows the validator's private key (except the validator itself).
    
-   The validator  **cannot interfere**  with the random number being created. This is because they need to sign the previous block, over which (with a high probability) they had no control.
    

However, notice that this still leaves room for three types of attacks from the validator:

1.  Frontrunning
2.  Gaming the input
3.  Refusing to mine the block.