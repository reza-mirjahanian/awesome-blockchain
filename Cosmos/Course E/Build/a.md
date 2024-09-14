### Modules as Sudo

Modules have the ability to perform actions that are not available to regular users. This is because modules are given **sudo** permissions by the state machine. Modules can reject another modules desire to execute a function but this logic must be explicit. Examples of this can be seen when modules create functions to modify parameters:

x/bank/keeper/msg\_server.go
```
if k.GetAuthority()!= msg.Authority {
returnnil, errorsmod.Wrapf(types.ErrInvalidSigner,"invalid authority; expected %s, got %s", k.GetAuthority(), msg.Authority)
}
```