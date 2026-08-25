$ErrorActionPreference = 'Stop'

Set-Location (Join-Path $PSScriptRoot '..')
$keyPath = Join-Path (Get-Location) 'src-tauri\keys\updater.key'
if (-not (Test-Path -LiteralPath $keyPath)) {
  throw 'Missing updater signing key: src-tauri\keys\updater.key'
}

$env:TAURI_SIGNING_PRIVATE_KEY = [System.IO.File]::ReadAllText($keyPath)
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = ''
$env:CI = 'true'
npm run tauri build -- --ci --bundles nsis --config src-tauri\tauri.release.conf.json
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}

$installer = Get-ChildItem 'src-tauri\target\release\bundle\nsis\*.exe' |
  Sort-Object LastWriteTime -Descending |
  Select-Object -First 1
if (-not $installer) {
  throw 'Windows NSIS installer was not generated'
}

$signature = "$($installer.FullName).sig"
if (-not (Test-Path -LiteralPath $signature)) {
  throw 'Updater signature was not generated'
}

Write-Host "Installer: $($installer.FullName)"
Write-Host "Signature: $signature"
