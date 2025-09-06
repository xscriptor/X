# XFetch Uninstallation Script for Windows
# This script will remove XFetch from your system

Write-Host "XFetch Uninstallation Script" -ForegroundColor Red
Write-Host "==============================" -ForegroundColor Red
Write-Host ""

# Define installation directory
$installDir = "$env:LOCALAPPDATA\xfetch"

# Check if XFetch is installed
if (!(Test-Path $installDir)) {
    Write-Host "XFetch installation directory not found: $installDir" -ForegroundColor Yellow
    Write-Host "XFetch may not be installed or already removed." -ForegroundColor Yellow
} else {
    Write-Host "Found XFetch installation at: $installDir" -ForegroundColor Yellow
    
    # Confirm uninstallation
    $confirm = Read-Host "Are you sure you want to uninstall XFetch? (y/N)"
    
    if ($confirm -eq "y" -or $confirm -eq "Y" -or $confirm -eq "yes" -or $confirm -eq "Yes") {
        Write-Host ""
        Write-Host "Removing XFetch..." -ForegroundColor Yellow
        
        # Remove installation directory
        try {
            Remove-Item -Path $installDir -Recurse -Force
            Write-Host "Removed installation directory: $installDir" -ForegroundColor Green
        } catch {
            Write-Host "Error removing installation directory: $_" -ForegroundColor Red
        }
        
        # Remove from PATH
        Write-Host "Removing from PATH..." -ForegroundColor Yellow
        try {
            $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
            if ($currentPath -like "*$installDir*") {
                $newPath = $currentPath -replace [regex]::Escape(";$installDir"), ""
                $newPath = $newPath -replace [regex]::Escape("$installDir;"), ""
                $newPath = $newPath -replace [regex]::Escape($installDir), ""
                [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
                Write-Host "Removed $installDir from user PATH" -ForegroundColor Green
                Write-Host "Please restart your terminal for changes to take effect" -ForegroundColor Yellow
            } else {
                Write-Host "Installation directory not found in PATH" -ForegroundColor Yellow
            }
        } catch {
            Write-Host "Error updating PATH: $_" -ForegroundColor Red
            Write-Host "You may need to manually remove $installDir from your PATH" -ForegroundColor Yellow
        }
        
        Write-Host ""
        Write-Host "XFetch has been successfully uninstalled!" -ForegroundColor Green
    } else {
        Write-Host "Uninstallation cancelled." -ForegroundColor Yellow
    }
}

Write-Host ""
Read-Host "Press Enter to exit"