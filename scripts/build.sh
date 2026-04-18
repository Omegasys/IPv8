#!/usr/bin/env bash

set -e

echo "🔧 Building IPv8 workspace..."

cargo build --workspace

echo "✅ Build complete."
