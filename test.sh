#!/bin/bash
cargo run -p origami-resonance -- --headless > /dev/null 2>&1 &
PID=$!
sleep 2
kill $PID
echo "Completed test"
