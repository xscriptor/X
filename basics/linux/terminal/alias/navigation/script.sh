#!/bin/bash
# Add common navigation aliases to Bash or Zsh

# Detect current shell
current_shell=$(basename "$0")

if [[ $current_shell == *zsh* ]]; then
  RC_FILE="$HOME/.zshrc"
elif [[ $current_shell == *bash* ]]; then
  RC_FILE="$HOME/.bashrc"
else
  echo "Unsupported shell: $current_shell"
  echo "This script only supports Bash or Zsh."
  exit 1
fi


# Aliases block
ALIASES_BLOCK=$(cat <<'EOF'
# ---- Custom Navigation Aliases ----
alias ..="cd .."
alias ...="cd ../.."
alias ....="cd ../../.."
alias ~="cd ~"
alias c="clear"
alias ll="ls -lh"
alias la="ls -A"
alias l="ls -CF"
# ---- End of Custom Aliases ----
EOF
)

# Add block only if not already present
if ! grep -q "Custom Navigation Aliases" "$RC_FILE"; then
  echo "" >> "$RC_FILE"
  echo "$ALIASES_BLOCK" >> "$RC_FILE"
  echo "Navigation aliases added to $RC_FILE"
else
  echo "Navigation aliases already present in $RC_FILE"
fi

# Reload shell config
echo "Reloading $RC_FILE..."
source "$RC_FILE" && echo "Aliases applied successfully."
