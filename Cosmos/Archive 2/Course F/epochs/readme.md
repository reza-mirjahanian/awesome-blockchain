**Epochs Module in Cosmos SDK**


### Panic isolation

If a given epoch hook panics, its state update is reverted, but we keep proceeding through the remaining hooks. This allows more advanced epoch logic to be used, without concern over state machine halting, or halting subsequent modules.

This does mean that if there is behavior you expect from a prior epoch hook, and that epoch hook reverted, your hook may also have an issue. So do keep in mind "what if a prior hook didn't get executed" in the safety checks you consider for a new epoch hook.

---

### Key Concepts

- **Epochs**: An epoch is a defined period during which certain operations or calculations occur. The `epochs` module in the Cosmos SDK provides a standardized way to handle epoch-based logic across modules.
- **Time-Based Triggers**: Allows for operations to execute at regular time intervals, useful for features like interest accrual, reward distributions, and parameter updates.
- **Cross-Module Synchronization**: Ensures that modules dependent on epochs can synchronize their operations based on a common epoch schedule.

---

### Module Parameters

| **Parameter**    | **Type**   | **Description**                                                        |
|------------------|------------|------------------------------------------------------------------------|
| `identifier`     | `string`   | Unique name for the epoch (e.g., "day", "week", "month").              |
| `start_time`     | `Time`     | The start time of the epoch schedule.                                  |
| `duration`       | `Duration` | Length of each epoch period (e.g., 24h for daily epochs).              |
| `current_epoch`  | `int64`    | Counter for the current epoch number.                                  |
| `current_start`  | `Time`     | Start time of the current epoch.                                       |
| `epoch_counting_started` | `bool` | Indicates if epoch counting has started for this epoch identifier. |

**Tips:**

- Define multiple epochs with different identifiers to handle different periodic operations.
- Ensure that `start_time` and `duration` are correctly set to avoid misalignment of epoch events.

---

### Core Functionality

- **Epoch Initialization**: Set up epochs during the chain genesis or via module parameters.
- **Hooks**: Modules can implement hooks to execute code at the start or end of an epoch.
- **Epoch Info Tracking**: Keeps track of epochs' progression, allowing modules to query current and upcoming epochs.

---

### Messages (Transaction Types)

The `epochs` module typically does not expose transaction messages directly to end-users. Instead, it provides infrastructure for other modules.

However, in extended implementations or custom modules, you might find messages such as:

#### 1. **MsgUpdateEpoch**

Allows governance to update the parameters of an existing epoch.

```go
type MsgUpdateEpoch struct {
    Authority   string    `json:"authority"`
    Identifier  string    `json:"identifier"`
    StartTime   time.Time `json:"start_time"`
    Duration    Duration  `json:"duration"`
}
```

- **Usage:**

  ```shell
  appd tx epochs update-epoch [identifier] [start_time] [duration] --from [authority]
  ```

- **Tip:** Ensure you have the appropriate authority (usually the governance module) to update epoch parameters.

---

### Queries

#### 1. **Query Epochs Info**

Retrieve information about all registered epochs.

```shell
appd query epochs epoch-infos
```

- **Output Example:**

  ```json
  {
    "epochs": [
      {
        "identifier": "day",
        "start_time": "2023-01-01T00:00:00Z",
        "duration": "24h",
        "current_epoch": "123",
        "current_epoch_start_time": "2023-05-05T00:00:00Z",
        "epoch_counting_started": true
      },
      {
        "identifier": "week",
        "start_time": "2023-01-01T00:00:00Z",
        "duration": "168h",
        "current_epoch": "17",
        "current_epoch_start_time": "2023-05-01T00:00:00Z",
        "epoch_counting_started": true
      }
    ]
  }
  ```

- **Tip:** Use this query to monitor epoch progression and synchronize module operations accordingly.

#### 2. **Query Current Epoch**

Retrieve the current epoch number and start time for a specific epoch identifier.

```shell
appd query epochs current-epoch [identifier]
```

- **Tip:** Replace `[identifier]` with the epoch identifier you're interested in (e.g., "day").

---

### Tips and Tricks

- **Multiple Epoch Schedules:**

  - Define epochs with different durations to manage operations that need to occur at different intervals.
  - **Example:** Use a "day" epoch for daily rewards and a "week" epoch for weekly bonuses.

- **Implementing Hooks:**

  - **Start Epoch Hook:** Execute logic at the beginning of an epoch.
  - **End Epoch Hook:** Execute logic at the end of an epoch.
  - **Tip:** Modules like `staking` or `incentives` can implement these hooks to automate tasks.

