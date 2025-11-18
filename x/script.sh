#!/usr/bin/env bash
set -euo pipefail

# ────────────────────────────────────────────────
# Xscriptor aliases installer
# x   → sudo
# xs  → sudo su          (classic root shell)
# xi  → sudo -i          (login shell, full root env)
# xsh → sudo -s          (root shell, keep user env)
# ────────────────────────────────────────────────

ALIASES="
# ───── Xscriptor Aliases ─────
alias x='sudo'
alias xs='sudo su'
alias xi='sudo -i'
alias xsh='sudo -s'
"

shopt -s expand_aliases
alias x='sudo'
alias xs='sudo su'
alias xi='sudo -i'
alias xsh='sudo -s'

if ! command -v zsh &>/dev/null; then
  if command -v pacman &>/dev/null; then
    x pacman -Sy --noconfirm zsh
  elif command -v apt &>/dev/null; then
    x apt update -y
    x apt install -y zsh
  elif command -v dnf &>/dev/null; then
    x dnf install -y zsh
  fi
fi

if [ "$SHELL" != "$(which zsh)" ] && command -v zsh &>/dev/null; then
  chsh -s "$(which zsh)"
fi

if [ ! -d "$HOME/.oh-my-zsh" ]; then
  export RUNZSH=no
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
fi

ZSH_CUSTOM="${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}"

declare -A plugins=(
  ["zsh-autosuggestions"]="https://github.com/zsh-users/zsh-autosuggestions"
  ["zsh-syntax-highlighting"]="https://github.com/zsh-users/zsh-syntax-highlighting"
  ["zsh-completions"]="https://github.com/zsh-users/zsh-completions"
  ["zsh-autocomplete"]="https://github.com/marlonrichert/zsh-autocomplete"
)

for name in "${!plugins[@]}"; do
  dest="$ZSH_CUSTOM/plugins/$name"
  if [ ! -d "$dest" ]; then
    git clone --depth=1 "${plugins[$name]}" "$dest"
  fi
done

ZSHRC="$HOME/.zshrc"

sed -i 's/^ZSH_THEME=.*/ZSH_THEME="random"/' "$ZSHRC" || echo 'ZSH_THEME="random"' >> "$ZSHRC"

if grep -q "^plugins=" "$ZSHRC"; then
  sed -i 's/^plugins=.*/plugins=(git zsh-autosuggestions zsh-syntax-highlighting zsh-completions zsh-autocomplete)/' "$ZSHRC"
else
  echo 'plugins=(git zsh-autosuggestions zsh-syntax-highlighting zsh-completions zsh-autocomplete)' >> "$ZSHRC"
fi

# Detect shell rc file
RCFILE=""
case "$(basename "$SHELL")" in
  bash) RCFILE="$HOME/.bashrc" ;;
  zsh)  RCFILE="$HOME/.zshrc" ;;
esac

# Add to user shell rc
if [[ -n "$RCFILE" ]]; then
  if ! grep -q "alias x='sudo'" "$RCFILE" 2>/dev/null; then
    echo "$ALIASES" >> "$RCFILE"
    echo "[+] Aliases added to $RCFILE"
  else
    echo "[=] Aliases already exist in $RCFILE"
  fi
else
  echo "[!] Unsupported shell. Add manually."
fi

# Add globally if possible
if [[ -w /etc/bash.bashrc ]]; then
  if ! grep -q "alias x='sudo'" /etc/bash.bashrc 2>/dev/null; then
    echo "$ALIASES" | x tee -a /etc/bash.bashrc >/dev/null
    echo "[+] Global aliases added to /etc/bash.bashrc"
  fi
fi

if ! grep -q "alias x='sudo'" "$HOME/.zshrc" 2>/dev/null; then
  echo "$ALIASES" >> "$HOME/.zshrc"
fi

echo " Done. Reload your shell:"
echo "   source ~/.bashrc  or  source ~/.zshrc"
