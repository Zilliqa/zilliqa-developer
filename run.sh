#!/bin/bash


echo $AWS_INFRA_MODE
echo $VALIDATOR_MODE

if [ "$VALIDATOR_MODE" == "true" ]; then
    echo "Running in validator mode."
    ./validate.sh
else
    echo "Running in non-validator mode."
    if [ "$AWS_INFRA_MODE" == "true" ]; then
        aws s3api head-object --bucket $S3_BUCKET_NAME --key latest_backup.txt || latest_backup_not_exist=true
        if [ $latest_backup_not_exist ]; then
            echo "latest_backup.txt does not exist, start from boot.json."
            MODE="noload"
        else
            echo "latest_backup.txt exists, proceeding to download"
            aws s3 cp "s3://$S3_BUCKET_NAME/latest_backup.txt" .
            backup_name=$(cat latest_backup.txt)
            echo "latest_backup is $backup_name"
            aws s3api head-object --bucket $S3_BUCKET_NAME --key persistence/$backup_name || backup_file_not_exist=true
            if [ $backup_file_not_exist ]; then
                echo "latest backup file does not exist, start from boot.json."
                MODE="noload"
            else
                 aws s3 cp "s3://$S3_BUCKET_NAME/persistence/$backup_name" .
                 rm -rf persistence/*
                 mkdir -p persistence
                 tar -zxf $backup_name persistence
                 MODE="load"
            fi
        fi
    fi

    echo $MODE
    if [ "$MODE" == "load" ]; then
        echo "loading persistence."
        isolatedServer -l -t 5000 -u "$UUID" | tee /zilliqa/logs/isolated-server.log
    else
        echo "loading from file."
        echo "UUID is: $UUID"
        isolatedServer -t 5000 -f boot.json -u "$UUID" | tee /zilliqa/logs/isolated-server.log
    fi
fi
