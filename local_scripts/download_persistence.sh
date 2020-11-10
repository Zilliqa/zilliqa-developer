#!/bin/bash


function print_help {
    echo "Please input a mainnet-persistence WITHOUT the .tar.gz extension (eg: mainnet-istana-830788)"
    echo "If you need to add access to the mainnet s3 persistence, add the following canonical id:"
    echo $(aws s3api list-buckets --query Owner.ID --output text)
    echo "to the persistence on mainnet s3, then run the command again"
    echo
    echo
    echo
}


if [ $# -eq 0 ]; then
    print_help
else
    echo "pwd is :$(pwd)"
    echo "Checking if download directory exists"
    if [ ! -d "$(pwd)/downloads" ]; then
        echo "Directory does not exist!, creating a downloads folder"
        mkdir -p "$(pwd)/downloads"
    fi
    echo "Persistence to be used: $1"
    echo "Attempting to copy persistence"
    if [ -f "$(pwd)/downloads/$1.tar.gz" ]; then
        echo "$1.tar.gz exists, skipping download."
    else
        aws s3 cp s3://$BUCKET_ID/persistence/"$1.tar.gz" "$(pwd)/downloads/"
        if [ $? -eq 0 ]; then
            echo "Copy successful!"
        else
            print_help
            exit
        fi
    fi
fi
