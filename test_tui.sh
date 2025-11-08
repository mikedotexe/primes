#!/bin/bash
# Test script to verify the TUI runs without crashing

echo "Testing enhanced TUI..."
timeout 2s cargo run --example membrane_lab_tui_enhanced 2>&1 | head -50

if [ $? -eq 124 ]; then
    echo "✓ TUI ran successfully (timed out as expected)"
else
    echo "✗ TUI failed to run"
fi