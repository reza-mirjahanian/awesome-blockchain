A **Bloom filter** is a **space-efficient probabilistic data structure** used to test whether an element is a member of a set. It is designed to be extremely fast and use very little memory.

The trade-off is that it's **probabilistic**:
* **False positives are possible**: It might tell you an element *is* in the set when it actually *isn't*.
* **False negatives are not possible**: If it tells you an element *is not* in the set, then it is definitively *not* in the set.

It **does not store the elements themselves**, only a compact representation of their presence. Standard Bloom filters **do not support deletion** of elements (though variations exist that do).

---
## Core Mechanics

At its heart, a Bloom filter consists of two main components:

1.  **A bit array (or bit vector) of size *m***: Initially, all *m* bits are set to 0.
2.  **_k_ independent hash functions**: These hash functions, $h_1, h_2, \ldots, h_k$, each map an input element to one of the *m* bit array positions (e.g., producing an output in the range $[0, m-1]$). These functions should be uniformly random and independent.

### Operations

#### 1. Adding an Element (`add(item)`)
To add an element to the Bloom filter:
1.  Feed the element to each of the *k* hash functions.
2.  This produces *k* array positions (indices).
3.  Set the bits at all these *k* positions in the bit array to 1.

**Example**:
Suppose we have a bit array of size $m=10$ and $k=3$ hash functions.
Bit array: `[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]`

To add "apple":
* $h_1(\text{"apple"}) = 2$
* $h_2(\text{"apple"}) = 5$
* $h_3(\text{"apple"}) = 8$

Set bits at indices 2, 5, and 8 to 1.
Bit array: `[0, 0, 1, 0, 0, 1, 0, 0, 1, 0]`

To add "banana":
* $h_1(\text{"banana"}) = 1$
* $h_2(\text{"banana"}) = 5$ (collision with "apple" for this hash function at this bit)
* $h_3(\text{"banana"}) = 9$

Set bits at indices 1, 5, and 9 to 1.
Bit array: `[0, 1, 1, 0, 0, 1, 0, 0, 1, 1]`

#### 2. Checking for an Element (`mightContain(item)` or `query(item)`)
To check if an element *might be* in the set:
1.  Feed the element to each of the *k* hash functions.
2.  This produces *k* array positions.
3.  Check the bits at all these *k* positions.
    * If **all** *k* bits are 1, the Bloom filter reports that the element *might be* in the set (it could be a false positive).
    * If **any** of the *k* bits are 0, the element is *definitively not* in the set.

**Example (continuing from above)**:
Bit array: `[0, 1, 1, 0, 0, 1, 0, 0, 1, 1]`

Check for "apple":
* $h_1(\text{"apple"}) = 2 \rightarrow \text{bit}[2]$ is 1.
* $h_2(\text{"apple"}) = 5 \rightarrow \text{bit}[5]$ is 1.
* $h_3(\text{"apple"}) = 8 \rightarrow \text{bit}[8]$ is 1.
All bits are 1. Output: "apple" **might be** in the set (True Positive in this case).

Check for "orange":
* $h_1(\text{"orange"}) = 3 \rightarrow \text{bit}[3]$ is 0.
* $h_2(\text{"orange"}) = 5 \rightarrow \text{bit}[5]$ is 1.
* $h_3(\text{"orange"}) = 7 \rightarrow \text{bit}[7]$ is 0.
Not all bits are 1 (bit[3] is 0). Output: "orange" is **definitively not** in the set.

Check for "grape" (which was never added):
Suppose:
* $h_1(\text{"grape"}) = 1 \rightarrow \text{bit}[1]$ is 1 (from "banana").
* $h_2(\text{"grape"}) = 5 \rightarrow \text{bit}[5]$ is 1 (from "apple" and "banana").
* $h_3(\text{"grape"}) = 8 \rightarrow \text{bit}[8]$ is 1 (from "apple").
All bits happen to be 1 due to other insertions. Output: "grape" **might be** in the set (False Positive).

---
## Mathematical Analysis

Let:
* *m*: be the number of bits in the filter.
* *k*: be the number of hash functions.
* *n*: be the number of elements expected to be inserted into the filter.

#### Probability of a Specific Bit Being 0
After one element is inserted, *k* bits are set to 1.
The probability that a specific bit is *not* set to 1 by a particular hash function during the insertion of one element is $1 - \frac{1}{m}$.
Since there are *k* hash functions, the probability that a specific bit is *not* set to 1 by *any* of the *k* hash functions (i.e., it remains 0) for a single element insertion is $(1 - \frac{1}{m})^k$.

After *n* elements are inserted, the probability that a specific bit is still 0 is:
$$P(\text{bit is 0}) = \left(1 - \frac{1}{m}\right)^{kn}$$
A common approximation for large *m* is:
$$P(\text{bit is 0}) \approx e^{-kn/m}$$

