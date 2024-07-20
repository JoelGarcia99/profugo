#!/bin/sh
set -e

echo "VERIFYING THE FOLDER 'backups' EXISTS"

if [ ! -d "backups" ]; then
	echo "CREATING FOLDER 'backups'"
	mkdir backups
fi

echo "VERIFYING THE FOLDER 'logs' EXISTS"

if [ ! -d "logs" ]; then
	echo "CREATING FOLDER 'logs'"
	mkdir logs
fi

echo "COMPILING PROFUGO"
cargo build --release

echo "DONE"

