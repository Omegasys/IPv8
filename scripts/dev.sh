#!/usr/bin/env bash

set -e

echo "🛠 Starting IPv8 development mode..."

echo "🔁 Watching for changes (requires cargo-watch)..."

if ! command -v cargo-watch &> /dev/null
then
    echo "⚠️ cargo-watch not found. Installing..."
    cargo install cargo-watch
fi

cargo watch -x "build --workspace"