#### Probability of a Specific Bit Being 1
The probability that a specific bit is 1 after *n* insertions is:
$$P(\text{bit is 1}) = 1 - \left(1 - \frac{1}{m}\right)^{kn} \approx 1 - e^{-kn/m}$$

#### Probability of a False Positive (*p*)
A false positive occurs when querying for an element *not* in the set, but all *k* bits corresponding to its hash values are 1 (due to other insertions).
The probability of a false positive, *p*, is:
$$p = (P(\text{bit is 1}))^k = \left(1 - \left(1 - \frac{1}{m}\right)^{kn}\right)^k \approx \left(1 - e^{-kn/m}\right)^k$$

#### Optimal Number of Hash Functions (*k*)
For a given *m* and *n*, the number of hash functions *k* that minimizes the false positive probability *p* is:
$$k_{opt} = \frac{m}{n} \ln 2 \approx 0.693 \frac{m}{n}$$
In practice, *k* must be an integer, so rounding is necessary. When $k = k_{opt}$, the probability that a bit is 1 is approximately $1/2$.

#### Optimal Size of Bit Array (*m*)
For a given *n* (number of elements) and a desired false positive probability *p*, the required number of bits *m* (assuming the optimal *k* is used) is:
$$m = -\frac{n \ln p}{(\ln 2)^2}$$
This formula is derived by substituting $k_{opt}$ into the false positive probability equation.

The number of bits per element, $m/n$, is:
$$\frac{m}{n} = -\frac{\ln p}{(\ln 2)^2} \approx -1.44 \log_2 p$$
For example, for a 1% false positive rate ($p=0.01$), you need approximately $m/n = -1.44 \log_2 0.01 \approx -1.44 \times (-6.64) \approx 9.59$ bits per element.

---
## Choosing Parameters

When designing a Bloom filter, you typically start with:
1.  **_n_**: The expected number of items to be stored.
2.  **_p_**: The desired maximum false positive probability.

From these, you calculate:
1.  **_m_**: The optimal size of the bit array: $m = -\frac{n \ln p}{(\ln 2)^2}$. Round *m* up to the nearest integer.
2.  **_k_**: The optimal number of hash functions: $k = \frac{m}{n} \ln 2$. Round *k* to the nearest integer.

**Example**:
* Suppose $n = 1,000,000$ (1 million items).
* Desired $p = 0.001$ (0.1% false positive rate).

1.  Calculate *m*:
    $m = -\frac{1,000,000 \times \ln(0.001)}{(\ln 2)^2} \approx -\frac{1,000,000 \times (-6.907755)}{ (0.693147)^2 } \approx -\frac{-6,907,755}{0.480453} \approx 14,377,640 \text{ bits}$
    This is approximately $14.38 \text{ Mbits} \approx 1.71 \text{ MBytes}$.

2.  Calculate *k*:
    $k = \frac{14,377,640}{1,000,000} \times \ln 2 \approx 14.377 \times 0.693147 \approx 9.96$
    So, use $k = 10$ hash functions.

A Bloom filter with approximately 1.71 MB of memory and 10 hash functions can store 1 million items with a false positive rate of about 0.1%.

---
## Implementation Details

### Hash Functions
The quality of hash functions is critical for a Bloom filter's performance. They must be:
1.  **Fast**: Hashing should be computationally inexpensive.
2.  **Uniformly Distributed**: They should map inputs to array indices as evenly as possible to avoid clustering bits.
3.  **Independent**: The output of one hash function should not be predictable from the output of others for the same input.

Common choices include:
* **MurmurHash (especially MurmurHash3)**: Good balance of speed and distribution.
* **xxHash**: Extremely fast with good distribution.
* **FNV (Fowler–Noll–Vo)**: Simple and reasonably effective.
* **Cryptographic hashes (like SHA-256)**: While having excellent distribution and collision resistance, they are often slower than necessary for Bloom filters. If used, you can take different slices of their output (e.g., divide a 256-bit hash into multiple 32-bit or 64-bit numbers and take modulo *m*).

**Generating *k* Hashes from Two Hashes**:
A common technique to efficiently generate *k* hash values without needing *k* completely independent hash functions is to use two base hash functions, $h_a(item)$ and $h_b(item)$. Then, the *i*-th hash function (for $i = 0, \dots, k-1$) can be computed as:
$$g_i(item) = (h_a(item) + i \times h_b(item)) \pmod m$$
Or a variant:
$$g_i(item) = (h_a(item) + i \times (h_b(item) \lor 1)) \pmod m$$
The `lor 1` ensures $h_b(item)$ is odd, which helps in probing all slots if $m$ is a power of two. This is known as "double hashing" or "enhanced double hashing".

### Bit Array
The bit array can be implemented using:
* An array of booleans (less space-efficient).
* An array of integers (e.g., `uint8_t`, `uint32_t`, `uint64_t`), where each integer holds multiple bits. Bitwise operations (`|` for setting, `&` for checking) are then used to manipulate individual bits.

