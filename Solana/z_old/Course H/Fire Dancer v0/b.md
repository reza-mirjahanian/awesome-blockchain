# Detailed Breakdown of the Text

## **Welcome and Introduction**
- **Workshop Title**: Salon Validator Educational Workshop
- **Topic**: Setting up with Firedancer v0.1 (Franken Dancer)
- **Host**: Tim
- **Presenter**: Anway from the Firedancer team
- **Agenda**:
  - Overview of Firedancer v0.1
  - Codebase setup and configuration

---

## **Overview of Firedancer v0.1 (Franken Dancer)**
- **Description**:
  - A Solana validator written in C.
  - Focuses on **security**, **performance**, and **scalability**.
- **Key Differences from Full Firedancer**:
  - Still uses the **Agave runtime** and consensus.
  - Implements its own **TPU signature verification**, **quick block packing**, and other leader-specific functionalities.
- **Codebase Details**:
  - Agave code exists as a **submodule** in the repository.
  - Submodule points to a fork of the Solana repo, which will soon update to the Agave repo.
  - Repository is updated in tandem with Agave's releases.
  - Current commit stack is up to date with **v1.1.1817**.
  - Additional Firedancer-specific commits are added on top.

---

## **Release Versioning**
- **Versioning Structure**:
  - **Major Version**: Always `0` for Franken Dancer.
  - **Minor Version**:
    - Starts at `100`.
    - Increments by `1` for minor updates.
    - Increments by `100` for major Firedancer-specific releases.
  - **Patch Version**: Encodes the version of the Agave codebase being used.
    - Example: Version `17.14` → Patch version `1.1714`.
- **Documentation**:
  - Detailed release structure is available in the **Firedancer Docs** link on the G repository.
  - Includes **hardware requirements** and getting started instructions.

---

## **Hardware Requirements**
- Similar to a typical Agave validator.
- **Architecture**:
  - Franken Dancer uses a **tile-based architecture**.
  - Validator is structured as a collection of processes, each with:
    - Its own **CPU**.
    - A bit of **shared memory**.
  - Processes use the entire CPU they are assigned.
  - Agave runtime and replay stages run alongside but cannot use CPUs dedicated to Franken Dancer.

---

## **Codebase Setup**
### **Cloning the Repository**
1. Clone the Firedancer repository:
   ```bash
   git clone --recurse-submodules <repository_url>
   ```
   - The `--recurse-submodules` flag ensures the Agave submodule is cloned.
   - Without this, you must run a separate command to clone the submodule.

2. Checkout the `v0.1` branch:
   ```bash
   git checkout v0.1
   ```

3. Update submodules:
   ```bash
   git submodule update
   ```

---

### **Dependencies and Compilation**
1. **Install Dependencies**:
   - Use the `deps.sh` script:
     ```bash
     ./deps.sh
     ```
   - Installs required libraries and packages.
   - Builds dependencies and stores them in the `opt` folder.

2. **Compile the Binary**:
   - Compile the main binary `fdct` (Firedancer Control):
     ```bash
     make
     ```
   - Includes both **C** and **Rust** code.
   - Compiles for the native machine type by default.
     - For cross-compilation, specify the machine type.

3. **Recommended Compilers**:
   - GCC versions: `11`, `12`, `13`.

---

## **Configuration**
### **Default Configuration File**
- **File**: `default.toml`
  - Contains default configuration values and documentation for each parameter.
  - Acts as a reference for customizing configurations.

### **Overriding Configurations**
- Pass a custom TOML file to override specific values:
  - Example: Specify a custom ledger directory.
    - Search for `ledger` in `default.toml` to find the relevant parameter.

### **Key Configuration Parameters**
- **User**:
  - The Solana user (e.g., `solana`) should not have superuser privileges.
  - Commands must be run as a privileged user.
- **RPC**:
  - Disabled by default. Must be explicitly enabled.
- **Paths**:
  - Ledger directory, snapshot directory, log file, identity path, vote account path.
- **Dynamic Port Range**:
  - Configurable for network communication.

---

### **Tile Layout Configuration**
- **Tiles**:
  - Processes that handle specific tasks in Franken Dancer.
  - Examples: `net_tile`, `shred_tile`, `pack_tile`.
- **Affinity Rules**:
  - Tiles and Solana Labs threads are assigned separate CPU cores.
  - Ensure no overlap between the two ranges.
- **Command**:
  - View the running tiles:
    ```bash
    ./fdct mem --config <config_file>
    ```
  - Example Output:
    - Lists all running tiles, such as `net_tile`, `pack_tile`, etc.
    - Displays the number of tiles and their CPU usage.

---

## **Running the Validator**
### **Initialization**
1. Run the `configure` command:
   ```bash
   sudo ./fdct configure init --config <config_file>
   ```
   - Configures memory, network channels, and other system parameters.

2. Start the validator:
   ```bash
   sudo ./fdct run --config <config_file>
   ```

### **Systemd Integration**
- Use a systemd unit file for easier management:
  - Example Commands:
    ```bash
    sudo systemctl start <service_name>
    sudo systemctl stop <service_name>
    ```

---

## **Monitoring the Validator**
### **Using Solana Binary**
- Build the Solana binary from the Firedancer repository.
- Commands:
  - **Check Validator Status**:
    ```bash
    solana validators
    ```
  - **Catchup Status**:
    ```bash
    solana catchup
    ```

### **Firedancer Monitor**
- Command:
  ```bash
  ./fdct monitor --config <config_file>
  ```
- **Key Metrics**:
  - **Stale**: Indicates if tiles are heartbeating properly.
  - **% Wait**: Time spent waiting for data.
  - **% Finish**: Time spent processing data.
  - **Back Pressure Count**: Indicates if tiles are overwhelmed.
  - **Total TPS**: Transactions processed per second.

### **Prometheus Metrics**
- Available on port `7999`.
- Compatible with Prometheus scrapers.

---

## **Logging**
- **Standard Error Logs**:
  - Minimal logs (level: `notice`).
  - View using `journalctl`.
- **Log File**:
  - Contains detailed logs (level: `info`).
  - Configurable in the TOML file.
- **Solana Logs**:
  - Integrated into Firedancer's logging system.

---

## **Upgrading or Restarting**
1. Wait for a restart window:
   ```bash
   solana-validator wait-for-restart-window --idle-time <minutes>
   ```
2. Stop the validator:
   ```bash
   sudo systemctl stop <service_name>
   ```
3. Restart with updated configuration or binary:
   ```bash
   sudo ./fdct configure init --config <config_file>
   sudo ./fdct run --config <config_file>
   ```

---

## **Troubleshooting Performance**
- **Indicators of Issues**:
  - Tiles stuck at 99% processing time.
  - High back pressure count.
  - Low % Wait time.
- **Solutions**:
  - Add more tiles for high-traffic tasks (e.g., `shred_tile`).
  - Adjust CPU core allocation in the layout configuration.

---

This breakdown organizes the workshop content into structured sections for clarity and ease of reference.