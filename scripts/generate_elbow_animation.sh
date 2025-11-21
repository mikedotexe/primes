#!/bin/bash
# Elbow Room Animation Pipeline
# Complete workflow: Data extraction → Event detection → Manim rendering

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
QUALITY="${1:-h}"  # Default to high quality (h = 1080p60, k = 4K60, l = 480p15)
OUTPUT_DIR="animations"

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     Elbow Room Animation Pipeline: Honorary Zero Dynamics    ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Step 1: Check dependencies
echo -e "${YELLOW}[1/5] Checking dependencies...${NC}"

# Check Rust/Cargo
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Error: cargo not found. Please install Rust.${NC}"
    exit 1
fi
echo -e "${GREEN}  ✓ Rust/Cargo available${NC}"

# Check Python
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}✗ Error: python3 not found. Please install Python 3.${NC}"
    exit 1
fi
echo -e "${GREEN}  ✓ Python 3 available${NC}"

# Check if manim is installed
if ! python3 -c "import manim" &> /dev/null; then
    echo -e "${YELLOW}  ⚠ Manim not found. Installing from requirements.txt...${NC}"
    pip3 install -r visualizations/requirements.txt
else
    echo -e "${GREEN}  ✓ Manim available${NC}"
fi

echo ""

# Step 2: Check for CSV data
echo -e "${YELLOW}[2/5] Checking for CSV data...${NC}"

if [ ! -f "solution_space_complete.csv" ]; then
    echo -e "${RED}✗ Error: solution_space_complete.csv not found.${NC}"
    echo -e "${YELLOW}  Run: cargo run --example solution_space_explorer${NC}"
    exit 1
fi
echo -e "${GREEN}  ✓ solution_space_complete.csv found${NC}"

# Count rows for reference
ROW_COUNT=$(wc -l < solution_space_complete.csv)
echo -e "${BLUE}    (${ROW_COUNT} rows of data)${NC}"

if [ -f "extended_m_results.csv" ]; then
    echo -e "${GREEN}  ✓ extended_m_results.csv found (optional)${NC}"
fi

echo ""

# Step 3: Extract elbow events
echo -e "${YELLOW}[3/5] Extracting elbow events...${NC}"

echo -e "${BLUE}  → Running: cargo run --release --example extract_elbow_events${NC}"
cargo run --release --example extract_elbow_events

if [ ! -f "elbow_events.json" ]; then
    echo -e "${RED}✗ Error: elbow_events.json not generated${NC}"
    exit 1
fi
echo -e "${GREEN}  ✓ elbow_events.json generated${NC}"

# Count events
EVENT_COUNT=$(python3 -c "import json; print(len(json.load(open('elbow_events.json'))['events']))")
echo -e "${BLUE}    (${EVENT_COUNT} elbow events detected)${NC}"

if [ "$EVENT_COUNT" -eq 0 ]; then
    echo -e "${YELLOW}  ⚠ No elbow events detected. Animation will show 'k*=0 universality' message.${NC}"
fi

echo ""

# Step 4: Render animations
echo -e "${YELLOW}[4/5] Rendering animations with Manim...${NC}"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Map quality flag
case "$QUALITY" in
    l|low)
        QUALITY_FLAG="-ql"
        QUALITY_NAME="Low (480p15)"
        ;;
    m|medium)
        QUALITY_FLAG="-qm"
        QUALITY_NAME="Medium (720p30)"
        ;;
    h|high)
        QUALITY_FLAG="-qh"
        QUALITY_NAME="High (1080p60)"
        ;;
    k|4k|production)
        QUALITY_FLAG="-qk"
        QUALITY_NAME="4K (2160p60)"
        ;;
    *)
        echo -e "${RED}✗ Error: Invalid quality option '$QUALITY'${NC}"
        echo -e "${YELLOW}  Usage: $0 [l|m|h|k]${NC}"
        echo -e "${YELLOW}    l = Low (480p15)${NC}"
        echo -e "${YELLOW}    m = Medium (720p30)${NC}"
        echo -e "${YELLOW}    h = High (1080p60) [default]${NC}"
        echo -e "${YELLOW}    k = 4K (2160p60)${NC}"
        exit 1
        ;;
