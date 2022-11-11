#!/bin/bash

function print_help() {
	echo "Please input a mainnet-persistence WITHOUT the .tar.gz extension (eg: mainnet-istana-830788)"
	echo
	echo
	echo
}

function print_help2() {
	echo "Please make sure that $(pwd)/downloads/$1.tar.gz exists in $(pwd)/downloads folder."
	echo
	echo
	echo
}

if [ $# -eq 0 ]; then
	print_help
else
	echo "pwd is :$(pwd)"
	echo "Checking if $(pwd).tar.gz exists"
	if [ ! -f "$(pwd)/downloads/$1.tar.gz" ]; then
		print_help2 "$1"
		exit
	else
		echo "It's ok for errors to appear between the 2 arrows."
		echo "    |"
		echo "    v"
		docker stop zilliqa-isolated-server
		docker rm zilliqa-isolated-server
		echo "    ^"
		echo "    |"

		sudo rm -rf "$(pwd)/persistence/$1"
		echo "Persistence to be used: $1"

		mkdir -p "$(pwd)/persistence/$1"
		tar -xf "$(pwd)/downloads/$1.tar.gz" -C "$(pwd)/persistence/$1"

		docker run -d --name zilliqa-isolated-server -p 5555:5555 --env MODE="load" -v "$(pwd)/persistence/$1/persistence":"/zilliqa/persistence" zilliqa-isolated-server:1.0
	fi
fi
