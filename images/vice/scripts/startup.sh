#!/bin/bash
set -e

source /opt/gow/bash-lib/utils.sh

gow_log "Starting VICE with DISPLAY=${DISPLAY}"

# Start Steam
exec /usr/bin/x64sc