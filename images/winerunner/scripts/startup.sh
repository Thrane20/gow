#!/bin/bash
set -e

source /opt/gow/bash-lib/utils.sh

gow_log "Starting GOW winerunner"

mkdir -p $HOME/app/

while :
do
	echo "Press [CTRL+C] to stop.."
	sleep 60
done