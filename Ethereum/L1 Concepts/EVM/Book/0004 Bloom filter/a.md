I'll compare Bloom filters and hashmaps across several key dimensions:

## **Purpose & Use Cases**

**Bloom Filter:**
- Probabilistic data structure for membership testing
- Answers "Is X possibly in the set?" or "Is X definitely NOT in the set?"
- Used for pre-filtering, caching decisions, duplicate detection
- Common in databases, web crawlers, CDNs

**HashMap:**
- Deterministic key-value storage and retrieval
- Answers "What is the value for key X?" with certainty
- Used for fast lookups, caching, indexing
- General-purpose data structure

## **Memory Efficiency**

**Bloom Filter:**
- Extremely space-efficient: typically uses $k \times m$ bits where $m$ is the bit array size
- Memory usage independent of actual data size
- Example: 1 million URLs might need only ~1.2MB with 1% false positive rate

**HashMap:**
- Stores actual keys and values
- Memory grows linearly with data size
- Overhead from hash table structure, collision handling
- Same 1 million URLs might need 50-100MB+

## **Accuracy**

**Bloom Filter:**
- **False positives possible**: May say an item exists when it doesn't
- **No false negatives**: If it says "not present," it's definitely not present
- False positive rate: $P \approx (1-e^{-kn/m})^k$ where $k$ = hash functions, $n$ = items, $m$ = bit array size

**HashMap:**
- **100% accurate** for membership and retrieval
- No false positives or negatives
- Deterministic results

## **Operations**

**Bloom Filter:**
```
- Insert: O(k) where k = number of hash functions
- Query: O(k)
- Delete: Not supported (would cause false negatives)
- Update: Not applicable
```

**HashMap:**
```
- Insert: O(1) average, O(n) worst case
- Query: O(1) average, O(n) worst case  
- Delete: O(1) average, O(n) worst case
- Update: O(1) average, O(n) worst case
```

## **When to Use Each**

**Choose Bloom Filter when:**
- You only need membership testing (not retrieval)
- Memory is extremely constrained
- False positives are acceptable
- You're doing pre-filtering before expensive operations
- Working with very large datasets

**Choose HashMap when:**
- You need to store and retrieve values
- You need 100% accuracy
- You need to update/delete items
- Memory isn't the primary constraint
- You need general-purpose key-value operations

## **Example Scenario**

**Web Crawler:**
- **Bloom Filter**: "Have we seen this URL before?" (fast pre-check)
- **HashMap**: If Bloom filter says "maybe," check HashMap for definitive answer and store metadata

This hybrid approach combines the memory efficiency of Bloom filters with the accuracy of hashmaps for optimal performance.