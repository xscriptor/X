#!/bin/bash
# navigation alias

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
echo "navigation aliases"
add_alias 'alias ..="cd .."'
add_alias 'alias ...="cd ../.."'
add_alias 'alias ....="cd ../../.."'
add_alias 'alias ~="cd ~"'
add_alias 'alias c="clear"'
add_alias 'alias ll="ls -lh"'
add_alias 'alias la="ls -A"'
add_alias 'alias l="ls -CF"'


# Reload only if Bash; otherwise, notify user
if [[ $RC_FILE == "$HOME/.bashrc" ]]; then
  echo "Reloading $RC_FILE..."
  source "$RC_FILE"
  echo "Aliases updated for Bash."
else
  echo "Aliases added for Zsh."
  echo "To apply the changes, open a new terminal or run 'zsh'."
fi