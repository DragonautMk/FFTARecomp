# Windows PowerShell 5.1+; Docker Desktop must use Linux containers.
[CmdletBinding()]
param(
    [string]$RomPath = 'rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba',
    [string]$DatasetDir,
    [string]$TranslationPath,
    [string]$OutputDir = 'docs/development/rom-coverage',
    [string]$EvidencePath = 'docs/development/rom-coverage/evidence.json',
    [switch]$OpenReport
)
$ErrorActionPreference = 'Stop'
$repoDir = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
function Get-ProjectPath([string]$Value) {
    if ([string]::IsNullOrWhiteSpace($Value)) { throw 'Paths must not be empty.' }
    if (-not [System.IO.Path]::IsPathRooted($Value)) { $Value = Join-Path $repoDir $Value }
    $Value = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Value)
    if ($Value.Contains(',') -or $Value.Contains('"') -or $Value.Contains("`n") -or $Value.Contains("`r")) {
        throw 'Paths must not contain commas, double quotes or newlines.'
    }
    return $Value
}
$logDir = Join-Path $repoDir 'target/build-logs'
[IO.Directory]::CreateDirectory($logDir) | Out-Null
$logFile = Join-Path $logDir ('rom-coverage-{0}-{1}.log' -f (Get-Date -Format 'yyyyMMdd-HHmmss'), ([Guid]::NewGuid().ToString('N').Substring(0,8)))
Write-Host "ROM coverage log: $logFile"
Start-Transcript -LiteralPath $logFile | Out-Null
try {
$romFile = Get-ProjectPath $RomPath
$dataset = if ($DatasetDir) { Get-ProjectPath $DatasetDir } else { $null }
$translation = if ($TranslationPath) { Get-ProjectPath $TranslationPath } else { $null }
$evidence = Get-ProjectPath $EvidencePath
$output = Get-ProjectPath $OutputDir
$inputs = @($romFile, $evidence)
if ($dataset) { $inputs += Join-Path $dataset 'manifest.toml' }
if ($translation) { $inputs += $translation }
foreach ($file in $inputs) {
    if (-not (Test-Path -LiteralPath $file -PathType Leaf)) { throw "Required input file does not exist: $file" }
}
if (-not (Get-Command docker -CommandType Application -ErrorAction SilentlyContinue)) {
    throw 'Docker Desktop with Buildx is required. Start Docker Desktop using Linux containers.'
}
[System.IO.Directory]::CreateDirectory($output) | Out-Null
$image = 'ffta-rom-coverage:local'
$buildArgs = @(
    'buildx', 'build', '--platform', 'linux/amd64',
    '--file', (Join-Path $repoDir 'Dockerfile.linux'), '--target', 'coverage',
    '--load', '--tag', $image, '--progress', 'plain', $repoDir
)
Write-Host "Building ROM coverage tools inside Docker..."
& docker @buildArgs
if ($LASTEXITCODE -ne 0) { throw "Docker coverage image build failed (exit $LASTEXITCODE)." }
$runArgs = @(
    'run', '--rm', '--platform', 'linux/amd64', '--network', 'none',
    '--mount', "type=bind,source=$romFile,target=/input/source.gba,readonly",
    '--mount', "type=bind,source=$evidence,target=/input/evidence.json,readonly",
    '--mount', "type=bind,source=$output,target=/output"
)
$generatorArgs = @('--rom', '/input/source.gba', '--evidence', '/input/evidence.json', '--output', '/output')
if ($dataset) {
    $runArgs += @('--mount', "type=bind,source=$dataset,target=/input/dataset,readonly")
    $generatorArgs += @('--dataset', '/input/dataset')
} else {
    $cache = Get-ProjectPath 'target/rom-data/coverage'
    [IO.Directory]::CreateDirectory($cache) | Out-Null
    $runArgs += @('--mount', "type=bind,source=$cache,target=/cache")
    $generatorArgs += @('--dataset-cache', '/cache')
}
if ($translation) {
    $runArgs += @('--mount', "type=bind,source=$translation,target=/input/translation.dll,readonly")
    $generatorArgs += @('--translation', '/input/translation.dll')
}
$runArgs += @($image) + $generatorArgs
& docker @runArgs
if ($LASTEXITCODE -ne 0) { throw "Docker ROM coverage generation failed (exit $LASTEXITCODE)." }
foreach ($name in @('index.html', 'coverage.json')) {
    $file = Join-Path $output $name
    if (-not (Test-Path -LiteralPath $file -PathType Leaf) -or (Get-Item -LiteralPath $file).Length -eq 0) {
        throw "No report produced: $file"
    }
}
Write-Host "ROM coverage on host: $output/index.html"
Write-Host "JSON data on host: $output/coverage.json"
if ($OpenReport) { Start-Process -FilePath (Join-Path $output 'index.html') }
} catch {
    Write-Host "ROM coverage failed: $_"
    throw
} finally {
    Stop-Transcript | Out-Null
    Write-Host "ROM coverage log: $logFile"
}
