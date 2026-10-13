$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot

Write-Host '==> [1/4] Building Windows workspace...' -ForegroundColor Cyan
$oldErrorAction = $ErrorActionPreference
$ErrorActionPreference = 'Continue'
& cargo build --release
$cargoExit = $LASTEXITCODE
$ErrorActionPreference = $oldErrorAction
if ($cargoExit -ne 0) { throw 'Windows workspace build failed' }

Write-Host '==> [2/4] Copying Windows installer and game...' -ForegroundColor Cyan
New-Item -ItemType Directory -Force (Join-Path $PSScriptRoot 'Installer') | Out-Null
New-Item -ItemType Directory -Force (Join-Path $PSScriptRoot 'Installer\payload') | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'target\release\installer.exe') -Destination (Join-Path $PSScriptRoot 'Installer\Installer.exe') -Force
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'target\release\recomp.exe') -Destination (Join-Path $PSScriptRoot 'Installer\payload\recomp.exe') -Force
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'target\release\ffta-play.exe') -Destination (Join-Path $PSScriptRoot 'Installer\payload\FFTARecomp.exe') -Force
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'gamedb.sqlite') -Destination (Join-Path $PSScriptRoot 'Installer\payload\gamedb.sqlite') -Force
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'Licenses') -Destination (Join-Path $PSScriptRoot 'Installer') -Recurse -Force
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'Licenses') -Destination (Join-Path $PSScriptRoot 'Installer\payload') -Recurse -Force

Write-Host '==> [3/4] Checking Windows toolchain...' -ForegroundColor Cyan
if (-not (Test-Path -LiteralPath (Join-Path $PSScriptRoot 'Installer\payload\toolchain\bin\gcc.exe'))) {
    & (Join-Path $PSScriptRoot 'Installer\scripts\fetch-toolchain.ps1')
} else {
    Write-Host 'Toolchain already exists, skipping.' -ForegroundColor Yellow
}

Write-Host '==> [4/4] Building Linux x86_64 installer...' -ForegroundColor Cyan
& (Join-Path $PSScriptRoot 'scripts\build-linux-windows.ps1') -OutputDir (Join-Path $PSScriptRoot 'Installer\Linux')
if ($LASTEXITCODE -ne 0) { throw 'Linux installer build failed' }
Write-Host "Done: $PSScriptRoot\Installer and $PSScriptRoot\Installer\Linux" -ForegroundColor Green
