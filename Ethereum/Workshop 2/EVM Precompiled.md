# Ethereum EVM Precompiled Contracts

EVM **precompiled contracts** are built-in, native functions living at fixed addresses (`0x01`, `0x02`, …).  
They expose complex cryptographic or utility operations at a **fraction** of the gas an equivalent Solidity implementation would cost.  
Conceptually they “behave like smart contracts built into the Ethereum protocol”: you call them with `CALL` / `STATICCALL`, but the execution is performed in optimized native code rather than EVM op-codes.

> They exist because many operations (ECDSA recovery, large-number math, elliptic-curve arithmetic, etc.) would be **prohibitively expensive** or outright impossible in pure Solidity.

---

## Standard Precompiles (`0x01`–`0x0A`) — Mainnet Ready

| Address | Name | Approx. Gas | Function |
|---|---|---|---|
| `0x01` | **ECRECOVER** | 3 000 | ECDSA public-key recovery: `(hash, v, r, s)` → signer address |
| `0x02` | **SHA256** | 60 + 12·⌈len/32⌉ | SHA-2-256 hash of input bytes |
| `0x03` | **RIPEMD160** | 600 + 120·⌈len/32⌉ | RIPEMD-160 hash (20-byte right-aligned output) |
| `0x04` | **IDENTITY** | 15 + 3·⌈len/32⌉ | Byte-for-byte copy (no transformation) |
| `0x05` | **MODEXP** | 200 + variable | Modular exponentiation `B^E mod M` on big integers; dynamic gas (EIP-2565) |
| `0x06` | **BN256ADD** (ECADD) | 150 | Alt-BN128 elliptic-curve point addition |
| `0x07` | **BN256MUL** (ECMUL) | 6 000 | Alt-BN128 scalar multiplication |
| `0x08` | **BN256PAIRING** | 34 000·k + 45 000 | Alt-BN128 pairing check on `k` pairs |
| `0x09` | **BLAKE2F** | ≈1·rounds | BLAKE2b compression function “F”; gas = `GFROUND`·rounds (EIP-152) |
| `0x0A` | **KZG_POINT_EVAL** | 50 000 | KZG point-evaluation proof for blob commitments (EIP-4844, Dencun) |

> `0x0B`–`0x11` are **proposed** BLS12-381 precompiles via **EIP-2537** – not yet active on mainnet.

---

## Why Precompiles Rock

* **Native speed** – clients run optimized C/Go/ASM routines.  
* **Dirt-cheap gas** – elliptic-curve ops that would cost **millions** of gas in Solidity are done for a few thousand.  
* **Constant upgrades** – gas schedules are repriced when libraries improve (e.g. EIP-1108, EIP-2565).

---

## Calling from Solidity

Built-ins for the most common:

```solidity
address signer = ecrecover(hash, v, r, s);
bytes32 h = sha256(data);
bytes20 h160 = ripemd160(data);
```

Everything else: manual `staticcall`.

```solidity
// SHA256 via precompile 0x02
(bytes32 result) = abi.decode(
    address(2).staticcall(abi.encodePacked(data)),
    (bytes32)
);

// MODEXP (0x05)
function modExp(uint256 b, uint256 e, uint256 m) public view returns (uint256) {
    bytes memory args = abi.encodePacked(
        uint256(32), uint256(32), uint256(32), b, e, m
    );
    (bool ok, bytes memory out) = address(5).staticcall(args);
    require(ok, "MODEXP failed");
    return abi.decode(out, (uint256));
}
```

In Yul/assembly:

```yul
let ok := staticcall(gas(), 0x06, input, 128, outPtr, 64) // ECADD
```

---

## Timeline & EIPs

| Fork | Key Precompile EIPs |
|---|---|
| **Frontier/Byzantium** | `0x01–0x04` (EIP-2), `0x05` (EIP-198), `0x06–0x08` (EIP-196/197), `0x09` (EIP-152) |
| **Istanbul (Dec 2019)** | EIP-1108 – lower gas for `0x06–0x08` |
| **Berlin (Apr 2021)** | EIP-2565 – reprice `0x05`; EIP-2929 – “warm” precompile addresses |
| **Dencun (Apr 2024)** | EIP-4844 – adds `0x0A` (KZG point evaluation) |
| **Future?** | EIP-2537 – BLS12-381 (`0x0B–0x11`); EIP-7266 – **remove** unused `0x09` |

---

## Developer Cheat-Sheet

* Always **use the precompile** if it matches your need – you’ll save orders-of-magnitude in gas.  
* Mind the **fixed or formulaic gas** table above; add the base `STATICCALL` cost (~700).  
* Watch new EIPs for fresh precompiles & repricings.

> “These operations were deemed desirable enough to have gas-efficient mechanisms; doing them in Solidity is considerably less gas-efficient.”

---

## Sources

* [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)  
* [EIP-1108](https://eips.ethereum.org/EIPS/eip-1108) – Alt-BN128 gas reduction  
* [EIP-2565](https://eips.ethereum.org/EIPS/eip-2565) – MODEXP repricing  
* [EIP-2537](https://eips.ethereum.org/EIPS/eip-2537) – BLS12-381 precompiles  
* [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844) – KZG point evaluation (Dencun)  
* [RareSkills Guide](https://rareskills.io/post/solidity-precompiles)