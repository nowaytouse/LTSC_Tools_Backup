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
            NetworkMode::Extreme => write!(f, "Extreme (CTCP & ECN + WinHTTP 代理)"),
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
pub struct GitProfileConfig {
    pub user_name: String,
    pub user_email: String,
    pub post_buffer_bytes: u64,
    pub safe_directory: String,
    pub enable_long_paths: bool,
    pub enable_autocrlf: bool,
    pub global_gitignore_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerShellProfileConfig {
    pub enable_utf8_encoding: bool,
    pub init_starship: bool,
    pub init_zoxide: bool,
    pub aliases: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfileConfig {
    pub mode: String,
    pub enable_tls12_tls13: bool,
    pub flush_dns: bool,
    pub enable_ctcp: bool,
    pub enable_ecn: bool,
    pub import_winhttp_proxy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorsProfileConfig {
    pub cargo_sparse_index: String,
    pub pip_index_url: String,
    pub npm_registry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeProfileConfig {
    pub user_settings: serde_json::Value,
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMatrixConfig {
    pub winget_core: Vec<WingetApp>,
    pub winget_dev: Vec<WingetApp>,
    pub scoop_tools: Vec<String>,
    pub cargo_packages: Vec<String>,
    pub npm_globals: Vec<String>,
    pub pip_packages: Vec<String>,
    pub uv_tools: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTweaksProfileConfig {
    pub activate_ultimate_performance: bool,
    pub disable_telemetry: bool,
    pub disable_bing_search: bool,
    pub explorer_open_to_this_pc: bool,
    pub explorer_show_file_extensions: bool,
    pub explorer_show_hidden_files: bool,
    pub enable_ntfs_long_paths: bool,
    pub system_responsiveness: u32,
    pub network_throttling_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupProfile {
    pub profile_version: String,
    pub metadata: serde_json::Value,
    pub git_config: GitProfileConfig,
    pub powershell_profile: PowerShellProfileConfig,
    pub network_config: NetworkProfileConfig,
    pub environment_mirrors: MirrorsProfileConfig,
    pub vscode_config: VSCodeProfileConfig,
    pub packages: PackageMatrixConfig,
    pub system_tweaks: SystemTweaksProfileConfig,
    pub ollama_models: Vec<String>,
}

impl SetupProfile {
    pub fn load_default() -> Self {
        let json_bytes = include_bytes!("assets/setup_profile.json");
        serde_json::from_slice(json_bytes).unwrap_or_else(|e| {
            eprintln!("Failed to parse default setup_profile.json: {}", e);
            panic!("Invalid default setup_profile.json embedded payload");
        })
    }

    #[allow(dead_code)]
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let profile = serde_json::from_str(&content)?;
        Ok(profile)
    }

    #[allow(dead_code)]
    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
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
    pub profile: SetupProfile,
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
            profile: SetupProfile::load_default(),
        }
    }
}
