---
title: "Static Application Security Testing of Consensus-Critical Code in the Cosmos Network"
layout: post
tags:
  - security
  - blockchain
comments: true
---

How effective are Static Application Security Testing (SAST) tools at finding bugs in consensus-critical code in application-specific blockchains?
That's the question we addressed in our new paper "Static Application Security Testing of Consensus-Critical Code in the Cosmos Network" that recently got accepted at the 
5th Conference on Blockchain Research & Applications for Innovative Networks and Services ([BRAINS 2023](https://brains.dnac.org/)).

<!--more-->

This past semester, my student Jasper Surmont (supported by my PhD student Weihong Wang) did a deep technical study of so-called
[application-specific blockchains](https://www.alchemy.com/overviews/what-is-an-appchain)
or "appchains" as they are sometimes called. Specifically, he studied the [Cosmos network](https://cosmos.network/) which has an elaborate SDK that allows
developers to write their own blockchain logic in Golang.

Cosmos appchains are essentially [replicated state machines](https://en.wikipedia.org/wiki/State_machine_replication),
kept in sync through the [Tendermint](https://docs.tendermint.com/v0.34/introduction/what-is-tendermint.html)
(now [CometBFT](https://cometbft.com/)) consensus engine. In Cosmos, the interface between
application and blockchain is particularly well-defined via the [Application-Blockchain Interface](https://docs.cosmos.network/main/intro/sdk-app-architecture) (ABCI).
We were interested in identifying software bugs specifically
in those parts of the application that interface with the consensus engine via ABCI, the so-called __consensus-critical code__. Any issues with this code have the potential
to disrupt the state machine replication and hence to grind the blockchain to a halt (which has [happened before in practice](https://thenewscrypto.com/thorchain-network-back-online-after-20-5-hours-of-outage/)).

Jasper ended up identifying and improving a set of [CodeQL](https://codeql.github.com/docs/) rules that look for specific vulnerabilities
(many of them having to do with introducing non-determinism
in the code in one way or another - which is fatal to a blockchain's requirement for deterministic execution). He then scanned a representative sample of codebases
that use the Cosmos SDK to identify how prevalent these issues are, and whether the rules were sufficiently precise to avoid so-called "false positives"
(cases that the rule flags as potential bugs which in practice turn out not to be really bugs after all). Reducing false positives for Static Application Security
Testing (SAST) is important: it makes these tools more useful and usable, thereby ultimately strengthening the security of app-specific blockchains.

If you're interested in the details, please refer to our paper which recently got accepted at [BRAINS 2023](https://brains.dnac.org/). You can find our pre-print copy on [arXiv](https://arxiv.org/abs/2308.10613) and the
data associated with the paper, including the refactored CodeQL rules on [GitHub](https://github.com/JasperSurmont/cosmos-sdk-codeql).