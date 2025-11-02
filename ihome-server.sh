#!/usr/bin/env bash
set -euo pipefail

# Config
LOG="output.log"
PID_FILE="output.pid"

CMD=(nohup cargo watch -x run --ignore "$LOG" > "$LOG" 2>&1 &)

start() {
  if [[ -f "$PID_FILE" ]] && kill -0 "$(cat "$PID_FILE")" 2>/dev/null; then
    echo "Already running (pid $(cat "$PID_FILE"))."
    return 0
  fi

  if ! command -v cargo-watch >/dev/null 2>&1 && ! command -v cargo-watch >/dev/null 2>&1; then
    echo "Warning: 'cargo watch' not found in PATH. Install it with 'cargo install cargo-watch'." >&2
  fi

  echo "Starting: nohup cargo watch -x run --ignore \"$LOG\" > $LOG 2>&1 &"

  nohup cargo watch -x run --ignore "$LOG" > "$LOG" 2>&1 &
  echo $! > "$PID_FILE"
  sleep 0.2
  echo "Started (pid $(cat "$PID_FILE")). Logging to $LOG"
}

stop() {
  if [[ ! -f "$PID_FILE" ]]; then
    echo "No PID file found ($PID_FILE). Is it running?"
    return 0
  fi

  PID=$(cat "$PID_FILE")
  if kill -0 "$PID" 2>/dev/null; then
    echo "Stopping pid $PID..."
    kill "$PID"
    for i in {1..10}; do
      if ! kill -0 "$PID" 2>/dev/null; then
        break
      fi
      sleep 0.3
    done
    if kill -0 "$PID" 2>/dev/null; then
      echo "Still running; sending SIGKILL..."
      kill -9 "$PID" || true
    fi
    echo "Stopped."
  else
    echo "Process $PID not running."
  fi
  rm -f "$PID_FILE"
}

status() {
  if [[ -f "$PID_FILE" ]]; then
    PID=$(cat "$PID_FILE")
    if kill -0 "$PID" 2>/dev/null; then
      echo "Running (pid $PID). Log: $LOG"
      return 0
    else
      echo "PID file exists but process $PID not running."
      return 1
    fi
  else
    echo "Not running (no $PID_FILE)."
    return 3
  fi
}

restart() {
  stop
  start
}

case "${1-}" in
  start)   start ;;
  stop)    stop ;;
  status)  status ;;
  restart) restart ;;
  *) 
    cat <<EOF
Usage: $0 {start|stop|status|restart}

This script runs:
  nohup cargo watch -x run --ignore "$LOG" > $LOG 2>&1 &

It stores the background PID in: $PID_FILE
Log file: $LOG

Examples:
  $0 start
  $0 status
  $0 stop
EOF
    exit 1
    ;;
esac
