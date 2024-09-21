
The Problem: Self-Built Blocks and Base Fee Volatility
======================================================

The speaker argues that self-built or "altruistic" blocks introduce unnecessary volatility in the base fee. This issue:

-   Is not great today
-   Might be getting worse
-   May be something to worry about moving forward

Background: Gas and Its Importance
==================================

The speaker, representing Block Native, emphasizes the importance of gas:

-   Gas matters to everyone, all the time
-   It's a core signal affecting all blockchain participants
-   Gas is a highly leveraged metric

Gas as a Price Signal
---------------------

-   Gas is compared to the pendulum in physics
-   The precision of the gas signal affects:
    -   All arbitrage on the network
    -   Spreads reflecting asset prices
-   A more precise gas signal leads to better network efficiency

Gas Market Complexity
---------------------

-   Gas is essentially the spot price for block space
-   Ethereum's gas market is complex and multi-dimensional
-   Contrast with Bitcoin's simpler gas market

New Discovery: Observing Gas Markets
====================================

The speaker introduces a new discovery made recently:

-   Analogy to improved telescopes allowing new observations
-   Block Native upgraded their Transaction Explorer tool
-   New observability capabilities revealed unexpected patterns

Block Building Methods in Ethereum
==================================

Two ways to build blocks in Ethereum:

1.  **Outsource to MEV Boost network**
    -   Specialized actors build blocks
    -   Includes MEV (Maximal Extractable Value)
    -   Generally more valuable blocks
2.  **Self-built or "altruistic" blocks**
    -   Built locally
    -   Avoids MEV
    -   Some argue this is better for the network

Controversial Perspective
-------------------------

-   The speaker references a talk by Marius Van Der Widen (Ethereum core developer)
    -   Title: "Don't let your friends do MEV"
    -   Argument: Don't run MEV Boost, it's bad for the network
-   The speaker challenges this view based on new observations

Key Finding: Altruistic Blocks Increase Base Fee Volatility
===========================================================

The speaker presents evidence that:

-   MEV Boost blocks lead to more stability
-   Altruistic (self-built) blocks increase unnecessary volatility in the base fee
-   This volatility adversely affects all ecosystem participants

Private Transactions and Mempool Pressure
=========================================

Recent data shows:

-   30% of all on-chain transactions are now transmitted privately
-   This creates two types of mempool pressure:
    1.  Private pressure
    2.  Public pressure

Identifying Self-Built Blocks
-----------------------------

-   Self-built blocks contain 0% private transactions
-   This becomes a signature for identifying altruistic blocks

Data Analysis: Impact of Self-Built Blocks
==========================================

The speaker presents two sets of data:

Set 1: Slot-by-Slot Analysis
----------------------------

-   Examines slots 205-210
-   Slot 207 shows 0% private transactions (likely self-built)
-   Results:
    -   Percent private spikes in the following slot
    -   Gas utilization drops in self-built block, then spikes
    -   Base fee shows unnecessary volatility

Set 2: Week-Long Analysis
-------------------------

-   Self-built blocks are common (7.5% to 9.2% of all blocks)
-   Self-built blocks consistently use less gas
-   "Recovery blocks" (following self-built blocks) consume more gas

Implications for Users
======================

-   Users should check the last block before transacting
-   If it was self-built:
    -   Base fee is about to drop
    -   Wait if you can't get into the next block
    -   The block after that will have a higher base fee

Timing Games in Block Building
==============================

-   MEV Boost blocks use more of the available time
-   Self-built blocks have less time to include transactions
-   This further reduces the value of self-built blocks

Base Fee Volatility
===================

-   Normal MEV Boost blocks: Low base fee volatility
-   Self-built blocks: Volatility to the downside
-   Recovery blocks: Volatility to the upside
-   Volatility range: 12% to 15%

Potential Future Issues
=======================

-   As private transactions increase (potentially to 50%+):
    -   Recovery may require multiple blocks
    -   Base fee could become truly unstable
    -   This could lead to an unusable blockchain

Proposed Solutions
==================

1.  Monitor and observe the data
2.  Encourage private transaction venues to share signals
3.  Encourage fewer altruistic blocks on the network
4.  Consider changes to the EIP-1559 fee mechanism
    -   Allow faster decay of base fee

Conclusion
==========

The speaker emphasizes that this is a critical issue affecting:

-   Network stability
-   User experience
-   Protocol functionality
-   Price stability

----------------------

### **1. Introduction: Altruistic Blocks and Base Fee Volatility**
 They assert that altruistic blocks—blocks built locally by validators without outsourcing to services like MEV Boost—introduce unnecessary volatility into the base fee of the Ethereum network. The argument is that these altruistic blocks are detrimental to the network’s stability and may become more problematic over time.

### **2. Block Native's Role and Focus**
The speaker introduces Block Native, a company focused on gas management, because gas fees affect everyone in the ecosystem, from Layer 1 (L1) and Layer 2 (L2) networks to individual users. Despite its importance, gas is often misunderstood and underappreciated. The speaker emphasizes that gas fees can cause unexpected negative externalities or interactions within the Ethereum network, introducing instability into the base fee.

