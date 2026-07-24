use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkMode {
    Basic,
    Optimized,
    Extreme,
}

impl std::fmt::Display for NetworkMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkMode::Basic => write!(f, "Basic (基础 TLS/DNS 协议硬化)"),
            NetworkMode::Optimized => write!(f, "Optimized (刷新 DNS & 优化 TCP 窗口)"),
            NetworkMode::Extreme => write!(f, "Extreme (CTCP & ECN + WinHTTP 代理同步)"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionTarget {
    FullSetup,
    NetworkOnly,
    AgentSkillsOnly,
    DevToolsOnly,
    VSCodeExtensionsOnly,
    SystemTweaksOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WingetApp {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupConfig {
    pub include_dev_tools: bool,
    pub include_optional_features: bool,
    pub include_system_tweaks: bool,
    pub include_agent_skills: bool,
    pub include_docker_wsl: bool,
    pub include_deep_win_tweaks: bool,
    pub include_git_shell_configs: bool,
    pub include_vscode_extensions: bool,
    pub include_npmrc_config: bool,
    pub include_ollama_models: bool,
    pub network_mode: NetworkMode,
    pub target_mode: ExecutionTarget,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            include_dev_tools: true,
            include_optional_features: true,
            include_system_tweaks: true,
            include_agent_skills: true,
            include_docker_wsl: true,
            include_deep_win_tweaks: true,
            include_git_shell_configs: true,
            include_vscode_extensions: true,
            include_npmrc_config: true,
            include_ollama_models: true,
            network_mode: NetworkMode::Optimized,
            target_mode: ExecutionTarget::FullSetup,
        }
    }
}

pub fn get_core_winget_apps() -> Vec<WingetApp> {
    vec![
        WingetApp { id: "7zip.7zip".to_string(), name: "7-Zip Archiver".to_string() },
        WingetApp { id: "VideoLAN.VLC".to_string(), name: "VLC Media Player".to_string() },
        WingetApp { id: "Google.Chrome".to_string(), name: "Google Chrome".to_string() },
        WingetApp { id: "Notepad++.Notepad++".to_string(), name: "Notepad++".to_string() },
        WingetApp { id: "ShareX.ShareX".to_string(), name: "ShareX Productivity".to_string() },
        WingetApp { id: "IrfanSkiljan.IrfanView".to_string(), name: "IrfanView Media Viewer".to_string() },
    ]
}

pub fn get_dev_winget_apps() -> Vec<WingetApp> {
    vec![
        // IDEs & Browsers (100% Homebrew Cask Parity)
        WingetApp { id: "Microsoft.VisualStudioCode".to_string(), name: "Visual Studio Code".to_string() },
        WingetApp { id: "Anysphere.Cursor".to_string(), name: "Cursor AI IDE".to_string() },
        WingetApp { id: "Brave.Brave".to_string(), name: "Brave Browser".to_string() },
        WingetApp { id: "LibreWolf.LibreWolf".to_string(), name: "LibreWolf Privacy Browser".to_string() },
        // Developer & Security
        WingetApp { id: "Bitwarden.CLI".to_string(), name: "Bitwarden CLI".to_string() },
        WingetApp { id: "Bitwarden.Bitwarden".to_string(), name: "Bitwarden Desktop".to_string() },
        WingetApp { id: "LocalSend.LocalSend".to_string(), name: "LocalSend File Transfer".to_string() },
        WingetApp { id: "GnuPG.Gpg4win".to_string(), name: "Gpg4win Security Suite".to_string() },
        WingetApp { id: "Microsoft.OpenJDK.21".to_string(), name: "Microsoft OpenJDK 21".to_string() },
        WingetApp { id: "EFF.Certbot".to_string(), name: "Certbot SSL Tool".to_string() },
        WingetApp { id: "Cryptomator.Cryptomator".to_string(), name: "Cryptomator Vault".to_string() },
        // Virtualization & Containers
        WingetApp { id: "Docker.DockerDesktop".to_string(), name: "Docker Desktop".to_string() },
        WingetApp { id: "RedHat.PodmanDesktop".to_string(), name: "Podman Desktop".to_string() },
        // Graphics & Design
        WingetApp { id: "KDE.Krita".to_string(), name: "Krita Digital Painting".to_string() },
        WingetApp { id: "Pureref.PureRef".to_string(), name: "PureRef Reference Board".to_string() },
        WingetApp { id: "Aseprite.Aseprite".to_string(), name: "Aseprite Pixel Art".to_string() },
        WingetApp { id: "BlenderFoundation.Blender".to_string(), name: "Blender 3D Creation".to_string() },
        WingetApp { id: "PeaZip.PeaZip".to_string(), name: "PeaZip Utility".to_string() },
        // Social, AI & Gaming
        WingetApp { id: "Discord.Discord".to_string(), name: "Discord Desktop".to_string() },
        WingetApp { id: "Telegram.TelegramDesktop".to_string(), name: "Telegram Desktop".to_string() },
        WingetApp { id: "OpenAI.ChatGPT".to_string(), name: "ChatGPT Desktop".to_string() },
        WingetApp { id: "Anthropic.Claude".to_string(), name: "Claude Desktop".to_string() },
        WingetApp { id: "Valve.Steam".to_string(), name: "Steam Gaming Client".to_string() },
    ]
}

pub fn get_scoop_tools() -> Vec<&'static str> {
    vec![
        // Version Control & Shell Infrastructure
        "git", "gh", "git-lfs", "restic", "chezmoi", "atuin", "direnv", "starship", "fastfetch",
        // Runtimes & Environment Managers
        "python", "nodejs-lts", "go", "zig", "deno", "fnm", "bun", "pnpm", "neovim", "tmux", "mise",
        // Compilers & Build Tools
        "cmake", "ninja", "nasm", "yasm", "sccache", "just", "mold", "gcc",
        // Modern CLI Utility Alternatives
        "ripgrep", "fd", "fzf", "bat", "eza", "topgrade", "tree", "fdupes", "jdupes", "parallel", "b3sum", "xxhash",
        // Code Hygiene & Linters
        "actionlint", "shellcheck", "shfmt",
        // Media, Video & Audio Converters
        "ffmpeg", "imagemagick", "exiftool", "yt-dlp", "gallery-dl", "transmission-cli", "poppler", "tesseract",
        // Proxy, Network & Database
        "aria2", "wget", "buku", "lz4", "zstd", "xz", "brotli", "sqlite", "postgresql",
        "sing-box", "mihomo", "nextdns", "smartdns",
        // Local AI & Machine Learning
        "ollama", "whisper-cpp",
    ]
}

