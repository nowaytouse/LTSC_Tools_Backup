use crate::config::{ExecutionTarget, NetworkMode, SetupConfig};
use crate::installer::SetupEngine;
use crate::utils::{is_admin, LogLevel, LogMessage};
use eframe::egui;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct SetupApp {
    config: SetupConfig,
    is_running: bool,
    progress: f32,
    logs: Vec<LogMessage>,
    log_rx: Option<Receiver<LogMessage>>,
    progress_rx: Option<Receiver<f32>>,
    admin_status: bool,
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    #[cfg(target_os = "windows")]
    let font_paths = [
        r"C:\Windows\Fonts\msyh.ttc",
        r"C:\Windows\Fonts\msyh.ttf",
        r"C:\Windows\Fonts\msyhl.ttc",
        r"C:\Windows\Fonts\simhei.ttf",
        r"C:\Windows\Fonts\simsun.ttc",
        r"C:\Windows\Fonts\kaiu.ttf",
    ];

    #[cfg(not(target_os = "windows"))]
    let font_paths = [
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/STHeiti Light.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    ];

    for path in &font_paths {
        if let Ok(font_bytes) = std::fs::read(path) {
            fonts.font_data.insert(
                "cjk_font".to_owned(),
                egui::FontData::from_owned(font_bytes),
            );

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "cjk_font".to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("cjk_font".to_owned());

            break;
        }
    }

    ctx.set_fonts(fonts);
}

impl Default for SetupApp {
    fn default() -> Self {
        Self {
            config: SetupConfig::default(),
            is_running: false,
            progress: 0.0,
            logs: vec![LogMessage::new(
                LogLevel::Info,
                "欢迎使用 Windows LTSC 终极一键配置环境 GUI 工具。包含 macOS 100% 同等能力软件库、VS Code/Cursor 插件、Agent Skills、Git账号、Docker/WSL2 及深度性能优化。"
            )],
            log_rx: None,
            progress_rx: None,
            admin_status: is_admin(),
        }
    }
}

impl SetupApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }

    fn start_setup(&mut self, target: ExecutionTarget) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.progress = 0.01;
        self.logs.clear();
        self.logs.push(LogMessage::new(LogLevel::Start, "初始化终极配置引擎线程..."));

        let (log_tx, log_rx): (Sender<LogMessage>, Receiver<LogMessage>) = channel();
        let (progress_tx, progress_rx): (Sender<f32>, Receiver<f32>) = channel();

        self.log_rx = Some(log_rx);
        self.progress_rx = Some(progress_rx);

        let mut config = self.config.clone();
        config.target_mode = target;

        thread::spawn(move || {
            let engine = SetupEngine::new(log_tx, progress_tx, config);
            engine.run_full_setup();
        });
    }

    fn export_logs(&self) {
        let home_dir = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());
        let log_file = std::path::Path::new(&home_dir).join("Desktop").join("ltsc_setup_log.txt");

        let mut content = String::new();
        for l in &self.logs {
            let prefix = match l.level {
                LogLevel::Info => "[INFO]",
                LogLevel::Ok => "[OK]",
                LogLevel::Warn => "[WARN]",
                LogLevel::Error => "[ERROR]",
                LogLevel::Start => "[START]",
                LogLevel::End => "[END]",
            };
            content.push_str(&format!("[{}] {} {}\n", l.time, prefix, l.message));
        }

        let _ = std::fs::write(&log_file, content);
    }

    fn poll_updates(&mut self) {
        if let Some(ref rx) = self.log_rx {
            while let Ok(msg) = rx.try_recv() {
                if msg.level == LogLevel::End {
                    self.is_running = false;
                }
                self.logs.push(msg);
            }
        }

        if let Some(ref rx) = self.progress_rx {
            while let Ok(val) = rx.try_recv() {
                self.progress = val;
            }
        }
    }
}

