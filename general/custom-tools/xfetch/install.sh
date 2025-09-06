#!/bin/bash

# XFetch Installation Script for Linux
# This script will install Rust (if not present) and build XFetch

echo -e "\033[36mXFetch Installation Script\033[0m"
echo -e "\033[36m============================\033[0m"
echo ""

# Check if Rust is installed
echo -e "\033[33mChecking for Rust installation...\033[0m"
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "\033[32mRust is already installed: $RUST_VERSION\033[0m"
else
    echo -e "\033[31mRust is not installed. Installing Rust...\033[0m"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        echo -e "\033[32mRust installed successfully: $RUST_VERSION\033[0m"
    else
        echo -e "\033[31mFailed to install Rust. Please install manually from https://rustup.rs/\033[0m"
        exit 1
    fi
fi

# Check if Cargo is available
echo -e "\033[33mChecking for Cargo...\033[0m"
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "\033[32mCargo is available: $CARGO_VERSION\033[0m"
else
    echo -e "\033[31mCargo is not available. Please reinstall Rust.\033[0m"
    exit 1
fi

echo ""
echo -e "\033[33mBuilding XFetch...\033[0m"

# Build the project
if cargo build --release; then
    echo -e "\033[32mBuild successful!\033[0m"
else
    echo -e "\033[31mBuild failed!\033[0m"
    exit 1
fi

echo ""
echo -e "\033[33mTesting XFetch...\033[0m"

# Test the built executable
if ./target/release/xfetch; then
    echo ""
    echo -e "\033[32mXFetch is working correctly!\033[0m"
else
    echo -e "\033[31mError running XFetch\033[0m"
fi

echo ""
echo -e "\033[36mInstallation complete!\033[0m"
echo -e "\033[37mYou can find the executable at: ./target/release/xfetch\033[0m"
echo -e "\033[37mTo install globally, run: cargo install --path .\033[0m"
echo -e "\033[37mOr copy the binary to /usr/local/bin: sudo cp ./target/release/xfetch /usr/local/bin/\033[0m"
echo ""