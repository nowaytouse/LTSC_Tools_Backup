# CHANGELOG - Windows LTSC Ultimate Workstation Setup

All notable changes to the `LTSC_Tools_Backup` project and the native Rust GUI executable (`ltsc_setup_gui.exe`) are documented in this file.

---

## [v1.0.0] - 2026-07-24 (Native Rust GUI Ultimate Release)

### 🚀 Major Highlights
- **Native Rust GUI Application (`ltsc_setup_gui.exe`)**:
  - Replaced legacy PowerShell scripts with a high-performance, single-file native Windows GUI binary built using `eframe` / `egui`.
  - Zero-dependency deployment: double-click to run on Windows without installing Rust, PowerShell execution policy bypasses, or manual CLI commands.
- **Embedded Binary Asset Extraction (`include_dir`)**:
  - Embedded all 55+ real AI Agent Skills (`bigquery`, `cloud-sql-*`, `ponytail`, `rtk`, `alloydb-*`, etc.), plugins, `AGENTS.md` global rules, and `mcp_config.json` directly into the `.exe` binary memory payload.
  - Automatically extracts and deploys to `%USERPROFILE%\.gemini\config` at runtime.

### 💻 macOS 100% Homebrew & Environment Parity
- **Winget GUI Apps Matrix**: VS Code, Cursor AI IDE, Chrome, Brave, LibreWolf, Docker Desktop, Podman Desktop, Krita, PureRef, Bitwarden, LocalSend, Cryptomator, Gpg4win, PeaZip, Blender, Discord, Telegram, Steam, ChatGPT, Claude.
- **Scoop CLI Stack Matrix**: Git, GH, Git-LFS, Fnm, Bun, Pnpm, Deno, Go, Rustup, Zig, OpenJDK 21, CMake, Ninja, Nasm, Yasm, Sccache, Just, Ripgrep, Fd, Fzf, Bat, Eza, Starship, Fastfetch, Topgrade, Restic, Chezmoi, Atuin, Direnv, Neovim, Tmux, Mise, Mold, Postgresql, Sqlite, Sing-box, Mihomo, NextDNS, SmartDNS, Ollama, Whisper-cpp.
- **Cargo Tools Matrix**: `rtk` (Rust Token Killer), `bkmr`, `cargo-edit`, `cargo-expand`, `cargo-audit`, `cargo-deny`, `dupe-krill`, `fclones`, `kondo`, `krokiet`, `rust-script`, `yek`.
- **NPM Globals & AI CLI Suite**: `@anthropic-ai/claude-code`, `@alibaba-group/open-code-review`, `@diff4/cli`, `acp-ts`, `context-mode`, `opencode-ai`, `pyright`, `run-deepseek-cli`, `typescript`, `typescript-language-server`, `uipro-cli`, `prettier`, `markdownlint-cli2`.
- **Pip & UV Tools**: `kimi-cli`, `ruff`, `pyupgrade`, `numpy`, `torch`, `opencv-python`, `scipy`, `scikit-learn`.

### 🔑 Git & Shell Integration
- **Git User Config**: Auto-configures `user.name = "nowaytouse"`, `user.email = "104445933+nowaytouse@users.noreply.github.com"`, `http.postBuffer = 524288000`, `safe.directory = "*"`, `core.longpaths = true`, and global gitignore (`%USERPROFILE%\.config\git\ignore`).
- **PowerShell 7 Profile (`$PROFILE`)**: Auto-provisions `$PROFILE` with UTF-8 console output encoding, `starship init`, `zoxide init`, and aliases (`g`, `ls`, `ll`, `cat`, `find`, `grep`, `top`).

### 🧩 IDE Extensions & Settings Automation
- **VS Code & Cursor Extensions**: Automatically installs 13 core extensions (`anthropic.claude-code`, `anysphere.cursorpyright`, `remote-containers`, `remote-ssh`, `gitlens`, `githistory`, `vscode-docker`, `python`, `golang.go`, `vscode-markdownlint`, `vim`).
- **User Preference `settings.json`**: Deploys `%APPDATA%\Code\User\settings.json` and `%APPDATA%\Cursor\User\settings.json`.

### ⚡ High-Speed Mirror Acceleration
- **Cargo Sparse Index Mirror**: Tsinghua Sparse Index in `%USERPROFILE%\.cargo\config.toml`.
- **Pip Index Mirror**: Tsinghua PyPI index in `%APPDATA%\pip\pip.ini`.
- **NPM Mirror**: npmmirror registry in `%USERPROFILE%\.npmrc`.
- **Scoop Buckets**: Auto-registers `extras`, `versions`, `nirsoft`, `sysinternals` buckets.

### 🛡️ Windows LTSC Performance & Privacy Tweaks
- Unlocks & activates **Ultimate Performance Power Plan** (卓越性能模式).
- Disables Windows Telemetry (`AllowTelemetry = 0`) & Bing Search in Start Menu.
- Explorer tweaks: Opens to "This PC", shows extensions & hidden files, enables NTFS Long Paths (`LongPathsEnabled = 1`).
- Hardware responsiveness: Sets `SystemResponsiveness = 0` and disables network throttling.

---
