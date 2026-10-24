# Windows PowerShell 5.1+; Docker Desktop must use Linux containers.
[CmdletBinding()]
param(
    [string]$RomPath = 'rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba',
    [string]$DatasetDir = 'target/rom-data-rank-gates/windows-managed-europe/offline dataset',
    [string]$TranslationPath = 'out/1194 - Final Fantasy Tactics Advance (E)(Surplus).dll',
    [string]$OutputDir = 'docs/development/rom-coverage',
    [string]$EvidencePath = 'docs/development/rom-coverage/evidence.json'
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
$romFile = Get-ProjectPath $RomPath
$dataset = Get-ProjectPath $DatasetDir
$translation = Get-ProjectPath $TranslationPath
$evidence = Get-ProjectPath $EvidencePath
$output = Get-ProjectPath $OutputDir
foreach ($file in @($romFile, $translation, $evidence, (Join-Path $dataset 'manifest.toml'))) {
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
& docker @buildArgs
if ($LASTEXITCODE -ne 0) { throw "Docker coverage image build failed (exit $LASTEXITCODE)." }
$runArgs = @(
    'run', '--rm', '--platform', 'linux/amd64', '--network', 'none',
    '--mount', "type=bind,source=$romFile,target=/input/source.gba,readonly",
    '--mount', "type=bind,source=$dataset,target=/input/dataset,readonly",
    '--mount', "type=bind,source=$translation,target=/input/translation.dll,readonly",
    '--mount', "type=bind,source=$evidence,target=/input/evidence.json,readonly",
    '--mount', "type=bind,source=$output,target=/output",
    $image, '--rom', '/input/source.gba', '--dataset', '/input/dataset',
    '--translation', '/input/translation.dll', '--evidence', '/input/evidence.json', '--output', '/output'
)
& docker @runArgs
if ($LASTEXITCODE -ne 0) { throw "Docker ROM coverage generation failed (exit $LASTEXITCODE)." }
Write-Host "ROM coverage on host: $output/index.html"
