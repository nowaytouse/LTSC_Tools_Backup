use crate::inventory::{MacosInventory, ParityProvider, ParityRule, ParityRules, SourceKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
            NetworkMode::Extreme => write!(f, "Extreme (RSS/RSC + Fast Open + CTCP/ECN)"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionTarget {
    FullSetup,
    NetworkOnly,
    StorageOnly,
    WindowsFeaturesOnly,
    AgentSkillsOnly,
    DevToolsOnly,
    VSCodeExtensionsOnly,
    SystemTweaksOnly,
    RollbackLatest,
}

impl ExecutionTarget {
    pub const ALL: [Self; 9] = [
        Self::FullSetup,
        Self::DevToolsOnly,
        Self::NetworkOnly,
        Self::StorageOnly,
        Self::WindowsFeaturesOnly,
        Self::VSCodeExtensionsOnly,
        Self::AgentSkillsOnly,
        Self::SystemTweaksOnly,
        Self::RollbackLatest,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::FullSetup => "完整部署",
            Self::DevToolsOnly => "开发工具",
            Self::NetworkOnly => "网络优化",
            Self::StorageOnly => "存储优化",
            Self::WindowsFeaturesOnly => "Windows 功能",
            Self::VSCodeExtensionsOnly => "IDE 同步",
            Self::AgentSkillsOnly => "Agent 配置",
            Self::SystemTweaksOnly => "系统优化",
            Self::RollbackLatest => "撤销上次调优",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::FullSetup => "按依赖顺序部署软件、开发环境和系统设置",
            Self::DevToolsOnly => "同步 WinGet、Scoop、Cargo、NPM 与 Python 工具",
            Self::NetworkOnly => "保存原配置后刷新 DNS 并应用选定的 TCP 设置",
            Self::StorageOnly => "启用 TRIM，并按 SSD/HDD 介质类型执行 Windows 官方优化",
            Self::WindowsFeaturesOnly => "按能力检测启用 WSL2、.NET 3.5、Sandbox 等可选功能",
            Self::VSCodeExtensionsOnly => "合并 VS Code / Cursor 设置并补齐扩展",
            Self::AgentSkillsOnly => "从程序内置资源恢复 Skills、Plugins 与 MCP 配置",
            Self::SystemTweaksOnly => "通过 Windows 原生注册表接口应用性能与隐私设置",
            Self::RollbackLatest => "按最近一次本机账本逆序恢复注册表、电源与网络设置",
        }
    }

    pub fn requires_admin(self) -> bool {
        matches!(
            self,
            Self::FullSetup
                | Self::NetworkOnly
                | Self::StorageOnly
                | Self::WindowsFeaturesOnly
                | Self::SystemTweaksOnly
                | Self::RollbackLatest
        )
    }
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
    pub create_rollback_journal: bool,
    pub activate_ultimate_performance: bool,
    pub extreme_ac_power_settings: bool,
    pub disable_hibernation: bool,
    pub enable_trim: bool,
    pub optimize_storage: bool,
    pub disable_telemetry: bool,
    pub disable_bing_search: bool,
    pub disable_consumer_features: bool,
    pub disable_advertising_id: bool,
    pub disable_activity_history: bool,
    pub disable_feedback_prompts: bool,
    pub explorer_open_to_this_pc: bool,
    pub explorer_show_file_extensions: bool,
    pub explorer_show_hidden_files: bool,
    pub enable_ntfs_long_paths: bool,
    pub enable_developer_mode: bool,
    pub system_responsiveness: u32,
    pub network_throttling_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsFeaturesProfileConfig {
    pub enable_wsl2: bool,
    pub enable_netfx3: bool,
    pub enable_windows_sandbox: bool,
    pub enable_hyper_v: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupProfile {
    pub profile_version: String,
    pub metadata: serde_json::Value,
    pub git_config: GitProfileConfig,
    pub powershell_profile: PowerShellProfileConfig,
    pub environment_mirrors: MirrorsProfileConfig,
    pub vscode_config: VSCodeProfileConfig,
    pub packages: PackageMatrixConfig,
    pub system_tweaks: SystemTweaksProfileConfig,
    pub windows_features: WindowsFeaturesProfileConfig,
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

    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.profile_version.trim().is_empty() {
            anyhow::bail!("profile_version 不能为空");
        }
        validate_winget_apps("winget_core", &self.packages.winget_core)?;
        validate_winget_apps("winget_dev", &self.packages.winget_dev)?;
        validate_unique_strings("scoop_tools", &self.packages.scoop_tools)?;
        validate_unique_strings("cargo_packages", &self.packages.cargo_packages)?;
        validate_unique_strings("npm_globals", &self.packages.npm_globals)?;
        validate_unique_strings("pip_packages", &self.packages.pip_packages)?;
        validate_unique_strings("uv_tools", &self.packages.uv_tools)?;
        validate_unique_strings("vscode extensions", &self.vscode_config.extensions)?;
        validate_unique_strings("ollama_models", &self.ollama_models)?;
        if self.system_tweaks.system_responsiveness > 100 {
            anyhow::bail!("system_responsiveness 必须在 0..=100 范围内");
        }
        if self.system_tweaks.network_throttling_index > u32::MAX as u64 {
            anyhow::bail!("network_throttling_index 超出 DWORD 范围");
        }
        ParityRules::load_embedded()?.validate()?;
        Ok(())
    }

    pub fn parity_report(&self, inventory: &MacosInventory) -> anyhow::Result<ParityReport> {
        let rules = ParityRules::load_embedded()?;
        rules.validate()?;
        let mut report = ParityReport::default();

        for source in &inventory.homebrew_formulae {
            self.classify_parity(&rules, SourceKind::Formula, source, &mut report);
        }
        for source in &inventory.homebrew_casks {
            self.classify_parity(&rules, SourceKind::Cask, source, &mut report);
        }
        for source in &inventory.cargo_packages {
            self.classify_parity(&rules, SourceKind::Cargo, source, &mut report);
        }
        for source in &inventory.npm_globals {
            self.classify_parity(&rules, SourceKind::Npm, source, &mut report);
        }
        for source in &inventory.uv_tools {
            self.classify_parity(&rules, SourceKind::Uv, source, &mut report);
        }
        Ok(report)
    }

    fn classify_parity(
        &self,
        rules: &ParityRules,
        source_kind: SourceKind,
        source: &str,
        report: &mut ParityReport,
    ) {
        if self.has_package(source) {
            report.automatic += 1;
            return;
        }

        let Some(rule) = rules.find(source_kind.clone(), source) else {
            report.unmapped.push(format!("{source_kind:?}:{source}"));
            return;
        };
        match rule.provider {
            ParityProvider::Winget
            | ParityProvider::Scoop
            | ParityProvider::Cargo
            | ParityProvider::Npm
            | ParityProvider::Pip
            | ParityProvider::Uv => {
                if self.rule_target_exists(rule) {
                    report.automatic += 1;
                } else {
                    report.unmapped.push(format!(
                        "{source_kind:?}:{source} -> {:?}:{}（目标未进入安装矩阵）",
                        rule.provider, rule.target
                    ));
                }
            }
            ParityProvider::Alternative => {
                if self.has_package(&rule.target) {
                    report.compatible += 1;
                } else {
                    report.unmapped.push(format!(
                        "{source_kind:?}:{source} -> alternative:{}（目标未进入安装矩阵）",
                        rule.target
                    ));
                }
            }
            ParityProvider::WindowsBuiltIn | ParityProvider::Wsl => report.compatible += 1,
            ParityProvider::MacOnly => report.mac_only += 1,
            ParityProvider::Manual => report.manual.push(format!(
                "{source_kind:?}:{source} -> {}（{}）",
                rule.target, rule.reason
            )),
        }
    }

    fn rule_target_exists(&self, rule: &ParityRule) -> bool {
        match rule.provider {
            ParityProvider::Winget => self
                .packages
                .winget_core
                .iter()
                .chain(&self.packages.winget_dev)
                .any(|app| app.id.eq_ignore_ascii_case(&rule.target)),
            ParityProvider::Scoop => contains_ci(&self.packages.scoop_tools, &rule.target),
            ParityProvider::Cargo => contains_ci(&self.packages.cargo_packages, &rule.target),
            ParityProvider::Npm => contains_ci(&self.packages.npm_globals, &rule.target),
            ParityProvider::Pip => contains_ci(&self.packages.pip_packages, &rule.target),
            ParityProvider::Uv => contains_ci(&self.packages.uv_tools, &rule.target),
            _ => self.has_package(&rule.target),
        }
    }

    fn has_package(&self, name: &str) -> bool {
        let winget_match = self
            .packages
            .winget_core
            .iter()
            .chain(&self.packages.winget_dev)
            .any(|app| {
                app.id.eq_ignore_ascii_case(name)
                    || app.name.eq_ignore_ascii_case(name)
                    || app
                        .id
                        .rsplit('.')
                        .next()
                        .is_some_and(|tail| tail.eq_ignore_ascii_case(name))
            });
        winget_match
            || contains_ci(&self.packages.scoop_tools, name)
            || contains_ci(&self.packages.cargo_packages, name)
            || contains_ci(&self.packages.npm_globals, name)
            || contains_ci(&self.packages.pip_packages, name)
            || contains_ci(&self.packages.uv_tools, name)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParityReport {
    pub automatic: usize,
    pub compatible: usize,
    pub mac_only: usize,
    pub manual: Vec<String>,
    pub unmapped: Vec<String>,
}

impl ParityReport {
    pub fn covered(&self) -> usize {
        self.automatic + self.compatible + self.mac_only + self.manual.len()
    }
}

fn contains_ci(values: &[String], needle: &str) -> bool {
    values
        .iter()
        .any(|value| value.eq_ignore_ascii_case(needle))
}

fn validate_winget_apps(label: &str, apps: &[WingetApp]) -> anyhow::Result<()> {
    let mut ids = std::collections::BTreeSet::new();
    for app in apps {
        if app.id.trim().is_empty() || app.name.trim().is_empty() {
            anyhow::bail!("{label} 中存在空的 id 或 name");
        }
        if !app
            .id
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "-._+".contains(character))
        {
            anyhow::bail!("{label} 中包含无效包 ID: {}", app.id);
        }
        if !ids.insert(app.id.to_ascii_lowercase()) {
            anyhow::bail!("{label} 中存在重复包 ID: {}", app.id);
        }
    }
    Ok(())
}

fn validate_unique_strings(label: &str, values: &[String]) -> anyhow::Result<()> {
    let mut seen = std::collections::BTreeSet::new();
    for value in values {
        if value.trim().is_empty() {
            anyhow::bail!("{label} 中存在空项目");
        }
        if !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "-._+@/:".contains(character))
        {
            anyhow::bail!("{label} 中包含不安全的命令参数字符: {value}");
        }
        if !seen.insert(value.to_ascii_lowercase()) {
            anyhow::bail!("{label} 中存在重复项目: {value}");
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupConfig {
    pub include_dev_tools: bool,
    pub include_agent_skills: bool,
    pub include_docker_wsl: bool,
    pub include_storage_optimization: bool,
    pub include_deep_win_tweaks: bool,
    pub include_git_shell_configs: bool,
    pub include_vscode_extensions: bool,
    pub include_npmrc_config: bool,
    pub include_ollama_models: bool,
    pub network_mode: NetworkMode,
    pub target_mode: ExecutionTarget,
    pub profile: SetupProfile,
    pub source_inventory: MacosInventory,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            include_dev_tools: true,
            include_agent_skills: true,
            include_docker_wsl: true,
            include_storage_optimization: true,
            include_deep_win_tweaks: true,
            include_git_shell_configs: true,
            include_vscode_extensions: true,
            include_npmrc_config: true,
            include_ollama_models: true,
            network_mode: NetworkMode::Optimized,
            target_mode: ExecutionTarget::FullSetup,
            profile: SetupProfile::load_default(),
            source_inventory: MacosInventory::load_embedded().unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SetupProfile;

    #[test]
    fn embedded_profile_is_valid() {
        let profile = SetupProfile::load_default();
        profile.validate().unwrap();
        let audit = &profile.metadata["provider_audit"];
        assert_eq!(
            audit["winget_manifest_ids_verified"].as_u64(),
            Some((profile.packages.winget_core.len() + profile.packages.winget_dev.len()) as u64)
        );
        assert_eq!(
            audit["scoop_manifests_verified"].as_u64(),
            Some(profile.packages.scoop_tools.len() as u64)
        );
    }

    #[test]
    fn duplicate_package_ids_are_rejected() {
        let mut profile = SetupProfile::load_default();
        profile
            .packages
            .winget_core
            .push(profile.packages.winget_core[0].clone());

        let error = profile.validate().unwrap_err().to_string();
        assert!(error.contains("重复包 ID"));
    }

    #[test]
    fn source_inventory_has_no_silent_unmapped_tools() {
        let profile = SetupProfile::load_default();
        let inventory = crate::inventory::MacosInventory::load_embedded().unwrap();
        let report = profile.parity_report(&inventory).unwrap();
        assert!(
            report.unmapped.is_empty(),
            "unmapped source tools: {:?}",
            report.unmapped
        );
        assert_eq!(report.covered(), inventory.item_count());
    }
}
