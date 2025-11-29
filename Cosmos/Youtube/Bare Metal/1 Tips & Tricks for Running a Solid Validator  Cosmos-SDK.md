
https://github.com/strangelove-ventures/horcrux

https://hackmd.io/@mLfyGjG0Qga6LNmJro90zQ/SJSXMg6Sa

Sentry nodes are a type of network node used in some blockchain systems to enhance security and performance, typically by acting as a secure intermediary between a validator node and the public internet. They can be configured as full nodes or light clients and are designed to protect validator nodes from direct exposure to the public network. In specific projects like [Xai]


***

### Validator Experience and Feedback Loop

Running a **validator** (starting with Osmosis, now operating on six chains) provides essential product insight. I'm active in validator chats, where I track security halts, chain halts, and upgrades, which provides exceptional **product feedback** on issues like poor UX or bugs. I monitor our validator nodes to diagnose the cause of events, such as a **memory spike**. For in-depth metrics and profiling of the Cosmos SDK, we piggyback off the full nodes run by our team to avoid the overhead of managing more nodes.

***

### Validator Pain Points and Cost Optimization

When starting a validator, you must tailor the machine specs to the chain's throughput. For a chain with low throughput, start with a cheaper machine. If the node starts **missing blocks**, you must identify the bottleneck—is it **CPU, RAM, or Disk**?—and then upgrade the node accordingly. Many people initially over-spec their machine. **Smaller validators** must be cognizant of **cost and margin**, making the necessary **trade-off** between running a heavy-duty machine on a busy chain and a lighter machine on one with lower activity.

I am a firm believer that validators should always be **building from source** instead of downloading pre-built binaries, as it is much safer. Although a chain may provide **GPG signing** of binaries, compiling from source is my default recommendation.

For general cost optimization, there are several key steps:
* On the validator node, **turn off most non-essential services**, such as **gRPC**.
* Set **pruning** to a reasonable level to avoid the need for large disk space.
* A smaller 500GB disk might require more manual maintenance (which can be scripted) compared to a 2TB disk, but this is a fine **trade-off** between time and cost.
* For high availability, using a **remote signer** with tools like **Horcrux** or normal **tmkms** and a system for automated failover is often enough.
* Running multiple nodes is recommended for those new to server operations until they better understand the devops aspects, but for experienced operators, running four nodes is often **overkill**; two can suffice.

Regarding **Sentry Nodes**, we run them on high-value networks, but I believe they are **overkill** for many chains. I have not seen a validator go down because of a DoS attack. If a chain were getting DoS'd, an attacker could likely see the Sentry's public connections, identify the private connection to the validator, and DoS the sentries, effectively achieving the same result as DoS'ing the validator directly. Sentry nodes only truly act as a firewall if you are using a **VPC** (Virtual Private Cloud) inside a data center.

***

### Validator Credibility and Chain Selection

When choosing which chains to validate, our main criteria are:
* Finding the **product interesting**.
* Having a **vested interest** in the project.
* Considering the **maintenance overhead**—chains that move too fast create difficulty in syncing and staying caught up.

For us, running a validator is a **passion and product feedback cycle**, not a full-time job.

Validator credibility is important. The newest generation of validators who spin up a node on every chain and then spin it down because the token price fell are not building trust. This is a problem because their choice to validate effectively binds their credibility to that chain, and their delegators who staked with them might be left on a dead chain if the team disappears. Validators should rethink their strategy if they are spinning up on every chain. Being on too many chains results in high overhead, which is a choice of **quantity over quality**.

***

### Outlook and Major Challenges

The major challenges for validators in the coming years will be:

1.  **Interchain Security V1**: This technology could be costly in terms of the **human element** for smaller validators, who will now potentially need to run on multiple other chains and maintain governance there, in addition to The Hub.

2.  **UX Improvements**: We need to be **less complacent** with the current poor UX, both on the user and validator front. Validators should **complain more** to the SDK and Tendermint teams about the issues they are facing so we can tackle them and **be prepared for the next wave**. If only a few people are questioning the designs and UX, we are failing to push the ecosystem forward.

We are running the infrastructure and indexing for **Numia**, a data analytics platform, which operates on open-source data from The Hub. This infrastructure work is one of the things that brings life to Binary Holdings.