---
## Code Snippets

Here are basic implementations in C++, Go, and Rust. These examples use simple hashing for illustration; in practice, use robust hash functions like MurmurHash3 or xxHash.

### C++

```cpp
#include <vector>
#include <string>
#include <functional> // For std::hash
#include <cmath>      // For std::log, std::pow, std::ceil
#include <iostream>

// Basic bitset implementation
class BitSet {
public:
    BitSet(size_t num_bits) : bits_((num_bits + 63) / 64, 0) {} // Use uint64_t blocks

    void set(size_t index) {
        size_t block_index = index / 64;
        size_t bit_offset = index % 64;
        if (block_index < bits_.size()) {
            bits_[block_index] |= (1ULL << bit_offset);
        }
    }

    bool get(size_t index) const {
        size_t block_index = index / 64;
        size_t bit_offset = index % 64;
        if (block_index < bits_.size()) {
            return (bits_[block_index] & (1ULL << bit_offset)) != 0;
        }
        return false; // Should not happen if index < m
    }

private:
    std::vector<uint64_t> bits_;
};


class BloomFilter {
public:
    BloomFilter(size_t num_items, double false_positive_prob)
        : n_(num_items), p_(false_positive_prob) {
        m_ = std::ceil(- (double)n_ * std::log(p_) / (std::log(2) * std::log(2)));
        if (m_ == 0) m_ = 1; // Ensure m is at least 1
        k_ = std::ceil(((double)m_ / n_) * std::log(2));
        if (k_ == 0) k_ = 1; // Ensure k is at least 1
        
        bit_set_ = BitSet(m_);
        std::cout << "Bloom Filter Initialized:" << std::endl;
        std::cout << "  Expected items (n): " << n_ << std::endl;
        std::cout << "  False positive rate (p): " << p_ << std::endl;
        std::cout << "  Calculated bits (m): " << m_ << std::endl;
        std::cout << "  Calculated hash functions (k): " << k_ << std::endl;
    }

    void add(const std::string& item) {
        std::hash<std::string> str_hash;
        // Use two hash functions to generate k hashes (Kirsch-Mitzenmacher optimization)
        uint64_t hash1 = str_hash(item);
        uint64_t hash2 = str_hash(item + "_alt_seed"); // Simple way to get a second hash

        for (size_t i = 0; i < k_; ++i) {
            uint64_t combined_hash = (hash1 + i * hash2);
            if (m_ > 0) { // Prevent modulo by zero if m_ was calculated as 0
                 bit_set_.set(combined_hash % m_);
            }
        }
    }

    bool might_contain(const std::string& item) const {
        if (m_ == 0) return false; // Empty filter or misconfigured
        std::hash<std::string> str_hash;
        uint64_t hash1 = str_hash(item);
        uint64_t hash2 = str_hash(item + "_alt_seed");

        for (size_t i = 0; i < k_; ++i) {
            uint64_t combined_hash = (hash1 + i * hash2);
            if (!bit_set_.get(combined_hash % m_)) {
                return false; // Definitely not in set
            }
        }
        return true; // Possibly in set
    }

    size_t get_m() const { return m_; }
    size_t get_k() const { return k_; }

private:
    size_t n_;       // Expected number of items
    double p_;       // Desired false positive probability
    size_t m_;       // Number of bits in the filter
    size_t k_;       // Number of hash functions
    BitSet bit_set_;
};

int main() {
    BloomFilter bf(1000, 0.01); // Expect 1000 items, 1% FP rate

    std::cout << "\nAdding items..." << std::endl;
    bf.add("apple");
    bf.add("banana");
    bf.add("cherry");

    std::cout << "\nChecking items:" << std::endl;
    std::cout << "apple: " << (bf.might_contain("apple") ? "Might be present" : "Definitely not present") << std::endl;
    std::cout << "banana: " << (bf.might_contain("banana") ? "Might be present" : "Definitely not present") << std::endl;
    std::cout << "grape: " << (bf.might_contain("grape") ? "Might be present (False Positive?)" : "Definitely not present") << std::endl;
    bf.add("grape"); // Add grape now
    std::cout << "grape (after add): " << (bf.might_contain("grape") ? "Might be present" : "Definitely not present") << std::endl;


    // Edge Case: Filter saturation (high FP rate)
    // Fill it more than expected
    std::cout << "\nTesting saturation (adding 2000 more items to a filter designed for 1000):" << std::endl;
    BloomFilter bf_small(10, 0.1); // 10 items, 10% FP rate
    std::cout << "  Small filter m: " << bf_small.get_m() << ", k: " << bf_small.get_k() << std::endl;
    for (int i = 0; i < 20; ++i) { // Add 2x the expected items
        bf_small.add("item" + std::to_string(i));
    }
    int false_positives = 0;
    int true_negatives = 0;
    for (int i = 20; i < 120; ++i) { // Test 100 items not added
        if (bf_small.might_contain("item" + std::to_string(i))) {
            false_positives++;
        } else {
            true_negatives++;
        }
    }
    std::cout << "  Saturation test (100 non-existent items):" << std::endl;
    std::cout << "    False positives: " << false_positives << std::endl;
    std::cout << "    True negatives: " << true_negatives << std::endl;
    std::cout << "    Observed FP rate: " << (double)false_positives / 100.0 << std::endl;


    // Edge Case: Empty filter
    std::cout << "\nTesting empty filter:" << std::endl;
    BloomFilter bf_empty(100, 0.01);
    std::cout << "  empty_filter.might_contain(\"test\"): " << (bf_empty.might_contain("test") ? "Might be present" : "Definitely not present") << std::endl;

    // Edge Case: Element added multiple times
    std::cout << "\nTesting adding element multiple times:" << std::endl;
    BloomFilter bf_multi_add(100, 0.01);
    bf_multi_add.add("duplicate");
    bf_multi_add.add("duplicate"); // Adding again has no further effect on bits if already set
    std::cout << "  bf_multi_add.might_contain(\"duplicate\"): " << (bf_multi_add.might_contain("duplicate") ? "Might be present" : "Definitely not present") << std::endl;
    
    return 0;
}

```

