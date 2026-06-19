#!/bin/bash
cargo run -p myco-flock -- --headless &
PID=$!
sleep 5
if ps -p $PID > /dev/null
then
   echo "myco-flock is still running. Process hanging!"
   kill $PID
else
   echo "myco-flock exited successfully."
fi
