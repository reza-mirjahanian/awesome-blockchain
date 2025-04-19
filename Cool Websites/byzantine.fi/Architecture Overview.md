Byzantine Architecture involves several components/entities (from left to right):

-   **The Restaker:** User providing liquidity into a Byzantine Vault to secure Ethereum as well as one or multiple AVSs/networks. Restakers are rewarded in tokens by the networks for the security they provide.

-   **Byzantine front-end:** The Byzantine dApp allows creating new Byzantine vaults and depositing into (or withdrawing from) it.

-   **Partner's front-end:** A partner's platform or dApp that allows Byzantine Vault creation, interaction and curation through the Byzantine SDK, usable permissionlessly.

-   **Byzantine Vaults:** The central piece of the architecture. ByzVaults can be created permissionlessly and for any strategy or desired risk level. A ByzVault has a dedicated restaking strategy that can be adjusted at any time by the defined curators.

-   **The Curator** (optional, can be the Vault creator themselves): In charge of rebalancing and managing the vault strategy.

-   **The Node Operators:** Entities operating Staking Validators (on Ethereum) and/or Restaking Operators (on the AVSs/networks). The method by which Validators are associated with Native Vaults depends on the creator's node operator selection (see the [types of Native Vaults](https://docs.byzantine.fi/byzantine-vaults/types-of-native-vaults)).

-   **The Restaking Protocols:** Ecosystems of Proof-of-Stake networks (known as AVSs on Eigen Layer, networks on Symbiotic, BSNs on Babylon, NCNs on Jito, ...) seeking economic shared security and rewarding the restakers.