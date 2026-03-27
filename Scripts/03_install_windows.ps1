# ==============================================================
# Windows One-Click Environment Setup Script
# Run in Administrator PowerShell:
#   Set-ExecutionPolicy Bypass -Scope Process -Force
#   .\install_windows.ps1
# ==============================================================

$ErrorActionPreference = "Continue"

function Log($msg) { Write-Host "`n[>>] $msg" -ForegroundColor Cyan }
function OK($msg)  { Write-Host "  [OK] $msg" -ForegroundColor Green }
function WARN($msg){ Write-Host "  [!!] $msg" -ForegroundColor Yellow }

# ==============================================================
# 0. Package Managers
# ==============================================================
Log "Checking package managers..."

if (-not (Get-Command scoop -ErrorAction SilentlyContinue)) {
    Log "Installing Scoop..."
    Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
    scoop bucket add extras
    scoop bucket add versions
    OK "Scoop installed"
} else {
    OK "Scoop already exists"
}

if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
    WARN "winget not found. Please install App Installer from Microsoft Store."
} else {
    OK "Winget already exists"
}

# ==============================================================
# 1. CLI Tools via Scoop
# ==============================================================
Log "Installing CLI tools via Scoop..."

$scoopTools = @(
    "git",
    "gh",
    "node",
    "python",
    "go",
    "zig",
    "deno",
    "fnm",
    "cmake",
    "ninja",
    "pandoc",
    "ripgrep",
    "wget",
    "aria2",
    "ffmpeg",
    "imagemagick",
    "exiftool",
    "yt-dlp",
    "gallery-dl",
    "restic",
    "p7zip",
    "fdupes",
    "jdupes",
    "parallel",
    "tree",
    "sqlite",
    "nasm",
    "yasm",
    "topgrade",
    "buku",
    "ollama",
    "tesseract",
    "poppler",
    "lz4",
    "zstd",
    "xz",
    "brotli",
    "transmission-cli"
)

foreach ($tool in $scoopTools) {
    Write-Host "  scoop install $tool" -ForegroundColor Gray
    scoop install $tool 2>&1 | Out-Null
}
OK "Scoop tools done"

# ==============================================================
# 2. GUI Apps via Winget
# ==============================================================
Log "Installing GUI apps via Winget..."

$wingetApps = @(
    @{ id="Bitwarden.CLI";       name="Bitwarden CLI" },
    @{ id="LocalSend.LocalSend"; name="LocalSend" },
    @{ id="GnuPG.Gpg4win";      name="GnuPG" },
    @{ id="Java.OpenJDK";        name="OpenJDK" },
    @{ id="EFF.Certbot";         name="Certbot" }
)

foreach ($app in $wingetApps) {
    Write-Host "  winget install $($app.id)" -ForegroundColor Gray
    winget install --id $app.id --silent --accept-package-agreements --accept-source-agreements 2>&1 | Out-Null
}
OK "GUI apps done"

# ==============================================================
# 3. Rust + Cargo Packages
# ==============================================================
Log "Checking Rust toolchain..."

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Log "Installing Rust..."
    Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
    .\rustup-init.exe -y --quiet
    Remove-Item rustup-init.exe
    $env:PATH += ";$env:USERPROFILE\.cargo\bin"
    OK "Rust installed"
} else {
    OK "Rust already exists"
}

$cargoPackages = @(
    "bkmr",
    "cargo-edit",
    "cargo-expand",
    "cargo-audit",
    "cargo-deny",
    "cargo-hack",
    "cargo-license",
    "cargo-machete",
    "cargo-mutants",
    "cargo-semver-checks",
    "cargo-udeps",
    "cargo-bloat",
    "cargo-about",
    "cargo-upgrades",
    "dupe-krill",
    "fclones",
    "flamegraph"
)

foreach ($pkg in $cargoPackages) {
    Write-Host "  cargo install $pkg" -ForegroundColor Gray
    cargo install $pkg 2>&1 | Out-Null
}
OK "Cargo packages done"

# ==============================================================
# 4. NPM Global Packages
# ==============================================================
Log "Installing NPM global packages..."

$npmPackages = @(
    "@anthropic-ai/claude-code",
    "acp-ts",
    "lodash",
    "openclaw",
    "opencode-ai",
    "run-deepseek-cli",
    "uipro-cli"
)

foreach ($pkg in $npmPackages) {
    Write-Host "  npm install -g $pkg" -ForegroundColor Gray
    npm install -g $pkg 2>&1 | Out-Null
}
OK "NPM packages done"

# ==============================================================
# 5. Python pip Packages
# ==============================================================
Log "Installing pip packages..."

$pipPackages = @(
    "flask",
    "flask-cors",
    "numpy",
    "scipy",
    "scikit-learn",
    "pillow",
    "opencv-python",
    "torch",
    "lightgbm",
    "openvino",
    "tqdm",
    "joblib",
    "sympy",
    "networkx",
    "PyWavelets",
    "certifi",
    "cryptography",
    "filelock",
    "fsspec"
)

foreach ($pkg in $pipPackages) {
    Write-Host "  pip install $pkg" -ForegroundColor Gray
    pip install $pkg --quiet 2>&1 | Out-Null
}
OK "Pip packages done"

# ==============================================================
# 6. uv + UV Tools
# ==============================================================
Log "Checking uv..."

if (-not (Get-Command uv -ErrorAction SilentlyContinue)) {
    Invoke-RestMethod https://astral.sh/uv/install.ps1 | Invoke-Expression
    OK "uv installed"
} else {
    OK "uv already exists"
}

uv tool install kimi-cli 2>&1 | Out-Null
OK "UV tools done"

# ==============================================================
# 7. Summary
# ==============================================================
Write-Host "`n============================================================" -ForegroundColor Magenta
Write-Host " Setup Complete!" -ForegroundColor Green
Write-Host " Skipped (macOS-only):" -ForegroundColor Yellow
Write-Host "   mlx/mlx-c  |  mas  |  osxphotos  |  fuse-t  |  GTK libs" -ForegroundColor Yellow
Write-Host " Please restart your terminal to reload PATH." -ForegroundColor Cyan
Write-Host "============================================================`n" -ForegroundColor Magenta
