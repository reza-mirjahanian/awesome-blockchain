# **Salon Validator Educational Workshop: Fire Dancer v0 Overview**

https://docs.firedancer.io/


## **Introduction**
- **Presenter**: Anway from the Fire Dancer team.
- **Topic**: Setting up Fire Dancer v0 and understanding the codebase.

---

## **Fire Dancer v0 Overview**
- **Description**: 
  - Fire Dancer v0.1 (Franken Dancer) is a Solana validator written in **C**.
  - Focuses on **security**, **performance**, and **scalability**.
  - Uses **Agave runtime** and **consensus** but implements its own:
    - TPU signature verification.
    - Quick block packing.
  - **Agave code** is included as a **submodule** in the repository.
  - Repository is updated in tandem with Agave releases.

---

## **Codebase and Versioning**
- **Repository**:
  - Forked from the **Solana repo**.
  - Will soon point to the **Agave repo**.
  - Updated with Fire Dancer-specific commits.
- **Versioning**:
  - **Fire Dancer v0.1** (Franken Dancer):
    - Major version remains **0**.
    - Minor version starts at **100** and increments by **1** for patches.
    - Increases by **100** for major Fire Dancer-specific releases.
  - **Patch version** encodes the Agave codebase version (e.g., `1.1714` for Agave v17.14).

---

## **Hardware Requirements**
- Similar to typical Agave validator requirements.
- **Key Consideration**:
  - Franken Dancer uses a **tile-based architecture**:
    - Each process gets its own **CPU** and **shared memory**.
    - Processes use the entire CPU, so **tuning** and **core count** are critical.
    - Replay stage cannot use CPUs dedicated to Fire Dancer.

---

## **Setting Up Fire Dancer**
1. **Clone the Repository**:
   - Use `git clone --recurse-submodules` to include the Agave submodule.
   - Check out the `v0.1` branch.
   - Run `git submodule update` to ensure the submodule is up-to-date.

2. **Install Dependencies**:
   - Use the `depends.sh` script to install dependencies.
   - Dependencies are stored in the `opt` folder (self-contained).

3. **Build the Binary**:
   - Compile the main binary (`fdct` - Fire Dancer Control).
   - Binary includes both **C** and **Rust** code.
   - Compiles for the **native machine type** by default (can cross-compile).

---

## **Configuration**
- **Configuration File**:
  - Uses a **TOML** file for configuration.
  - `default.toml` contains all default values and documentation.
  - Override specific values by passing a custom TOML file.

- **Key Configuration Options**:
  - **User**: Specifies the non-privileged user for running the validator.
  - **RPC**: Off by default (enable for queries).
  - **Logging**: Configure log levels for standard error and log files.
  - **Paths**: Ledger directory, snapshot directory, identity, and vote account paths.

---

## **Running the Validator**
1. **Configure**:
   - Run `fdct configure init` with the TOML file to set up huge pages, caches, and channels.
   - Requires **sudo** privileges.

2. **Run**:
   - Use `fdct run` with the same TOML file.
   - Starts multiple **tiles** (processes) for different tasks.

3. **Systemd Integration**:
   - Create a systemd unit file to manage the validator as a service.
   - Use `systemctl start` and `systemctl stop` to manage the service.

---

## **Monitoring the Validator**
- **Fire Dancer Monitor**:
  - Use `fdct monitor` to view tile performance.
  - Key metrics:
    - **Percent Weight**: Time tiles wait for data.
    - **Percent Finish**: Time tiles spend working.
    - **Back Pressure**: Indicates if tiles are overloaded.
    - **Overrun Count**: Messages not processed due to slowness.

- **Solana Commands**:
  - Use `solana-gossip`, `solana catchup`, and `solana validators` to monitor validator status.
  - Check for delinquent slots, block production, and catch-up status.

- **Prometheus Metrics**:
  - Fire Dancer publishes metrics on port **7999**.
  - Compatible with Prometheus for advanced monitoring.

---

## **Logging**
- **Log Locations**:
  - **Standard Error**: Logs with lower verbosity (notice level).
  - **Log File**: More detailed logs (info level).
- **Log Levels**:
  - Configurable in the TOML file.
  - Solana logs are integrated into Fire Dancer’s logging system.

---

## **Troubleshooting and Tuning**
- **Tile Performance**:
  - If a tile is stuck at **99%**, it may indicate a bottleneck.
  - Add more tiles for overloaded processes (e.g., shred tile).
- **Layout Tuning**:
  - Ensure **CPU affinity** is properly configured.
  - Avoid overlapping CPU ranges for Fire Dancer tiles and Solana threads.

---

## **Upgrading and Maintenance**
- **Wait for Restart Window**:
  - Use `solana-wait-for-restart-window` to find an idle slot for maintenance.
- **Stopping the Validator**:
  - Use `systemctl stop` or `pkill fdct`.
- **Restarting**:
  - Always run `fdct configure init` before `fdct run`.

---

## **Additional Resources**
- **Fire Dancer Documentation**:
  - Available on the GitHub repository.
  - Includes detailed guides on setup, configuration, and monitoring.
- **Community Support**:
  - Reach out for questions or further assistance.