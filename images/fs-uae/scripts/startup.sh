#!/bin/bash
set -e

source /opt/gow/bash-lib/utils.sh

gow_log "Starting fs-uae with DISPLAY=${DISPLAY}"

# Start Steam
exec /opt/src/fs-uae/FS-UAE-Launcher/Linux/x86-64/fs-uae-launcher