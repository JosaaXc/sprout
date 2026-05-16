$ErrorActionPreference = "Stop"

function Step($label) {
    Write-Host ""
    Write-Host "-> $label" -ForegroundColor Cyan
}

Set-Location (Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path))

Step "cargo fmt --check"
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "X formatting issues. Run ``cargo fmt --all`` to auto-fix." -ForegroundColor Red
    exit 1
}

Step "cargo clippy"
cargo clippy --all-targets --all-features --locked -- -D warnings
if ($LASTEXITCODE -ne 0) { exit 1 }

Step "cargo test"
cargo test --all --locked
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host ""
Write-Host "[OK] preflight passed — safe to push" -ForegroundColor Green
