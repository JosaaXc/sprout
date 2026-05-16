# Sprout installer for Windows (PowerShell 5+).
#
# Usage:
#   irm https://raw.githubusercontent.com/JosaaXc/sprout/main/install.ps1 | iex
#
# Override version or install directory by setting env vars before piping:
#   $env:SPROUT_VERSION = "v0.1.0"; $env:SPROUT_INSTALL_DIR = "C:\Tools\sprout"
#   irm https://raw.githubusercontent.com/JosaaXc/sprout/main/install.ps1 | iex

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$Repo       = "JosaaXc/sprout"
$BinName    = "sprout"
$Version    = if ($env:SPROUT_VERSION) { $env:SPROUT_VERSION } else { "latest" }
$InstallDir = if ($env:SPROUT_INSTALL_DIR) { $env:SPROUT_INSTALL_DIR } else { "$env:LOCALAPPDATA\Programs\sprout" }

# --- detect arch ---
if (-not [Environment]::Is64BitOperatingSystem) {
    Write-Error "32-bit Windows is not supported. See https://github.com/$Repo/releases for manual install."
    exit 1
}

$arch = if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "aarch64" } else { "x86_64" }
$target = "$arch-pc-windows-msvc"
$archive = "$BinName-$target.zip"

# --- resolve version ---
if ($Version -eq "latest") {
    Write-Host "-> Resolving latest version..."
    try {
        $release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest" -UseBasicParsing
        $Version = $release.tag_name
    } catch {
        Write-Error "Could not resolve the latest release. Check https://github.com/$Repo/releases or set `$env:SPROUT_VERSION."
        exit 1
    }
}

$url = "https://github.com/$Repo/releases/download/$Version/$archive"

Write-Host "-> Installing sprout $Version ($target)"
Write-Host "   from $url"

$tmp = Join-Path $env:TEMP "sprout-install-$([Guid]::NewGuid().ToString('N'))"
New-Item -Type Directory -Path $tmp -Force | Out-Null

try {
    $archivePath = Join-Path $tmp $archive
    Invoke-WebRequest -Uri $url -OutFile $archivePath -UseBasicParsing
    Expand-Archive -Path $archivePath -DestinationPath $tmp -Force

    New-Item -Type Directory -Path $InstallDir -Force | Out-Null
    $exeSrc = Join-Path $tmp "$BinName.exe"
    $exeDst = Join-Path $InstallDir "$BinName.exe"
    Move-Item -Path $exeSrc -Destination $exeDst -Force

    Write-Host ""
    Write-Host "[OK] sprout installed to $exeDst" -ForegroundColor Green

    # --- PATH ---
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        Write-Host ""
        Write-Host "-> Adding $InstallDir to your User PATH..."
        $newPath = if ([string]::IsNullOrEmpty($userPath)) { $InstallDir } else { "$InstallDir;$userPath" }
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        Write-Host "   Restart your terminal (or open a new one) for the PATH change to take effect." -ForegroundColor Yellow
    }

    Write-Host ""
    & $exeDst --version 2>$null

    function Write-Banner {
        $cyan = [ConsoleColor]::Cyan
        Write-Host ""
        Write-Host "╭───────────────────────────────────────────────────────────────╮" -ForegroundColor $cyan
        Write-Host "│                                                               │" -ForegroundColor $cyan
        Write-Host "│    🌱  Welcome to Sprout!                                     │" -ForegroundColor $cyan
        Write-Host "│                                                               │" -ForegroundColor $cyan
        Write-Host "│    The missing scaffolding CLI for Spring Boot.               │" -ForegroundColor $cyan
        Write-Host "│                                                               │" -ForegroundColor $cyan
        Write-Host "│    🚀  Quick start:                                           │" -ForegroundColor $cyan
        Write-Host "│         sprout g                                              │" -ForegroundColor $cyan
        Write-Host "│                                                               │" -ForegroundColor $cyan
        Write-Host "│    📖  Docs:    https://github.com/JosaaXc/sprout             │" -ForegroundColor $cyan
        Write-Host "│    ⭐  Star us: https://github.com/JosaaXc/sprout/stargazers  │" -ForegroundColor $cyan
        Write-Host "│    🐛  Issues:  https://github.com/JosaaXc/sprout/issues      │" -ForegroundColor $cyan
        Write-Host "│                                                               │" -ForegroundColor $cyan
        Write-Host "╰───────────────────────────────────────────────────────────────╯" -ForegroundColor $cyan
        Write-Host ""
    }
    Write-Banner
} finally {
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}
