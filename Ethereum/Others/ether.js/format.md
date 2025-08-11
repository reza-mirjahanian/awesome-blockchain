
```javascript
// Convert user-provided strings in ether to wei for a value
eth = parseEther("1.0")
// 1000000000000000000n

// Convert user-provided strings in gwei to wei for max base fee
feePerGas = parseUnits("4.5", "gwei")
// 4500000000n

// Convert a value in wei to a string in ether to display in a UI
formatEther(eth)
// '1.0'

// Convert a value in wei to a string in gwei to display in a UI
formatUnits(feePerGas, "gwei")
// '4.5'
```