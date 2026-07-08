# CM2Editer release packaging script
# Usage: powershell -File package.ps1
# Output: dist/CM2Editer_v0.1.1.zip

$ErrorActionPreference = "Stop"
$version = "0.1.1"
$root = $PSScriptRoot
$distDir = Join-Path $root "dist\CM2Editer_v$version"
$exe = Join-Path $root "target\release\CM2Editer.exe"

if (-not (Test-Path $exe)) {
    Write-Host "ERROR: $exe not found. Run cargo build --release first." -ForegroundColor Red
    exit 1
}

# Clean + create structure
if (Test-Path $distDir) { Remove-Item -Recurse -Force $distDir }
$null = New-Item -ItemType Directory -Path (Join-Path $distDir "assets") -Force

# Copy main binary
Copy-Item $exe -Destination $distDir

# Copy data directories
Copy-Item (Join-Path $root "assets\namespaces") -Destination (Join-Path $distDir "assets\namespaces") -Recurse
Copy-Item (Join-Path $root "assets\coordinates") -Destination (Join-Path $distDir "assets\coordinates") -Recurse

# Copy fonts (Regular + Bold only)
$fontDst = Join-Path $distDir "assets\fonts"
$null = New-Item -ItemType Directory -Path $fontDst -Force
$fontFiles = Get-ChildItem (Join-Path $root "assets\fonts") -Recurse -Filter "SourceHanSansSC-*.otf"
foreach ($f in $fontFiles) {
    Copy-Item $f.FullName -Destination $fontDst
}

# Copy readme
Copy-Item (Join-Path $root "README.md") -Destination $distDir

# Print listing
Write-Host ""
Write-Host "=== Package contents ===" -ForegroundColor Green
$total = 0
Get-ChildItem -Path $distDir -Recurse -File | ForEach-Object {
    $kb = $_.Length / 1KB
    $total += $_.Length
    $rel = $_.FullName.Replace($distDir + "\", "")
    Write-Host ("  {0,8:N1} KB  {1}" -f $kb, $rel)
}

Write-Host ""
$totalMsg = ("Total: {0:N1} MB" -f ($total / 1MB))
Write-Host $totalMsg -ForegroundColor Green

# Create zip
$zip = Join-Path $root "dist\CM2Editer_v$version.zip"
if (Test-Path $zip) { Remove-Item $zip }
$items = Get-ChildItem $distDir | ForEach-Object { $_.FullName }
Compress-Archive -Path $items -DestinationPath $zip -Force

$zipSize = (Get-Item $zip).Length / 1MB
$msg = "Package: $zip ({0:N1} MB)" -f $zipSize
Write-Host $msg -ForegroundColor Green

# Clean up temp
Remove-Item -Recurse -Force $distDir
Write-Host "Done." -ForegroundColor Green
