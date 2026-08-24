use crate::config::*;
use crate::utils::{run_native_cmd, run_native_cmd_timeout, run_powershell_cmd, run_powershell_cmd_timeout, LogLevel, LogMessage};
use include_dir::{include_dir, Dir, DirEntry};
use std::cell::Cell;
use std::path::Path;
use std::sync::mpsc::Sender;

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/assets");

pub struct SetupEngine {
    tx: Sender<LogMessage>,
    progress_tx: Sender<f32>,
    config: SetupConfig,
    errors: Cell<usize>,
}

impl SetupEngine {
    pub fn new(tx: Sender<LogMessage>, progress_tx: Sender<f32>, config: SetupConfig) -> Self {
        Self { tx, progress_tx, config, errors: Cell::new(0) }
    }

    fn log(&self, level: LogLevel, msg: impl Into<String>) {
        if level == LogLevel::Error {
            self.errors.set(self.errors.get() + 1);
        }
        let _ = self.tx.send(LogMessage::new(level, msg));
    }

    fn finish(&self, success: &str) {
        let errors = self.errors.get();
        let (level, message) = if errors == 0 { (LogLevel::End, success.to_string()) } else { (LogLevel::Error, format!("部署结束，但有 {errors} 项失败；请按红色日志修复后重试。")) };
        let _ = self.tx.send(LogMessage::new(level, message));
    }

    fn progress(&self, val: f32) {
        let _ = self.progress_tx.send(val);
    }