### **3. The Importance of Precise Gas Signals**
Gas is described as a levered metric, where the accuracy and precision of gas signals directly influence the network’s efficiency. As gas signals become more scattered, asset prices become more volatile. The speaker stresses that precise gas signals are crucial for ensuring network efficiency.

### **4. Spot Demand for Block Space**
The core function of gas in the Ethereum ecosystem is to reflect the spot demand for block space, which sets the spot price. In simpler systems like Bitcoin, there’s little complexity regarding block space demand. However, in more complex systems like Ethereum, the multidimensional nature of block space leads to emergent behaviors and greater volatility.

### **5. Discovery of New Insights on Gas Markets**
The speaker reveals that recent upgrades to Block Native's platform allowed for new observability capabilities into gas markets. This enhanced observability led to the discovery that altruistic blocks—blocks built without using external block-building networks like MEV Boost—can cause unnecessary volatility in the base fee.

### **6. Ethereum Block Production: Two Approaches**
The speaker explains that on Ethereum, there are two ways to produce blocks:
   - **MEV Boost Blocks:** Outsourcing block production to a network of specialized actors who build blocks on behalf of the validator. These blocks contain valuable transactions through Maximal Extractable Value (MEV).
   - **Altruistic Blocks:** Self-built blocks where validators avoid using MEV Boost, typically under the belief that it is better for the network and users.

### **7. Controversial Take: Altruistic Blocks Are Harmful**
The speaker challenges the common belief that altruistic blocks are beneficial by arguing that these blocks actually increase volatility in the base fee. The MEV Boost blocks, contrary to popular belief, are more stable and create less volatility. Altruistic blocks leave out private transactions and cause the base fee to swing unnecessarily, adversely affecting the entire ecosystem.

### **8. Growing Use of Private Transaction Venues**
The speaker provides data to show that around 30% of all on-chain transactions are now transmitted privately, bypassing the public mempool (where transactions await inclusion in a block). This significant shift toward private venues is driven by users and trading bots who wish to avoid exposure to MEV.

### **9. Mechanics of the Mempool**
The mempool operates as a continuous flow system, with transactions building up pressure until they are included in a block every 12 seconds. The speaker explains that in the case of altruistic blocks, which often don’t include private transactions, this pressure isn’t fully relieved, leading to volatility in gas usage and base fees in subsequent blocks.

### **10. Altruistic Blocks Cause Base Fee Volatility**
   - **Drop in Gas Utilization:** When altruistic blocks are produced, they use less gas because private transactions are excluded. This lowers the demand in that block.
   - **Spike in Gas Utilization:** In the subsequent block, there’s a spike in gas utilization because private transactions that weren’t included in the previous block are now added, driving an increase in the base fee.
   
The speaker illustrates this point with data, showing how self-built blocks drive volatility in the base fee due to swings in gas utilization.

### **11. Longer-Term Trends in Block Production**
The speaker presents data showing that altruistic blocks are common, representing around 7-10% of all blocks produced on Ethereum. However, these blocks are consistently less full than MEV Boost blocks. The speaker introduces the concept of **recovery blocks**—the blocks that immediately follow altruistic blocks, which tend to use significantly more gas to compensate for the lower gas utilization in the altruistic block.

### **12. Implications for Users**
Users are advised to monitor the last block produced before submitting transactions. If the last block was altruistic, gas fees are likely to drop in the next block, creating an opportunity to save on gas costs. Conversely, if the block was a recovery block, users might want to wait as gas fees will likely spike.

### **13. Timing Games at the Slot Boundary**
The speaker highlights that altruistic blocks are built more quickly (within 8 to 10 seconds, as opposed to the full 12 seconds), reducing the time available for gathering transactions from the mempool. This results in lower-value blocks, further contributing to the inefficiency of altruistic blocks.

### **14. Base Fee Volatility: MEV Boost vs. Altruistic Blocks**
The speaker compares base fee volatility across MEV Boost and altruistic blocks. The key takeaway is that MEV Boost blocks tend to have very low base fee volatility, while altruistic blocks produce significant swings in the base fee. Recovery blocks following altruistic blocks further exacerbate the volatility.

### **15. Potential Future Instability**
The speaker raises concerns about the future, noting that if the percentage of private transactions continues to grow (from 30% to potentially 40% or 50%), the network could face even greater instability. The recovery process might need multiple blocks, further amplifying base fee volatility.

### **16. Possible Solutions**
The speaker offers several possible solutions to mitigate the problem:
   - **Monitor and Observe:** Encourage the community to pay attention to data on altruistic blocks and their impact on gas markets.
   - **Share Signals from Private Venues:** Work with private venues to share signals that could improve access to transaction demand data.
   - **Discourage Altruistic Blocks:** Reduce the number of altruistic blocks to decrease base fee volatility.
   - **Adjust EIP-1559:** Modify the EIP-1559 gas fee mechanism to allow for faster base fee declines to counteract the volatility caused by altruistic blocks.

------------
