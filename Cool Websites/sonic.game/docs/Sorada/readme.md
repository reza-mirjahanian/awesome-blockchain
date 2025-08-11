🧠 **Sorada Architecture Overview**  

![alt text](image.png)

🔹 *A high-performance, distributed system designed to optimize Solana read operations through modular services and resilient data storage.*

---

⚙️ **Core Services**

1. **Lite RPC**  
   - *Lightweight RPC service optimized for Solana read requests*  
   - 🚀 Stripped-down version of a Solana full node  
   - 📥 Supports only data retrieval endpoints (e.g., `getBlock`, `getTransaction`)  
   - ⚡ Enhances speed and efficiency by removing unnecessary components  

2. **Ingestor**  
   - 🔄 Pulls real-time data from the **Sonic Hypergrid**  
   - 💾 Persists ingested data into **Big Table** storage  
   - 🕒 Ensures up-to-date data availability for querying  

3. **Replicator**  
   - 🔁 Copies indexed data from **Big Table** to a **Distributed Filesystem**  
   - 🛡️ Improves data durability and fault tolerance  
   - 🌐 Enables resilient, long-term data preservation across multiple nodes  

---

🗄️ **Data Repositories**

1. **Cache**  
   - ⏱️ Enables fast retrieval of frequently accessed data  
   - 📦 Reduces load on primary storage by serving repetitive requests efficiently  
   - 🔄 *Temporary storage layer for performance optimization*

2. **Big Table**  
   - 🏗️ Primary data store for structured, indexed data  
   - 📊 Scales horizontally to handle large volumes of blockchain data  
   - 🔍 Optimized for high-throughput read operations  

3. **Distributed Filesystem**  
   - 🧩 Self-replicating storage across multiple nodes  
   - 🔒 Ensures data safety and resilience against node failures  
   - 📁 Used for long-term, secure data persistence via replication from Big Table  

---

🎯 **Key Insights**

- 🧩 **Modular design** enables scalability and focused optimization  
- 🔁 **Data flows** from real-time ingestion → primary storage → replicated backup  
- 💡 **Separation of concerns**: Each service handles a specific role in the data lifecycle  
- 🌐 **Resilience by design**: Replication + distributed storage = high availability  
- ⚖️ **Performance + Reliability**: Cache for speed, Big Table for scale, DFS for safety 🛡️