### **Understanding Hash Tables: Kademlia Peer-to-Peer Distributed Hash Table (DHT)**

---

#### **Introduction to Hash Tables**

- **Hash Table Concept**:  
  - A **dictionary** maps **keys to values** (e.g., “Chihuahua” → “tiny dog with a huge personality”).
  - **API Example**:  
    - `Put`: Insert key-value pairs (e.g., `"dog" → "4-legged animal"`).
    - `Get`: Retrieve value by key (e.g., `"dog"` → "4-legged animal").
  - A **hash map** is a computer version of this concept. It uses a **hash function** to generate fixed-length keys.

---

#### **Why Build a Distributed Hash Table (DHT)?**

- **Key Space in Hash Maps**:  
  - Represents all possible keys.
  - Example: 4-bit key space has 16 possible values, 8-bit has 255, and 256-bit has a much larger number.
  
- **Problem with Large Hash Maps**:  
  - What happens when the hash map becomes too large to fit on one computer?
  - **Solution**: Add more computers and break up the key space across them.

---

#### **Introduction to Kademlia and Distributed Hash Tables**

1. **Breaking Down the Key Space**:  
   - Distribute keys among multiple computers.  
   - Example: In a 4-bit hash table, we have a total of 16 values (keys).

2. **Assigning Computers to Key Space**:  
   - **Problem**: Computers exist in the physical world, but key spaces are digital.
   - **Solution**: Assign computers random numbers in the key space.
   - **Random Number Assignment**: Cryptographic hash functions ensure even distribution of computers.

---

#### **Defining Closeness Between Keys and Computers**

- **Exclusive OR (XOR) Metric**:  
  - Kademlia uses the **XOR metric** to measure **distance** between keys.
  - **How XOR Works**:  
    - For two binary numbers, identical bits return 0, different bits return 1.
    - XOR is **non-Euclidean**, which simplifies calculations and ensures symmetric results.
    - **Example**: XOR of node 5 (`0101`) and node 13 (`1101`) results in a distance of 8.

---

#### **Routing Tables and Findability Problem**

1. **How to Find Values on Other Nodes?**  
   - Each node maintains **contact information** about other nodes in a structure called **K-buckets**.

2. **Building a Routing Table**:  
   - **Node Distance**: Binary tree represents **distances**, not node identifiers.
   - Nodes are placed in the routing table based on their XOR distance from the node itself.
   - **Bucket Structure**:  
     - The number of **K-buckets** increases as distance from the node increases.  
     - Closer nodes have more detailed information, while distant nodes have less.

3. **Bucket Overflow**:  
   - When a bucket overflows, the least necessary node is forgotten, ensuring the node stores **more information** about closer nodes.

---

#### **Key Properties of Kademlia's DHT**

- **Even Distribution**: Random number selection ensures nodes are distributed evenly across the key space.
- **Distance Triangulation**: XOR metric allows **triangulation** of the value, enabling efficient lookup.
- **Efficiency**: XOR is a low-level assembly instruction, making it **fast** and ideal for DHT operations.