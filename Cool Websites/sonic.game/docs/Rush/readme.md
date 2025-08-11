# **🚀 Rush ECS Framework: Key Concepts**

## **🎯 Core Purpose**
- **Declarative ECS framework** built with Rust  
- **Simplifies blockchain integration** for game developers  
- **Goal**: Let any game easily become a **Fully Onchain Game (FOCG)** or **Autonomous World (AW)**  

---

## **🛠️ How Rush Works**
### **🔹 Traditional vs. Onchain Game Dev**
| Traditional Game Dev | Onchain Game Dev (Without Rush) |
|----------------------|----------------------------------|
| Uses **Game Engine** to handle complexity | Must learn **entire blockchain stack** |
| Focus on **game design** | Deal with **blockchain implementation** |
| Fast iteration | High resource cost (time/money) |

### **🔹 Rush's Solution**
- **Removes blockchain complexity** through:  
  - **Declarative Configuration** (TOML DSL)  
  - **Entity-Component-System (ECS)** pattern  
  - **Code Generation** (auto-generates smart contracts & SDKs)  

---

## **🛤️ Rush User Journey**
1. **Build game** in preferred engine (Unity, Unreal, etc.)  
2. **Download Rush CLI**  
3. **Define game world** via **Blueprints** (TOML)  
4. **Rush generates**:  
   - Smart contracts  
   - Engine-specific SDKs  
5. **Deploy & manage** onchain program  

---

## **✨ The Rush Way (Core Principles)**  
- **Declarative** → No low-level logic needed  
- **Rapid** → Fast iteration, loose coupling  
- **ECS-based** → Games = simple data structures  
- **Simple** → Dev experience first  
- **Product-first** → Focus on real utility, not hype  
- **"Ship faster, break things"** → Prefer speed over perfection  
- **Fun** → Gaming should be enjoyable!  

---

## **⚛️ Rush Subatomics (Core Concepts)**  
| Concept | Description | Example |
|---------|-------------|---------|
| **World** | Onchain game state tracker | "Farmland" world |
| **Entities** | Data structure for game objects | `Sheep {x, y, w, h}` |
| **Components** | Data fields in entities | `Sheep.x` (position) |
| **Systems** | Rules for state changes | `MoveForward(sheep)` |
| **Instances** | Actual entity data | `Sheep(x=123, y=-10)` |
| **Blueprints** | TOML world definitions | *(See sample below)* |
| **Gaming Primitives** | Onchain entity templates | `[entity.sheep]` |

### **📜 Sample Blueprint**
```toml
[world]
name = "Your Farmland"
entities = ["player", "sheep"]

[entity.sheep]
x = "f64"  # Position
y = "f64"
w = "f64"  # Size
h = "f64"
```

---

## **📅 Current Status**  
- **Prerelease phase** → Gathering community feedback