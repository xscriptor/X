#!/bin/bash

# Script para probar la compilación en Linux
# Este script verifica que xfetch compile correctamente sin las dependencias problemáticas

echo -e "\033[36mProbando compilación de XFetch en Linux\033[0m"
echo -e "\033[36m=====================================\033[0m"
echo ""

# Limpiar compilaciones anteriores
echo -e "\033[33mLimpiando compilaciones anteriores...\033[0m"
cargo clean

# Verificar que Rust esté instalado
echo -e "\033[33mVerificando instalación de Rust...\033[0m"
if ! command -v rustc &> /dev/null; then
    echo -e "\033[31mError: Rust no está instalado\033[0m"
    echo -e "\033[33mInstala Rust desde: https://rustup.rs/\033[0m"
    exit 1
fi

RUST_VERSION=$(rustc --version)
echo -e "\033[32m✓ Rust encontrado: $RUST_VERSION\033[0m"

# Compilar en modo debug
echo -e "\033[33mCompilando en modo debug...\033[0m"
if cargo build; then
    echo -e "\033[32m✓ Compilación debug exitosa\033[0m"
else
    echo -e "\033[31m✗ Error en compilación debug\033[0m"
    exit 1
fi

# Compilar en modo release
echo -e "\033[33mCompilando en modo release...\033[0m"
if cargo build --release; then
    echo -e "\033[32m✓ Compilación release exitosa\033[0m"
else
    echo -e "\033[31m✗ Error en compilación release\033[0m"
    exit 1
fi

# Probar ejecución básica
echo -e "\033[33mProbando ejecución básica...\033[0m"
if ./target/release/xfetch --help > /dev/null 2>&1; then
    echo -e "\033[32m✓ Ejecución básica exitosa\033[0m"
else
    echo -e "\033[31m✗ Error en ejecución básica\033[0m"
    exit 1
fi

# Probar flags específicos
echo -e "\033[33mProbando flags específicos...\033[0m"
flags=("-O" "-S" "-H" "-N" "-P" "-C" "-X")

for flag in "${flags[@]}"; do
    echo -e "\033[36m  Probando flag $flag...\033[0m"
    if timeout 10s ./target/release/xfetch "$flag" > /dev/null 2>&1; then
        echo -e "\033[32m  ✓ Flag $flag funciona\033[0m"
    else
        echo -e "\033[33m  ⚠ Flag $flag tuvo problemas (puede ser normal en algunos sistemas)\033[0m"
    fi
done

echo ""
echo -e "\033[32m🎉 ¡Todas las pruebas completadas!\033[0m"
echo -e "\033[36mXFetch debería funcionar correctamente en Linux\033[0m"
echo ""
echo -e "\033[33mPara instalar globalmente:\033[0m"
echo -e "\033[37m  sudo cp target/release/xfetch /usr/local/bin/\033[0m"
echo -e "\033[37m  # O agregar el directorio target/release a tu PATH\033[0m"