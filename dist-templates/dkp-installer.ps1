#Requires -Version 5.1
# dkp installer — downloads the appropriate pre-built binary from GitHub Releases
[CmdletBinding()]
param(
    [string]$Version = ""
)

$ErrorActionPreference = "Stop"

$Repo    = "dkp-standard/cli"
$BinName = "dkp.exe"
$InstallDir = Join-Path $env:LOCALAPPDATA "dkp"

# Resolve version
if (-not $Version) {
    $releaseJson = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
    $Version = $releaseJson.tag_name
}

$Artifact = "dkp-$Version-x86_64-pc-windows-msvc"
$Url      = "https://github.com/$Repo/releases/download/$Version/$Artifact.zip"
$TmpZip   = Join-Path $env:TEMP "dkp-$Version.zip"
$TmpDir   = Join-Path $env:TEMP "dkp-$Version"

Write-Host "Downloading dkp $Version for Windows x86_64..."
Invoke-WebRequest -Uri $Url -OutFile $TmpZip -UseBasicParsing

Write-Host "Extracting..."
Expand-Archive -LiteralPath $TmpZip -DestinationPath $TmpDir -Force

Write-Host "Installing to $InstallDir..."
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}
Copy-Item (Join-Path $TmpDir $Artifact $BinName) (Join-Path $InstallDir $BinName) -Force
Remove-Item $TmpZip, $TmpDir -Recurse -Force

# Add to user PATH if not already present
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    Write-Host "Added $InstallDir to your user PATH. Restart your terminal to use 'dkp'."
}

Write-Host "Installed dkp $Version to $InstallDir\$BinName"