### Go

```go
package main

import (
	"fmt"
	"hash/fnv"
	"math"
)

// BitSet is a simple bitset implementation
type BitSet struct {
	bits []uint64
	size uint64
}

func NewBitSet(numBits uint64) *BitSet {
	return &BitSet{
		bits: make([]uint64, (numBits+63)/64),
		size: numBits,
	}
}

func (bs *BitSet) Set(idx uint64) {
	if idx >= bs.size {
		return // Out of bounds
	}
	bs.bits[idx/64] |= (1 << (idx % 64))
}

func (bs *BitSet) Get(idx uint64) bool {
	if idx >= bs.size {
		return false // Out of bounds
	}
	return (bs.bits[idx/64] & (1 << (idx % 64))) != 0
}

// BloomFilter structure
type BloomFilter struct {
	m         uint64   // Number of bits
	k         uint64   // Number of hash functions
	n         uint64   // Expected number of items
	p         float64  // Desired false positive probability
	bitSet    *BitSet
}

// NewBloomFilter creates a new Bloom filter
func NewBloomFilter(numItems uint64, falsePositiveProb float64) *BloomFilter {
	m := math.Ceil(- (float64(numItems) * math.Log(falsePositiveProb)) / (math.Ln2 * math.Ln2))
	if m == 0 { m = 1}
	k := math.Ceil((m / float64(numItems)) * math.Ln2)
	if k == 0 { k = 1}

	bf := &BloomFilter{
		m:      uint64(m),
		k:      uint64(k),
		n:      numItems,
		p:      falsePositiveProb,
		bitSet: NewBitSet(uint64(m)),
	}
	fmt.Printf("Bloom Filter Initialized:\n")
	fmt.Printf("  Expected items (n): %d\n", bf.n)
	fmt.Printf("  False positive rate (p): %f\n", bf.p)
	fmt.Printf("  Calculated bits (m): %d\n", bf.m)
	fmt.Printf("  Calculated hash functions (k): %d\n", bf.k)
	return bf
}

// hashN returns the i-th hash value for the given data.
// It uses two FNV hashes to generate k hashes.
func (bf *BloomFilter) hashN(data []byte, i uint64) uint64 {
	h1 := fnv.New64a()
	h1.Write(data)
	hashVal1 := h1.Sum64()

	h2 := fnv.New64a()
	h2.Write(append(data, byte(i))) // Slightly vary input for the "second" hash part
	hashVal2 := h2.Sum64()
	
	// Note: Using (hash1 + i * hash2) is a common way.
	// For simplicity, we ensure hashVal2 is different by appending i to data.
	// A more robust way is to use two distinct good hash functions.
	// hashVal1 = FNV-1a(data)
	// hashVal2 = FNV-1a(data + some_seed)
	// return (hashVal1 + i * hashVal2) % bf.m
	// Here, for simplicity and since FNV is fast:
	
	// To get k distinct hash functions, a simple approach:
	// Create a new FNV hasher, write original data, then write the seed 'i'.
	// This is not strictly the (h1 + i*h2) form but provides k different hashes.
	hasher := fnv.New64a()
	hasher.Write(data)
	// Append a byte representing 'i' to vary the hash input.
	// This makes each h_i distinct.
	// A more robust method is using two initial hash functions and combining them.
	// For example, h_i(x) = (hash_A(x) + i * hash_B(x)) % m
	// For this simple example, let's seed with i differently.
	seedBytes := make([]byte, 8)
	seedBytes[0] = byte(i) // simple way to differentiate hash calculations for k
	hasher.Write(seedBytes)

	if bf.m == 0 {
		return 0 // Avoid panic on modulo zero
	}
	return hasher.Sum64() % bf.m
}


func (bf *BloomFilter) Add(item string) {
	data := []byte(item)
	for i := uint64(0); i < bf.k; i++ {
		idx := bf.hashN(data, i)
		bf.bitSet.Set(idx)
	}
}

func (bf *BloomFilter) MightContain(item string) bool {
	if bf.m == 0 { return false }
	data := []byte(item)
	for i := uint64(0); i < bf.k; i++ {
		idx := bf.hashN(data, i)
		if !bf.bitSet.Get(idx) {
			return false // Definitely not in set
		}
	}
	return true // Possibly in set
}

func (bf *BloomFilter) M() uint64 { return bf.m }
func (bf *BloomFilter) K() uint64 { return bf.k }

func main() {
	bf := NewBloomFilter(1000, 0.01)

	fmt.Println("\nAdding items...")
	bf.Add("apple")
	bf.Add("banana")
	bf.Add("cherry")

	fmt.Println("\nChecking items:")
	fmt.Printf("apple: %v\n", bf.MightContain("apple"))
	fmt.Printf("banana: %v\n", bf.MightContain("banana"))
	fmt.Printf("grape: %v (False Positive?)\n", bf.MightContain("grape"))
	bf.Add("grape")
	fmt.Printf("grape (after add): %v\n", bf.MightContain("grape"))

	// Edge Case: Filter saturation
	fmt.Println("\nTesting saturation (adding 200 more items to a filter designed for 100):")
	bfSmall := NewBloomFilter(10, 0.1) // 10 items, 10% FP rate
	fmt.Printf("  Small filter m: %d, k: %d\n", bfSmall.M(), bfSmall.K())
	for i := 0; i < 20; i++ { // Add 2x the expected items
		bfSmall.Add(fmt.Sprintf("item%d", i))
	}
	falsePositives := 0
	trueNegatives := 0
	for i := 20; i < 120; i++ { // Test 100 items not added
		if bfSmall.MightContain(fmt.Sprintf("item%d", i)) {
			falsePositives++
		} else {
			trueNegatives++
		}
	}
	fmt.Printf("  Saturation test (100 non-existent items):\n")
	fmt.Printf("    False positives: %d\n", falsePositives)
	fmt.Printf("    True negatives: %d\n", trueNegatives)
	fmt.Printf("    Observed FP rate: %f\n", float64(falsePositives)/100.0)

	// Edge Case: Empty filter
	fmt.Println("\nTesting empty filter:")
	bfEmpty := NewBloomFilter(100, 0.01)
	fmt.Printf("  emptyFilter.MightContain(\"test\"): %v\n", bfEmpty.MightContain("test"))

	// Edge Case: Element added multiple times
	fmt.Println("\nTesting adding element multiple times:")
	bfMultiAdd := NewBloomFilter(100, 0.01)
	bfMultiAdd.Add("duplicate")
	bfMultiAdd.Add("duplicate")
	fmt.Printf("  bfMultiAdd.MightContain(\"duplicate\"): %v\n", bfMultiAdd.MightContain("duplicate"))
}
```

