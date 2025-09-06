#!/bin/bash

# Xscriptor Theme Installer for Konsole (KDE)
# This script installs the Xscriptor theme for Konsole

set -e

echo "Installing Xscriptor theme for Konsole..."

# Check if Konsole is installed
if ! command -v konsole &> /dev/null; then
    echo "Error: Konsole is not installed on this system."
    echo "Please install Konsole first: sudo apt install konsole"
    exit 1
fi

# Create Konsole colorschemes directory if it doesn't exist
COLORSCHEMES_DIR="$HOME/.local/share/konsole"
mkdir -p "$COLORSCHEMES_DIR"

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Copy the theme file
THEME_FILE="$SCRIPT_DIR/Xscriptor.colorscheme"
if [ ! -f "$THEME_FILE" ]; then
    echo "Error: Theme file not found at $THEME_FILE"
    exit 1
fi

cp "$THEME_FILE" "$COLORSCHEMES_DIR/"
echo "Theme file copied to $COLORSCHEMES_DIR/"

# Set appropriate permissions
chmod 644 "$COLORSCHEMES_DIR/Xscriptor.colorscheme"

# Create a default profile that uses the Xscriptor theme
PROFILE_FILE="$COLORSCHEMES_DIR/Xscriptor.profile"
cat > "$PROFILE_FILE" << 'EOF'
[Appearance]
ColorScheme=Xscriptor
Font=Consolas,11,-1,5,50,0,0,0,0,0

[General]
Name=Xscriptor
Parent=FALLBACK/

[Scrolling]
HistoryMode=1
HistorySize=10000

[Terminal Features]
BlinkingCursorEnabled=true
EOF

chmod 644 "$PROFILE_FILE"

echo "Xscriptor theme and profile installed successfully!"
echo ""
echo "To apply the theme:"
echo "1. Open Konsole"
echo "2. Go to Settings > Edit Current Profile"
echo "3. Go to the Appearance tab"
echo "4. In the Color Scheme dropdown, select 'Xscriptor'"
echo "5. Click OK"
echo ""
echo "Alternatively, you can create a new profile:"
echo "1. Go to Settings > Manage Profiles"
echo "2. Click 'New Profile'"
echo "3. Select 'Xscriptor' as the base profile"
echo "4. Set it as default if desired"
echo ""
echo "The theme will be applied immediately to the current session."