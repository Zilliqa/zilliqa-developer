#!/bin/bash


echo $MODE
if [ "$MODE" == "load" ]; then
    echo "loading persistence."
    mkdir -p 
    isolatedServer -l >> /zilliqa/logs/isolated-server.logs
else
    echo "loading from file."
    isolatedServer -f boot.json >> /zilliqa/logs/isolated-server.logs
fi
