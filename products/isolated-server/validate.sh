#!/bin/bash

function validate_persistence() {
	rm -rf validate_persistence
	mkdir validate_persistence
	rsync -r persistence/* validate_persistence
	# Pause
	curl -d '{"id": "1","jsonrpc": "2.0","method": "TogglePause","params": ["'"$UUID"'"]}' -H "Content-Type: application/json" -X POST "http://localhost:5555/"
	curl -d '{"id": "1","jsonrpc": "2.0","method": "CheckPause","params": ["'"$UUID"'"]}' -H "Content-Type: application/json" -X POST "http://localhost:5555/"
	rsync -r persistence/* validate_persistence
	# Unpause
	curl -d '{"id": "1","jsonrpc": "2.0","method": "TogglePause","params": ["'"$UUID"'"]}' -H "Content-Type: application/json" -X POST "http://localhost:5555/"
	curl -d '{"id": "1","jsonrpc": "2.0","method": "CheckPause","params": ["'"$UUID"'"]}' -H "Content-Type: application/json" -X POST "http://localhost:5555/"
	cp constants.xml validate_persistence
	cp dsnodes.xml validate_persistence

	backup_name="persistence_$current_date""_""$current_time.tar.gz"
	echo "$backup_name"

	base_dir=$(pwd)
	cd validate_persistence || exit
	validate_output="$(validateDB)"
	if echo "$validate_output" | grep -q "Success"; then
		echo "validation successful!"
		# zip persistence upload to s3
		echo "$backup_name" >latest_backup.txt
		tar -zcf "$backup_name" persistence
		aws s3 cp "$backup_name" "s3://$S3_BUCKET_NAME/persistence/$backup_name"
		aws s3 cp "latest_backup.txt" "s3://$S3_BUCKET_NAME"
	else
		echo "validation failed."
	fi

	cd "$base_dir" || exit
	sleep 1
}

while true; do
	current_date="$(date +%Y%m%d)"
	current_time="$(date +%H%M%S)"

	if [ "$current_time" = "160000" ]; then
		validate_persistence
	fi

	#temp_current_time="$(date +%S)"
	#if [ $temp_current_time = "00" ]; then
	#    validate_persistence
	#fi

	sleep 0.5
done
