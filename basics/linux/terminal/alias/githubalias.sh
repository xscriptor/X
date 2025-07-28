#!/bin/bash
# githubaliasadd
# Detect shell
if [[ $SHELL == */zsh ]]; then
  RC_FILE="$HOME/.zshrc"
elif [[ $SHELL == */bash ]]; then
  RC_FILE="$HOME/.bashrc"
else
  echo "Shell no soportado: $SHELL"
  echo "Este script solo funciona con Bash o Zsh."
  exit 1
fi
# Función para añadir alias si no existe
add_alias() {
  local alias_line="$1"
  if ! grep -Fxq "$alias_line" "$RC_FILE"; then
    echo "$alias_line" >> "$RC_FILE"
    echo "Added: $alias_line"
  else
    echo "Already Exists: $alias_line"
  fi
}

echo "Adding alias to $RC_FILE..."

add_alias 'alias gc="git clone"'
add_alias 'alias ga="git add ."'
add_alias 'alias gcom="git commit -m"'
add_alias 'alias gp="git push"'
add_alias 'alias gpuom="git push -u origin main"'
add_alias 'alias gpuod="git push -u origin dev"'
# Recarga configuración
echo "Recharging $RC_FILE..."
source "$RC_FILE"
echo "Alias updated for $SHELL"
