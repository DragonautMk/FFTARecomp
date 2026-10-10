$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

# 1. Build the main project (whole workspace)
Write-Host "==> [1/4] Building main project..." -ForegroundColor Cyan
cargo build --release
if ($LASTEXITCODE) { throw "Main project build failed" }

# 2. Build the installer
Write-Host "==> [2/4] Building installer..." -ForegroundColor Cyan
cargo build --release -p installer
if ($LASTEXITCODE) { throw "Installer build failed" }

# 3. Copy the exe next to the static payload
Write-Host "==> [3/4] Copying installer.exe to Installer\..." -ForegroundColor Cyan
New-Item -ItemType Directory -Force Installer | Out-Null
Copy-Item target\release\installer.exe Installer\ -Force

# 4. Generate the toolchain only if missing
Write-Host "==> [4/4] Checking toolchain..." -ForegroundColor Cyan
if (-not (Test-Path Installer\payload\toolchain\bin\gcc.exe)) {
    & "$PSScriptRoot\Installer\scripts\fetch-toolchain.ps1"
} else {
    Write-Host "Toolchain already exists, skipping." -ForegroundColor Yellow
}

Write-Host "Done: $PSScriptRoot\Installer" -ForegroundColor Green