#!/bin/bash
# githubaliasadd

# Detect current shell
current_shell=$(ps -p $$ -o comm=)

if [[ $current_shell == "zsh" ]]; then
  RC_FILE="$HOME/.zshrc"
elif [[ $current_shell == "bash" ]]; then
  RC_FILE="$HOME/.bashrc"
else
  echo "Unsupported shell: $current_shell"
  echo "This script only supports Bash or Zsh."
  exit 1
fi

# Aliases block
ALIASES_BLOCK=$(cat <<'EOF'
# ---- Custom Git Aliases ----
alias gc="git clone"
alias ga="git add ."
alias gcom="git commit -m"
alias gp="git push"
alias gpuom="git push -u origin main"
alias gpuod="git push -u origin dev"

# Habitual job
alias gs="git status"
alias gl="git log --oneline --graph --decorate"
alias gco="git checkout"
alias gcb="git checkout -b"
alias gd="git diff"

# Pull and fetch
alias gpl="git pull"
alias gf="git fetch"
# ---- End of Custom Git Aliases ----
EOF
)

# Add block only if not already present
if ! grep -q "Custom Git Aliases" "$RC_FILE"; then
  echo "" >> "$RC_FILE"
  echo "$ALIASES_BLOCK" >> "$RC_FILE"
  echo "Git aliases added to $RC_FILE"
else
  echo "Git aliases already present in $RC_FILE"
fi

# Reload shell config
echo "Reloading $RC_FILE..."
source "$RC_FILE" && echo "Aliases applied successfully."
