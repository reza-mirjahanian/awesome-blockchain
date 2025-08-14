

# Scope & Focus

* **Domain:** Listening to on-chain events on Solana via streaming vs polling.
* **Goal:** Compare event ingestion approaches for *low-latency*, *high-throughput*, and *reliable* streaming.

---

# Core Ideas

* Streaming approaches greatly outperform polling for real-time use cases, reducing redundant requests and latency.
* Solana’s \~400ms block cadence demands systems to handle bursty traffic and enforce backpressure.
* Four main strategies considered: **Polling**, **WebSockets**, **Geyser Plugins**, **Webhooks**—each trades latency, complexity, and scalability distinctively.

---

# Methods & Best Use

### 1)  Polling

* **Definition:** Periodic JSON-RPC fetches (`getBlock`, `getSignaturesForAddress`).
* **Strengths:**

  * Simplicity and easy debugging.
  * Custom logic is straightforward.
* **Weaknesses:**

  * Inefficient when nothing changes.
  * Latency vs cost trade-off depending on polling frequency.
* **Ideal for:** Low-volume or non-urgent tasks demanding control.

### 2)  WebSockets (Pub/Sub)

* **Definition:** Subscribing to `account`, `program`, `logs`, `signature`, `slot` events via RPC.
* **Strengths:**

  * Lower latency compared to polling.
  * Great for quick prototyping with push updates.
* **Weaknesses:**

  * Fragile in production—reconnect storms can drop events.
* **Ideal for:** Prototyping or non-critical UI updates.

### 3)  Geyser Plugins

* **Definition:** Streaming on-chain data directly from validators into external systems (e.g., DB, Kafka).
* **Strengths:**

  * Ultra-low latency—great for HFT, liquidations.
  * Flexible downstream pipelines.
* **Weaknesses:**

  * Requires mature infrastructure and operations.
* **Ideal for:** Latency-sensitive DeFi workflows.

### 4)  Webhooks

* **Definition:** Push notifications sent to endpoints on specific event matches.
* **Strengths:**

  * Simple to deploy; scalable.
  * Handles large address sets efficiently.
* **Weaknesses:**

  * Not optimized for microsecond-level latency.
* **Ideal for:** Alerts, activity feeds, ETL triggers, non-critical triggers.

---

# Comparison Table

| Option     | Latency     | Reliability                 | Throughput | Ops Complexity | Best Use Case                  |
| ---------- | ----------- | --------------------------- | ---------- | -------------- | ------------------------------ |
| Polling    | Medium–High | High (with own retry logic) | Low–Medium | Low            | Custom logic, low-volume scans |
| WebSockets | Low         | Medium (prone to drops)     | Medium     | Low–Medium     | UIs, prototypes                |
| Geyser     | Ultra-Low   | High (with setup)           | Very High  | High           | HFT, real-time analytics       |
| Webhooks   | Low         | High (managed)              | High       | Low            | Alerts, feeds, ETL, workflows  |

---

# Solana-Specific Considerations

* Blocks \~every 400 ms with thousands of transactions → design for bursts.
* Use various commitment levels (processed/confirmed/finalized) to balance latency vs correctness.
* Implement deduplication using signature + index keys for at-least-once delivery.
* Handle reorgs by reconciling slots and rerouting affected data.
* Choose subscription types aligning with your data model (state vs event).
* Normalize and index data (program, account, slot, signature) and build materialized views for queries.

---

# Architectural Patterns

### Resilient Consumer Pipeline

* **Ingestion:** Push streams → durable queue.
* **Deduplication:** Based on signature or idempotency keys.
* **Checkpointing:** Store last finalized slot and last processed signature.
* **Retry logic:** Exponential backoff with jitter; poison-queue handling.
* **Observability:** Track lag, drop rate, reorg corrections, latency.
* **SLOs:** Define end-to-end budgets from ingest to persistence.

### Job Type Recommendations

* **Feeds / notifications:** Webhooks → queue → DB → UI push
* **Risk alerts:** Webhooks with confirmed-level triggers, finalize reconciliation
* **Trading / liquidations:** Geyser with colocated compute
* **Dashboards:** WebSockets with periodic polling for correctness

---

# Typical Failure Modes & Mitigations

* **WebSocket drops:** Resume with cursor-based polling and reconnect logic.
* **Queue overload:** Utilize backpressure and workload prioritization.
* **Webhook timeouts:** Ensure idempotent handlers and fast ACK, return non-2xx for retries.
* **Schema changes:** Use versioned payloads; dual-write during migrations.
* **Partition hotspots:** Shard data streams by program, account, or slot range.

---

# Subscription Type Guidance (Pub/Sub)

* `accountSubscribe` → watch specific account changes.
* `programSubscribe` → catch all account activity for a program.
* `logsSubscribe` → access logged events for semantics.
* `signatureSubscribe` → monitor transaction status.
* `slotSubscribe` → monitor chain progress for control loops.

---

# Data Modeling Heuristics

* Unique keys: `(signature, instruction_index, inner_index)`.
* Create denormalized projections for entities like positions, orders, NFT sales.
* Use append-only, time-series tables keyed by `block_time` and program/account combos.

---

# Testing & Validation

* Use property-based tests for event ordering and idempotency.
* Run chaos experiments: consumer kill, drop/reorder events, ensure recovery.
* Store raw events to enable replay and deterministic reindexing.

---

# Use Cases & Their Patterns

* **Bots:** Actions on NFT listings or margin health → reactive trades.
* **Alerts:** Program logs → PagerDuty; balance drift → messages.
* **Indexing:** Program transactions → DB ingestion.
* **Notifications:** Transfers → Slack/email triggers.
* **Analytics:** Events → ETL → long-term storage.
* **Workflows:** Events → trigger multi-step logic pipelines.

---

# Latency Economies

* Co-locate services for faster network hops.
* Measure routing and deserialization at p99.
* For notifications: ACK immediately, process afterward.
* For trades: inline critical path processing only.

---

# Implementation Patterns

### Webhook Handler Pseudocode

1. Parse and validate schema.
2. Generate idempotency key; dedupe.
3. Enqueue job and ACK quickly.
4. Process asynchronously; persist and trace metrics.

### WebSocket Recovery Loop

* Track `lastFinalizedSlot`.
* On disconnect, poll missing slots; catch up via `getSignaturesForAddress`.
* Resume Subscriptions post-catch-up.

---

# Decision Blueprint

* **Fastest path today?** → Webhooks.
* **Live dashboard?** → WebSockets + polling fallback.
* **Microlatency-critical?** → Geyser.
* **Custom logic or filters?** → Polling.

---

# Key Facts from the Article

* Solana blocks every \~400 ms with high tx volume.
* Four listening methods: Polling, WebSockets, Geyser, Webhooks.
* WebSockets are low-latency but fragile.
* Geyser is fastest but ops-intensive.
* Webhooks balance ease, scale, and cost-effectiveness.

---

