<#
.SYNOPSIS
    Installs velo from GitHub Releases.
.PARAMETER Version
    Release version without the leading "v" (default: latest).
.PARAMETER InstallDir
    Install location (default: %USERPROFILE%\.velo\bin).
.PARAMETER NoPath
    Do not add the install directory to the user PATH.
#>
param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\.velo\bin",
    [switch]$NoPath
)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
# Windows PowerShell 5.1 defaults to older TLS, which GitHub rejects
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$Repo = "BazilSuhail/Velocity"
$Binary = "velo.exe"
$ArchiveName = "velo"
$Target = "x86_64-pc-windows-msvc"

# Resolve the release tag
if ($Version -eq "latest") {
    $ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"
    Write-Host "Fetching latest release from $ApiUrl ..." -ForegroundColor Cyan
    $Release = Invoke-RestMethod -Uri $ApiUrl -Headers @{ "User-Agent" = "install.ps1" }
    $Tag = $Release.tag_name
    if (-not $Tag) { throw "Could not determine the latest release tag." }
}
else {
    $Tag = "v$Version"
}

$Archive = "$ArchiveName-$Target.zip"
$DownloadUrl = "https://github.com/$Repo/releases/download/$Tag/$Archive"
$TempZip = Join-Path ([IO.Path]::GetTempPath()) ("velo-" + [guid]::NewGuid().ToString() + ".zip")

try {
    Write-Host "Downloading $DownloadUrl ..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
    if (-not (Test-Path -LiteralPath $TempZip -PathType Leaf)) {
        throw "Download failed: $DownloadUrl was not saved."
    }

    if (-not (Test-Path -LiteralPath $InstallDir -PathType Container)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    Write-Host "Extracting to $InstallDir ..." -ForegroundColor Cyan
    Expand-Archive -LiteralPath $TempZip -DestinationPath $InstallDir -Force
    if (-not (Test-Path -LiteralPath (Join-Path $InstallDir $Binary) -PathType Leaf)) {
        throw "Archive did not contain $Binary."
    }
}
finally {
    Remove-Item -LiteralPath $TempZip -Force -ErrorAction SilentlyContinue
}

if (-not $NoPath) {
    $UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    $Entries = @($UserPath -split ';' | Where-Object { $_ -ne '' })
    if ($Entries -notcontains $InstallDir) {
        $NewPath = if ($UserPath) { "$InstallDir;$UserPath" } else { $InstallDir }
        [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
        Write-Host "Added $InstallDir to user PATH." -ForegroundColor Green
    }
    if (($env:PATH -split ';') -notcontains $InstallDir) {
        $env:PATH = "$InstallDir;$env:PATH"
    }
}

Write-Host ""
Write-Host "Velocity $Tag installed successfully!" -ForegroundColor Green
Write-Host "  Binary: $InstallDir\$Binary" -ForegroundColor Cyan
Write-Host ""
Write-Host "Open a new terminal, then run: velo --help" -ForegroundColor Yellow