pub fn get_vscode_extensions() -> Vec<&'static str> {
    vec![
        "anthropic.claude-code",
        "anysphere.cursorpyright",
        "anysphere.remote-containers",
        "anysphere.remote-ssh",
        "davidanson.vscode-markdownlint",
        "donjayamanne.githistory",
        "eamodio.gitlens",
        "github.vscode-github-actions",
        "golang.go",
        "ms-azuretools.vscode-docker",
        "ms-python.python",
        "ms-python.debugpy",
        "vscodevim.vim",
    ]
}

pub fn get_cargo_packages() -> Vec<&'static str> {
    vec![
        // Token Killer & Repositories
        "rtk", "bkmr", "yek",
        // Cargo Developer Suites
        "cargo-edit", "cargo-expand", "cargo-audit", "cargo-deny", "cargo-hack",
        "cargo-license", "cargo-machete", "cargo-mutants", "cargo-semver-checks", "cargo-udeps",
        "cargo-bloat", "cargo-about", "cargo-upgrades", "rust-script", "flamegraph",
        // Storage & Disk Utilities
        "dupe-krill", "fclones", "kondo", "krokiet",
    ]
}

pub fn get_npm_globals() -> Vec<&'static str> {
    vec![
        "@anthropic-ai/claude-code", "@alibaba-group/open-code-review", "@diff4/cli",
        "acp-ts", "context-mode", "lodash", "openclaw", "opencode-ai",
        "pyright", "run-deepseek-cli", "typescript", "typescript-language-server",
        "uipro-cli", "prettier", "markdownlint-cli2",
    ]
}

pub fn get_pip_packages() -> Vec<&'static str> {
    vec![
        "flask", "flask-cors", "numpy", "scipy", "scikit-learn", "pillow",
        "opencv-python", "torch", "lightgbm", "openvino", "tqdm", "joblib",
        "sympy", "networkx", "PyWavelets", "certifi", "cryptography", "filelock", "fsspec",
        "ruff", "pyupgrade",
    ]
}

pub fn get_uv_tools() -> Vec<&'static str> {
    vec!["kimi-cli", "ruff"]
}
