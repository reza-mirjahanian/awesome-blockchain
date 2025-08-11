PolygonTransparentProxy.sol
===========================

Inherits from `TransparentUpgradeableProxy`, a Openzeppelin v5 contract, with the following modifications:

*   Admin is a parameter in the constructor instead of being deployed.
*   Let the admin get access to the proxy.
*   Replace `_msgSender()` with `msg.sender`

Functions[¶](#functions)
------------------------

### `constructor`[¶](#constructor)

Initializes an upgradeable proxy managed by an instance of a `ProxyAdmin` with an `initialOwner` backed by the implementation at `_logic`, and optionally initialized with `_data` as explained in `ERC1967Proxy-constructor`.

```
  function constructor(
  ) public

```

### `_proxyAdmin`[¶](#_proxyadmin)

Returns the admin of this proxy.

```
  function _proxyAdmin(
  ) internal returns (address)

```

### `_fallback`[¶](#_fallback)

If caller is the admin, process the call internally. Otherwise, transparently fallback to the proxy behavior.

```
  function _fallback(
  ) internal
```