- **Synchronizing Modules:**

  - Use common epoch identifiers across modules that need to synchronize operations.
  - **Example:** Both `liquidity` and `rewards` modules can use the "day" epoch to coordinate daily calculations.

- **Monitoring Epoch Progression:**

  - Set up alerts or monitoring tools to track epoch transitions, ensuring your module logic executes as expected.
  - **Tip:** Use the `Epoch Hooks` events emitted during epoch transitions for monitoring.

- **Epoch Duration Adjustment:**

  - Adjust epoch durations via governance to respond to network needs.
  - **Example:** Shorten epochs during high activity periods to distribute rewards more frequently.

---

### Code Examples

#### Defining an Epoch in Genesis

Include the epochs module configuration in your genesis file:

```json
"epochs": {
  "epochs": [
    {
      "identifier": "day",
      "start_time": "2023-01-01T00:00:00Z",
      "duration": "24h",
      "current_epoch": "0",
      "current_epoch_start_time": "2023-01-01T00:00:00Z",
      "epoch_counting_started": false
    }
  ]
}
```

#### Implementing Epoch Hooks in a Module

```go
type AppModule struct {
    keeper Keeper
}

func (am AppModule) BeginEpoch(ctx sdk.Context, identifier string, epochNumber int64) {
    if identifier == "day" {
        // Your code to execute at the start of each day epoch
        am.keeper.DistributeDailyRewards(ctx)
    }
}

func (am AppModule) EndEpoch(ctx sdk.Context, identifier string, epochNumber int64) {
    if identifier == "week" {
        // Your code to execute at the end of each week epoch
        am.keeper.CalculateWeeklyStatistics(ctx)
    }
}
```

- **Tip:** Register your module's hooks with the epochs module during initialization.

#### Registering Hooks with Epochs Module

```go
func (k Keeper) InitHooks(epochsKeeper epochskeeper.Keeper) {
    epochsKeeper.AddEpochHooks(k)
}
```

---

### Best Practices

- **Consistent Identifier Usage:**

  - Use consistent identifiers for epochs across different modules to avoid confusion.
  - **Tip:** Document the purpose of each epoch identifier in your project's documentation.

- **Governance Oversight:**

  - Changes to epoch parameters should go through the governance process to maintain network consensus.
  - **Tip:** Encourage community discussion before proposing epoch changes.

- **Testing Epoch Logic:**

  - Thoroughly test your epoch-based logic using simulated epoch transitions.
  - **Tip:** Use unit tests and integration tests to cover various scenarios.

- **Time Synchronization:**

  - Ensure validators' system clocks are accurately synchronized to prevent issues with time-based operations.
  - **Tip:** Recommend validators use NTP services for clock synchronization.

---

### Advanced Configuration

- **Dynamic Epoch Adjustment:**

  - Implement logic to adjust epoch durations dynamically based on network conditions.
  - **Example:** Shorten epochs during high congestion to process rewards more frequently.

- **Epoch Migration:**

  - If you need to change the duration or start time of an epoch, implement a migration process to adjust existing state without disrupting the network.

  ```go
  func MigrateEpoch(ctx sdk.Context, k Keeper, identifier string, newDuration time.Duration) {
      epochInfo := k.GetEpochInfo(ctx, identifier)
      epochInfo.Duration = newDuration
      k.SetEpochInfo(ctx, epochInfo)
  }
  ```

- **Cross-Chain Epoch Synchronization:**

  - In IBC-enabled networks, synchronize epochs across chains for coordinated operations.
  - **Tip:** Use relayers to transmit epoch information between chains if necessary.

---

### Important Considerations

- **Time Zones and Daylight Saving Time:**

  - Epochs are timezone-agnostic; they operate on UTC time.
  - **Tip:** Always use UTC when configuring epochs to avoid inconsistencies.

- **Validator Uptime:**

  - Validators must remain online to process epoch transitions; downtime can affect epoch-based operations.
  - **Tip:** Implement redundant systems to maintain validator uptime.

- **Edge Cases in Epoch Transitions:**

  - Be cautious of edge cases, such as very short epochs or misconfigured durations that could lead to rapid epoch cycling.
  - **Tip:** Set minimum and maximum duration limits in your module's parameters.

---

### Essential Commands Summary

| **Command**                                          | **Description**                                            |
|------------------------------------------------------|------------------------------------------------------------|
| `appd query epochs epoch-infos`                      | Query all epoch information.                               |
| `appd query epochs current-epoch [identifier]`       | Query current epoch number and start time for an epoch.    |
| `appd tx epochs update-epoch [params]`               | Update an existing epoch's parameters (if permitted).      |

- **Note:** Replace `appd` with the actual name of your application's daemon.

---

