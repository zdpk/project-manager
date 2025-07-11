# PM Installation Script for Windows PowerShell
# Usage: iwr -useb https://raw.githubusercontent.com/zdpk/project-manager/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$REPO = "zdpk/project-manager"
$BINARY_NAME = "pm"

Write-Host "Installing PM for Windows..." -ForegroundColor Green

# Note: Currently only macOS Apple Silicon is supported
Write-Host "Error: Windows is not currently supported." -ForegroundColor Red
Write-Host "PM currently only supports macOS Apple Silicon (M1/M2)." -ForegroundColor Yellow
Write-Host "Please use WSL with Linux or wait for Windows support." -ForegroundColor Yellow
exit 1