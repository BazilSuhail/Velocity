param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\.velo\bin"
)

$Repo = "BazilSuhail/Velocity"
$Binary = "velo.exe"
$ArchiveName = "velo"
$Target = "x86_64-pc-windows-msvc"

# Resolve latest version if not specified
if ($Version -eq "latest") {
    $ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"
    Write-Host "Fetching latest release from $ApiUrl ..." -ForegroundColor Cyan
    $Release = Invoke-RestMethod -Uri $ApiUrl -Headers @{ "User-Agent" = "install.ps1" }
    $Tag = $Release.tag_name
}
else {
    $Tag = "v$Version"
}

# Build download URL
$Archive = "$ArchiveName-$Target.zip"
$DownloadUrl = "https://github.com/$Repo/releases/download/$Tag/$Archive"

# Create install directory
if (-not (Test-Path -LiteralPath $InstallDir -PathType Container)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Download and extract
$TempZip = Join-Path $env:TEMP $Archive
Write-Host "Downloading $DownloadUrl ..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip

Write-Host "Extracting ..." -ForegroundColor Cyan
Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
Remove-Item -LiteralPath $TempZip -Force

# Add to PATH if not already there
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    $NewPath = "$InstallDir;$UserPath"
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "Added $InstallDir to user PATH." -ForegroundColor Green
}

Write-Host ""
Write-Host "Velocity installed successfully!" -ForegroundColor Green
Write-Host "  Binary: $InstallDir\$Binary" -ForegroundColor Cyan
Write-Host "  Version: $Tag" -ForegroundColor Cyan
Write-Host ""
Write-Host "Restart your terminal, then run: velo --help" -ForegroundColor Yellow