    pub fn run_full_setup(&self) {
        self.log(LogLevel::Info, format!("读取并初始化 JSON 配置配置表版本 v{} ({})", self.config.profile.profile_version, self.config.profile.metadata.get("name").unwrap_or(&serde_json::Value::Null)));

        match self.config.target_mode {
            ExecutionTarget::FullSetup => self.run_all_steps(),
            ExecutionTarget::NetworkOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: 网络与代理接口硬化...");
                self.step_network();
                self.progress(1.0);
                self.finish("网络与代理接口优化完成。");
            }
            ExecutionTarget::AgentSkillsOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: AI Agent Skills & Hooks 嵌入式释出...");
                self.step_agent_skills();
                self.progress(1.0);
                self.finish("AI Agent Skills & Hooks 释出完成。");
            }
            ExecutionTarget::DevToolsOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: 100+ 开发者软件库部署...");
                self.step_package_managers();
                self.step_dev_suite();
                self.progress(1.0);
                self.finish("开发者软件库部署完成。");
            }
            ExecutionTarget::VSCodeExtensionsOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: VS Code & Cursor 扩展及偏好设置同步...");
                self.step_vscode_and_tools_config();
                self.progress(1.0);
                self.finish("VS Code & Cursor 扩展及配置同步完成。");
            }
            ExecutionTarget::SystemTweaksOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: Windows 性能与隐私深度优化...");
                self.step_deep_win_tweaks();
                self.progress(1.0);
                self.finish("Windows 性能与隐私深度优化完成。");
            }
        }
    }

    fn run_all_steps(&self) {
        self.log(LogLevel::Start, "开始 Windows LTSC 显式全量一键配置流程...");
        self.progress(0.02);

        // 1. Network Optimization & Proxy Sync
        self.step_network();
        self.progress(0.08);

        // 2. Package Managers Bootstrap & Scoop Buckets
        self.step_package_managers();
        self.progress(0.16);

        // 3. Mirrors & Environment Tuning (Cargo / Pip / NPM)
        self.step_environment_mirrors();
        self.progress(0.22);

        // 4. UWP Apps Restore
        self.step_uwp_apps();
        self.progress(0.28);

        // 5. Docker & WSL2 Platform
        if self.config.include_docker_wsl {
            self.step_docker_wsl();
        } else {
            self.log(LogLevel::Info, "显式跳过: Docker & WSL2 虚拟化内核配置");
        }
        self.progress(0.35);

        // 6. Agent Skills & Rules Embedded Extraction & Sync
        if self.config.include_agent_skills {
            self.step_agent_skills();
        } else {
            self.log(LogLevel::Info, "显式跳过: AI Agent Skills / Hooks 配置");
        }
        self.progress(0.45);

        // 7. Git & PowerShell Shell Custom Profile
        if self.config.include_git_shell_configs {
            self.step_git_shell_configs();
        } else {
            self.log(LogLevel::Info, "显式跳过: Git 全局配置与 PowerShell Profile 自动化");
        }
        self.progress(0.55);

        // 8. VS Code & Cursor Extensions & Settings
        if self.config.include_vscode_extensions {
            self.step_vscode_and_tools_config();
        } else {
            self.log(LogLevel::Info, "显式跳过: VS Code & Cursor 扩展与配置文件同步");
        }
        self.progress(0.65);

        // 9. Core Desktop Apps
        self.step_core_apps();
        self.progress(0.72);

        // 10. Developer Suite (100% Homebrew Parity)
        if self.config.include_dev_tools {
            self.step_dev_suite();
        } else {
            self.log(LogLevel::Info, "显式跳过: 开发者 CLI / 工具链配置");
        }
        self.progress(0.88);

        // 11. Local AI Model Pre-pull
        if self.config.include_ollama_models {
            self.step_ollama_models();
        }
        self.progress(0.92);

        // 12. Deep Windows LTSC Optimization Suite
        if self.config.include_deep_win_tweaks {
            self.step_deep_win_tweaks();
        } else {
            self.log(LogLevel::Info, "显式跳过: 深度 Windows 性能与隐私优化");
        }
        self.progress(0.97);

        // 13. Audit Summary
        self.step_audit();
        self.progress(1.0);

        self.finish("Windows LTSC 配置完成；建议重启系统使系统级设置生效。");
    }

    fn step_network(&self) {
        let (label, net_script) = match self.config.network_mode {
            NetworkMode::Basic => (
                "Basic: TLS 1.2 与 DNS 刷新",
                r##"
                    $ErrorActionPreference = "Stop"
                    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
                    ipconfig /flushdns | Out-Null
                    if ($LASTEXITCODE -ne 0) { throw "DNS flush failed: $LASTEXITCODE" }
                "##,
            ),
            NetworkMode::Optimized => (
                "Optimized: TLS 1.2、DNS 刷新与 TCP 自动调优",
                r##"
                    $ErrorActionPreference = "Stop"
                    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
                    ipconfig /flushdns | Out-Null
                    if ($LASTEXITCODE -ne 0) { throw "DNS flush failed: $LASTEXITCODE" }
                    netsh int tcp set global autotuninglevel=normal | Out-Null
                    if ($LASTEXITCODE -ne 0) { throw "TCP autotuning failed: $LASTEXITCODE" }
                "##,
            ),
            NetworkMode::Extreme => (
                "Extreme: Optimized + CTCP、ECN 与 WinHTTP 代理同步",
                r##"
                    $ErrorActionPreference = "Stop"
                    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
                    ipconfig /flushdns | Out-Null
                    if ($LASTEXITCODE -ne 0) { throw "DNS flush failed: $LASTEXITCODE" }
                    netsh int tcp set global autotuninglevel=normal | Out-Null
                    if ($LASTEXITCODE -ne 0) { throw "TCP autotuning failed: $LASTEXITCODE" }
                    netsh int tcp set global congestionprovider=ctcp | Out-Null
                    if ($LASTEXITCODE -ne 0) { throw "CTCP configuration failed: $LASTEXITCODE" }
                    netsh int tcp set global ecncapability=enabled | Out-Null
                    if ($LASTEXITCODE -ne 0) { throw "ECN configuration failed: $LASTEXITCODE" }
                    netsh winhttp import proxy source=ie | Out-Null
                    if ($LASTEXITCODE -ne 0) { throw "WinHTTP proxy import failed: $LASTEXITCODE" }
                "##,
            ),
        };
        self.log(LogLevel::Info, format!("网络模式: {label}"));

        let (ok, out) = run_powershell_cmd(net_script);
        if ok {
            self.log(LogLevel::Ok, format!("网络配置完成: {label}"));
        } else {
            self.log(LogLevel::Error, format!("网络配置失败: {}", out));
        }
    }

    fn step_package_managers(&self) {
        self.log(LogLevel::Info, "检查并部署实际使用的包管理器 (Winget / Scoop)...");

        let winget_script = r##"
            $ErrorActionPreference = "Stop"
            if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
                try {
                    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
                    Install-PackageProvider -Name NuGet -Force | Out-Null
                    Install-Module -Name Microsoft.WinGet.Client -Force -Repository PSGallery -Scope AllUsers | Out-Null
                    Import-Module Microsoft.WinGet.Client
                    Repair-WinGetPackageManager -AllUsers
                } catch {
                    $bundle = Join-Path $env:TEMP "winget.msixbundle"
                    Invoke-WebRequest -Uri "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle" -OutFile $bundle
                    Add-AppxPackage -Path $bundle
                    Remove-Item $bundle -Force -ErrorAction SilentlyContinue
                }
            }
        "##;
        let (bootstrap_ok, bootstrap_out) = run_powershell_cmd_timeout(winget_script, 600);
        let (winget_ok, winget_version) = run_native_cmd_timeout("winget", &["--version"], 30);
        if bootstrap_ok && winget_ok {
            self.log(LogLevel::Ok, format!("Winget 已就绪: {}", winget_version.trim()));
        } else {
            self.log(LogLevel::Error, format!("Winget 启动失败: {}", if bootstrap_out.is_empty() { winget_version } else { bootstrap_out }));
        }

        let scoop_cmd = r##"
            $ErrorActionPreference = "Stop"
            $scoopCommand = Get-Command scoop -ErrorAction SilentlyContinue
            if ($scoopCommand) {
                $scoop = $scoopCommand.Source
            } else {
                Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process -Force
                $installer = Join-Path $env:TEMP "install-scoop.ps1"
                Invoke-RestMethod -Uri "https://get.scoop.sh" -OutFile $installer
                $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
                if ($isAdmin) { & $installer -RunAsAdmin } else { & $installer }
                if ($LASTEXITCODE -ne 0) { throw "Scoop installer exited with $LASTEXITCODE" }
                Remove-Item $installer -Force -ErrorAction SilentlyContinue
                $scoop = Join-Path $env:USERPROFILE "scoop\shims\scoop.cmd"
            }
            if (-not (Test-Path $scoop)) { throw "Scoop shim was not created" }
            $bucketList = (& $scoop bucket list | Out-String)
            foreach ($bucket in @("extras", "versions")) {
                if ($bucketList -notmatch "(?m)^\s*$bucket\s+") {
                    & $scoop bucket add $bucket
                    if ($LASTEXITCODE -ne 0) { throw "Failed to add Scoop bucket: $bucket" }
                }
            }
        "##;
        let (bootstrap_ok, bootstrap_out) = run_powershell_cmd_timeout(scoop_cmd, 300);
        let (scoop_ok, scoop_version) = run_native_cmd_timeout("scoop", &["--version"], 30);
        if bootstrap_ok && scoop_ok {
            self.log(LogLevel::Ok, format!("Scoop 已就绪: {}", scoop_version.lines().next().unwrap_or("unknown")));
        } else {
            self.log(LogLevel::Error, format!("Scoop 启动失败: {}", if bootstrap_out.is_empty() { scoop_version } else { bootstrap_out }));
        }
    }

    fn step_environment_mirrors(&self) {
        let mirrors = &self.config.profile.environment_mirrors;
        self.log(LogLevel::Info, format!("配置显式加速镜像 -> Cargo: {}, Pip: {}, NPM: {}", mirrors.cargo_sparse_index, mirrors.pip_index_url, mirrors.npm_registry));

        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());

        // Cargo Config Mirror
        let cargo_dir = Path::new(&home_dir).join(".cargo");
        let cargo_config = format!(
            r#"[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "{}"
"#,
            mirrors.cargo_sparse_index
        );
        let cargo_file = cargo_dir.join("config.toml");
        match std::fs::create_dir_all(&cargo_dir).and_then(|_| std::fs::write(&cargo_file, cargo_config)) {
            Ok(_) => self.log(LogLevel::Ok, format!("Cargo 镜像已配置 -> {}", cargo_file.display())),
            Err(error) => self.log(LogLevel::Error, format!("Cargo 镜像配置失败: {error}")),
        }

        // Pip Mirror Config
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| format!("{}/AppData/Roaming", home_dir));
        let pip_dir = Path::new(&appdata).join("pip");
        let pip_ini = format!(
            r#"[global]
index-url = {}
trusted-host = pypi.tuna.tsinghua.edu.cn
"#,
            mirrors.pip_index_url
        );
        let pip_file = pip_dir.join("pip.ini");
        match std::fs::create_dir_all(&pip_dir).and_then(|_| std::fs::write(&pip_file, pip_ini)) {
            Ok(_) => self.log(LogLevel::Ok, format!("Pip 镜像已配置 -> {}", pip_file.display())),
            Err(error) => self.log(LogLevel::Error, format!("Pip 镜像配置失败: {error}")),
        }
    }

    fn step_uwp_apps(&self) {
        self.log(LogLevel::Info, "显式检查并修复 LTSC 原生 UWP 应用 (计算器 / 照片 / 画图 / 终端)...");
        let uwp_script = r##"
            $ErrorActionPreference = "Stop"
            $apps = @("Microsoft.WindowsCalculator", "Microsoft.Windows.Photos", "Microsoft.Paint", "Microsoft.ScreenSketch", "Microsoft.WindowsTerminal")
            foreach ($app in $apps) {
                if (-not (Get-AppxPackage -Name $app -ErrorAction SilentlyContinue)) {
                    $manifest = Get-ChildItem "$env:ProgramFiles\WindowsApps" -Filter "AppxManifest.xml" -Recurse -ErrorAction SilentlyContinue | Where-Object { $_.FullName -like "*$app*" } | Select-Object -First 1 -ExpandProperty FullName
                    if ($manifest) { Add-AppxPackage -DisableDevelopmentMode -Register $manifest -ErrorAction Stop }
                }
            }
            $missing = @($apps | Where-Object { -not (Get-AppxPackage -Name $_ -ErrorAction SilentlyContinue) })
            if ($missing.Count -gt 0) { throw ("仍缺少 UWP 应用: " + ($missing -join ", ")) }
        "##;
        let (ok, out) = run_powershell_cmd_timeout(uwp_script, 300);
        if ok {
            self.log(LogLevel::Ok, "LTSC 内置 UWP 软件恢复完成");
        } else {
            self.log(LogLevel::Error, format!("UWP 应用恢复失败: {}", last_line(&out)));
        }
    }

    fn step_docker_wsl(&self) {
        self.log(LogLevel::Info, "显式开启 Docker & WSL2 虚拟化内核组件 (VirtualMachinePlatform, WSL2)...");
        let docker_script = r##"
            $ErrorActionPreference = "Stop"
            Enable-WindowsOptionalFeature -Online -FeatureName VirtualMachinePlatform -All -NoRestart -ErrorAction Stop | Out-Null
            Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux -All -NoRestart -ErrorAction Stop | Out-Null
            wsl --set-default-version 2
            if ($LASTEXITCODE -ne 0) { throw "wsl exited with $LASTEXITCODE" }
        "##;
        let (ok, out) = run_powershell_cmd_timeout(docker_script, 300);
        if ok {
            self.log(LogLevel::Ok, "WSL2 平台与 VirtualMachinePlatform 开启成功");
        } else {
            self.log(LogLevel::Error, format!("WSL2 平台启用失败: {}", last_line(&out)));
        }
    }

    fn step_agent_skills(&self) {
        self.log(LogLevel::Info, "显式解压二进制内嵌的 55+ 真实 AI Agent Skills / Hooks / mcp_config 到 .gemini/config...");

        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());
        let target_base = Path::new(&home_dir).join(".gemini").join("config");

        match self.extract_embedded_assets(&ASSETS_DIR, &target_base) {
            Ok(count) => {
                self.log(LogLevel::Ok, format!("已成功显式释出 {} 个 Agent Skills 与规则文件到 {}", count, target_base.display()));
            }
            Err(e) => {
                self.log(LogLevel::Error, format!("Agent Skills 解压失败: {}", e));
            }
        }
    }

    fn extract_embedded_assets(&self, dir: &Dir, target_base: &Path) -> std::io::Result<usize> {
        let mut count = 0;
        for entry in dir.entries() {
            match entry {
                DirEntry::Dir(d) => {
                    let path = target_base.join(d.path());
                    std::fs::create_dir_all(&path)?;
                    count += self.extract_embedded_assets(d, target_base)?;
                }
                DirEntry::File(f) => {
                    let path = target_base.join(f.path());
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&path, f.contents())?;
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    fn step_git_shell_configs(&self) {
        let git_cfg = &self.config.profile.git_config;
        let ps_cfg = &self.config.profile.powershell_profile;
        self.log(LogLevel::Info, "[STEP] Git 全量配置 — 每项展示实际 key=value 与执行结果");

        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());
        let git_ignore_dir = std::path::Path::new(&home_dir).join(".config").join("git");
        let git_ignore_file = git_ignore_dir.join("ignore");
        match std::fs::create_dir_all(&git_ignore_dir) {
            Ok(_) => {
                let content = git_cfg.global_gitignore_rules.join("\n") + "\n";
                match std::fs::write(&git_ignore_file, &content) {
                    Ok(_) => self.log(LogLevel::Ok, format!("[FILE] 写入 {} ({} 条规则)", git_ignore_file.display(), git_cfg.global_gitignore_rules.len())),
                    Err(e) => self.log(LogLevel::Error, format!("[FILE] FAIL {} => {}", git_ignore_file.display(), e)),
                }
            }
            Err(e) => self.log(LogLevel::Error, format!("[FILE] 无法创建目录 {} => {}", git_ignore_dir.display(), e)),
        }

        self.git_config("user.name", &git_cfg.user_name);
        self.git_config("user.email", &git_cfg.user_email);
        self.git_config("http.postBuffer", &git_cfg.post_buffer_bytes.to_string());
        if !git_cfg.safe_directory.trim().is_empty() {
            self.git_config("safe.directory", &git_cfg.safe_directory);
        } else {
            let _ = run_native_cmd("git", &["config", "--global", "--unset-all", "safe.directory", r"^\*$"]);
            self.log(LogLevel::Ok, "[GIT] 已移除不安全的 safe.directory=* 通配配置");
        }
        self.git_config("core.longpaths", if git_cfg.enable_long_paths { "true" } else { "false" });
        self.git_config("core.autocrlf", if git_cfg.enable_autocrlf { "true" } else { "false" });
        self.git_config("core.excludesfile", "~/.config/git/ignore");
        self.git_config("filter.lfs.required", "true");

        // PowerShell Profile
        let mut alias_block = String::new();
        if ps_cfg.enable_utf8_encoding {
            alias_block.push_str("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8\n");
        }
        if ps_cfg.init_starship {
            alias_block.push_str("if (Get-Command starship -ErrorAction SilentlyContinue) { Invoke-Expression (&starship init powershell) }\n");
        }
        if ps_cfg.init_zoxide {
            alias_block.push_str("if (Get-Command zoxide -ErrorAction SilentlyContinue) { Invoke-Expression (& { (zoxide init powershell | Out-String) }) }\n");
        }
        for (k, v) in &ps_cfg.aliases {
            alias_block.push_str(&format!("Set-Alias -Name {} -Value {} -ErrorAction SilentlyContinue\n", k, v));
        }
        let ps_script = format!(
            r##"$ErrorActionPreference = "Stop"
$d = Join-Path $env:USERPROFILE 'Documents\PowerShell'
if (-not (Test-Path $d)) {{ New-Item -Path $d -ItemType Directory -Force | Out-Null }}
$f = Join-Path $d 'Microsoft.PowerShell_profile.ps1'
$start = '# >>> LTSC Tools managed >>>'
$end = '# <<< LTSC Tools managed <<<'
$managed = @'
# >>> LTSC Tools managed >>>
{}# <<< LTSC Tools managed <<<
'@
$content = if (Test-Path $f) {{ Get-Content $f -Raw }} else {{ '' }}
$startIndex = $content.IndexOf($start)
$endIndex = $content.IndexOf($end)
if ($startIndex -ge 0 -and $endIndex -gt $startIndex) {{
    $endIndex += $end.Length
    $content = $content.Substring(0, $startIndex).TrimEnd() + "`r`n`r`n" + $managed + $content.Substring($endIndex)
}} else {{
    $content = $content.TrimEnd() + "`r`n`r`n" + $managed
}}
Set-Content -Path $f -Value $content.TrimStart() -Encoding utf8
Write-Output "updated:$f"
"##,
            alias_block
        );
        let (ok, out) = run_powershell_cmd(&ps_script);
        if ok {
            self.log(LogLevel::Ok, format!("[FILE] PowerShell Profile: {}", out.trim()));
        } else {
            self.log(LogLevel::Error, format!("[FILE] PowerShell Profile FAIL: {}", out.trim()));
        }
    }

    fn git_config(&self, key: &str, value: &str) {
        let (ok, out) = run_native_cmd("git", &["config", "--global", key, value]);
        if ok {
            self.log(LogLevel::Ok, format!("[GIT] git config --global {} = {}", key, value));
        } else {
            self.log(LogLevel::Error, format!("[GIT] FAIL {} = {} => {}", key, value, out.trim()));
        }
    }

    fn step_vscode_and_tools_config(&self) {
        let vscode_cfg = &self.config.profile.vscode_config;
        let mirrors = &self.config.profile.environment_mirrors;

        self.log(LogLevel::Info, format!("显式部署 NPM 加速源: {} 与 {} 款 IDE 扩展", mirrors.npm_registry, vscode_cfg.extensions.len()));

        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());
        let npmrc_path = Path::new(&home_dir).join(".npmrc");
        let npmrc_content = format!("registry={}\nallow-scripts=@alibaba-group/open-code-review,context-mode,opencode-ai,better-sqlite3\n", mirrors.npm_registry);
        match std::fs::write(&npmrc_path, &npmrc_content) {
            Ok(_) => self.log(LogLevel::Ok, format!("[FILE] {} registry={}", npmrc_path.display(), mirrors.npm_registry)),
            Err(e) => self.log(LogLevel::Error, format!("[FILE] FAIL {} => {}", npmrc_path.display(), e)),
        }

        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| format!("{}/AppData/Roaming", home_dir));
        let settings_content = serde_json::to_string_pretty(&vscode_cfg.user_settings).unwrap_or_default();

        for (label, dir) in &[("Code", Path::new(&appdata).join("Code").join("User")), ("Cursor", Path::new(&appdata).join("Cursor").join("User"))] {
            let target = dir.join("settings.json");
            match std::fs::create_dir_all(dir).and_then(|_| std::fs::write(&target, &settings_content)) {
                Ok(_) => self.log(LogLevel::Ok, format!("[FILE] {} settings.json -> {}", label, target.display())),
                Err(e) => self.log(LogLevel::Error, format!("[FILE] FAIL {} settings.json => {}", label, e)),
            }
        }

        let total_exts = vscode_cfg.extensions.len();
        for (i, ext) in vscode_cfg.extensions.iter().enumerate() {
            self.log(LogLevel::Info, format!("[{}/{}] 显式安装 IDE 扩展: {}...", i + 1, total_exts, ext));
            let (ok1, _) = run_native_cmd_timeout("code", &["--install-extension", ext], 30);
            let (ok2, _) = run_native_cmd_timeout("cursor", &["--install-extension", ext], 30);
            if ok1 || ok2 {
                self.log(LogLevel::Ok, format!("IDE 扩展: {} [安装完成]", ext));
            } else {
                self.log(LogLevel::Error, format!("IDE 扩展: {} [安装失败]", ext));
            }
            self.progress(0.58 + (i as f32 / total_exts as f32) * 0.07);
        }
    }

    fn step_core_apps(&self) {
        let apps = &self.config.profile.packages.winget_core;
        let total = apps.len();
        self.log(LogLevel::Info, format!("显式批量部署 {} 款核心桌面软件 (Winget)...", total));
        for (i, app) in apps.iter().enumerate() {
            self.log(LogLevel::Info, format!("[{}/{}] 显式调用 Winget 安装: {} ({})", i + 1, total, app.name, app.id));
            self.install_winget_app(&app.id, &app.name);
            self.progress(0.65 + (i as f32 / total as f32) * 0.07);
        }
    }

    fn step_dev_suite(&self) {
        let pkgs = &self.config.profile.packages;
        self.log(LogLevel::Info, "显式开始部署全量 100+ 开发者 IDE 及 CLI 工具套件...");

        // 1. Dev Winget Apps
        let total_apps = pkgs.winget_dev.len();
        self.log(LogLevel::Info, format!("准备显式安装 {} 款开发桌面软件 (Winget)...", total_apps));
        for (i, app) in pkgs.winget_dev.iter().enumerate() {
            self.log(LogLevel::Info, format!("[Winget {}/{}] 正在安装: {} ({})", i + 1, total_apps, app.name, app.id));
            self.install_winget_app(&app.id, &app.name);
            self.progress(0.72 + (i as f32 / total_apps as f32) * 0.05);
        }

        // 2. Scoop Tools
        let total_scoop = pkgs.scoop_tools.len();
        self.log(LogLevel::Info, format!("准备显式安装 {} 款 CLI 工具 (Scoop)...", total_scoop));
        for (i, tool) in pkgs.scoop_tools.iter().enumerate() {
            self.log(LogLevel::Info, format!("[Scoop {}/{}] 正在安装: {}...", i + 1, total_scoop, tool));
            self.install_scoop_tool(tool);
            self.progress(0.77 + (i as f32 / total_scoop as f32) * 0.04);
        }

        // 3. Rust Cargo Packages
        let total_cargo = pkgs.cargo_packages.len();
        self.log(LogLevel::Info, format!("准备显式编译/安装 {} 款 Cargo 工具套件...", total_cargo));
        for (i, cargo_pkg) in pkgs.cargo_packages.iter().enumerate() {
            self.log(LogLevel::Info, format!("[Cargo {}/{}] 正在检查/编译: {}...", i + 1, total_cargo, cargo_pkg));
            self.install_cargo_package(cargo_pkg);
            self.progress(0.81 + (i as f32 / total_cargo as f32) * 0.03);
        }

        // 4. NPM Globals
        let total_npm = pkgs.npm_globals.len();
        self.log(LogLevel::Info, format!("准备显式安装 {} 款 NPM 全局包...", total_npm));
        for (i, npm_pkg) in pkgs.npm_globals.iter().enumerate() {
            self.log(LogLevel::Info, format!("[NPM {}/{}] 正在安装: {}...", i + 1, total_npm, npm_pkg));
            self.install_npm_global(npm_pkg);
            self.progress(0.84 + (i as f32 / total_npm as f32) * 0.02);
        }

        // 5. Pip & UV
        let total_pip = pkgs.pip_packages.len();
        self.log(LogLevel::Info, format!("准备显式安装 {} 款 Python 依赖与 UV 工具...", total_pip));
        for (i, pip_pkg) in pkgs.pip_packages.iter().enumerate() {
            self.log(LogLevel::Info, format!("[Pip {}/{}] 正在安装: {}...", i + 1, total_pip, pip_pkg));
            self.install_pip_package(pip_pkg);
            self.progress(0.86 + (i as f32 / total_pip as f32) * 0.01);
        }
        for uv_tool in &pkgs.uv_tools {
            self.install_uv_tool(uv_tool);
        }
    }

    fn step_ollama_models(&self) {
        let models = &self.config.profile.ollama_models;
        for m in models {
            self.log(LogLevel::Info, format!("显式预拉取本地 AI 大模型: ollama pull {}...", m));
            let (ok, _) = run_native_cmd_timeout("ollama", &["pull", m], 120);
            if ok {
                self.log(LogLevel::Ok, format!("本地 AI 模型 {} 已就绪", m));
            } else {
                self.log(LogLevel::Error, format!("Ollama 模型 {} 拉取失败", m));
            }
        }
    }

    fn step_deep_win_tweaks(&self) {
        let t = &self.config.profile.system_tweaks;
        self.log(LogLevel::Info, "[STEP] Windows 注册表与系统优化 — 每项独立显式执行");

        // 1. Ultimate Performance power plan
        if t.activate_ultimate_performance {
            let (ok, out) = run_powershell_cmd("powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 2>$null; $p = powercfg -l | Select-String 'Ultimate|卓越' | ForEach-Object { ($_ -split '\\s+')[3] }; if ($p) { powercfg -s $p; Write-Output \"activated:$p\" } else { Write-Output 'not_found' }");
            if ok {
                self.log(LogLevel::Ok, format!("[REG] 卓越性能电源方案: {}", out.trim()));
            } else {
                self.log(LogLevel::Warn, format!("[REG] 卓越性能电源方案 WARN: {}", out.trim()));
            }
        }

        // 2. Disable Telemetry
        if t.disable_telemetry {
            self.set_reg_dword("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry", 0);
        }

        // 3. Disable Bing search in Start
        if t.disable_bing_search {
            self.set_reg_dword("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search", "BingSearchEnabled", 0);
            self.set_reg_dword("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search", "DisableSearchBoxSuggestions", 1);
        }

        // 4. Explorer — open to This PC
        if t.explorer_open_to_this_pc {
            self.set_reg_dword("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "LaunchTo", 1);
        }

        // 5. Show file extensions
        if t.explorer_show_file_extensions {
            self.set_reg_dword("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "HideFileExt", 0);
        }

        // 6. Show hidden files
        if t.explorer_show_hidden_files {
            self.set_reg_dword("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Hidden", 1);
        }

        // 7. NTFS long paths
        if t.enable_ntfs_long_paths {
            self.set_reg_dword("HKLM", r"SYSTEM\CurrentControlSet\Control\FileSystem", "LongPathsEnabled", 1);
        }

        // 8. Dev mode unlock
        self.set_reg_dword("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock", "AllowDevelopmentWithoutDevLicense", 1);

        // 9. CPU responsiveness
        self.set_reg_dword("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile", "SystemResponsiveness", t.system_responsiveness);

        // 10. Network throttling off
        self.set_reg_dword("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile", "NetworkThrottlingIndex", t.network_throttling_index as u32);

        self.log(LogLevel::Ok, "[STEP] Windows 系统优化全部显式写入完成");
    }

    fn set_reg_dword(&self, hive: &str, path: &str, name: &str, value: u32) {
        let cmd = format!("New-Item -Path '{hive}:{path}' -Force -ErrorAction SilentlyContinue | Out-Null; Set-ItemProperty -Path '{hive}:{path}' -Name '{name}' -Value {value} -Type DWord -Force", hive = hive, path = path, name = name, value = value);
        let (ok, out) = run_powershell_cmd(&cmd);
        if ok {
            self.log(LogLevel::Ok, format!("[REG] {hive}:{path}\\{name} = {value}", hive = hive, path = path, name = name, value = value));
        } else {
            self.log(LogLevel::Error, format!("[REG] FAIL {hive}:{path}\\{name} = {value} => {}", out.trim()));
        }
    }

    fn step_audit(&self) {
        self.log(LogLevel::Info, "显式进行最终组件与命令行 CLI 审计...");
        let mut tools = vec!["winget", "scoop"];
        if self.config.include_dev_tools {
            tools.extend(["git", "python", "node", "cargo", "uv", "rtk", "pwsh"]);
        }
        if self.config.include_docker_wsl {
            tools.push("wsl");
        }
        let audit_script = format!(
            r##"
            $ErrorActionPreference = "Stop"
            $tools = @({})
            $found = @()
            $missing = @()
            foreach ($t in $tools) {{
                if (Get-Command $t -ErrorAction SilentlyContinue) {{ $found += $t }} else {{ $missing += $t }}
            }}
            Write-Output ("全套就绪指令: " + ($found -join ", "))
            if ($missing.Count -gt 0) {{ throw ("缺失指令: " + ($missing -join ", ")) }}
        "##,
            tools.iter().map(|tool| format!("\"{tool}\"")).collect::<Vec<_>>().join(", ")
        );
        let (ok, out) = run_powershell_cmd(&audit_script);
        if ok && !out.is_empty() {
            self.log(LogLevel::Ok, out);
        } else {
            self.log(LogLevel::Error, format!("最终组件审计失败: {}", out.trim()));
        }
    }

    fn install_winget_app(&self, id: &str, name: &str) {
        let (listed, output) = run_native_cmd_timeout("winget", &["list", "--id", id, "-e", "--accept-source-agreements", "--disable-interactivity"], 90);
        if listed && output.to_ascii_lowercase().contains(&id.to_ascii_lowercase()) {
            self.log(LogLevel::Ok, format!("[Winget] {} -> 已安装", name));
            return;
        }

        let args = ["install", "--id", id, "-e", "--silent", "--disable-interactivity", "--accept-package-agreements", "--accept-source-agreements"];
        self.log(LogLevel::Info, format!("[CMD] winget {}", args.join(" ")));
        let (ok, out) = self.run_package_command("Winget", "winget", &args, 900);
        if ok {
            self.log(LogLevel::Ok, format!("[Winget] {} -> OK", name));
        } else {
            self.log(LogLevel::Error, format!("[Winget] {} -> 失败 ({})", name, last_line(&out)));
        }
    }

    fn install_scoop_tool(&self, name: &str) {
        let (listed, output) = run_native_cmd_timeout("scoop", &["list", name], 60);
        if listed && package_is_listed(&output, name) {
            self.log(LogLevel::Ok, format!("Scoop 工具: {} [已安装]", name));
            return;
        }
        let (ok, out) = self.run_package_command("Scoop", "scoop", &["install", name], 600);
        if ok {
            self.log(LogLevel::Ok, format!("Scoop 工具: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Error, format!("Scoop 工具: {} [失败: {}]", name, last_line(&out)));
        }
    }

    fn install_cargo_package(&self, name: &str) {
        let (listed, output) = run_native_cmd_timeout("cargo", &["install", "--list"], 60);
        if listed && package_is_listed(&output, name) {
            self.log(LogLevel::Ok, format!("Cargo 包: {} [已安装]", name));
            return;
        }
        let (ok, out) = self.run_package_command("Cargo", "cargo", &["install", name], 1_800);
        if ok {
            self.log(LogLevel::Ok, format!("Cargo 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Error, format!("Cargo 包: {} [失败: {}]", name, last_line(&out)));
        }
    }

    fn install_npm_global(&self, name: &str) {
        if run_native_cmd_timeout("npm", &["list", "-g", name, "--depth=0"], 60).0 {
            self.log(LogLevel::Ok, format!("NPM 包: {} [已安装]", name));
            return;
        }
        let (ok, out) = self.run_package_command("NPM", "npm", &["install", "-g", name, "--loglevel=error"], 600);
        if ok {
            self.log(LogLevel::Ok, format!("NPM 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Error, format!("NPM 包: {} [失败: {}]", name, last_line(&out)));
        }
    }

    fn install_pip_package(&self, name: &str) {
        if run_native_cmd_timeout("python", &["-m", "pip", "show", name], 60).0 {
            self.log(LogLevel::Ok, format!("Pip 包: {} [已安装]", name));
            return;
        }
        let (ok, out) = self.run_package_command("Pip", "python", &["-m", "pip", "install", name, "--quiet"], 900);
        if ok {
            self.log(LogLevel::Ok, format!("Pip 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Error, format!("Pip 包: {} [失败: {}]", name, last_line(&out)));
        }
    }

    fn install_uv_tool(&self, name: &str) {
        let (listed, output) = run_native_cmd_timeout("uv", &["tool", "list"], 60);
        if listed && package_is_listed(&output, name) {
            self.log(LogLevel::Ok, format!("UV 工具: {} [已安装]", name));
            return;
        }
        let (ok, out) = self.run_package_command("UV", "uv", &["tool", "install", name], 600);
        if ok {
            self.log(LogLevel::Ok, format!("UV 工具: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Error, format!("UV 工具: {} [失败: {}]", name, last_line(&out)));
        }
    }

    fn run_package_command(&self, label: &str, program: &str, args: &[&str], timeout: u64) -> (bool, String) {
        let first = run_native_cmd_timeout(program, args, timeout);
        if first.0 {
            return first;
        }
        self.log(LogLevel::Warn, format!("{label} 首次执行失败，自动重试一次: {}", last_line(&first.1)));
        run_native_cmd_timeout(program, args, timeout)
    }
}

fn package_is_listed(output: &str, name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    output.lines().any(|line| {
        let line = line.trim_start().to_ascii_lowercase();
        line.strip_prefix(&name).and_then(|rest| rest.chars().next()).is_some_and(char::is_whitespace)
    })
}

fn last_line(output: &str) -> &str {
    output.trim().lines().last().unwrap_or("无诊断输出")
}

#[cfg(test)]
mod tests {
    use super::package_is_listed;

    #[test]
    fn package_list_match_is_exact() {
        let output = "git-filter-repo 2.47\ngit 2.53\nkimi-cli v1.49";
        assert!(package_is_listed(output, "git"));
        assert!(package_is_listed(output, "kimi-cli"));
        assert!(!package_is_listed(output, "kimi"));
    }
}
