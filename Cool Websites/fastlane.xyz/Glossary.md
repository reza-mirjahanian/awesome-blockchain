| Searcher | A trader / bot operator specializing in "searching" for and executing MEV-related transactions. These are the primary "users" of the PFL protocol. |
| --- |  --- |

| Validator | A special type of node operator who is nominated to add new blocks to the blockchain. |
| --- |  --- |


| PFL Sentry | A heavily modified fork of Bor that runs the PFL auction. |
| --- |  --- |
| PFL Relay | The RPC endpoint / API that searchers submit PFL bundles to. This endpoint takes care of distributing transactions to our global network of PFL Sentries. |
| PFL Bundle | A collection of exactly 2 transactions (the Opportunity Transaction, followed by the Searcher Transaction) that a Searcher can submit to the PFL Relay to participate in an auction. |
| Opportunity Transaction | This is the transaction that creates an "opportunity" on the blockchain for the searcher to trade behind, such as an oracle update. |
| Searcher Transaction | This is the transaction that registers the searchers bid in the auction, pays their success fee and contains parameters that are used to call the searchers callback if they win. |