### Rust

```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::f64::consts::LN_2; // ln(2)

// Basic bitvec implementation
pub struct BitVec {
    bits: Vec<u64>,
    size: usize,
}

impl BitVec {
    pub fn new(num_bits: usize) -> Self {
        BitVec {
            bits: vec![0; (num_bits + 63) / 64],
            size: num_bits,
        }
    }

    pub fn set(&mut self, index: usize) {
        if index >= self.size { return; } // Out of bounds
        let block_index = index / 64;
        let bit_offset = index % 64;
        self.bits[block_index] |= 1 << bit_offset;
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.size { return false; } // Out of bounds
        let block_index = index / 64;
        let bit_offset = index % 64;
        (self.bits[block_index] & (1 << bit_offset)) != 0
    }
}


pub struct BloomFilter {
    m: usize, // Number of bits
    k: usize, // Number of hash functions
    n: usize, // Expected number of items
    p: f64,   // Desired false positive probability
    bit_vec: BitVec,
}

impl BloomFilter {
    pub fn new(num_items: usize, false_positive_prob: f64) -> Self {
        let m_float = - (num_items as f64 * false_positive_prob.ln()) / (LN_2 * LN_2);
        let mut m = m_float.ceil() as usize;
        if m == 0 { m = 1; }

        let k_float = (m as f64 / num_items as f64) * LN_2;
        let mut k = k_float.ceil() as usize;
        if k == 0 { k = 1; }
        
        println!("Bloom Filter Initialized:");
        println!("  Expected items (n): {}", num_items);
        println!("  False positive rate (p): {}", false_positive_prob);
        println!("  Calculated bits (m): {}", m);
        println!("  Calculated hash functions (k): {}", k);

        BloomFilter {
            m,
            k,
            n: num_items,
            p: false_positive_prob,
            bit_vec: BitVec::new(m),
        }
    }

    // Generate k hash values for an item
    // Uses two hash functions to derive k hashes (Kirsch-Mitzenmacher optimization idea)
    fn get_hashes<T: Hash>(&self, item: &T) -> Vec<usize> {
        let mut hasher1 = DefaultHasher::new();
        item.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        // Add a "salt" or slightly different input for the second base hash
        // For demonstration, we can hash the first hash value again with a different seed (implicitly)
        // or hash item with a different hasher type if available/simple.
        // Here, let's just use a simple perturbation of hash1 for hash2
        // A better approach: use two truly different hash functions.
        // For example, use `item.hash(&mut some_other_hasher_type);`
        // Or `(item + "some_seed_string").hash(&mut hasher2);` if T is String.
        // For a generic T, this is trickier. Let's make hash2 from hash1 for simplicity.
        let mut h2_temp_hasher = DefaultHasher::new();
        (hash1.wrapping_add(1)).hash(&mut h2_temp_hasher); // Small change to get a "different" hash
        let hash2 = h2_temp_hasher.finish();

        let mut hashes = Vec::with_capacity(self.k);
        if self.m == 0 { return hashes; }

        for i in 0..self.k {
            // (hash1 + i * hash2) % m
            // Ensure hash2 is odd to help with probing if m is power of two, not strictly necessary for general m
            // let h2_eff = if hash2 % 2 == 0 { hash2.wrapping_add(1) } else { hash2 };
            // hashes.push((hash1.wrapping_add((i as u64).wrapping_mul(h2_eff))) as usize % self.m);
            // Simpler approach without ensuring hash2 is odd (DefaultHasher output is u64):
             hashes.push((hash1.wrapping_add((i as u64).wrapping_mul(hash2))) as usize % self.m);
        }
        hashes
    }

    pub fn add<T: Hash>(&mut self, item: &T) {
        if self.m == 0 { return; }
        let hashes = self.get_hashes(item);
        for hash_val in hashes {
            self.bit_vec.set(hash_val);
        }
    }

    pub fn might_contain<T: Hash>(&self, item: &T) -> bool {
        if self.m == 0 { return false; } // Empty filter
        let hashes = self.get_hashes(item);
        for hash_val in hashes {
            if !self.bit_vec.get(hash_val) {
                return false; // Definitely not in set
            }
        }
        true // Possibly in set
    }

    pub fn get_m(&self) -> usize { self.m }
    pub fn get_k(&self) -> usize { self.k }
}


fn main() {
    let mut bf = BloomFilter::new(1000, 0.01);

    println!("\nAdding items...");
    bf.add(&"apple");
    bf.add(&"banana");
    bf.add(&"cherry");

    println!("\nChecking items:");
    println!("apple: {}", if bf.might_contain(&"apple") { "Might be present" } else { "Definitely not present" });
    println!("banana: {}", if bf.might_contain(&"banana") { "Might be present" } else { "Definitely not present" });
    println!("grape: {} (False Positive?)", if bf.might_contain(&"grape") { "Might be present" } else { "Definitely not present" });
    bf.add(&"grape");
    println!("grape (after add): {}", if bf.might_contain(&"grape") { "Might be present" } else { "Definitely not present" });


    // Edge Case: Filter saturation
    println!("\nTesting saturation (adding 20 more items to a filter designed for 10):");
    let mut bf_small = BloomFilter::new(10, 0.1); // 10 items, 10% FP rate
    println!("  Small filter m: {}, k: {}", bf_small.get_m(), bf_small.get_k());

    for i in 0..20 { // Add 2x the expected items
        bf_small.add(&format!("item{}", i));
    }
    let mut false_positives = 0;
    let mut true_negatives = 0;
    for i in 20..120 { // Test 100 items not added
        if bf_small.might_contain(&format!("item{}", i)) {
            false_positives += 1;
        } else {
            true_negatives +=1;
        }
    }
    println!("  Saturation test (100 non-existent items):");
    println!("    False positives: {}", false_positives);
    println!("    True negatives: {}", true_negatives);
    println!("    Observed FP rate: {}", false_positives as f64 / 100.0);

    // Edge Case: Empty filter
    println!("\nTesting empty filter:");
    let bf_empty = BloomFilter::new(100, 0.01);
    println!("  empty_filter.might_contain(\"test\"): {}", if bf_empty.might_contain(&"test") { "Might be present" } else { "Definitely not present" });
    
    // Edge Case: Element added multiple times
    println!("\nTesting adding element multiple times:");
    let mut bf_multi_add = BloomFilter::new(100, 0.01);
    bf_multi_add.add(&"duplicate");
    bf_multi_add.add(&"duplicate"); // Adding again has no further effect
    println!("  bf_multi_add.might_contain(\"duplicate\"): {}", if bf_multi_add.might_contain(&"duplicate") { "Might be present" } else { "Definitely not present" });
}

```

