
# Maximum Extractable Value (MEV) and Liquidations

  

### Introduction to MEV

  

-   **MEV Definition**: Maximum Extractable Value (MEV) refers to the revenue from ordering transactions in a block strategically.
-   **Three Popular Methods of MEV**:
    -   **Sandwiching**: Buying before and selling after another transaction.
    -   **Liquidations**: Focus of this text.
    -   **Arbitrage**: Buying on one exchange and selling on another within a short time frame to profit from price differences.

  

----------

  

### Understanding Liquidations

  

-   **General Concept**: Related to protocols like Aave where users can lend assets (e.g., ETH or USDC) and borrow using these as collateral.
-   **Threshold**: Positions can be liquidated if they fall below a certain health factor threshold.
-   **Liquidation Process**:
    -   Not automatic; someone has to initiate the liquidation transaction on-chain.
    -   The liquidator receives a reward for completing the transaction.

  

----------

  

### Liquidation in Practice

  

-   **Example Case**:
    -   **Scenario**: Bob deposits 10 ETH and borrows 5 ETH worth of DAI.
    -   **Health Factor**: Drops below 1; Bob’s position is eligible for liquidation.
    -   **Liquidation Mechanics**:
        -   Liquidator can repay up to 50% of Bob’s borrowed amount (2.5 ETH worth of DAI).
        -   Liquidator receives 2.5 ETH (collateral) plus a 5% bonus (0.25 ETH), totaling 2.75 ETH.
        -   Flash loans can be used to cover the initial capital needed for liquidation.

  

----------

  

### Developer Perspective on Liquidations

  

-   **Key Challenges**:
    
    -   **Spotting Opportunities**: Identifying profitable liquidation chances quickly.
    -   **Computation Efficiency**: Ensuring the liquidation is executed efficiently and profitably.
    -   **Gas Cost**: Calculating and optimizing to avoid spending more on gas than the earned profit.
-   **Health Factor Calculation**: When the health factor drops below 1, a liquidation call can be made to the pool.
    
-   **Opportunity for Automation**: Liquidation bots can be programmed to optimize and execute these operations.
    

  

----------

  

### Example Code and Tools

  

-   **Developers**:
    
    -   Code for liquidations can be derived from Aave’s developer resources.
    -   Optimization of the bot to ensure transactions are executed before competitors.
-   **Real-world Example**:
    
    -   Transaction showcasing liquidation using a flash loan from Balancer.
    -   Platforms like eigenfi provide tools and stats for tracking MEV-related activities.

  

----------

  

### Profitability of Liquidations

  

-   **Current Market Trends**:
    
    -   **Less Profitable**: Compared to sandwiching and arbitrage due to lower volatility.
    -   **Volatility Dependency**: Most profitable during high volatility periods.
-   **Statistics**:
    
    -   72 liquidations in the past 30 days with a total profit of $137,000.
    -   Individual transaction profits can be minimal and require precise gas cost computations.

  

----------

  

### Recommendations

  

-   **Deep Dive into Documentation**: Explore Aave’s documentation for a thorough understanding.
-   **Experiment with Code**: Use tools like ChatGPT for generating simple liquidation bot scripts.
-   **Optimization and Deployment**: Ensure code is optimized for gas efficiency before deployment.

  