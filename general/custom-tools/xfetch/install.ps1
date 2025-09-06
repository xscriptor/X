# XFetch Installation Script for Windows
# This script will install Rust (if not present) and build XFetch

Write-Host "XFetch Installation Script" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
Write-Host "Checking for Rust installation..." -ForegroundColor Yellow
try {
    $rustVersion = & rustc --version 2>$null
    Write-Host "Rust is already installed: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "Rust is not installed. Installing Rust..." -ForegroundColor Red
    Write-Host "Please visit https://rustup.rs/ to install Rust manually." -ForegroundColor Yellow
    Write-Host "After installing Rust, run this script again." -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

# Check if Cargo is available
Write-Host "Checking for Cargo..." -ForegroundColor Yellow
try {
    $cargoVersion = & cargo --version 2>$null
    Write-Host "Cargo is available: $cargoVersion" -ForegroundColor Green
} catch {
    Write-Host "Cargo is not available. Please reinstall Rust." -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "Building XFetch..." -ForegroundColor Yellow

# Build the project
try {
    & cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Build successful!" -ForegroundColor Green
    } else {
        Write-Host "Build failed!" -ForegroundColor Red
        Read-Host "Press Enter to exit"
        exit 1
    }
} catch {
    Write-Host "Error during build: $_" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "Testing XFetch..." -ForegroundColor Yellow

# Test the built executable
try {
    & .\target\release\xfetch.exe
    Write-Host ""
    Write-Host "XFetch is working correctly!" -ForegroundColor Green
} catch {
    Write-Host "Error running XFetch: $_" -ForegroundColor Red
}

Write-Host ""
Write-Host "Installing XFetch globally..." -ForegroundColor Yellow

# Create installation directory
$installDir = "$env:LOCALAPPDATA\xfetch"
if (!(Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Write-Host "Created installation directory: $installDir" -ForegroundColor Green
}

# Copy executable to installation directory
try {
    Copy-Item ".\target\release\xfetch.exe" "$installDir\xfetch.exe" -Force
    Write-Host "Copied xfetch.exe to $installDir" -ForegroundColor Green
} catch {
    Write-Host "Error copying executable: $_" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

# Add to PATH if not already present
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$installDir*") {
    try {
        $newPath = "$currentPath;$installDir"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        Write-Host "Added $installDir to user PATH" -ForegroundColor Green
        Write-Host "Please restart your terminal or run: refreshenv" -ForegroundColor Yellow
    } catch {
        Write-Host "Error updating PATH: $_" -ForegroundColor Red
        Write-Host "You can manually add $installDir to your PATH" -ForegroundColor Yellow
    }
} else {
    Write-Host "Installation directory already in PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Cyan
Write-Host "XFetch has been installed to: $installDir" -ForegroundColor White
Write-Host "You can now run 'xfetch' from any terminal (after restart)" -ForegroundColor White
Write-Host ""
Write-Host "To uninstall, delete the directory: $installDir" -ForegroundColor Gray
Write-Host "And remove it from your PATH environment variable" -ForegroundColor Gray
Write-Host ""
Read-Host "Press Enter to exit"