# ⚡ alloy-dyn-abi

**Dynamic Solidity type encoder**
Provides runtime encoding & decoding of Solidity types.

---

## 🔎 What It Is

* A **runtime representation** of Ethereum's type system.
* Lets you work with Solidity types when they are **not known at compile time**.
* Especially useful for **EIP-712 signing**.

> ⚠️ **Recommendation:**
> Use the **static encoder/decoder** whenever possible:
>
> * ✅ Cheaper (less gas, better performance)
> * ✅ Safer (type checking at compile time)
> * ❌ Dynamic encoder is slower and more error-prone

---

## 📚 Examples

### 🟢 Basic Usage

```rust
use alloy_dyn_abi::{DynSolType, DynSolValue};
use alloy_primitives::hex;

// 1. Parse a Solidity type at runtime
let my_type: DynSolType = "uint16[2][]".parse().unwrap();

// 2. Decode raw ABI data
let my_data = hex!(
    "0000000000000000000000000000000000000000000000000000000000000020" // offset
    "0000000000000000000000000000000000000000000000000000000000000001" // length
    "0000000000000000000000000000000000000000000000000000000000000002" // .[0][0]
    "0000000000000000000000000000000000000000000000000000000000000003" // .[0][1]
);
let decoded = my_type.abi_decode(&my_data)?;

// 3. Define expected Rust-side representation
let expected = DynSolValue::Array(vec![
    DynSolValue::FixedArray(vec![2u16.into(), 3u16.into()])
]);
assert_eq!(decoded, expected);

// 4. Roundtrip check (decode -> encode -> same data)
let encoded = decoded.abi_encode();
assert_eq!(encoded, my_data);

# Ok::<(), alloy_dyn_abi::Error>(())
```

---

### 📩 EIP-712 Example

```rust,ignore
use alloy_dyn_abi::eip712::TypedData;
use alloy_sol_types::sol;

sol! {
    struct Person {
        string name;
        address wallet;
    }

    struct Mail {
        Person from;
        Person to;
        string contents;
    }
}

let sender = Person {
    name: "Cow".to_string(),
    wallet: "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826".parse().unwrap(),
};
let recipient = Person {
    name: "Bob".to_string(),
    wallet: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB".parse().unwrap(),
};
let mail = Mail {
    from: sender,
    to: recipient,
    contents: "Hello, Bob!".to_string()
};

let typed_data = TypedData::from_struct(&mail, None);
let hash = typed_data.eip712_signing_hash().unwrap();

assert_eq!(
    hash,
    b256!("25c3d40a39e639a4d0b6e4d2ace5e1281e039c88494d97d8d08f99a6ea75d775")
);
```

---

## ⚙️ How It Works

The system is built on **enums** that describe Solidity at runtime:

| Enum          | Represents                  | Equivalent To                  |
| ------------- | --------------------------- | ------------------------------ |
| `DynSolType`  | Solidity type               | Rust enum over `SolType` trait |
| `DynSolValue` | Solidity value in Rust form | Enum over `SolType::RustType`  |
| `DynToken`    | ABI token                   | Enum over `Token` trait        |

📌 **Type Info Handling:**

* **Static encoder:** encodes type info into Rust type system at compile time.
* **Dynamic encoder:** encodes type info as a **runtime instance** of `DynSolType`.

🧩 **Detokenizing rule:**

```
DynSolType + DynToken → DynSolValue
```

🔄 Conversion to user-defined Rust types must be done **manually**.

* `From` implementations are available.
* Complex types (arrays, tuples, structs) require extra handling.
* Fallible casts exist for each variant.

---

## 🛠️ `DynToken::decode_populate`

* At runtime, the shape of data is **not known at compile time**.
* Cannot pre-allocate memory with exact size.
* Instead:

  1. Pre-allocate a `DynToken` with the **same shape** as expected type.
  2. Fill it with decoded data.

⚠️ **Important Notes:**

* This behavior is very different from the static decoder.
* Directly using `DynToken` is **not recommended**.
* Prefer methods on `DynSolType` for encoding & decoding.
