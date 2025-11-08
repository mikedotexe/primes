#!/bin/bash
# Take snapshots of the TUI by sending it to a file

echo "🎬 Recording TUI snapshot..."

# Create a simple input script
cat > tui_inputs.txt << 'EOF'
g

q
EOF

# Run with input piped, output to file
echo "Running TUI with automated inputs..."
cargo run --example membrane_lab_tui_enhanced < tui_inputs.txt > tui_snapshot.txt 2>&1 &
TUI_PID=$!

# Give it time to start and generate
sleep 3

# Send interrupt to capture the screen state
kill -INT $TUI_PID 2>/dev/null

# Also try to capture using script command for better terminal capture
echo "Attempting terminal recording..."
script -q tui_session.log bash -c "timeout 2 cargo run --example membrane_lab_tui_enhanced" || true

echo "✅ Snapshot complete!"
echo "Check: tui_snapshot.txt and tui_session.log"

# Clean up
rm -f tui_inputs.txt