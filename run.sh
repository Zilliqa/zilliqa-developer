#!/bin/bash


echo $MODE
if [ "$MODE" == "load" ]; then
    echo "loading persistence."
    isolatedServer -l -t 5000 | tee /zilliqa/logs/isolated-server.log
else
    echo "loading from file."
    isolatedServer -t 5000 -f boot.json | tee /zilliqa/logs/isolated-server.log
fi
