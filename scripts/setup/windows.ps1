# 管理者権限の確認
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Warning "Please run this script as Administrator"
    Break
}

Write-Host "Setting up Windows environment..." -ForegroundColor Green

# Chocolateyのインストール（未インストールの場合）
if (!(Get-Command choco -ErrorAction SilentlyContinue)) {
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
}

# 必要なパッケージのインストール
choco install -y visualstudio2019buildtools
choco install -y visualstudio2019-workload-vctools
choco install -y openssl
choco install -y pkgconfiglite

# sccacheのインストール
cargo install sccache

# 環境変数の設定
$env:RUSTC_WRAPPER = "sccache"
[System.Environment]::SetEnvironmentVariable("RUSTC_WRAPPER", "sccache", [System.EnvironmentVariableTarget]::User)

Write-Host "Windows setup complete!" -ForegroundColor Green
Write-Host "Please restart your terminal for the changes to take effect." -ForegroundColor Yellow