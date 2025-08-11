### **CosmWasm Tips & Tricks**  

---

#### **1. Setup & Tooling**  
- **Install Rust**:  
  ```bash  
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
  ```  
- **Add Wasm Target**:  
  ```bash  
  rustup target add wasm32-unknown-unknown  
  ```  
- **Essential Tools**:  
  - `cargo-generate`: Scaffold projects from templates.  
    ```bash  
    cargo install cargo-generate  
    ```  
  - `cosmwasm-check`: Validate Wasm binaries.  
    ```bash  
    cargo install cosmwasm-check  
    ```  
- **IDE Setup**:  
  - Use **Rust Analyzer** in VS Code for autocompletion.  

---

#### **2. Project Structure**  
- **Template**:  
  ```  
  src/  
  ├── contract.rs   # Entry points (instantiate, execute, query)  
  ├── msg.rs        # Message types (InstantiateMsg, ExecuteMsg, QueryMsg)  
  ├── state.rs      # Storage definitions (Item, Map)  
  └── lib.rs        # Module exports  
  ```  
- **Key Dependencies**:  
  ```toml  
  [dependencies]  
  cosmwasm-std = "1.2.0"  
  cw-storage-plus = "1.2.0"  
  schemars = "0.8.12"  # JSON schema generation  
  ```  

---

#### **3. Contract Entry Points**  
- **`instantiate`**: Initialize contract state.  
  ```rust  
  pub fn instantiate(  
      deps: DepsMut,  
      _env: Env,  
      info: MessageInfo,  
      msg: InstantiateMsg,  
  ) -> Result<Response, StdError> {  
      CONFIG.save(deps.storage, &Config { owner: info.sender })?;  
      Ok(Response::default())  
  }  
  ```  
- **`execute`**: Handle state changes.  
  ```rust  
  pub fn execute(  
      deps: DepsMut,  
      env: Env,  
      info: MessageInfo,  
      msg: ExecuteMsg,  
  ) -> Result<Response, StdError> {  
      match msg {  
          ExecuteMsg::UpdateOwner { new_owner } => update_owner(deps, info, new_owner),  
      }  
  }  
  ```  
- **`query`**: Read-only operations.  
  ```rust  
  pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {  
      match msg {  
          QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),  
      }  
  }  
  ```  

---

#### **4. State Management**  
- **`Item`**: Store single structs (e.g., config).  
  ```rust  
  pub const CONFIG: Item<Config> = Item::new("config");  
  ```  
- **`Map`**: Key-value storage (e.g., user balances).  
  ```rust  
  pub const BALANCES: Map<&Addr, u128> = Map::new("balances");  
  ```  
- **Composite Keys**: Use tuples for multi-key maps.  
  ```rust  
  pub const DELEGATIONS: Map<(Addr, Addr), u128> = Map::new("delegations");  
  ```  

---

#### **5. Error Handling**  
- **Custom Errors**:  
  ```rust  
  #[derive(Error, Debug, PartialEq)]  
  pub enum ContractError {  
      #[error("Unauthorized")]  
      Unauthorized {},  
      #[error("Invalid input")]  
      InvalidInput {},  
  }  
  ```  
- **Propagation**: Use `?` for concise error handling.  
  ```rust  
  let owner = CONFIG.load(deps.storage)?;  
  ```  

---

#### **6. Testing**  
- **Unit Tests**:  
  ```rust  
  #[cfg(test)]  
  mod tests {  
      use super::*;  
      use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};  

      #[test]  
      fn test_instantiate() {  
          let mut deps = mock_dependencies();  
          let info = mock_info("alice", &[]);  
          let res = instantiate(deps.as_mut(), mock_env(), info, InstantiateMsg {}).unwrap();  
          assert_eq!(0, res.messages.len());  
      }  
  }  
  ```  
