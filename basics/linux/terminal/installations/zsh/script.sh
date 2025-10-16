#!/usr/bin/env bash
set -euo pipefail

echo "────────────────────────────────────────────"
echo "[ZSh Setup] Starting zsh update"
echo "────────────────────────────────────────────"

# fin the shell
if ! command -v zsh &>/dev/null; then
  echo "[+] Zsh no está instalado. Instalando..."
  sudo pacman -Sy --noconfirm zsh
else
  echo "[✓] Zsh Installed."
fi

# see if is zsh
if [ "$SHELL" != "$(which zsh)" ]; then
  echo "[+] Setting Zsh as default..."
  chsh -s "$(which zsh)"
else
  echo "[✓] Zsh as default."
fi

# Install zsh
if [ ! -d "$HOME/.oh-my-zsh" ]; then
  echo "[+] setting Oh My Zsh..."
  export RUNZSH=no
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
else
  echo "[✓] Oh My Zsh installed."
fi

# Install plugins
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
    echo "[+] Installing plugin: $name"
    git clone --depth=1 "${plugins[$name]}" "$dest"
  else
    echo "[✓] Plugin Installed: $name"
  fi
done

ZSHRC="$HOME/.zshrc"

echo "[+] settingup .zshrc..."

# update selector to 'random'
sed -i 's/^ZSH_THEME=.*/ZSH_THEME="random"/' "$ZSHRC" || echo 'ZSH_THEME="random"' >> "$ZSHRC"

# add plugins
if grep -q "^plugins=" "$ZSHRC"; then
  sed -i 's/^plugins=.*/plugins=(git zsh-autosuggestions zsh-syntax-highlighting zsh-completions zsh-autocomplete)/' "$ZSHRC"
else
  echo 'plugins=(git zsh-autosuggestions zsh-syntax-highlighting zsh-completions zsh-autocomplete)' >> "$ZSHRC"
fi

echo "[✓] Installation finished."
echo "────────────────────────────────────────────"
echo "[ℹ] reload terminal or execute 'exec zsh' to apply."
echo "────────────────────────────────────────────"
