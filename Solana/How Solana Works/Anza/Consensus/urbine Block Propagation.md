## 🌪️ Turbine in 30 s  
Multi-layer gossip tree that blasts **shreds** to the entire cluster with **constant fan-out**.

---

## 🧬 Layer Tree  
- **Layer 0** = single **root** (rotated every shred)  
- **Layer 1..N** grow by factor `DATA_PLANE_FANOUT`  
- **Weighted shuffle** by stake → heavy validators bubble up first 🏋️‍♂️  
- Shuffle re-seeded every shred (slot-leader id, slot, shred index, type) 🎲

---

## 📦 Shred Flow  
1. Leader → root  
2. Root → `DATA_PLANE_FANOUT` nodes (layer 1)  
3. Each node retransmits to **unique** next layer subset  
4. Repeat until **all** nodes have all shreds  

> Each node only talks to ≤ `DATA_PLANE_FANOUT` peers → **O(log n)** traffic.

---

## 🔢 FEC Math (Binomial)  
**Goal**: keep block success ≈ 100 % as loss compounds per hop.

- **Packet failure**: `P = 1 – (1 – net_loss)²`  
- **Shred group size**: `N = K + M` (data + coding)  
- **Group failure**: `S = 1 – Σ₀ᴹ binom(P,N,i)`  
- **Block success**: `B = (1 – S) ^ (G / N)`

| FEC | G (shreds) | S | B |
|---|---|---|---|
| 16:4 | 8000 | 0.689 | 10⁻²⁰³ ❌ |
| 16:16 | 12800 | 0.0021 | 0.426 ⚠️ |
| 32:32 | 12800 | 0.000048 | 0.990 ✅ |