- **Integration Tests**: Use `cw-multi-test`.  
  ```rust  
  use cw_multi_test::{App, ContractWrapper, Executor};  

  fn setup() -> App {  
      let mut app = App::default();  
      let code = ContractWrapper::new(execute, instantiate, query);  
      app.store_code(Box::new(code));  
      app  
  }  
  ```  

---

#### **7. Security Best Practices**  
- **Validate Inputs**:  
  ```rust  
  deps.api.addr_validate(&address)?;  
  ```  
- **Avoid `unwrap()`**: Use `?` or `map_err()`.  
- **Access Control**:  
  ```rust  
  if info.sender != config.owner {  
      return Err(ContractError::Unauthorized {});  
  }  
  ```  
- **Gas Optimization**:  
  - Batch storage operations.  
  - Use `may_load` instead of `load` for optional reads.  

---

#### **8. Interacting with Cosmos Modules**  
- **Bank Module**: Send tokens.  
  ```rust  
  use cosmwasm_std::BankMsg;  

  let msg = BankMsg::Send {  
      to_address: recipient.to_string(),  
      amount: vec![coin(100, "uosmo")],  
  };  
  Ok(Response::new().add_message(msg))  
  ```  
- **Staking Module**: Delegate tokens.  
  ```rust  
  use cosmwasm_std::StakingMsg;  

  let msg = StakingMsg::Delegate {  
      validator: "validator_address".to_string(),  
      amount: coin(100, "uosmo"),  
  };  
  ```  

---

#### **9. Contract Migration**  
- **Upgrade Flow**:  
  1. Upload new code.  
  2. Execute `MigrateMsg` on the old contract.  
  ```rust  
  pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {  
      let old_version = CONTRACT_VERSION.load(deps.storage)?;  
      // Update state logic  
      Ok(Response::default())  
  }  
  ```  

---

#### **10. Common Pitfalls**  
- **Unbounded Loops**: Avoid iterating large datasets.  
- **Panics**: Never use `panic!()` (breaks determinism).  
- **Overflows**: Use `checked_add`/`checked_sub`.  
  ```rust  
  let new_balance = old_balance.checked_add(amount)?;  
  ```  

---

#### **11. Advanced Tips**  
- **Cross-Contract Calls**:  
  ```rust  
  let msg = WasmMsg::Execute {  
      contract_addr: "other_contract".to_string(),  
      msg: to_binary(&OtherContractMsg::Action {})?,  
      funds: vec![],  
  };  
  Ok(Response::new().add_message(msg))  
  ```  
- **Query Optimization**:  
  - Use `prefix` or `range` for efficient data retrieval.  
  - Cache frequent queries.  

---

#### **12. CosmWasm vs. Solidity**  

| **Feature**       | **CosmWasm**                          | **Solidity**                |  
|--------------------|---------------------------------------|-----------------------------|  
| **Reentrancy**     | Impossible (message-based)            | Possible (use checks-effects-interactions) |  
| **Gas Costs**      | Predictable (Wasm bytecode)           | Variable (EVM opcodes)      |  
| **Upgradability**  | Code migration + instantiation        | Proxy patterns              |  

---

#### **13. Key Commands**  
- **Build**:  
  ```bash  
  RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown  
  ```  
- **Optimize**:  
  ```bash  
  docker run --rm -v "$(pwd)":/code \  
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \  
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \  
    cosmwasm/rust-optimizer:0.12.13  
  ```  

---

#### **14. Ecosystem Tools**  
- **CosmJS**: JavaScript client for CosmWasm.  
- **Keplr Wallet**: Integrate with frontend dApps.  
- **Terra Station**: Deploy contracts on Terra.  

---

#### **15. Resources**  
- **Docs**: [CosmWasm Documentation](https://docs.cosmwasm.com)  
- **Templates**: [cw-template](https://github.com/CosmWasm/cw-template)  
- **Community**: [Cosmos Discord](https://discord.gg/cosmosnetwork)