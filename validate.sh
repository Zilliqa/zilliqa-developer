#!/bin/bash
  


function validate_persistence () {
    rm -rf validate_persistence
    mkdir validate_persistence
    cp -r persistence validate_persistence
    cp constants.xml validate_persistence
    cp dsnodes.xml validate_persistence

    backup_name="persistence_$current_date""_""$current_time.tar.gz"
    echo "$backup_name"
    s3_bucket_name="$S3_BUCKET_NAME"

    base_dir=$(pwd)
    cd validate_persistence
    validate_output="$(validateDB)"
    if echo $validate_output | grep -q "Success"; then
        echo "validation successful!"
        # zip persistence upload to s3
        echo $backup_name > latest_backup.txt
        tar -zcf $backup_name persistence
        aws s3 cp "latest_backup.txt" "s3://$s3_bucket_name"
        aws s3 cp "$backup_name" "s3://$s3_bucket_name/persistence/$backup_name"
    else
        echo "validation failed."
    fi

    cd $base_dir
    sleep 1
}



while [ true ]
do
    current_date="$(date +%Y%m%d)"
    current_time="$(date +%H%M%S)"

    temp_current_time="$(date +%S)"
    #if [ $current_time = "160000" ]; then
    if [ $temp_current_time = "00" ]; then
        validate_persistence
    fi

    sleep 0.5
done
