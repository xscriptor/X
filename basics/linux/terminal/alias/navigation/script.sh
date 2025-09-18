#!/bin/bash
# Add custom aliases to Bash or Zsh

# Detect current shell config file
if [[ $SHELL == */zsh ]]; then
  RC_FILE="$HOME/.zshrc"
elif [[ $SHELL == */bash ]]; then
  RC_FILE="$HOME/.bashrc"
else
  echo "Unsupported shell: $SHELL"
  echo "This script only supports Bash or Zsh."
  exit 1
fi

# Function to append a block if not already present
add_block() {
  local marker="$1"
  local block="$2"

  if ! grep -q "$marker" "$RC_FILE"; then
    echo "" >> "$RC_FILE"
    echo "$block" >> "$RC_FILE"
    echo "Added block: $marker"
  else
    echo "Block already exists: $marker"
  fi
}

# --- Navigation Aliases ---
NAVIGATION_ALIASES=$(cat <<'EOF'
# ===== Custom Navigation Aliases =====
alias ..="cd .."
alias ...="cd ../.."
alias ....="cd ../../.."
alias ~="cd ~"
alias c="clear"
alias ll="ls -lh"
alias la="ls -A"
alias l="ls -CF"
# ===== End Navigation Aliases =====
EOF
)

# Add blocks
add_block "Custom Navigation Aliases" "$NAVIGATION_ALIASES"
add_block "Custom Git Aliases" "$GIT_ALIASES"
add_block "Custom Package Aliases" "$PKG_ALIASES"

# Reload config
echo "Reloading $RC_FILE..."
# shellcheck source=/dev/null
source "$RC_FILE" && echo "Aliases applied successfully."
