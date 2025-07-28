#!/bin/bash
# githubaliasadd

# Detect shell
if [[ $SHELL == */zsh ]]; then
  RC_FILE="$HOME/.zshrc"
elif [[ $SHELL == */bash ]]; then
  RC_FILE="$HOME/.bashrc"
else
  echo "Unsupported shell: $SHELL"
  echo "This script only supports Bash or Zsh."
  exit 1
fi

# Function to add alias if it doesn't already exist
add_alias() {
  local alias_line="$1"
  if ! grep -Fxq "$alias_line" "$RC_FILE"; then
    echo "$alias_line" >> "$RC_FILE"
    echo "Added: $alias_line"
  else
    echo "Already exists: $alias_line"
  fi
}

echo "Adding aliases to $RC_FILE..."

add_alias 'alias gc="git clone"'
add_alias 'alias ga="git add ."'
add_alias 'alias gcom="git commit -m"'
add_alias 'alias gp="git push"'
add_alias 'alias gpuom="git push -u origin main"'
add_alias 'alias gpuod="git push -u origin dev"'

# Reload only if Bash; otherwise, notify user
if [[ $RC_FILE == "$HOME/.bashrc" ]]; then
  echo "Reloading $RC_FILE..."
  source "$RC_FILE"
  echo "Aliases updated for Bash."
else
  echo "Aliases added for Zsh."
  echo "To apply the changes, open a new terminal or run 'zsh'."
fi
