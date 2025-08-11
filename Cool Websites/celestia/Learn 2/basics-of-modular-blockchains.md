#### Summary

1.  The first approach to building blockchains was a monolithic design where a single blockchain does everything.
2.  The idea of a modular blockchain is that it can specialize in a couple of functions instead of trying to do everything. Namely, modular blockchains decouple consensus from execution.
3.  The monolithic approach causes some inherent problems with scaling, which includes expensive hardware, limited control, and high overhead.

-------------
##### The functions that modular blockchains can specialize in are:

-   **Execution:** Process transactions.
-   **Settlement:** Dispute resolution and bridge (optional).
-   **Consensus:** Order transactions.
-   **Data availability:** Ensure data is available.
-   

---------

This monolithic approach causes some inherent problems with scaling while retaining the core principle of decentralization:

-   **High hardware requirements:** Monolithic chains can increase the number of transactions they process, but it comes at a cost. That cost is higher hardware requirements for nodes to verify the chain.
-   **Bootstrapping validators:** Deploying a new monolithic blockchain requires the overhead of bootstrapping a secure validator set and maintaining a consensus network.
-   **Limited control:** Apps must follow the predetermined rules of the chain they deploy to. This includes the programming model, ability to fork, and community culture, among others.
-   

----------------

#### Sovereignty

When an app is built on a shared monolithic blockchain, it is bound by predetermined rules. The rules might be around social consensus (when it's okay to hard fork) or around technical rules (what programming languages you can write smart contracts in).

Modular blockchains enable control over the rules of an application through sovereignty. Developers can make changes to the tech stack without permission from outside applications. For example, they could make a more performant execution environment or change how transaction processing works - who wants parallel transactions?

Importantly, sovereignty gives independence. Developers and the community can freely set the rules for their sovereign chain that aligns with their app and community's ethos. It is sovereignty that places autonomy back in the hands of the community.