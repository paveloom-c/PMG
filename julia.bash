#!/bin/bash

# A script to run Julia scripts (as arguments) using a daemon

PORT=3001
ROOT_PATH="$(dirname "$(realpath -s "$0")")"

# Check if a kill was requested
if [ "$1" = "kill" ] ; then
    julia --startup-file=no --project="${ROOT_PATH}" -e "using DaemonMode; sendExitCode(${PORT})" 2>/dev/null
else
    # If the daemon isn't up yet
    if ! lsof -i tcp:${PORT} | grep -q julia ; then
        # Start it up
        julia -t auto --startup-file=no --project="${ROOT_PATH}" -e "using DaemonMode; serve(${PORT}; async=true)" &
        sleep 1
    fi
    # Run the script
    julia \
        --startup-file=no \
        --project="${ROOT_PATH}" \
        -e "cd(\"${ROOT_PATH}/scripts\"); using DaemonMode; runargs(${PORT})" "$(realpath -s "$1")" "${@:2}"
fi
