#[cfg(target_os = "macos")]
use crate::utils::{run_native_cmd_timeout, CancellationToken};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[cfg(target_os = "macos")]
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacosInventory {
    pub schema_version: u32,
    pub captured_at: String,
    pub homebrew_formulae: Vec<String>,
    pub homebrew_casks: Vec<String>,
    pub homebrew_taps: Vec<String>,
    pub cargo_packages: Vec<String>,
    pub npm_globals: Vec<String>,
    pub uv_tools: Vec<String>,
    pub vscode_extensions: Vec<String>,
    pub cursor_extensions: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Formula,
    Cask,
    Cargo,
    Npm,
    Uv,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParityProvider {
    Winget,
    Scoop,
    Cargo,
    Npm,
    Pip,
    Uv,
    WindowsBuiltIn,
    Wsl,
    Alternative,
    MacOnly,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParityRule {
    pub source_kind: SourceKind,
    pub source: String,
    pub provider: ParityProvider,
    pub target: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParityRules {
    pub schema_version: u32,
    pub rules: Vec<ParityRule>,
}

impl ParityRules {
    pub fn load_embedded() -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(include_bytes!(
            "assets/parity_rules.json"
        ))?)
    }

    pub fn find(&self, source_kind: SourceKind, source: &str) -> Option<&ParityRule> {
        self.rules.iter().find(|rule| {
            rule.source_kind == source_kind && rule.source.eq_ignore_ascii_case(source)
        })
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.schema_version != 1 {
            anyhow::bail!("不支持的 parity_rules schema：{}", self.schema_version);
        }
        let mut keys = BTreeSet::new();
        for rule in &self.rules {
            if rule.source.trim().is_empty()
                || rule.target.trim().is_empty()
                || rule.reason.trim().is_empty()
            {
                anyhow::bail!("parity_rules 包含空字段：{:?}", rule);
            }
            let key = format!(
                "{:?}:{}",
                rule.source_kind,
                rule.source.to_ascii_lowercase()
            );
            if !keys.insert(key) {
                anyhow::bail!(
                    "parity_rules 包含重复映射：{:?}/{}",
                    rule.source_kind,
                    rule.source
                );
            }
        }
        Ok(())
    }
}

impl MacosInventory {
    pub fn load_embedded() -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(include_bytes!(
            "assets/macos_inventory.json"
        ))?)
    }

    #[cfg(target_os = "macos")]
    pub fn capture() -> anyhow::Result<Self> {
        let cancellation = CancellationToken::default();
        let mut inventory = Self {
            schema_version: 1,
            captured_at: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            ..Self::default()
        };

        inventory.homebrew_formulae =
            required_lines("Homebrew formulae", "brew", &["leaves"], &cancellation)?;
        inventory.homebrew_casks = optional_lines(
            "Homebrew casks",
            "brew",
            &["list", "--cask"],
            &cancellation,
            &mut inventory.warnings,
        );
        // Tap names can disclose personal/private repository namespaces and are
        // not installation targets on Windows. Keep the public snapshot useful
        // for parity without publishing source-account metadata.
        inventory.homebrew_taps.clear();
        inventory
            .warnings
            .push("Homebrew taps 未写入公开快照；Windows parity 仅使用 formula/cask 名称".into());

        let cargo_output = optional_output(
            "Cargo tools",
            "cargo",
            &["install", "--list"],
            &cancellation,
            &mut inventory.warnings,
        );
        inventory.cargo_packages = parse_cargo_install_list(&cargo_output);

        let npm_output = optional_output(
            "NPM globals",
            "npm",
            &["--global", "ls", "--depth=0", "--json"],
            &cancellation,
            &mut inventory.warnings,
        );
        inventory.npm_globals = parse_npm_globals(&npm_output).unwrap_or_else(|error| {
            inventory
                .warnings
                .push(format!("NPM globals 解析失败：{error}"));
            Vec::new()
        });

        let uv_output = optional_output(
            "uv tools",
            "uv",
            &["tool", "list"],
            &cancellation,
            &mut inventory.warnings,
        );
        inventory.uv_tools = parse_uv_tool_list(&uv_output);

        inventory.vscode_extensions = optional_lines(
            "VS Code extensions",
            "code",
            &["--list-extensions"],
            &cancellation,
            &mut inventory.warnings,
        );
        inventory.cursor_extensions = optional_lines(
            "Cursor extensions",
            "cursor",
            &["--list-extensions"],
            &cancellation,
            &mut inventory.warnings,
        );

        inventory.normalize();
        Ok(inventory)
    }

    #[cfg(target_os = "macos")]
    pub fn save_atomic(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let temporary = path.with_extension("json.tmp");
        std::fs::write(&temporary, serde_json::to_vec_pretty(self)?)?;
        std::fs::rename(&temporary, path)?;
        Ok(())
    }

    pub fn item_count(&self) -> usize {
        self.homebrew_formulae.len()
            + self.homebrew_casks.len()
            + self.cargo_packages.len()
            + self.npm_globals.len()
            + self.uv_tools.len()
    }

    #[cfg(target_os = "macos")]
    fn normalize(&mut self) {
        normalize_list(&mut self.homebrew_formulae);
        normalize_list(&mut self.homebrew_casks);
        normalize_list(&mut self.homebrew_taps);
        normalize_list(&mut self.cargo_packages);
        normalize_list(&mut self.npm_globals);
        normalize_list(&mut self.uv_tools);
        normalize_list(&mut self.vscode_extensions);
        normalize_list(&mut self.cursor_extensions);
        normalize_list(&mut self.warnings);
    }
}

