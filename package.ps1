# CM2Editer 发行包生成脚本
# 用法: powershell -File package.ps1
# 输出: dist/CM2Editer_v0.1.1.zip

$version = "0.1.1"
$distDir = "dist\CM2Editer_v$version"
$exe = "target\release\CM2Editer.exe"

if (-not (Test-Path $exe)) {
    Write-Host "错误: 未找到 $exe，请先执行 cargo build --release" -ForegroundColor Red
    exit 1
}

# 清理旧目录
if (Test-Path $distDir) { Remove-Item -Recurse -Force $distDir }

# 拷贝主程序
New-Item -ItemType Directory -Path $distDir -Force | Out-Null
Copy-Item $exe -Destination $distDir\

# 拷贝必需数据目录
Copy-Item "assets\namespaces" -Destination "$distDir\assets\" -Recurse
Copy-Item "assets\coordinates" -Destination "$distDir\assets\" -Recurse

# 拷贝字体（仅 Regular + Bold）
$fontDst = "$distDir\assets\fonts"
New-Item -ItemType Directory -Path $fontDst -Force | Out-Null
Get-ChildItem "assets\fonts" -Recurse -Filter "SourceHanSansSC-*.otf" | ForEach-Object {
    Copy-Item $_.FullName -Destination $fontDst\
}

# 拷贝说明
Copy-Item "README.md" -Destination $distDir\

# 大小统计
$totalSize = (Get-ChildItem -Path $distDir -Recurse | Where-Object { -not $_.PSIsContainer } | Measure-Object -Property Length -Sum).Sum

Write-Host ""
Write-Host "=== 发行包内容 ===" -ForegroundColor Green
Get-ChildItem -Path $distDir -Recurse | Where-Object { -not $_.PSIsContainer } | ForEach-Object {
    $size = "{0,8:N1} KB" -f ($_.Length / 1KB)
    $rel = $_.FullName.Substring((Resolve-Path $distDir).Path.Length + 1)
    Write-Host "  $size  $rel"
}

Write-Host ""
    Write-Host ("总大小: {0:N1} MB" -f ($totalSize / 1MB)) -ForegroundColor Green

# 打包 zip
$zip = "dist\CM2Editer_v$version.zip"
if (Test-Path $zip) { Remove-Item $zip }
$7z = Get-Command "7z" -ErrorAction SilentlyContinue
if ($7z) {
    & 7z a -tzip $zip "$distDir\*" | Out-Null
    $zipSize = "{0:N1} MB" -f ((Get-Item $zip).Length / 1MB)
    Write-Host ""
    Write-Host "打包: $zip ($zipSize)" -ForegroundColor Green
} else {
    # 用系统自带 Compress-Archive
    Compress-Archive -Path "$distDir\*" -DestinationPath $zip -Force
    $zipSize = "{0:N1} MB" -f ((Get-Item $zip).Length / 1MB)
    Write-Host ""
    Write-Host "打包: $zip ($zipSize) [Compress-Archive]" -ForegroundColor Green
}

# 清理临时目录
Remove-Item -Recurse -Force $distDir
Write-Host "临时目录已清理。发行包: $zip" -ForegroundColor Green
