#!/usr/bin/env bash
set -euo pipefail

# Config
LOG="output.log"
PID_FILE="output.pid"

start() {
  if ! command -v cargo-watch >/dev/null 2>&1 && ! command -v cargo-watch >/dev/null 2>&1; then
    echo "Warning: 'cargo watch' not found in PATH. Install it with 'cargo install cargo-watch'." >&2
  fi

  nohup cargo watch -x run --ignore "$LOG" --ignore "$PID_FILE" > "$LOG" 2>&1 &
  echo $! > "$PID_FILE"
  sleep 0.2
  echo "iHome Server Started. PID saved to $PID_FILE. Logging to $LOG"
}

case "${1-}" in
  start)   start ;;
  *) 
    cat <<EOF
Usage: $0 {start}

This script runs:
  nohup cargo watch -x run --ignore "$LOG" > $LOG 2>&1 &

It stores the background PID in: $PID_FILE
Log file: $LOG

Examples:
  $0 start

EOF
    exit 1
    ;;
esac
