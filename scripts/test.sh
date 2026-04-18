#!/usr/bin/env bash

set -e

echo "🧪 Running IPv8 full test suite..."

echo "➡ Unit tests"
cargo test --workspace --lib

echo "➡ Integration tests"
cargo test --workspace --test '*'

echo "➡ Documentation tests"
cargo test --doc

echo "✅ All tests completed."