---
## Use Cases

Bloom filters are valuable when:
* **Space is critical**: They use significantly less memory than storing the actual elements.
* **False positives are acceptable**: The application can tolerate occasional incorrect "yes" answers.
* **False negatives are unacceptable**: The application requires that "no" answers are always correct.
* **Query speed is important**: Add and query operations are O(k), which is very fast.

**Common applications include**:
1.  **Database Query Optimization**:
    * **Weakly consistent replicated data**: Check if a row *might* be on a remote node before making an expensive network call.
    * **Avoiding disk reads for non-existent keys**: In key-value stores (like LevelDB, RocksDB, Cassandra), Bloom filters can quickly determine if a key is unlikely to be in a specific SSTable file on disk, saving an I/O operation. Most lookups are for non-existent keys in some workloads.
2.  **Network Security and Routing**:
    * **Malicious URL/IP filtering**: Browsers (like Chrome's Safe Browse) use Bloom filters to check if a URL is on a list of known malicious sites without sending every URL to a server.
    * **Packet routing**: Routers can use Bloom filters to check if a destination has been seen recently or to implement certain forwarding policies.
3.  **Content Delivery Networks (CDNs)**:
    * To determine if an item is *not* in a cache, thus requiring a fetch from the origin server. A false positive means an unnecessary check at origin, but a false negative is avoided.
4.  **Spell Checkers/Grammar Checkers**:
    * Quickly check if a word *might* be in a large dictionary. If the Bloom filter says no, it's definitely misspelled (or not in the dictionary). If yes, a more thorough check is needed.
5.  **Distributed Systems and Big Data**:
    * **Counting distinct elements (HyperLogLog often better here, but Bloom filters can be part of solutions)**: Summarizing set memberships across nodes.
    * **Genomics**: Storing and querying large sets of k-mers.
6.  **User-facing Applications**:
    * **Preventing one-time offers/notifications**: E.g., "Don't show this message to the user again if they've already seen it." A Bloom filter can store IDs of users who have seen the message.

---
## Edge Cases and Considerations

1.  **Bloom Filter Saturation**:
    * If you add significantly more elements (*n'*) than the filter was designed for (*n*), the bit array will become increasingly dense with 1s.
    * As the proportion of set bits approaches 1, the false positive rate approaches 100%. At this point, the filter becomes useless, always returning "might be present."
    * It's crucial to estimate *n* accurately or use a dynamic/scalable Bloom filter variant if *n* is unknown or can grow indefinitely.

2.  **Choice of *m* and *k***:
    * **Too small *m***: High false positive rate or requires very few items.
    * **Too large *m***: Wastes space, though it reduces FP rate.
    * **Too small *k***: Bits are not spread out enough, leading to higher FP for a given *m*.
    * **Too large *k***:
        * Slower operations (O(k)).
        * Bits fill up faster (more bits set per item), leading to higher FP for a given *m*.
    * The optimal *k* balances these effects.

3.  **Hash Function Quality**:
    * Poor hash functions (non-uniform, correlated) can lead to clustering of set bits, increasing the actual false positive rate beyond the theoretical calculation.
    * They also need to be fast, as *k* hashes are computed for each operation.

4.  **No Deletion (in standard Bloom filters)**:
    * Once an element is added, its corresponding bits are set to 1. These bits cannot be reset to 0 because doing so might inadvertently remove other elements that share those bits.
    * If deletion is required, variations like Counting Bloom Filters must be used.

5.  **Empty Filter**:
    * A newly initialized Bloom filter (all bits are 0) will correctly report that any element `mightContain(item)` is `false`.

6.  **Adding the Same Element Multiple Times**:
    * Adding an element that is already "in" the filter (i.e., its bits are already set) has no additional effect on the bit array. The `mightContain` result will remain the same.

---
## Variations and Advanced Topics

1.  **Counting Bloom Filter (CBF)**:
    * **Concept**: Instead of a bit array, uses an array of small counters (e.g., 4-bit, 8-bit, or 16-bit counters).
    * **Add**: When adding an item, increment the counters at the *k* hash positions.
    * **Query**: Check if all *k* counters are non-zero.
    * **Delete**: To remove an item, decrement the counters at its *k* hash positions.
    * **Trade-offs**:
        * Supports deletion.
        * Requires significantly more space (e.g., 4x to 16x more space if using 4-bit to 16-bit counters instead of single bits).
        * Risk of counter overflow if not sized appropriately or if items are added many times.
        * Deletion can introduce false negatives if an item that was "removed" (counters decremented) is still present due to another identical item that was not removed, and their shared counter reaches zero. Proper handling of "multiple identical items" vs "distinct items" is key. Generally, CBFs are for distinct items; if an item is removed, its contribution is fully removed.

2.  **Scalable Bloom Filter (SBF)**:
    * **Concept**: Addresses the issue of a fixed-size Bloom filter that saturates if more than the expected number of items (*n*) are added.
    * An SBF is a series of Bloom filters $BF_1, BF_2, \ldots, BF_s$, each with increasing capacity and a tighter false positive probability.
    * When the current filter ($BF_i$) reaches its capacity (or a "fullness" threshold), a new, larger filter ($BF_{i+1}$) is added. New items are added to the newest filter.
    * **Add**: Add item to the newest filter. If it becomes too full, create a new filter.
    * **Query**: Check all filters. If any filter says "might contain," then the SBF says "might contain." The overall false positive rate is bounded by a target.
    * **Trade-offs**: Adapts to varying numbers of items but can be slower for queries (must check multiple filters) and more complex.

3.  **Partitioned Bloom Filter**:
    * **Concept**: The *m* bits of the array are divided into *k* slices (partitions). Each hash function $h_i$ maps to an index within its respective slice $i$.
    * **Advantage**: Can improve CPU cache performance because each hash function only accesses a specific region of the bit array. This can make it faster than a standard Bloom filter where hash functions access random locations across the entire array.
    * Typically, each slice would be $m/k$ bits.

4.  **d-left Counting Bloom Filter**:
    * A variation that uses d-left hashing for better load balancing and space efficiency, particularly for counting versions.

5.  **Blocked Bloom Filter**:
    * Organizes bits into blocks that fit CPU cache lines to improve performance. Similar motivation to Partitioned Bloom Filters.

6.  **Cuckoo Filter**: (Covered in comparison below) An alternative that supports deletion and can offer better space efficiency.

---
## Comparison with Similar Data Structures

| Feature                | Bloom Filter                     | Hash Set / Hash Table       | Bit Map (Dense)           | Counting Bloom Filter    | Cuckoo Filter             |
|------------------------|----------------------------------|-----------------------------|---------------------------|--------------------------|---------------------------|
| **Stores Elements?** | No                               | Yes                         | No (implicitly by index)  | No                       | Partial (fingerprints)    |
| **Space Efficiency** | Very High (probabilistic)        | Moderate to Low             | High (for dense sets)     | Moderate (counters)      | Very High                 |
| **False Positives?** | Yes (tunable)                    | No                          | No                        | Yes (tunable)            | Yes (tunable)             |
| **False Negatives?** | No                               | No                          | No                        | No (generally)           | No                        |
| **`add` Time** | O(k)                             | Amortized O(1) / O(N) worst | O(1)                      | O(k)                     | Amortized O(1)            |
| **`query` Time** | O(k)                             | Amortized O(1) / O(N) worst | O(1)                      | O(k)                     | Amortized O(1)            |
| **`delete` Support?** | No (Standard)                    | Yes                         | Yes (if value can be unset) | Yes                      | Yes                       |
| **Primary Use Case** | Fast, space-saving membership test where FPs are OK | Exact set membership/map    | Tracking presence in a dense, known range | Probabilistic set with deletes | Probabilistic set with deletes, better space than CBF |
| **Memory per element** | Low (e.g., ~10 bits for 1% FP)  | High (stores element + overhead) | 1 bit (if domain is small) | Higher than Bloom (e.g., 40 bits for 4-bit counters and 1% FP) | Low (often better than Bloom for low FP rates) |

**Key Distinctions**:

* **Bloom Filter vs. Hash Set/Table**:
    * Hash sets store the actual items and provide definitive answers (no false positives).
    * Bloom filters save space by not storing items and accept false positives. Choose Bloom filters when memory is paramount and FPs are tolerable.

* **Bloom Filter vs. Bit Map**:
    * A simple bit map uses one bit per possible element in a domain. If the domain is `0` to `N-1`, you need `N` bits. This is efficient if many elements in the domain are present (dense set).
    * Bloom filters are for sparse sets where the domain of possible elements is huge (e.g., all possible strings), but the number of actual elements *n* is much smaller. They use hash functions to map these large-domain elements to a small bit array.

* **Bloom Filter vs. Cuckoo Filter**:
    * **Cuckoo Filters** are a more recent alternative that also provides probabilistic set membership.
    * **Advantages of Cuckoo Filters over Bloom Filters**:
        * **Support deletion** efficiently and without the space overhead of Counting Bloom Filters.
        * Often achieve **better space utilization** for a given false positive rate, especially for low FP rates (e.g., < 3%).
        * Can sometimes be faster.
    * **How Cuckoo Filters Work (Simplified)**: They use a hash table structure where each item has a small number of possible locations (typically two, determined by two hash functions). If both locations are occupied, one of the existing items is "kicked out" (cuckoo hashing) to an alternative location. They store a small "fingerprint" of the item rather than just setting bits.
    * **Trade-offs**: Cuckoo filters have a maximum load factor; if they become too full, insertions can fail (requiring a rebuild or an overflow structure), though this is usually managed. Implementation can be more complex than a standard Bloom filter.

