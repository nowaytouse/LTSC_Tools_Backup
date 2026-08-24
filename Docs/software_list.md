# LTSC Software & Capability Coverage (macOS Parity Edition)

## Included By Default (Core Baseline)

The GUI's embedded profile (`src\assets\setup_profile.json`) installs these baseline applications via Winget:

- `7zip.7zip` (7-Zip Archiver)
- `VideoLAN.VLC` (VLC Media Player)
- `Google.Chrome` (Google Chrome)
- `Notepad++.Notepad++` (Notepad++)
- `ShareX.ShareX` (ShareX Screen Capture & Productivity)
- `IrfanSkiljan.IrfanView` (IrfanView Media Viewer)

## Included With Developer Mode Enabled (Default)

Developer mode provisions Windows-supported equivalents from the 2026-08-24 host snapshot (103 Homebrew formulae and 15 casks):

### Desktop Applications (Winget)

- **IDEs & Editors**: `Microsoft.VisualStudioCode`, `Anysphere.Cursor`
- **Browsers**: `Brave.Brave`, `LibreWolf.LibreWolf`
- **Developer & Security Tools**: `Bitwarden.CLI`, `Bitwarden.Bitwarden`, `LocalSend.LocalSend`, `GnuPG.Gpg4win`, `Microsoft.OpenJDK.21`, `Microsoft.PowerShell`, `CondaForge.Miniforge3`, `EFF.Certbot`, `Cryptomator.Cryptomator`, `Docker.DockerDesktop`, `RedHat.PodmanDesktop`
- **Graphics & Utilities**: `KDE.Krita`, `Pureref.PureRef`, `PeaZip.PeaZip`, `BlenderFoundation.Blender`

### CLI Runtimes, Utilities & Tools (Scoop)

- **Core Version Control & Cloud**: `git`, `gh`, `git-lfs`, `restic`, `chezmoi`, `atuin`, `direnv`
- **Runtimes & Managers**: `python`, `nodejs-lts`, `go`, `zig`, `deno`, `fnm`, `bun`, `pnpm`, `mise`, `pyenv`, `pipx`, `ruby`, `volta`
- **Build Systems & Compilers**: `cmake`, `meson`, `ninja`, `nasm`, `yasm`, `sccache`, `just`, `mold`, `gcc`
- **Search, Shell & Modern CLI Utility Alternatives**: `ripgrep`, `fd`, `fzf`, `bat`, `eza`, `starship`, `zoxide`, `fastfetch`, `topgrade`, `tree`, `fdupes`, `jdupes`, `parallel`, `tealdeer`
- **Linters & Formatters**: `actionlint`, `shellcheck`, `shfmt`
- **Media, Audio & Conversion**: `ffmpeg`, `imagemagick`, `exiftool`, `apngasm`, `gifsicle`, `gifski`, `gpac`, `mediainfo`, `pngquant`, `dovi-tool`, `yt-dlp`, `gallery-dl`, `transmission-cli`, `poppler`, `tesseract`
- **Network, Compression & Storage**: `aria2`, `wget`, `buku`, `lz4`, `zstd`, `xz`, `brotli`, `sqlite`, `sing-box`, `mihomo`
- **Local AI & ML**: `ollama`

### Rust & Cargo Ecosystem (`cargo install`)

- **Analysis & Linting**: `cargo-audit`, `cargo-deny`, `cargo-machete`, `cargo-mutants`, `cargo-semver-checks`, `cargo-udeps`, `cargo-bloat`, `cargo-about`, `cargo-upgrades`
- **Cargo Extensions**: `cargo-edit`, `cargo-expand`, `cargo-hack`, `cargo-license`, `flamegraph`, `rust-script`
- **Token Optimization & AI Tools**: `rtk` (Rust Token Killer), `bkmr`, `yek`
- **Storage & Disk Utilities**: `dupe-krill`, `fclones`, `kondo`, `krokiet`

### NPM Global CLI Packages (`npm install -g`)

- **AI & LLM Workflows**: `@anthropic-ai/claude-code`, `opencode-ai`, `run-deepseek-cli`, `uipro-cli`, `acp-ts`, `openclaw`, `context-mode`
- **Development & Code Analysis**: `pyright`, `typescript`, `typescript-language-server`, `prettier`, `markdownlint-cli2`, `@alibaba-group/open-code-review`, `@diff4/cli`

### Python & UV Tooling (`pip` & `uv tool install`)

- **Data Science & ML Stack**: `numpy`, `scipy`, `scikit-learn`, `pillow`, `opencv-python`, `torch`, `lightgbm`, `openvino`, `sympy`, `networkx`, `PyWavelets`
- **Utilities & Web**: `flask`, `flask-cors`, `tqdm`, `joblib`, `certifi`, `cryptography`, `filelock`, `fsspec`
- **Code Hygiene**: `ruff`, `pyupgrade`
- **Global UV Tools**: `kimi-cli`, `ruff`

## Operational Parameters

```powershell
.\src\Scripts\00_QuickSetup.ps1 -SkipDevTools
.\src\Scripts\00_QuickSetup.ps1 -SkipOptionalFeatures
.\src\Scripts\00_QuickSetup.ps1 -SkipSystemTweaks
.\src\Scripts\00_QuickSetup.ps1 -NetworkMode Basic|Optimized|Extreme
```

macOS-only packages are intentionally excluded instead of installing unrelated substitutes.

Last Updated: 2026-08-24
