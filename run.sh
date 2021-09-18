#!/bin/bash


echo $MODE
if [ "$MODE" == "load" ]; then
    echo "loading persistence."
    isolatedServer -l -t 5000 -u "$UUID" | tee /zilliqa/logs/isolated-server.log
else
    echo "loading from file."
    echo "UUID is: $UUID"
    isolatedServer -t 5000 -f boot.json -u "$UUID" | tee /zilliqa/logs/isolated-server.log
fi
