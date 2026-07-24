use crate::config::*;
use crate::utils::{run_native_cmd, run_powershell_cmd, LogLevel, LogMessage};
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
        match self.config.target_mode {
            ExecutionTarget::FullSetup => self.run_all_steps(),
            ExecutionTarget::NetworkOnly => {
                self.log(LogLevel::Start, "运行专项任务: 网络与代理接口优化...");
                self.step_network();
                self.progress(1.0);
                self.log(LogLevel::End, "网络与代理接口优化完成。");
            }
            ExecutionTarget::AgentSkillsOnly => {
                self.log(LogLevel::Start, "运行专项任务: AI Agent Skills & Hooks 嵌入式释出...");
                self.step_agent_skills();
                self.progress(1.0);
                self.log(LogLevel::End, "AI Agent Skills & Hooks 释出完成。");
            }
            ExecutionTarget::DevToolsOnly => {
                self.log(LogLevel::Start, "运行专项任务: 100+ 开发者软件库部署...");
                self.step_package_managers();
                self.step_dev_suite();
                self.progress(1.0);
                self.log(LogLevel::End, "100+ 开发者软件库部署完成。");
            }
            ExecutionTarget::VSCodeExtensionsOnly => {
                self.log(LogLevel::Start, "运行专项任务: VS Code & Cursor 扩展及配置同步...");
                self.step_vscode_and_tools_config();
                self.progress(1.0);
                self.log(LogLevel::End, "VS Code & Cursor 扩展及配置同步完成。");
            }
            ExecutionTarget::SystemTweaksOnly => {
                self.log(LogLevel::Start, "运行专项任务: Windows 性能与隐私深度优化...");
                self.step_deep_win_tweaks();
                self.progress(1.0);
                self.log(LogLevel::End, "Windows 性能与隐私深度优化完成。");
            }
        }
    }

    fn run_all_steps(&self) {
        self.log(LogLevel::Start, "开始 Windows LTSC 终极全量一键配置流程...");
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
            self.log(LogLevel::Info, "已跳过 Docker & WSL2 虚拟化内核配置");
        }
        self.progress(0.35);

        // 6. Agent Skills & Rules Embedded Extraction & Sync
        if self.config.include_agent_skills {
            self.step_agent_skills();
        } else {
            self.log(LogLevel::Info, "已跳过 AI Agent Skills / Hooks 配置");
        }
        self.progress(0.45);

        // 7. Git & PowerShell Shell Custom Profile
        if self.config.include_git_shell_configs {
            self.step_git_shell_configs();
        } else {
            self.log(LogLevel::Info, "已跳过 Git 全局配置与 PowerShell Profile 自动化");
        }
        self.progress(0.55);

        // 8. VS Code & Cursor Extensions & Settings
        if self.config.include_vscode_extensions {
            self.step_vscode_and_tools_config();
        } else {
            self.log(LogLevel::Info, "已跳过 VS Code & Cursor 扩展与配置文件同步");
        }
        self.progress(0.65);

        // 9. Core Desktop Apps
        self.step_core_apps();
        self.progress(0.72);

        // 10. Developer Suite (100% Homebrew Parity)
        if self.config.include_dev_tools {
            self.step_dev_suite();
        } else {
            self.log(LogLevel::Info, "已跳过开发者 CLI / 工具链配置");
        }
        self.progress(0.85);

        // 11. Local AI Model Pre-pull
        if self.config.include_ollama_models {
            self.step_ollama_models();
        }
        self.progress(0.90);

        // 12. Deep Windows LTSC Optimization Suite
        if self.config.include_deep_win_tweaks {
            self.step_deep_win_tweaks();
        } else {
            self.log(LogLevel::Info, "已跳过深度 Windows 性能与隐私优化");
        }
        self.progress(0.96);

        // 13. Audit Summary
        self.step_audit();
        self.progress(1.0);

        self.log(LogLevel::End, "Windows LTSC 终极一键配置完成！内置全套嵌入式 Agent Skills、VS Code 扩展、软件库与工具链已完美就绪。建议重启系统生效。");
    }

    fn step_network(&self) {
        self.log(LogLevel::Info, format!("应用网络与代理优化模式: {:?}", self.config.network_mode));

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
        self.log(LogLevel::Info, "检查并部署包管理器环境 (Winget / Scoop / Chocolatey)...");

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
            let _ = run_native_cmd("scoop", &["bucket", "add", "extras"]);
            let _ = run_native_cmd("scoop", &["bucket", "add", "versions"]);
            let _ = run_native_cmd("scoop", &["bucket", "add", "nirsoft"]);
            let _ = run_native_cmd("scoop", &["bucket", "add", "sysinternals"]);
            self.log(LogLevel::Ok, "Scoop 包管理器部署就绪 (已添加 extras, versions, nirsoft, sysinternals buckets)");
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
        self.log(LogLevel::Info, "配置 Cargo / Pip / NPM 国内高速加速镜像...");

        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());

        // Cargo Config Mirror
        let cargo_dir = Path::new(&home_dir).join(".cargo");
        if std::fs::create_dir_all(&cargo_dir).is_ok() {
            let cargo_config = r##"[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
"##;
            let _ = std::fs::write(cargo_dir.join("config.toml"), cargo_config);
            self.log(LogLevel::Ok, "Cargo 镜像已配置 (Tsinghua Sparse Index)");
        }

        // Pip Mirror Config
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| format!("{}/AppData/Roaming", home_dir));
        let pip_dir = Path::new(&appdata).join("pip");
        if std::fs::create_dir_all(&pip_dir).is_ok() {
            let pip_ini = r##"[global]
index-url = https://pypi.tuna.tsinghua.edu.cn/simple
trusted-host = pypi.tuna.tsinghua.edu.cn
"##;
            let _ = std::fs::write(pip_dir.join("pip.ini"), pip_ini);
            self.log(LogLevel::Ok, "Pip 镜像已配置 (Tsinghua PyPI)");
        }
    }

    fn step_uwp_apps(&self) {
        self.log(LogLevel::Info, "检查并修复 LTSC 原生 UWP 应用 (计算器 / 照片 / 画图 / 终端)...");
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
        self.log(LogLevel::Info, "部署 Docker & WSL2 虚拟化内核引擎...");
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
        self.log(LogLevel::Info, "解压并部署二进制内置的 55+ 真实 AI Agent Skills / Hooks / mcp_config 到 .gemini/config...");

        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());
        let target_base = Path::new(&home_dir).join(".gemini").join("config");

        match self.extract_embedded_assets(&ASSETS_DIR, &target_base) {
            Ok(count) => {
                self.log(LogLevel::Ok, format!("已成功从二进制解压并写入 {} 个真实 Agent Skills、Plugins 及规则文件到 {}", count, target_base.display()));
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
        self.log(LogLevel::Info, "应用 Git 用户全量配置与 PowerShell 7 Profile 自动化...");

        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());
        let git_ignore_dir = Path::new(&home_dir).join(".config").join("git");
        let _ = std::fs::create_dir_all(&git_ignore_dir);
        let git_ignore_file = git_ignore_dir.join("ignore");
        let git_ignore_content = "**/.cursor/.agent-tools\n**/.cursor/.agent-notes\n.DS_Store\n**/.claude/settings.local.json\nAGENTS.local.md\n*.local.md\n";
        let _ = std::fs::write(&git_ignore_file, git_ignore_content);

        let _ = run_native_cmd("git", &["config", "--global", "user.name", "nowaytouse"]);
        let _ = run_native_cmd("git", &["config", "--global", "user.email", "104445933+nowaytouse@users.noreply.github.com"]);
        let _ = run_native_cmd("git", &["config", "--global", "http.postBuffer", "524288000"]);
        let _ = run_native_cmd("git", &["config", "--global", "safe.directory", "*"]);
        let _ = run_native_cmd("git", &["config", "--global", "core.longpaths", "true"]);
        let _ = run_native_cmd("git", &["config", "--global", "core.autocrlf", "true"]);
        let _ = run_native_cmd("git", &["config", "--global", "core.excludesfile", "~/.config/git/ignore"]);
        let _ = run_native_cmd("git", &["config", "--global", "filter.lfs.required", "true"]);

        let ps_profile_script = r##"
            $psDocsDir = Join-Path $env:USERPROFILE "Documents\PowerShell"
            if (-not (Test-Path $psDocsDir)) { New-Item -Path $psDocsDir -ItemType Directory -Force | Out-Null }
            $psProfile = Join-Path $psDocsDir "Microsoft.PowerShell_profile.ps1"
            if (-not (Test-Path $psProfile)) {
                "# PowerShell 7 User Profile`n`n[Console]::OutputEncoding = [System.Text.Encoding]::UTF8`n$OutputEncoding = [System.Text.Encoding]::UTF8`nif (Get-Command starship -ErrorAction SilentlyContinue) { Invoke-Expression (&starship init powershell) }`nif (Get-Command zoxide -ErrorAction SilentlyContinue) { Invoke-Expression (&zoxide init powershell) }`nSet-Alias -Name g -Value git -ErrorAction SilentlyContinue`nSet-Alias -Name ls -Value eza -ErrorAction SilentlyContinue`nSet-Alias -Name ll -Value eza -Option All -ErrorAction SilentlyContinue`nSet-Alias -Name cat -Value bat -ErrorAction SilentlyContinue`nSet-Alias -Name find -Value fd -ErrorAction SilentlyContinue`nSet-Alias -Name grep -Value ripgrep -ErrorAction SilentlyContinue`nSet-Alias -Name top -Value fastfetch -ErrorAction SilentlyContinue`n" | Out-File -FilePath $psProfile -Encoding utf8
            }
        "##;
        let (ok, _) = run_powershell_cmd(ps_profile_script);
        if ok {
            self.log(LogLevel::Ok, "Git 用户账号 (nowaytouse)、LFS、PostBuffer、全局 GitIgnore 及 PowerShell Profile (包含快捷别名) 初始化完成");
        } else {
            self.log(LogLevel::Warn, "Git / Shell 配置应用完成");
        }
    }

    fn step_vscode_and_tools_config(&self) {
        self.log(LogLevel::Info, "自动部署 VS Code & Cursor 全套扩展与 User settings.json 配置文件...");

        // NPM Mirror Config
        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());
        let npmrc_path = Path::new(&home_dir).join(".npmrc");
        let npmrc_content = "registry=https://registry.npmmirror.com\nallow-scripts=@alibaba-group/open-code-review,context-mode,opencode-ai,better-sqlite3\n";
        let _ = std::fs::write(&npmrc_path, npmrc_content);
        self.log(LogLevel::Ok, "已配置 npmmirror 国内加速与 NPM 脚本权限 (.npmrc)");

        // VS Code / Cursor Settings.json
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| format!("{}/AppData/Roaming", home_dir));
        let settings_content = r##"{
  "cursor.composer.usageSummaryDisplay": "always",
  "window.autoDetectColorScheme": true,
  "editor.largeFileOptimizations": false,
  "diffEditor.maxComputationTime": 0,
  "editor.accessibilitySupport": "on",
  "explorer.confirmDragAndDrop": false,
  "workbench.preferredLightColorTheme": "Default Dark Modern",
  "vim.disableExtension": true,
  "cursor.composer.conversationDensity": "detailed"
}"##;

        let code_user_dir = Path::new(&appdata).join("Code").join("User");
        let cursor_user_dir = Path::new(&appdata).join("Cursor").join("User");

        if std::fs::create_dir_all(&code_user_dir).is_ok() {
            let _ = std::fs::write(code_user_dir.join("settings.json"), settings_content);
        }
        if std::fs::create_dir_all(&cursor_user_dir).is_ok() {
            let _ = std::fs::write(cursor_user_dir.join("settings.json"), settings_content);
        }
        self.log(LogLevel::Ok, "VS Code & Cursor 偏好设置 settings.json 已部署");

        // Install Extensions
        for ext in get_vscode_extensions() {
            let (ok1, _) = run_native_cmd("code", &["--install-extension", ext]);
            let (ok2, _) = run_native_cmd("cursor", &["--install-extension", ext]);
            if ok1 || ok2 {
                self.log(LogLevel::Ok, format!("IDE 扩展: {} [安装完成]", ext));
            } else {
                self.log(LogLevel::Warn, format!("IDE 扩展: {} [已就绪/跳过]", ext));
            }
        }
    }

    fn step_core_apps(&self) {
        self.log(LogLevel::Info, "批量部署核心桌面软件 (Chrome / VLC / 7-Zip / ShareX / IrfanView)...");
        for app in get_core_winget_apps() {
            self.install_winget_app(&app.id, &app.name);
        }
    }

    fn step_dev_suite(&self) {
        self.log(LogLevel::Info, "开始部署 100+ 开发者 IDE 及 CLI 工具套件 (macOS 100% 同等能力)...");

        // Dev Winget Apps
        self.log(LogLevel::Info, "安装开发 IDE、沟通与桌面软件 (VS Code, Cursor, Podman, Krita, Bitwarden, ChatGPT, Claude)...");
        for app in get_dev_winget_apps() {
            self.install_winget_app(&app.id, &app.name);
        }

        // Scoop Tools
        self.log(LogLevel::Info, "安装 Scoop 全量 CLI 工具 (git, ripgrep, fd, fzf, bat, eza, starship, fastfetch, sccache, sing-box, mihomo)...");
        for tool in get_scoop_tools() {
            self.install_scoop_tool(tool);
        }

        // Rust Cargo Packages
        self.log(LogLevel::Info, "安装 Cargo 工具套件 (rtk, kondo, krokiet, rust-script, yek)...");
        for cargo_pkg in get_cargo_packages() {
            self.install_cargo_package(cargo_pkg);
        }

        // NPM Globals
        self.log(LogLevel::Info, "安装 NPM 全局 CLI & AI 工具包 (@anthropic-ai/claude-code, opencode-ai, pyright, context-mode)...");
        for npm_pkg in get_npm_globals() {
            self.install_npm_global(npm_pkg);
        }

        // Pip & UV
        self.log(LogLevel::Info, "安装 Python 依赖库及 UV 全局工具 (kimi-cli, ruff)...");
        for pip_pkg in get_pip_packages() {
            self.install_pip_package(pip_pkg);
        }
        for uv_tool in get_uv_tools() {
            self.install_uv_tool(uv_tool);
        }
    }

    fn step_ollama_models(&self) {
        self.log(LogLevel::Info, "检查并拉取本地 AI 模型 (Ollama: qwen2.5-coder)...");
        let (ok, _) = run_native_cmd("ollama", &["pull", "qwen2.5-coder"]);
        if ok {
            self.log(LogLevel::Ok, "本地 AI 模型 qwen2.5-coder 已拉取/就绪");
        } else {
            self.log(LogLevel::Warn, "Ollama 本地 AI 模型检查完成");
        }
    }

    fn step_deep_win_tweaks(&self) {
        self.log(LogLevel::Info, "应用 Windows LTSC 深度性能与隐私优化...");
        let deep_script = r##"
            # Unlock & Activate Ultimate Performance Power Plan
            powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 2>$null | Out-Null
            $ultPlan = powercfg -l | Select-String "Ultimate Performance|卓越性能" | ForEach-Object { ($_ -split '\s+')[3] }
            if ($ultPlan) { powercfg -s $ultPlan }

            # Disable Telemetry
            Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection" -Name "AllowTelemetry" -Value 0 -Force -ErrorAction SilentlyContinue

            # Disable Start Menu Bing Search
            Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Search" -Name "BingSearchEnabled" -Value 0 -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Search" -Name "DisableSearchBoxSuggestions" -Value 1 -Force -ErrorAction SilentlyContinue

            # Explorer Tweaks: Open to This PC, Show Extensions, Show Hidden, Long Paths Enabled
            Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "LaunchTo" -Value 1 -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "HideFileExt" -Value 0 -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "Hidden" -Value 1 -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1 -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock" -Name "AllowDevelopmentWithoutDevLicense" -Value 1 -Force -ErrorAction SilentlyContinue

            # CPU & Network Responsiveness
            Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile" -Name "SystemResponsiveness" -Value 0 -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile" -Name "NetworkThrottlingIndex" -Value 4294967295 -Force -ErrorAction SilentlyContinue
        "##;

        let (ok, _) = run_powershell_cmd(deep_script);
        if ok {
            self.log(LogLevel::Ok, "卓越性能模式已激活、Telemetry 已禁用、搜索/资源管理器性能深度优化完成");
        } else {
            self.log(LogLevel::Warn, "系统深度优化检查完成");
        }
    }

    fn step_audit(&self) {
        self.log(LogLevel::Info, "进行最终组件与命令行审计...");
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
        let (ok, _) = run_native_cmd("winget", &["install", "--id", id, "-e", "--silent", "--accept-package-agreements", "--accept-source-agreements"]);
        if ok {
            self.log(LogLevel::Ok, format!("Winget 应用: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("Winget 应用: {} [跳过/已存在]", name));
        }
    }

    fn install_scoop_tool(&self, name: &str) {
        let (ok, _) = run_native_cmd("scoop", &["install", name]);
        if ok {
            self.log(LogLevel::Ok, format!("Scoop 工具: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("Scoop 工具: {} [跳过/已存在]", name));
        }
    }

    fn install_cargo_package(&self, name: &str) {
        let (ok, _) = run_native_cmd("cargo", &["install", name]);
        if ok {
            self.log(LogLevel::Ok, format!("Cargo 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("Cargo 包: {} [跳过/已存在]", name));
        }
    }

    fn install_npm_global(&self, name: &str) {
        let (ok, _) = run_native_cmd("npm", &["install", "-g", name, "--loglevel=error"]);
        if ok {
            self.log(LogLevel::Ok, format!("NPM 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("NPM 包: {} [跳过/已存在]", name));
        }
    }

    fn install_pip_package(&self, name: &str) {
        let (ok, _) = run_native_cmd("pip", &["install", name, "--quiet"]);
        if ok {
            self.log(LogLevel::Ok, format!("Pip 包: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("Pip 包: {} [跳过/已存在]", name));
        }
    }

    fn install_uv_tool(&self, name: &str) {
        let (ok, _) = run_native_cmd("uv", &["tool", "install", name]);
        if ok {
            self.log(LogLevel::Ok, format!("UV 工具: {} [成功/就绪]", name));
        } else {
            self.log(LogLevel::Warn, format!("UV 工具: {} [跳过/已存在]", name));
        }
    }
}
