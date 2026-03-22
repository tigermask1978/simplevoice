#!/usr/bin/env bash
# Usage: ./scripts/download_model.sh [tiny|base|small|medium]
set -e

MODEL=${1:-small}
DEST="models/ggml-${MODEL}.bin"
BASE_URL="https://huggingface.co/ggerganov/whisper.cpp/resolve/main"

mkdir -p models

if [ -f "$DEST" ]; then
  echo "Model already exists: $DEST"
  exit 0
fi

echo "Downloading whisper ${MODEL} model..."
curl -L --progress-bar "${BASE_URL}/ggml-${MODEL}.bin" -o "$DEST"
echo "Saved to $DEST"
