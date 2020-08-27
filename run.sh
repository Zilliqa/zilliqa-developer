#!/bin/bash


echo $MODE
if [ "$MODE" == "load" ]; then
    echo "loading persistence."
    isolatedServer -l -t 5000 >> /zilliqa/logs/isolated-server.logs
else
    echo "loading from file."
    isolatedServer -t 5000 -f boot.json >> /zilliqa/logs/isolated-server.logs
fi
