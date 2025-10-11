## memqsim

### A Research Proposal for Open Scientific Investigation

This research project introduces **memqsim**, a **CPU-only hierarchical memory architecture** designed explicitly to **simulate the largest possible number of qubits at the lowest possible cost**.
Unlike conventional GPU-based simulators that prioritize speed, MemQSim focuses entirely on **maximizing qubit capacity** through scalable memory composition, leveraging inexpensive and volatile resources to push the limits of classical quantum simulation.

The system organizes memory into multiple layers **DRAM, SSDs, disks, and distributed storage across spot instances** treating slower storage tiers as logical extensions of system memory. This approach accepts higher latency in exchange for **orders-of-magnitude increases in addressable state size**, allowing simulations that would otherwise require supercomputing resources to be executed affordably on transient cloud infrastructure.

To achieve **extreme cost efficiency**, MemQSim leverages **cloud spot instances** preemptible computing nodes available at up to **90% lower cost** than standard on-demand servers. The simulator is engineered to tolerate interruptions seamlessly through **resilient checkpointing and rapid recovery**, ensuring that even large-scale simulations can persist across preemptions without significant cost overhead.

The research will empirically determine:

* The **maximum qubit count** achievable within fixed budget constraints using hierarchical memory;
* The **cost-to-qubit scaling curve**, identifying the most efficient architecture for extending state capacity;
* The **practical feasibility of achieving near-supercomputer memory footprints** with only low-cost, transient resources.

MemQSim’s core contribution is to **redefine the purpose of classical quantum simulation**:
not to execute circuits faster, but to **simulate as many qubits as possible for the lowest cost achievable**, transforming large-scale simulation from an exclusive, resource-intensive task into an accessible, open scientific practice.

By publishing all source code, results, and methodologies openly, this project seeks to **democratize high-qubit simulation** and enable independent researchers, institutions, and students worldwide to explore, test, and verify quantum algorithms without reliance on proprietary supercomputers or high-performance GPU clusters.
