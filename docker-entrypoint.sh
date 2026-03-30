#!/bin/sh
set -e

OLLAMA_URLS="${OLLAMA_URLS:-http://localhost:11434}"
PORT="${PORT:-11435}"
TIMEOUT="${TIMEOUT:-300}"

exec /app/ollamaMQ --port "$PORT" --ollama-urls "$OLLAMA_URLS" --timeout "$TIMEOUT" "$@"
