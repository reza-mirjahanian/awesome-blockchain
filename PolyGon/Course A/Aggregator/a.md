
## Aggregator
the **Aggregator** is in charge of gluing several proofs stating
batch correct execution into a single one. This glued proof ensures that if it is correct,
then all the individual proofs for each batch are also accurate. The mechanisms used by
the Aggregator in order to generate such a aggregated proof are known as **proof recursion** and **proof aggregation**. 
This approach of aggregating proofs aims to **increase** the
throughput of the system. We will give some insights on how both mechanisms work.
In this document, we will also discuss topics such as the “prove anything” paradigm,
a mechanism that allows the proof of any input, regardless of whether it is **erroneous**. In
the case of an **invalid input**, instead of proving a state change, a no state change will
be proven. We will also explore zkCounters, which are a mechanism we use to prevent
our batch from fitting into the available execution traces in the system. In the event of
exceeding such length limits, an out of counters (OCC) error will occur. To conclude, we
will discuss how to eliminate zkCounters in the future by implementing a method known
as **Variable Degree Composite Proofs** (VADCOPs).
