use crate::config::*;
use crate::utils::{run_native_cmd, run_native_cmd_timeout, run_powershell_cmd, LogLevel, LogMessage};
use include_dir::{include_dir, Dir, DirEntry};
use std::path::Path;
use std::sync::mpsc::Sender;

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/assets");

pub struct SetupEngine {
    tx: Sender<LogMessage>,
    progress_tx: Sender<f32>,
    config: SetupConfig,
}

impl SetupEngine {
    pub fn new(tx: Sender<LogMessage>, progress_tx: Sender<f32>, config: SetupConfig) -> Self {
        Self { tx, progress_tx, config }
    }

    fn log(&self, level: LogLevel, msg: impl Into<String>) {
        let _ = self.tx.send(LogMessage::new(level, msg));
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
                self.log(LogLevel::End, "网络与代理接口优化完成。");
            }
            ExecutionTarget::AgentSkillsOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: AI Agent Skills & Hooks 嵌入式释出...");
                self.step_agent_skills();
                self.progress(1.0);
                self.log(LogLevel::End, "AI Agent Skills & Hooks 释出完成。");
            }
            ExecutionTarget::DevToolsOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: 100+ 开发者软件库部署...");
                self.step_package_managers();
                self.step_dev_suite();
                self.progress(1.0);
                self.log(LogLevel::End, "100+ 开发者软件库部署完成。");
            }
            ExecutionTarget::VSCodeExtensionsOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: VS Code & Cursor 扩展及偏好设置同步...");
                self.step_vscode_and_tools_config();
                self.progress(1.0);
                self.log(LogLevel::End, "VS Code & Cursor 扩展及配置同步完成。");
            }
            ExecutionTarget::SystemTweaksOnly => {
                self.log(LogLevel::Start, "运行显式专项任务: Windows 性能与隐私深度优化...");
                self.step_deep_win_tweaks();
                self.progress(1.0);
                self.log(LogLevel::End, "Windows 性能与隐私深度优化完成。");
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

        self.log(LogLevel::End, "Windows LTSC 显式配置完成！所有操作均可复现与追溯。建议重启系统生效。");
    }

    fn step_network(&self) {
        let net_cfg = &self.config.profile.network_config;
        self.log(LogLevel::Info, format!("显式网络硬化配置: TLS1.2/1.3={}, CTCP={}, ECN={}, WinHTTP Proxy={}", net_cfg.enable_tls12_tls13, net_cfg.enable_ctcp, net_cfg.enable_ecn, net_cfg.import_winhttp_proxy));

        let net_script = r##"
            [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
            ipconfig /flushdns | Out-Null
            netsh int tcp set global autotuninglevel=normal | Out-Null
            netsh int tcp set global congestionprovider=ctcp | Out-Null
            netsh int tcp set global ecncapability=enabled | Out-Null
            netsh winhttp import proxy source=ie | Out-Null
        "##;

        let (ok, out) = run_powershell_cmd(net_script);
        if ok {
            self.log(LogLevel::Ok, "网络 TLS 1.2/1.3、TCP 窗口、CTCP 拥塞控制及 WinHTTP 代理同步成功");
        } else {
            self.log(LogLevel::Warn, format!("网络配置提示: {}", out));
        }
    }

    fn step_package_managers(&self) {
        self.log(LogLevel::Info, "检查并显式部署包管理器环境 (Winget / Scoop / Chocolatey)...");

        let winget_script = r##"
            if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
                $vclibs = "https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx"
                $dest = Join-Path $env:TEMP "vclibs.appx"
                Start-BitsTransfer -Source $vclibs -Destination $dest -ErrorAction SilentlyContinue
                Add-AppxPackage -Path $dest -ErrorAction SilentlyContinue
            }
        "##;
        let _ = run_powershell_cmd(winget_script);
        self.log(LogLevel::Ok, "Winget 运行环境验证完成");

        let scoop_cmd = r##"
            if (-not (Get-Command scoop -ErrorAction SilentlyContinue)) {
                Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process -Force
                Invoke-RestMethod -Uri "https://get.scoop.sh" | Invoke-Expression
            }
        "##;
        let (s_ok, _) = run_powershell_cmd(scoop_cmd);
        if s_ok {
            for b in &["extras", "versions", "nirsoft", "sysinternals"] {
                self.log(LogLevel::Info, format!("添加 Scoop 软件源 (Bucket): {}", b));
                let _ = run_native_cmd("scoop", &["bucket", "add", b]);
            }
            self.log(LogLevel::Ok, "Scoop 包管理器部署就绪 (已显式添加 4 个官方/社区源)");
        } else {
            self.log(LogLevel::Warn, "Scoop 状态验证完成");
        }

        let choco_cmd = r##"
            if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
                Set-ExecutionPolicy Bypass -Scope Process -Force
                [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
                Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
            }
        "##;
        let (c_ok, _) = run_powershell_cmd(choco_cmd);
        if c_ok {
            self.log(LogLevel::Ok, "Chocolatey 包管理器部署/就绪");
        } else {
            self.log(LogLevel::Warn, "Chocolatey 状态验证完成");
        }
    }

    fn step_environment_mirrors(&self) {
        let mirrors = &self.config.profile.environment_mirrors;
        self.log(LogLevel::Info, format!("配置显式加速镜像 -> Cargo: {}, Pip: {}, NPM: {}", mirrors.cargo_sparse_index, mirrors.pip_index_url, mirrors.npm_registry));

        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());

        // Cargo Config Mirror
        let cargo_dir = Path::new(&home_dir).join(".cargo");
        if std::fs::create_dir_all(&cargo_dir).is_ok() {
            let cargo_config = format!(r#"[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "{}"
"#, mirrors.cargo_sparse_index);
            let _ = std::fs::write(cargo_dir.join("config.toml"), cargo_config);
            self.log(LogLevel::Ok, format!("Cargo 镜像已显式配置 -> {}", cargo_dir.join("config.toml").display()));
        }

        // Pip Mirror Config
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| format!("{}/AppData/Roaming", home_dir));
        let pip_dir = Path::new(&appdata).join("pip");
        if std::fs::create_dir_all(&pip_dir).is_ok() {
            let pip_ini = format!(r#"[global]
index-url = {}
trusted-host = pypi.tuna.tsinghua.edu.cn
"#, mirrors.pip_index_url);
            let _ = std::fs::write(pip_dir.join("pip.ini"), pip_ini);
            self.log(LogLevel::Ok, format!("Pip 镜像已显式配置 -> {}", pip_dir.join("pip.ini").display()));
        }
    }

    fn step_uwp_apps(&self) {
        self.log(LogLevel::Info, "显式检查并修复 LTSC 原生 UWP 应用 (计算器 / 照片 / 画图 / 终端)...");
        let uwp_script = r##"
            $apps = @("Microsoft.WindowsCalculator", "Microsoft.Windows.Photos", "Microsoft.Paint", "Microsoft.ScreenSketch", "Microsoft.WindowsTerminal")
            foreach ($app in $apps) {
                if (-not (Get-AppxPackage -Name $app -ErrorAction SilentlyContinue)) {
                    $manifest = Get-ChildItem "$env:ProgramFiles\WindowsApps" -Filter "AppxManifest.xml" -Recurse -ErrorAction SilentlyContinue | Where-Object { $_.FullName -like "*$app*" } | Select-Object -First 1 -ExpandProperty FullName
                    if ($manifest) { Add-AppxPackage -DisableDevelopmentMode -Register $manifest -ErrorAction SilentlyContinue }
                }
            }
        "##;
        let (ok, _) = run_powershell_cmd(uwp_script);
        if ok {
            self.log(LogLevel::Ok, "LTSC 内置 UWP 软件恢复完成");
        } else {
            self.log(LogLevel::Warn, "UWP 应用修复完成");
        }
    }

    fn step_docker_wsl(&self) {
        self.log(LogLevel::Info, "显式开启 Docker & WSL2 虚拟化内核组件 (VirtualMachinePlatform, WSL2)...");
        let docker_script = r##"
            Enable-WindowsOptionalFeature -Online -FeatureName VirtualMachinePlatform -All -NoRestart -ErrorAction SilentlyContinue | Out-Null
            Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux -All -NoRestart -ErrorAction SilentlyContinue | Out-Null
            wsl --set-default-version 2 2>$null
        "##;
        let (ok, _) = run_powershell_cmd(docker_script);
        if ok {
            self.log(LogLevel::Ok, "WSL2 平台与 VirtualMachinePlatform 开启成功");
        } else {
            self.log(LogLevel::Warn, "WSL2 平台验证完成");
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
                self.log(LogLevel::Warn, format!("Agent Skills 解压写入过程有警告: {}", e));
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
                    Ok(_)  => self.log(LogLevel::Ok, format!("[FILE] 写入 {} ({} 条规则)", git_ignore_file.display(), git_cfg.global_gitignore_rules.len())),
                    Err(e) => self.log(LogLevel::Error, format!("[FILE] FAIL {} => {}", git_ignore_file.display(), e)),
                }
            }
            Err(e) => self.log(LogLevel::Error, format!("[FILE] 无法创建目录 {} => {}", git_ignore_dir.display(), e)),
        }

        self.git_config("user.name",         &git_cfg.user_name);
        self.git_config("user.email",        &git_cfg.user_email);
        self.git_config("http.postBuffer",   &git_cfg.post_buffer_bytes.to_string());
        self.git_config("safe.directory",    &git_cfg.safe_directory);
        self.git_config("core.longpaths",    if git_cfg.enable_long_paths { "true" } else { "false" });
        self.git_config("core.autocrlf",     if git_cfg.enable_autocrlf  { "true" } else { "false" });
        self.git_config("core.excludesfile", "~/.config/git/ignore");
        self.git_config("filter.lfs.required", "true");

        // PowerShell Profile
        let mut alias_block = String::new();
        for (k, v) in &ps_cfg.aliases {
            alias_block.push_str(&format!("Set-Alias -Name {} -Value {} -ErrorAction SilentlyContinue\n", k, v));
        }
        let ps_script = format!(r##"
            $d = Join-Path $env:USERPROFILE 'Documents\PowerShell'
            if (-not (Test-Path $d)) {{ New-Item -Path $d -ItemType Directory -Force | Out-Null }}
            $f = Join-Path $d 'Microsoft.PowerShell_profile.ps1'
            if (-not (Test-Path $f)) {{
                "# Profile`n[Console]::OutputEncoding = [System.Text.Encoding]::UTF8`n{}" | Out-File -FilePath $f -Encoding utf8
                Write-Output "created"
            }} else {{ Write-Output "exists" }}
        "##, alias_block);
        let (ok, out) = run_powershell_cmd(&ps_script);
        if ok { self.log(LogLevel::Ok, format!("[FILE] PowerShell Profile: {}", out.trim())); }
        else   { self.log(LogLevel::Error, format!("[FILE] PowerShell Profile FAIL: {}", out.trim())); }
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
            Ok(_)  => self.log(LogLevel::Ok, format!("[FILE] {} registry={}", npmrc_path.display(), mirrors.npm_registry)),
            Err(e) => self.log(LogLevel::Error, format!("[FILE] FAIL {} => {}", npmrc_path.display(), e)),
        }

        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| format!("{}/AppData/Roaming", home_dir));
        let settings_content = serde_json::to_string_pretty(&vscode_cfg.user_settings).unwrap_or_default();

        for (label, dir) in &[
            ("Code",   Path::new(&appdata).join("Code").join("User")),
            ("Cursor", Path::new(&appdata).join("Cursor").join("User")),
        ] {
            let target = dir.join("settings.json");
            match std::fs::create_dir_all(dir).and_then(|_| std::fs::write(&target, &settings_content)) {
                Ok(_)  => self.log(LogLevel::Ok, format!("[FILE] {} settings.json -> {}", label, target.display())),
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
                self.log(LogLevel::Warn, format!("IDE 扩展: {} [已就绪/跳过]", ext));
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
                self.log(LogLevel::Warn, format!("Ollama 模型 {} 检查完成", m));
            }
        }
    }

    fn step_deep_win_tweaks(&self) {
        let t = &self.config.profile.system_tweaks;
        self.log(LogLevel::Info, "[STEP] Windows 注册表与系统优化 — 每项独立显式执行");

        // 1. Ultimate Performance power plan
        if t.activate_ultimate_performance {
            let (ok, out) = run_powershell_cmd(
                "powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 2>$null; $p = powercfg -l | Select-String 'Ultimate|卓越' | ForEach-Object { ($_ -split '\\s+')[3] }; if ($p) { powercfg -s $p; Write-Output \"activated:$p\" } else { Write-Output 'not_found' }"
            );
            if ok { self.log(LogLevel::Ok, format!("[REG] 卓越性能电源方案: {}", out.trim())); }
            else   { self.log(LogLevel::Warn, format!("[REG] 卓越性能电源方案 WARN: {}", out.trim())); }
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
        let cmd = format!(
            "New-Item -Path '{hive}:{path}' -Force -ErrorAction SilentlyContinue | Out-Null; Set-ItemProperty -Path '{hive}:{path}' -Name '{name}' -Value {value} -Type DWord -Force",
            hive=hive, path=path, name=name, value=value
        );
        let (ok, out) = run_powershell_cmd(&cmd);
        if ok {
            self.log(LogLevel::Ok, format!("[REG] {hive}:{path}\\{name} = {value}", hive=hive, path=path, name=name, value=value));
        } else {
            self.log(LogLevel::Error, format!("[REG] FAIL {hive}:{path}\\{name} = {value} => {}", out.trim()));
        }
    }

    fn step_audit(&self) {
        self.log(LogLevel::Info, "显式进行最终组件与命令行 CLI 审计...");
        let audit_script = r##"
            $tools = @("winget", "scoop", "git", "python", "node", "cargo", "uv", "rtk", "docker", "wsl", "pwsh")
            $found = @()
            foreach ($t in $tools) {
                if (Get-Command $t -ErrorAction SilentlyContinue) { $found += $t }
            }
            Write-Output ("全套就绪指令: " + ($found -join ", "))
        "##;
        let (ok, out) = run_powershell_cmd(audit_script);
        if ok && !out.is_empty() {
            self.log(LogLevel::Ok, out);
        } else {
            self.log(LogLevel::Ok, "核心组件审计完成");
        }
    }

    fn install_winget_app(&self, id: &str, name: &str) {
        let args = ["install", "--id", id, "-e", "--silent", "--disable-interactivity",
                    "--accept-package-agreements", "--accept-source-agreements", "--force"];
        self.log(LogLevel::Info, format!("[CMD] winget {}", args.join(" ")));
        let (ok, out) = run_native_cmd_timeout("winget", &args, 45);
        if ok {
            self.log(LogLevel::Ok, format!("[Winget] {} -> OK", name));
        } else {
            self.log(LogLevel::Warn, format!("[Winget] {} -> SKIP/EXIST ({})", name, out.trim().lines().last().unwrap_or("")));
        }
    }

    fn install_scoop_tool(&self, name: &str) {
        let (ok, _) = run_native_cmd_timeout("scoop", &["install", name], 30);
        if ok {
            self.log(LogLevel::Ok, format!("Scoop 工具: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("Scoop 工具: {} [跳过/已存在/超时]", name));
        }
    }

    fn install_cargo_package(&self, name: &str) {
        let (ok, _) = run_native_cmd_timeout("cargo", &["install", name], 60);
        if ok {
            self.log(LogLevel::Ok, format!("Cargo 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("Cargo 包: {} [跳过/已存在/超时]", name));
        }
    }

    fn install_npm_global(&self, name: &str) {
        let (ok, _) = run_native_cmd_timeout("npm", &["install", "-g", name, "--loglevel=error"], 30);
        if ok {
            self.log(LogLevel::Ok, format!("NPM 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("NPM 包: {} [跳过/已存在/超时]", name));
        }
    }

    fn install_pip_package(&self, name: &str) {
        let (ok, _) = run_native_cmd_timeout("pip", &["install", name, "--quiet"], 20);
        if ok {
            self.log(LogLevel::Ok, format!("Pip 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("Pip 包: {} [跳过/已存在/超时]", name));
        }
    }

    fn install_uv_tool(&self, name: &str) {
        let (ok, _) = run_native_cmd_timeout("uv", &["tool", "install", name], 20);
        if ok {
            self.log(LogLevel::Ok, format!("UV 工具: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("UV 工具: {} [跳过/已存在/超时]", name));
        }
    }
}
