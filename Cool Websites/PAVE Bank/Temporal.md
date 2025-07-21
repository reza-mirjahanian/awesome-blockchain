**Temporal** (from [Temporal.io](https://temporal.io)) is a **durable execution system** that helps developers build **reliable, long-running applications** without worrying about infrastructure failures.

---

## 🧠 What Is Temporal?

**Temporal** is an **open-source platform** that lets you write business logic as **code**, not scripts or state machines, and ensures that it **executes reliably**, even in the face of:

* Crashes
* Network outages
* Deployments
* Hardware failures

It’s built on the concept of **"durable workflows"**—your logic runs like a function call, but can last **minutes, hours, or months**, and **automatically resumes** after any interruption.

---

## 🚀 Core Use Case

**Building business logic that *must* run and *will* eventually complete, even if it takes days, weeks, or months.** Common examples:

* Payment processing with retries
* Multi-step order fulfillment
* Scheduled emails
* User signup flows (multi-step + timeouts)
* Video transcoding pipelines
* Infrastructure provisioning (e.g., CI/CD, Terraform orchestration)

---

## 🧱 Core Concepts

| Concept             | Description                                                                                                            |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **Workflow**        | The durable function you write; it’s resumable, reliable, and runs on Temporal's execution engine.                     |
| **Activity**        | The building blocks called from within a workflow (e.g., send email, charge card). Failures are retried automatically. |
| **Worker**          | The process that runs your workflows and activities. Can scale horizontally.                                           |
| **Task Queue**      | Workers listen to queues; Temporal schedules workflow steps into task queues.                                          |
| **Temporal Server** | Central orchestration engine (can be self-hosted or used via [Temporal Cloud](https://temporal.io/cloud)).             |

---

## 🔄 How Durable Execution Works

Temporal uses **event sourcing** + **state replaying**:

1. Your workflow is executed once and decisions (events) are stored.
2. If the workflow crashes, it restarts and replays past events to restore the exact state.
3. This makes code execution **deterministic** and resilient.

So you write code *as if* it runs once and synchronously—but it **actually runs over time**, with built-in retries, timers, and durability.

---

## ✅ Benefits

| Benefit                  | Why It Matters                                                           |
| ------------------------ | ------------------------------------------------------------------------ |
| **Fault-tolerance**      | Workflows survive crashes, timeouts, or restarts without manual retries. |
| **Simplified retries**   | Automatic retries of activities, with backoff, jitter, etc.              |
| **Long-running support** | Sleep for hours, wait for signals/events, and resume instantly.          |
| **Observability**        | Built-in event history, tracing, and logging.                            |
| **Language SDKs**        | Available for Go, Java, TypeScript, Python, PHP, and .NET.               |
| **Temporal Cloud**       | Fully managed service so you don’t need to host the Temporal server.     |

---

## 📘 Example (TypeScript SDK)

```ts
// workflow.ts
import { defineSignal, setHandler, sleep } from '@temporalio/workflow';

export async function exampleWorkflow(name: string): Promise<void> {
  console.log(`Starting long-running workflow for ${name}`);
  await sleep('5 days');  // Durable timer, not blocking a thread
  console.log(`5 days passed. Resuming...`);
}
```

You could crash the server, restart after a week, and this workflow would **continue where it left off**.

---

## 🧪 Who Uses Temporal?

* **Netflix** – video encoding pipelines
* **Coinbase** – crypto transactions
* **Snap** – user session handling
* **Instacart, Datadog, HashiCorp, Stripe**, and many more

---

## 🆚 Comparison with Related Tools

| Tool                     | Key Difference                                                                 |
| ------------------------ | ------------------------------------------------------------------------------ |
| **Airflow**              | Designed for data pipelines, not arbitrary long-running code.                  |
| **Step Functions (AWS)** | Limited to visual workflows, state machines, and AWS ecosystem.                |
| **Celery/Sidekiq**       | Lacks durable, resumable workflows or advanced state handling.                 |
| **Temporal**             | Lets you write logic as normal code with durability, retry, and observability. |

---

## 🔧 Deployment Options

1. **Self-host Temporal Server**
2. **Use Temporal Cloud** (managed SaaS)

---

## 🧠 Summary

> **Temporal = durable, resumable code execution for long-running, business-critical workflows.**

You write normal-looking code.
Temporal makes it **resilient, scalable, observable**, and **safe to run over months**.

---

