There's a variety of information about timeouts in [Running in production](https://docs.cometbft.com/v0.38/core/running-in-production#configuration-parameters). You can also find more detailed explanation in the paper describing the Tendermint consensus algorithm, adopted by CometBFT:


Note that in a successful round, the only timeout that we absolutely wait no matter what is `timeout_commit`. Here's a brief summary of the timeouts:

-   `timeout_propose` \= how long a validator should wait for a proposal block before prevoting nil
-   `timeout_propose_delta` \= how much `timeout_propose` increases with each round
-   `timeout_prevote` \= how long a validator should wait after receiving +2/3 prevotes for anything (ie. not a single block or nil)
-   `timeout_prevote_delta` \= how much the `timeout_prevote` increases with each round
-   `timeout_precommit` \= how long a validator should wait after receiving +2/3 precommits for anything (ie. not a single block or nil)
-   `timeout_precommit_delta` \= how much the `timeout_precommit` increases with each round
-   `timeout_commit` \= how long a validator should wait after committing a block, before starting on the new height (this gives us a chance to receive some more precommits, even though we already have +2/3)