# Run as Administrator
Write-Host "`nStarting Windows Cleanup..." -ForegroundColor Cyan

# List of unwanted pre-installed apps to remove
$appsToRemove = @(
    "*bing*",
    "*xbox*",
    "*zune*",
    "*solitaire*",
    "*skype*",
    "*gethelp*",
    "*people*",
    "*maps*",
    "*feedback*",
    "*candycrush*",
    "*weather*",
    "*onenote*",
    "*3dbuilder*",
    "*mixedreality*",
    "*officehub*",
    "*cortana*",
    "*spotify*"
)

foreach ($app in $appsToRemove) {
    Write-Host "Removing $app..." -ForegroundColor Yellow
    Get-AppxPackage -Name $app -AllUsers | Remove-AppxPackage -ErrorAction SilentlyContinue
    Get-AppxProvisionedPackage -Online | Where-Object DisplayName -like $app | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue
}

# Disable telemetry services
Write-Host "`nDisabling telemetry services..." -ForegroundColor Cyan

$services = @(
    "DiagTrack",        # Diagnostic Tracking Service
    "dmwappushsvc",     # WAP Push Service
    "WMPNetworkSvc"     # Windows Media Player Network Sharing Service
)

foreach ($svc in $services) {
    Write-Host "Disabling service: $svc" -ForegroundColor DarkYellow
    Stop-Service -Name $svc -Force -ErrorAction SilentlyContinue
    Set-Service -Name $svc -StartupType Disabled -ErrorAction SilentlyContinue
}

# Disable telemetry-related scheduled tasks
Write-Host "`nDisabling telemetry-related scheduled tasks..." -ForegroundColor Cyan

$tasks = @(
    "\Microsoft\Windows\Application Experience\ProgramDataUpdater",
    "\Microsoft\Windows\Autochk\Proxy",
    "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator",
    "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip",
    "\Microsoft\Windows\Customer Experience Improvement Program\Uploader",
    "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"
)

foreach ($task in $tasks) {
    Write-Host "Disabling task: $task" -ForegroundColor DarkYellow
    schtasks /Change /TN $task /Disable | Out-Null
}

Write-Host "`nCleanup complete." -ForegroundColor Green