#[cfg(target_os = "macos")]
fn required_lines(
    label: &str,
    program: &str,
    args: &[&str],
    cancellation: &CancellationToken,
) -> anyhow::Result<Vec<String>> {
    let result = run_native_cmd_timeout(program, args, 90, cancellation);
    if !result.succeeded() {
        anyhow::bail!("{label} 采集失败：{}", result.diagnostic());
    }
    Ok(parse_nonempty_lines(&result.output))
}

#[cfg(target_os = "macos")]
fn optional_lines(
    label: &str,
    program: &str,
    args: &[&str],
    cancellation: &CancellationToken,
    warnings: &mut Vec<String>,
) -> Vec<String> {
    parse_nonempty_lines(&optional_output(
        label,
        program,
        args,
        cancellation,
        warnings,
    ))
}

#[cfg(target_os = "macos")]
fn optional_output(
    label: &str,
    program: &str,
    args: &[&str],
    cancellation: &CancellationToken,
    warnings: &mut Vec<String>,
) -> String {
    let result = run_native_cmd_timeout(program, args, 90, cancellation);
    if result.succeeded() {
        result.output
    } else {
        warnings.push(format!("{label} 未采集：{}", result.diagnostic()));
        String::new()
    }
}

#[cfg(target_os = "macos")]
fn normalize_list(values: &mut Vec<String>) {
    let unique: BTreeSet<_> = values
        .drain(..)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect();
    *values = unique.into_iter().collect();
}

#[cfg(target_os = "macos")]
fn parse_nonempty_lines(output: &str) -> Vec<String> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(any(target_os = "macos", test))]
fn parse_cargo_install_list(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| !line.starts_with(char::is_whitespace))
        .filter_map(|line| line.trim_end_matches(':').split_once(" v"))
        .map(|(name, _)| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .collect()
}

#[cfg(any(target_os = "macos", test))]
fn parse_npm_globals(output: &str) -> anyhow::Result<Vec<String>> {
    if output.trim().is_empty() {
        return Ok(Vec::new());
    }
    let value: serde_json::Value = serde_json::from_str(output)?;
    let dependencies = value
        .get("dependencies")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| anyhow::anyhow!("缺少 dependencies 对象"))?;
    Ok(dependencies.keys().cloned().collect())
}

#[cfg(any(target_os = "macos", test))]
fn parse_uv_tool_list(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| !line.starts_with(char::is_whitespace) && !line.starts_with('-'))
        .filter_map(|line| {
            line.split_once(" v")
                .map(|(name, _)| name.trim().to_string())
        })
        .filter(|name| !name.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        parse_cargo_install_list, parse_npm_globals, parse_uv_tool_list, MacosInventory,
        ParityRules,
    };

    #[test]
    fn provider_outputs_are_parsed_without_versions_or_binaries() {
        assert_eq!(
            parse_cargo_install_list(
                "cargo-audit v0.22.2:\n    cargo-audit\nrtk v0.42.4 (git):\n    rtk\n"
            ),
            ["cargo-audit", "rtk"]
        );
        assert_eq!(
            parse_npm_globals(
                r#"{"dependencies":{"typescript":{"version":"7"},"@openai/codex":{"version":"1"}}}"#
            )
            .unwrap(),
            ["@openai/codex", "typescript"]
        );
        assert_eq!(
            parse_uv_tool_list("kimi-cli v1.49.0\n- kimi\nosxphotos v0.76.1\n- osxphotos\n"),
            ["kimi-cli", "osxphotos"]
        );
    }

    #[test]
    fn embedded_inventory_is_valid_and_private() {
        let inventory = MacosInventory::load_embedded().unwrap();
        assert_eq!(inventory.schema_version, 1);
        assert!(!inventory.captured_at.is_empty());
        assert!(!inventory.homebrew_formulae.is_empty());
        let serialized = serde_json::to_string(&inventory).unwrap();
        assert!(!serialized.contains("/Users/"));
        assert!(!serialized.contains("hostname"));
        assert!(inventory.homebrew_taps.is_empty());
    }

    #[test]
    fn embedded_parity_rules_are_valid() {
        ParityRules::load_embedded().unwrap().validate().unwrap();
    }
}
