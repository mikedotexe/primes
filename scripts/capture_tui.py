#!/usr/bin/env python3
"""
Capture TUI output by simulating keypresses
"""
import subprocess
import time
import sys
from pathlib import Path

# Create a script that runs the TUI with automated input
script_content = """#!/usr/bin/expect -f
set timeout 10
spawn cargo run --example membrane_lab_tui_enhanced
expect "Interactive Membrane Laboratory"
sleep 1

# Capture initial screen
send "\\t"
sleep 0.5
send "\\t"
sleep 0.5

# Go to construction tab
send "\\t"
sleep 0.5

# Generate a prime
send "\\r"
sleep 1

# Go to heat map
send "\\t"
send "\\t"
sleep 0.5

# Go to statistics
send "\\t"
sleep 0.5

# Show help
send "?"
sleep 1

# Exit help and quit
send "h"
sleep 0.5
send "q"
expect eof
"""

# Write expect script
with open("tui_automation.exp", "w") as f:
    f.write(script_content)

# Make it executable
subprocess.run(["chmod", "+x", "tui_automation.exp"])

# Run it and capture output
print("Capturing TUI screens...")
result = subprocess.run(["./tui_automation.exp"], capture_output=True, text=True)

# Save the output
with open("tui_capture.txt", "w") as f:
    f.write("=== TUI VISUAL CAPTURE ===\n\n")
    f.write(result.stdout)

print("Captured output saved to tui_capture.txt")

# Clean up
Path("tui_automation.exp").unlink()