# Windows PowerShell 5.1+; reuse the coverage image without building or running extraction.
[CmdletBinding()]
param([switch]$OpenReport, [string]$Image = 'ffta-rom-coverage:local')
$ErrorActionPreference = 'Stop'
$repoDir = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
if ($repoDir.Contains(',') -or $repoDir.Contains('"') -or $repoDir.Contains("`n") -or $repoDir.Contains("`r")) {
    throw 'Docker mount paths cannot contain commas, double quotes or newlines.'
}
if (-not (Get-Command docker -CommandType Application -ErrorAction SilentlyContinue)) {
    throw 'Start Docker Desktop with Linux containers. Alternative: python3 scripts/report_home.py'
}
& docker image inspect $Image > $null
if ($LASTEXITCODE -ne 0) {
    throw "Cannot access coverage image '$Image'. Check Docker Desktop/image, or run python3 scripts/report_home.py. No build or extraction was started."
}
$temporary = Join-Path $repoDir ('.report-home-' + [Guid]::NewGuid().ToString('N') + '.tmp')
$previousEncoding = [Console]::OutputEncoding
try {
    [Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
    $arguments = @('run', '--rm', '--network', 'none', '--entrypoint', 'python3',
        '--mount', "type=bind,source=$repoDir,target=/project,readonly", $Image,
        '/project/scripts/report_home.py', '--root', '/project', '--stdout')
    $page = & docker @arguments
    if ($LASTEXITCODE -ne 0) { throw "Homepage generation failed (exit $LASTEXITCODE); previous page preserved." }
    [IO.File]::WriteAllText($temporary, (($page -join "`n") + "`n"), [Text.UTF8Encoding]::new($false))
    $destination = Join-Path $repoDir 'index.html'
    if (Test-Path -LiteralPath $destination) { [IO.File]::Replace($temporary, $destination, $null) }
    else { [IO.File]::Move($temporary, $destination) }
    Write-Host "Homepage: $destination"
    if ($OpenReport) { Start-Process -FilePath $destination }
} finally {
    [Console]::OutputEncoding = $previousEncoding
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary }
}
