**[Zero-Delay Synapse Hoisting]**
**Learning:** Found an unnecessary O(n) heap allocation (`Vec::push`) during `retain_mut` operations for immediate connections (delay=0) inside Spiking Neural Network inner loops.
**Action:** For values processed instantly, hoist them into a local accumulator buffer rather than enqueuing and immediately dequeuing them in a dynamically sized `Vec`.