esac

echo -e "${BLUE}  Quality: ${QUALITY_NAME}${NC}"
echo ""

# Render complete animation
echo -e "${BLUE}  → Rendering complete animation...${NC}"
manim -p $QUALITY_FLAG visualizations/manim_elbow_room.py ElbowRoomComplete

# Find the output file (Manim creates nested directories)
MANIM_OUTPUT=$(find media/videos/manim_elbow_room -name "ElbowRoomComplete.mp4" | head -1)

if [ -z "$MANIM_OUTPUT" ]; then
    echo -e "${RED}✗ Error: Animation file not found${NC}"
    exit 1
fi

# Copy to animations directory with timestamp
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_FILE="${OUTPUT_DIR}/elbow_room_${TIMESTAMP}.mp4"
cp "$MANIM_OUTPUT" "$OUTPUT_FILE"

echo -e "${GREEN}  ✓ Animation rendered successfully${NC}"
echo -e "${BLUE}    → ${OUTPUT_FILE}${NC}"

# Optional: Render individual scenes (commented out by default)
# echo ""
# echo -e "${BLUE}  → Rendering individual scenes...${NC}"
# manim $QUALITY_FLAG visualizations/manim_elbow_room.py HonoraryZeroIntro
# manim $QUALITY_FLAG visualizations/manim_elbow_room.py ElbowEventMontage
# manim $QUALITY_FLAG visualizations/manim_elbow_room.py StatisticalContext

echo ""

# Step 5: Generate summary report
echo -e "${YELLOW}[5/5] Generating summary report...${NC}"

REPORT_FILE="${OUTPUT_DIR}/summary_${TIMESTAMP}.txt"

cat > "$REPORT_FILE" << EOF
Elbow Room Animation Summary Report
====================================

Generated: $(date)

Data Sources:
  - solution_space_complete.csv (${ROW_COUNT} rows)

Elbow Events Detected: ${EVENT_COUNT}

Animation Details:
  - Quality: ${QUALITY_NAME}
  - Output: ${OUTPUT_FILE}
  - Duration: ~$((8 + EVENT_COUNT * 3 + 6)) seconds (estimated)

Events:
EOF

# Extract event details from JSON
python3 << 'PYEOF' >> "$REPORT_FILE"
import json

with open('elbow_events.json') as f:
    data = json.load(f)

for i, event in enumerate(data['events'], 1):
    print(f"  {i}. Base {event['base']}, ({event['outer']},{event['inner']}): " +
          f"M={event['m_before']}→{event['m_after']}, " +
          f"k*={event['k_star_before']}→{event['k_star_after']}, " +
          f"Δdensity={event['density_jump']:+.4f}")

if len(data['events']) == 0:
    print("  (No events detected - k*=0 universality is absolute)")

print("\nStatistical Summary:")
print(f"  Total configurations analyzed: {data['metadata']['total_configurations_analyzed']}")
print(f"  Elbow event rate: {len(data['events']) / data['metadata']['total_configurations_analyzed'] * 100:.2f}%")

PYEOF

echo -e "${GREEN}  ✓ Summary report generated${NC}"
echo -e "${BLUE}    → ${REPORT_FILE}${NC}"

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                    PIPELINE COMPLETE ✓                        ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Output Files:${NC}"
echo -e "${BLUE}  • Animation: ${OUTPUT_FILE}${NC}"
echo -e "${BLUE}  • Report:    ${REPORT_FILE}${NC}"
echo -e "${BLUE}  • Data:      elbow_events.json${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo -e "  1. Review animation: open ${OUTPUT_FILE}"
echo -e "  2. Read summary: cat ${REPORT_FILE}"
echo -e "  3. Analyze data: jq . elbow_events.json"
echo ""
echo -e "${BLUE}Storyboard Documentation:${NC}"
echo -e "  See: ELBOW_ROOM_STORYBOARD.md"
echo ""