impl eframe::App for SetupApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_updates();

        if self.is_running {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading("🚀 Windows LTSC Ultimate Workstation Setup");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.admin_status {
                        ui.label(egui::RichText::new("🛡️ 管理员权限: 已具备").color(egui::Color32::GREEN).strong());
                    } else {
                        ui.label(egui::RichText::new("⚠️ 管理员权限: 未获取 (建议右键以管理员身份运行)").color(egui::Color32::RED).strong());
                    }
                });
            });
            ui.add_space(8.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                // Left Column: Controls & Settings
                columns[0].vertical(|ui| {
                    ui.group(|ui| {
                        ui.heading("⚙️ 全量配置与优化选项");
                        ui.add_space(4.0);

                        ui.checkbox(&mut self.config.include_dev_tools, "💻 部署全套 100+ 开发者软件库 (macOS 100% 对齐)");
                        ui.label(egui::RichText::new("VS Code, Cursor, Git, Python, Node, Rust, rtk, claude-code, kimi-cli, sing-box 等").small().color(egui::Color32::GRAY));

                        ui.add_space(4.0);
                        ui.checkbox(&mut self.config.include_vscode_extensions, "🧩 同步 VS Code / Cursor 扩展插件与 settings.json");
                        ui.label(egui::RichText::new("包含 Python, Go, Docker, Vim, GitLens, MarkdownLint, Claude-Code 插件").small().color(egui::Color32::GRAY));

                        ui.add_space(4.0);
                        ui.checkbox(&mut self.config.include_git_shell_configs, "🔑 部署 Git 用户全量配置 & PowerShell 7 Profile 自动化");
                        ui.label(egui::RichText::new("包含 nowaytouse Git 账号、500MB PostBuffer、Starship / Zoxide / UTF-8 初始化").small().color(egui::Color32::GRAY));

                        ui.add_space(4.0);
                        ui.checkbox(&mut self.config.include_agent_skills, "🤖 同步 55+ 真实 AI Agent Skills / Hooks / 规则 (.gemini/config)");
                        ui.label(egui::RichText::new("建立 AGENTS.md 全局规则、rtk token-killer 与 lean-ctx 语境工具链").small().color(egui::Color32::GRAY));

                        ui.add_space(4.0);
                        ui.checkbox(&mut self.config.include_docker_wsl, "🐳 部署 Docker & WSL2 虚拟化内核平台");
                        ui.label(egui::RichText::new("开启 VirtualMachinePlatform、WSL2 及 Docker/Podman Engine").small().color(egui::Color32::GRAY));

                        ui.add_space(4.0);
                        ui.checkbox(&mut self.config.include_deep_win_tweaks, "🚀 Windows LTSC 深度性能与隐私优化");
                        ui.label(egui::RichText::new("解锁卓越性能模式、禁用 Telemetry/Bing 搜索、优化资源管理器与 CPU/内存响应").small().color(egui::Color32::GRAY));

                        ui.add_space(4.0);
                        ui.checkbox(&mut self.config.include_ollama_models, "🧠 自动预拉取本地 AI 模型 (Ollama: qwen2.5-coder)");

                        ui.add_space(6.0);
                        ui.label("🌐 网络、代理与 WinHTTP 优化模式:");
                        egui::ComboBox::from_id_salt("net_mode_combo")
                            .selected_text(format!("{}", self.config.network_mode))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.config.network_mode, NetworkMode::Basic, "Basic (基础 TLS/DNS 协议硬化)");
                                ui.selectable_value(&mut self.config.network_mode, NetworkMode::Optimized, "Optimized (刷新 DNS & 优化 TCP 窗口)");
                                ui.selectable_value(&mut self.config.network_mode, NetworkMode::Extreme, "Extreme (CTCP & ECN + WinHTTP 代理)");
                            });
                    });

                    ui.add_space(8.0);

                    ui.vertical_centered(|ui| {
                        let btn_text = if self.is_running { "⏳ 正在一键终极部署中..." } else { "▶️ 一键开始全套配置" };
                        let start_btn = egui::Button::new(egui::RichText::new(btn_text).size(18.0).strong())
                            .min_size(egui::vec2(280.0, 42.0))
                            .fill(if self.is_running { egui::Color32::DARK_GRAY } else { egui::Color32::from_rgb(0, 120, 215) });

                        if ui.add_enabled(!self.is_running, start_btn).clicked() {
                            self.start_setup(ExecutionTarget::FullSetup);
                        }

                        ui.add_space(6.0);

                        ui.horizontal(|ui| {
                            if ui.add_enabled(!self.is_running, egui::Button::new("🌐 仅优化网络")).clicked() {
                                self.start_setup(ExecutionTarget::NetworkOnly);
                            }
                            if ui.add_enabled(!self.is_running, egui::Button::new("🤖 仅释出 Agent Skills")).clicked() {
                                self.start_setup(ExecutionTarget::AgentSkillsOnly);
                            }
                            if ui.add_enabled(!self.is_running, egui::Button::new("🧩 仅同步 IDE 插件")).clicked() {
                                self.start_setup(ExecutionTarget::VSCodeExtensionsOnly);
                            }
                            if ui.add_enabled(!self.is_running, egui::Button::new("🚀 仅应用系统优化")).clicked() {
                                self.start_setup(ExecutionTarget::SystemTweaksOnly);
                            }
                        });
                    });
                });

                // Right Column: Progress & Real-time Console Log Output
                columns[1].vertical(|ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading("📋 实时日志与进度");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("💾 导出日志到桌面").clicked() {
                                    self.export_logs();
                                }
                            });
                        });
                        ui.add_space(4.0);

                        ui.add(egui::ProgressBar::new(self.progress).show_percentage().animate(self.is_running));

                        ui.add_space(8.0);

                        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                            for log in &self.logs {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("[{}]", log.time)).color(egui::Color32::DARK_GRAY).monospace());

                                    let (prefix, color) = match log.level {
                                        LogLevel::Info  => ("[*]", egui::Color32::LIGHT_GRAY),
                                        LogLevel::Ok    => ("[+]", egui::Color32::GREEN),
                                        LogLevel::Warn  => ("[!]", egui::Color32::YELLOW),
                                        LogLevel::Error => ("[-]", egui::Color32::RED),
                                        LogLevel::Start => (">>>", egui::Color32::LIGHT_BLUE),
                                        LogLevel::End   => ("<<<", egui::Color32::LIGHT_GREEN),
                                    };

                                    ui.label(egui::RichText::new(prefix).color(color).strong().monospace());
                                    ui.label(egui::RichText::new(&log.message).color(color));
                                });
                            }
                        });
                    });
                });
            });
        });
    }
}
