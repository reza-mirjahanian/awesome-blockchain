
**IBC (Inter-Blockchain Communication) Relayers and Packet Processing**

**Overview**

* IBC relayers listen to the Hub and pick up events, reconstruct packets, and deliver them to other chains (e.g., Ethereum)
* The receiving chain verifies the packet and processes it accordingly

**Packet Processing**

* **Step 1: Packet Sent**
	+ The Hub sends a packet to the relayer
	+ The relayer reconstructs the packet from the packet hash
* **Step 2: Verification**
	+ The receiving chain (e.g., Ethereum) verifies the packet using a Handler
	+ Verifies that the packet was sent by the Cosmos Hub and hasn't been tampered with
	+ Checks that the packet hasn't timed out
* **Step 3: Processing**
	+ If verification is successful, the receiving chain processes the packet
	+ In the case of a transfer, the application mints tokens corresponding to the denomination and gives them to the user account

**Acknowledgments**

* **Step 4: Acknowledgment**
	+ The receiving chain writes an acknowledgment and commits it to its blockchain
	+ The acknowledgment is emitted via events
* **Step 5: Verification (Again)**
	+ The relayer picks up the acknowledgment and reconstructs it
	+ The Hub verifies the acknowledgment using its client representing the receiving chain
	+ Verifies that the acknowledgment was stored in the state of the receiving chain

**Error Handling**

* **Timeouts**
	+ Packets can be timed out, and the sending chain can process timeouts and revert any changes
* **Failed Acknowledgments**
	+ If the transfer fails, the receiving chain returns a failed acknowledgment, and the tokens are refunded
* **Channel Types**
	+ Channels can have a type (e.g., ordered or unordered)
	+ Ordered channels require packets to be received in order, while unordered channels do not
* **Misbehavior Processing**
	+ Clients can be tricked into verifying a malicious block
	+ Misbehavior processing allows for proof of misbehavior and freezing of the client to prevent further packet processing

**Security Guarantees**

* IBC is built with defense in depth to prevent collusion by validator sets
* Security parameters can be set to require a certain percentage of validators to sign over a block for it to be verified
* Misbehavior processing provides an additional layer of security