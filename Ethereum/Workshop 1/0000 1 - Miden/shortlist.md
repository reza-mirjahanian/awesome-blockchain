
---

1. Most blockchains today are **transparent**, limiting privacy and mass adoption.
2. Transparent chains require every node to **re-execute transactions** for verification.
3. Re-execution leads to **execution bloat** and **state bloat** over time.
4. Transparent chains make **all activities traceable**, harming privacy.
5. Chain analysis tools will become cheap and widely accessible.
6. Complete traceability discourages mainstream blockchain adoption.
7. Smart contracts on transparent chains act as **honeypots** for attackers.
8. Bugs are more easily exploitable when contract states are public.
9. Miden’s core idea: **push state and execution to the client side**.
10. Users keep their own state and generate **zero-knowledge proofs (ZKPs)**.
11. The network only **verifies proofs**, not raw transactions.
12. Proofs are hard to generate but **very fast to verify**.
13. This model enables **massive parallel execution** of transactions.
14. Current blockchains evolved from Bitcoin → Ethereum → Solana → SUI.
15. Miden is proposed as the **next step in scalability + privacy**.
16. Privacy should be a **fundamental right** and also a **practical necessity**.
17. Encrypting data on transparent chains (e.g., Tornado Cash) is **10x more expensive**.
18. Client-side proving is a **cheaper way to achieve privacy**.
19. Mass adoption requires **100,000 TPS** and support for **10B accounts**.
20. Future users may include **AI agents** with blockchain accounts.
21. Estimated state requirements: **4–40 TB** depending on asset storage.
22. Current blockchain throughput is \~100M gas/sec; need \~10B gas/sec.
23. Bandwidth for mass adoption: \~10–60 MB/sec.
24. Ethereum today runs at \~2.5 MB/sec bandwidth.
25. Miden introduces **client-side proving** as a new execution model.
26. Transactions in Miden are executed and proven locally.
27. The L2 or Ethereum only verifies the **valid proof**.
28. In data handling, Miden commits only **commitments**, not full data.
29. Each user can effectively act as their own **zk-rollup**.
30. Miden architecture uses the **actor model**.
31. Actors = accounts (self-contained state machines).
32. Messages between actors = **nodes**.
33. Each transaction = **state change of a single account**.
34. Asset transfer requires two transactions: one to create a node, another to consume it.
35. Accounts in Miden are similar to Ethereum accounts but store **multiple assets directly**.
36. No need for ERC-20 contracts → improves parallelism.
37. Nodes are like advanced UTXOs with **executable scripts**.
38. Spend conditions for nodes are encoded as scripts.
39. To consume a node, a user must satisfy its script conditions.
40. Every transaction produces a **ZKP proof of correct execution**.
41. From the network’s view: account = old state commitment → new state commitment + proof.
42. This resembles how rollups interact with Ethereum.
43. Proofs can be **batched recursively** into one block proof.
44. Recursive proof verification ensures efficiency.
45. The network achieves **high scalability** by verifying compact block proofs.
46. Potential throughput: **10,000+ TPS** with dedicated hardware (FPGAs/VPUs).
47. Bottleneck: how many proofs can be batched and verified per block.
48. Users must store their own state → new responsibility.
49. Local proving currently takes \~10 seconds in browser; goal = \~1 second.
50. Delegated proving servers can help reduce latency.
51. Shared contracts (e.g., AMMs) require **network execution** to avoid race conditions.
52. Two storage modes: **public accounts** (full state) vs **private accounts** (commitments).
53. Privacy is **cheaper** in Miden (opposite of Ethereum).
54. Public storage costs more because it requires more on-chain data.
55. Node database uses **Merkle Mountain Range** for efficient append-only storage.
56. Nullifier database prevents double-spends using unlinkable hashes.
57. Nullifiers break linkability between node creation and consumption.
58. Inspired by Zcash’s commitment + nullifier model.
59. Global state consists of three DBs: Accounts, Nodes, Nullifiers.
60. Validators only need commitments, not full state, to produce/verify blocks.

---
