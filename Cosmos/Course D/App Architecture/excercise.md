
[#](https://ida.interchain.io/academy/2-cosmos-concepts/1-architecture.html#test-yourself-a-pseudo-coding-exercise)Test yourself - a pseudo-coding exercise
-----------------------------------------------------------------------------------------------------------------------------------------------------------


The previous expandable is meant as a design exercise. If you want to go from the design phase to the **implementation** phase, head to one (or both) of the following hands-on exercises:

1.  [SDK v0.50 Native](https://ida.interchain.io/hands-on-exercise/0-native/), which is a short exercise to partially build your own checkers chain without the help of external tools, using SDK v0.50.
2.  [Rebuild Your Cosmos Chain With Ignite](https://ida.interchain.io/hands-on-exercise/1-ignite-cli/), which is a completely separate exercise, using Ignite CLI and an older version of the Cosmos SDK to start from scratch.
    Relevant to this section on ABCI is the [Auto-Expiring Games](https://ida.interchain.io/hands-on-exercise/2-ignite-cli-adv/4-game-forfeit.html) section, where you use the Cosmos SDK to implement expiration in `EndBlock`. Be warned that this is an advanced section of the hands-on-exercise.
    In the same vein, and advanced too, is the [Add Leaderboard Module](https://ida.interchain.io/hands-on-exercise/4-run-in-prod/3-add-leaderboard.html) section, where you add an action in `EndBlock` to reduce computation burdens.