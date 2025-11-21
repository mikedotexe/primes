#!/bin/bash
# Render Base-15 Elbow Room Animation
# Focused 19-second narrative for the flagship elbow event

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
QUALITY="${1:-h}"  # Default to high quality (h = 1080p60)
OUTPUT_DIR="animations"
SCENE_NAME="Base15ElbowRoom"

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Base-15 Elbow Room Animation: Honorary Zero → Elbow Story  ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check dependencies
echo -e "${YELLOW}[1/3] Checking dependencies...${NC}"

if ! command -v python3 &> /dev/null; then
    echo -e "${RED}✗ Error: python3 not found. Please install Python 3.${NC}"
    exit 1
fi
echo -e "${GREEN}  ✓ Python 3 available${NC}"

if ! python3 -c "import manim" &> /dev/null; then
    echo -e "${YELLOW}  ⚠ Manim not found. Installing from requirements.txt...${NC}"
    pip3 install -r visualizations/requirements.txt
else
    echo -e "${GREEN}  ✓ Manim available${NC}"
fi

echo ""

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

echo -e "${YELLOW}[2/3] Rendering animation...${NC}"
echo -e "${BLUE}  Quality: ${QUALITY_NAME}${NC}"
echo -e "${BLUE}  Scene: ${SCENE_NAME}${NC}"
echo ""

# Render animation
echo -e "${BLUE}  → Running Manim...${NC}"
manim -p $QUALITY_FLAG visualizations/base15_elbow_room.py $SCENE_NAME

# Find the output file
MANIM_OUTPUT=$(find media/videos/base15_elbow_room -name "${SCENE_NAME}.mp4" | head -1)

if [ -z "$MANIM_OUTPUT" ]; then
    echo -e "${RED}✗ Error: Animation file not found${NC}"
    exit 1
fi

# Create output directory and copy with timestamp
mkdir -p "$OUTPUT_DIR"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_FILE="${OUTPUT_DIR}/base15_elbow_room_${TIMESTAMP}.mp4"
cp "$MANIM_OUTPUT" "$OUTPUT_FILE"

echo -e "${GREEN}  ✓ Animation rendered successfully${NC}"
echo -e "${BLUE}    → ${OUTPUT_FILE}${NC}"

echo ""

# Generate summary
echo -e "${YELLOW}[3/3] Summary${NC}"

cat << EOF

═══════════════════════════════════════════════════════════════
                    RENDERING COMPLETE ✓
═══════════════════════════════════════════════════════════════

Animation Details:
  - Event: Base 15, outer=13, inner=1
  - Transition: M=1→M=2, k*=0→k*=1
  - Duration: ~19 seconds
  - Quality: ${QUALITY_NAME}

Output:
  ${OUTPUT_FILE}

Real Primes Shown:
  • 13 0 1 0 8 1 0 1 0 13 (base 15) = 499,935,695,863
  • 13 0 1 0 14 1 0 1 0 13 (base 15) = 499,935,999,613

Narrative Arc:
  0-4s:   Base-15 digit line + Honorary Zero at 7.5
  4-8s:   M=1 membrane template with ghost padding rows
  8-11s:  Flat density bar chart (all k equal at M=1)
  11-15s: Transition to M=2, elbow shifts to k=1
  15-19s: Real prime examples on the M=2, k=1 ridge

Next Steps:
  1. Review animation: open ${OUTPUT_FILE}
  2. For general framework (all events): ./scripts/generate_elbow_animation.sh

═══════════════════════════════════════════════════════════════

EOF
