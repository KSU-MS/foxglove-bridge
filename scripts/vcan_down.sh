#!/bin/bash
#
# Tears down a vcan interface

# Must have root privileges to run this script
if (( $EUID != 0 )); then
  echo "This script must be run as root"
  exit 1
fi

# Default to use "vcan0" if none is specified on the cmd line
IFACE=vcan0
[ -n "$1" ] && IFACE=$1

# Check if vcanX is already down
if ! ip link show up | grep -q ${IFACE}; then
    echo "${IFACE} is already down."
else
    echo "Bringing down ${IFACE}..."

    # Bring down vcanX interface
    sudo ip link set down ${IFACE}

    echo "${IFACE} has been brought down."
fi
