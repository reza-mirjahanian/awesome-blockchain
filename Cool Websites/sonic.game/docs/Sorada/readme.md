### 🔍 Blockchain Data Challenges
- **Blockchain Networks** aggregate data via *blocks, transactions, and metadata*.
- Querying APIs are *openly available and well-documented* for direct data fetching 📡.
- Baseline APIs suffice for *small DApps with minimal historical needs* but fail for larger requirements due to:
  - **Performance** limitations 🚀.
  - **Scalability** constraints 📈.

### ⚡ Performance Issues
- Companies lack control over *response times and data optimizations* in network APIs.
- Impacts *website speed, SEO, and conversion rates* 💨.
- Critical for *business requirements* affecting user experience and efficiency.

### 📊 Scalability Issues
- Inability to *adapt elastically* to user demands via network APIs.
- Affects *financial feasibility* of operations beyond just performance 🏦.
- Limits provisioning for growing data needs.

### 🛠️ Blockchain Indexing Solution
- **Blockchain Indexing** involves *querying historical data from the network and storing it in-house* 🗄️.
- Simplified pipeline:
  1. Fetch *blocks and transactions* using network APIs.
  2. Store data in *internal infrastructure*.
  3. Query via *internally provisioned endpoints* for products.
- Provides *internal control* over **performance** and **scalability** 🔄.

### 📈 Sonic SVM Case Study
- Sonic's infrastructure showed *85% archival read requests* vs *15% write requests* 📊.
- Reads include *getTransaction, getBlock, getSignaturesForAddress* 🔍.
- Bottlenecks in *bandwidth and storage costs* from direct requests to Hypergrid 💥.
- Solved by *decoupling archival reads* into Sorada, reducing infrastructure strain.

### 🌟 What is Sorada?
- **Sorada** is Sonic's *data solution* for decoupling **archival read requests** from **network write requests** 🔀.
- Shifts reads to *separate data-optimized infrastructure*, mitigating *bandwidth and storage bottlenecks* 🛡️.
- Enables SVM validators to *allocate more compute to transaction processing* ⚙️.
- Optimizes *read performance* by *30–40x* 🚀.