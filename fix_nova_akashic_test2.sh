#!/bin/bash
sed -i 's/vm1.ip.0 < 1 \&\& vm1.ip.1 < 3/vm1.ip.0 == 0/g' experiments/chimera-lang/src/nova_akashic_test.rs
sed -i 's/vm2.ip.0 < 1 \&\& vm2.ip.1 < 2/vm2.ip.0 == 0/g' experiments/chimera-lang/src/nova_akashic_test.rs
