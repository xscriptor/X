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
    echo "$ALIASES" | sudo tee -a /etc/bash.bashrc >/dev/null
    echo "[+] Global aliases added to /etc/bash.bashrc"
  fi
fi

echo " Done. Reload your shell:"
echo "   source ~/.bashrc  or  source ~/.zshrc"
