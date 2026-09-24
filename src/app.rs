use crate::config::{ExecutionTarget, NetworkMode, SetupConfig, SetupProfile};
use crate::installer::{run_setup_worker, SetupEvent, SetupOutcome, SetupSummary};
use crate::inventory::MacosInventory;
use crate::utils::{is_admin, CancellationToken, LogLevel, LogMessage};
use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::thread;

const BG: egui::Color32 = egui::Color32::from_rgb(12, 17, 25);
const PANEL: egui::Color32 = egui::Color32::from_rgb(17, 24, 35);
const CARD: egui::Color32 = egui::Color32::from_rgb(24, 33, 47);
const CARD_HOVER: egui::Color32 = egui::Color32::from_rgb(31, 43, 60);
const BORDER: egui::Color32 = egui::Color32::from_rgb(48, 62, 82);
const TEXT: egui::Color32 = egui::Color32::from_rgb(232, 237, 245);
const MUTED: egui::Color32 = egui::Color32::from_rgb(143, 156, 177);
const ACCENT: egui::Color32 = egui::Color32::from_rgb(91, 140, 255);
const SUCCESS: egui::Color32 = egui::Color32::from_rgb(72, 201, 146);
const WARNING: egui::Color32 = egui::Color32::from_rgb(245, 183, 75);
const DANGER: egui::Color32 = egui::Color32::from_rgb(244, 104, 116);

enum RunState {
    Idle,
    Running,
    Cancelling,
    Finished(SetupSummary),
}

impl RunState {
    fn active(&self) -> bool {
        matches!(self, Self::Running | Self::Cancelling)
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Idle => "准备就绪",
            Self::Running => "正在部署",
            Self::Cancelling => "正在取消",
            Self::Finished(summary) => summary.outcome.label(),
        }
    }

    fn color(&self) -> egui::Color32 {
        match self {
            Self::Idle => MUTED,
            Self::Running => ACCENT,
            Self::Cancelling => WARNING,
            Self::Finished(summary) => match summary.outcome {
                SetupOutcome::Succeeded => SUCCESS,
                SetupOutcome::CompletedWithErrors | SetupOutcome::Crashed => DANGER,
                SetupOutcome::Cancelled => WARNING,
            },
        }
    }
}

pub struct SetupApp {
    config: SetupConfig,
    selected_target: ExecutionTarget,
    run_state: RunState,
    progress: f32,
    logs: Vec<LogMessage>,
    event_rx: Option<Receiver<SetupEvent>>,
    cancellation: Option<CancellationToken>,
    worker: Option<thread::JoinHandle<()>>,
    inventory_rx: Option<Receiver<Result<MacosInventory, String>>>,
    inventory_worker: Option<thread::JoinHandle<()>>,
    inventory_cancellation: Option<CancellationToken>,
    inventory_error: Option<String>,
    admin_status: bool,
    json_editor_text: String,
    show_json_editor: bool,
    json_editor_error: Option<String>,
    auto_scroll: bool,
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    static EMBEDDED_CJK_FONT: &[u8] = include_bytes!("assets/cjk_font.ttf");

    fonts.font_data.insert(
        "embedded_cjk_font".to_owned(),
        egui::FontData::from_static(EMBEDDED_CJK_FONT).into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "embedded_cjk_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("embedded_cjk_font".to_owned());

    let font_paths = [
        r"C:\Windows\Fonts\msyh.ttc",
        r"C:\Windows\Fonts\msyh.ttf",
        r"C:\Windows\Fonts\simhei.ttf",
        r"C:\Windows\Fonts\simsun.ttc",
    ];

    for (index, path) in font_paths.iter().enumerate() {
        if let Ok(bytes) = std::fs::read(path) {
            let name = format!("system_cjk_{index}");
            fonts
                .font_data
                .insert(name.clone(), egui::FontData::from_owned(bytes).into());
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .push(name);
        }
    }
    ctx.set_fonts(fonts);
}

fn setup_style(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    let mut style = (*ctx.global_style()).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.panel_fill = BG;
    style.visuals.window_fill = PANEL;
    style.visuals.extreme_bg_color = BG;
    style.visuals.faint_bg_color = CARD;
    style.visuals.selection.bg_fill = ACCENT;
    style.visuals.widgets.inactive.bg_fill = CARD;
    style.visuals.widgets.inactive.weak_bg_fill = CARD;
    style.visuals.widgets.hovered.bg_fill = CARD_HOVER;
    style.visuals.widgets.active.bg_fill = ACCENT;
    style.spacing.item_spacing = egui::vec2(10.0, 9.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    ctx.set_global_style(style);
}

impl Default for SetupApp {
    fn default() -> Self {
        let mut config = SetupConfig::default();
        let mut inventory_warning = None;
        let inventory_error = match MacosInventory::load_cached_or_embedded() {
            Ok(inventory) => {
                config.source_inventory = inventory;
                None
            }
            Err(error) => match MacosInventory::load_embedded().and_then(|inventory| {
                inventory.validate()?;
                Ok(inventory)
            }) {
                Ok(inventory) => {
                    config.source_inventory = inventory;
                    inventory_warning = Some(format!(
                        "本机 Mac 清单缓存无效（{error}）；已退回内置清单，请联网更新"
                    ));
                    None
                }
                Err(fallback_error) => Some(format!(
                    "本机及内置 Mac 清单均无效：{error}；{fallback_error}"
                )),
            },
        };
        let json_editor_text = serde_json::to_string_pretty(&config.profile).unwrap_or_default();
        let parity_message = config
            .profile
            .parity_report(&config.source_inventory)
            .map(|report| {
                format!(
                    "源 Mac 清单 {}：{} 项已分类，{} 项需手动补齐。",
                    config.source_inventory.captured_at,
                    report.covered(),
                    report.manual.len()
                )
            })
            .unwrap_or_else(|error| format!("源 Mac 清单校验失败：{error}"));
        let mut logs = vec![
            LogMessage::new(
                LogLevel::Info,
                "Windows 原生 Rust 部署引擎已就绪；没有 WebView/Electron，也不执行项目外置 PS1。",
            ),
            LogMessage::new(LogLevel::Info, parity_message),
        ];
        if let Some(warning) = inventory_warning {
            logs.push(LogMessage::new(LogLevel::Warn, warning));
        }
        Self {
            config,
            selected_target: ExecutionTarget::DevToolsOnly,
            run_state: RunState::Idle,
            progress: 0.0,
            logs,
            event_rx: None,
            cancellation: None,
            worker: None,
            inventory_rx: None,
            inventory_worker: None,
            inventory_cancellation: None,
            inventory_error,
            admin_status: is_admin(),
            json_editor_text,
            show_json_editor: false,
            json_editor_error: None,
            auto_scroll: true,
        }
    }
}

impl SetupApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        setup_style(&cc.egui_ctx);
        Self::default()
    }

    fn start_setup(&mut self) {
        if self.run_state.active() {
            return;
        }
        if let Some(error) = &self.inventory_error {
            self.push_log(LogLevel::Error, error.clone());
            return;
        }
        if self.selected_target.requires_admin() && !self.admin_status {
            self.push_log(
                LogLevel::Error,
                "该任务需要管理员权限；请关闭程序并以管理员身份重新运行。",
            );
            return;
        }
        if !self.apply_profile_editor() {
            self.show_json_editor = true;
            return;
        }

        let mut config = self.config.clone();
        config.target_mode = self.selected_target;
        if let Err(error) = config.resolved() {
            self.push_log(LogLevel::Error, format!("无法生成部署计划：{error}"));
            return;
        }

        self.progress = 0.0;
        self.logs.clear();
        self.run_state = RunState::Running;
        let cancellation = CancellationToken::default();
        self.cancellation = Some(cancellation.clone());
        let (tx, rx) = channel();
        self.event_rx = Some(rx);

        self.worker = Some(thread::spawn(move || {
            run_setup_worker(tx, config, cancellation)
        }));
    }

    fn refresh_inventory(&mut self) {
        if self.run_state.active() || self.inventory_rx.is_some() {
            return;
        }
        let (tx, rx) = channel();
        let cancellation = CancellationToken::default();
        let worker_cancellation = cancellation.clone();
        self.inventory_rx = Some(rx);
        self.inventory_cancellation = Some(cancellation);
        self.inventory_worker = Some(thread::spawn(move || {
            let result = MacosInventory::fetch_latest(&worker_cancellation)
                .map_err(|error| error.to_string());
            let _ = tx.send(result);
        }));
        self.push_log(LogLevel::Info, "正在从仓库更新 Mac 工具清单…");
    }

    fn cancel_setup(&mut self) {
        if let Some(cancellation) = &self.cancellation {
            cancellation.cancel();
            self.run_state = RunState::Cancelling;
            self.push_log(LogLevel::Warn, "已请求取消；正在终止当前子进程树…");
        }
    }

    fn poll_events(&mut self) {
        let inventory_result =
            self.inventory_rx
                .as_ref()
                .and_then(|receiver| match receiver.try_recv() {
                    Ok(result) => Some(result),
                    Err(TryRecvError::Disconnected) => Some(Err("清单更新线程失联".into())),
                    Err(TryRecvError::Empty) => None,
                });
        if let Some(result) = inventory_result {
            match result {
                Ok(inventory) => {
                    self.push_log(
                        LogLevel::Ok,
                        format!(
                            "Mac 清单已更新：{} 项，采集于 {}",
                            inventory.item_count(),
                            inventory.captured_at
                        ),
                    );
                    self.config.source_inventory = inventory;
                    self.inventory_error = None;
                }
                Err(error) => self.push_log(LogLevel::Error, error),
            }
            self.inventory_rx = None;
            self.inventory_cancellation = None;
            if let Some(worker) = self.inventory_worker.take() {
                let _ = worker.join();
            }
        }
        let mut events = Vec::new();
        let mut disconnected = false;
        if let Some(receiver) = &self.event_rx {
            loop {
                match receiver.try_recv() {
                    Ok(event) => events.push(event),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        disconnected = true;
                        break;
                    }
                }
            }
        }

        let mut finished = false;
        for event in events {
            match event {
                SetupEvent::Log(message) => self.logs.push(message),
                SetupEvent::Progress(progress) => self.progress = progress,
                SetupEvent::Finished(summary) => {
                    self.run_state = RunState::Finished(summary);
                    self.cancellation = None;
                    finished = true;
                }
            }
        }
        if self.logs.len() > 8_000 {
            self.logs.drain(..self.logs.len() - 8_000);
        }
        if disconnected && self.run_state.active() && !finished {
            self.push_log(
                LogLevel::Error,
                "部署线程失联；GUI 已退出运行态，请查看最后一条日志。",
            );
            self.run_state = RunState::Finished(SetupSummary::crashed(std::time::Duration::ZERO));
            self.cancellation = None;
            finished = true;
        }
        if finished {
            self.event_rx = None;
            if let Some(worker) = self.worker.take() {
                let _ = worker.join();
            }
        }
    }

    fn apply_profile_editor(&mut self) -> bool {
        let profile = match serde_json::from_str::<SetupProfile>(&self.json_editor_text) {
            Ok(profile) => profile,
            Err(error) => {
                self.json_editor_error = Some(format!("JSON 解析失败：{error}"));
                return false;
            }
        };
        if let Err(error) = profile.validate() {
            self.json_editor_error = Some(format!("配置校验失败：{error}"));
            return false;
        }
        self.config.profile = profile;
        self.json_editor_error = None;
        true
    }

    fn reset_profile_editor(&mut self) {
        self.config.profile = SetupProfile::load_default();
        self.json_editor_text =
            serde_json::to_string_pretty(&self.config.profile).unwrap_or_default();
        self.json_editor_error = None;
    }

    fn push_log(&mut self, level: LogLevel, message: impl Into<String>) {
        self.logs.push(LogMessage::new(level, message));
    }

    fn export_logs(&mut self) {
        let path = export_path("ltsc_setup_log.txt");
        let mut content = String::new();
        for log in &self.logs {
            let level = match log.level {
                LogLevel::Info => "INFO",
                LogLevel::Ok => "OK",
                LogLevel::Warn => "WARN",
                LogLevel::Error => "ERROR",
                LogLevel::Start => "START",
                LogLevel::End => "END",
            };
            content.push_str(&format!("[{}] [{level}] {}\n", log.time, log.message));
        }
        match std::fs::write(&path, content) {
            Ok(()) => self.push_log(LogLevel::Ok, format!("日志已导出：{}", path.display())),
            Err(error) => self.push_log(LogLevel::Error, format!("日志导出失败：{error}")),
        }
    }

    fn export_profile(&mut self) {
        if !self.apply_profile_editor() {
            self.show_json_editor = true;
            self.push_log(LogLevel::Error, "Profile 尚未通过校验，未导出旧配置。");
            return;
        }
        let path = export_path("setup_profile.json");
        match self.config.profile.save_to_file(&path) {
            Ok(()) => self.push_log(LogLevel::Ok, format!("配置已导出：{}", path.display())),
            Err(error) => self.push_log(LogLevel::Error, format!("配置导出失败：{error}")),
        }
    }

    fn export_inventory(&mut self) {
        let path = export_path("macos_inventory.json");
        match serde_json::to_vec_pretty(&self.config.source_inventory)
            .map_err(anyhow::Error::from)
            .and_then(|content| std::fs::write(&path, content).map_err(anyhow::Error::from))
        {
            Ok(()) => self.push_log(LogLevel::Ok, format!("Mac 清单已导出：{}", path.display())),
            Err(error) => self.push_log(LogLevel::Error, format!("Mac 清单导出失败：{error}")),
        }
    }

    fn render_header(&self, root: &mut egui::Ui) {
        egui::Panel::top("header")
            .exact_size(72.0)
            .frame(
                egui::Frame::default()
                    .fill(PANEL)
                    .stroke(egui::Stroke::new(1.0_f32, BORDER))
                    .inner_margin(egui::Margin::symmetric(22, 14)),
            )
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("LTSC 工作环境")
                                .size(21.0)
                                .strong()
                                .color(TEXT),
                        );
                        ui.label(
                            egui::RichText::new("补齐 Mac 工具 · 部署 Windows · 随时查看结果")
                                .size(12.0)
                                .color(MUTED),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        status_chip(ui, self.run_state.label(), self.run_state.color());
                        ui.add_space(8.0);
                        status_chip(
                            ui,
                            if self.admin_status {
                                "管理员"
                            } else {
                                "非管理员"
                            },
                            if self.admin_status { SUCCESS } else { WARNING },
                        );
                    });
                });
            });
    }

    fn render_navigation(&mut self, root: &mut egui::Ui) {
        egui::Panel::left("navigation")
            .exact_size(218.0)
            .resizable(false)
            .frame(
                egui::Frame::default()
                    .fill(PANEL)
                    .stroke(egui::Stroke::new(1.0_f32, BORDER))
                    .inner_margin(egui::Margin::same(14)),
            )
            .show(root, |ui| {
                let scroll_height = (ui.available_height() - 48.0).max(180.0);
                egui::ScrollArea::vertical()
                    .id_salt("navigation_scroll")
                    .max_height(scroll_height)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("部署范围")
                                .size(12.0)
                                .strong()
                                .color(MUTED),
                        );
                        ui.add_space(4.0);
                        for target in ExecutionTarget::ALL {
                            let selected = self.selected_target == target;
                            let button = egui::Button::new(
                                egui::RichText::new(target.label())
                                    .size(14.0)
                                    .color(if selected { TEXT } else { MUTED }),
                            )
                            .selected(selected)
                            .fill(if selected { ACCENT } else { CARD })
                            .corner_radius(7)
                            .min_size(egui::vec2(ui.available_width(), 38.0));
                            if ui.add_enabled(!self.run_state.active(), button).clicked() {
                                self.selected_target = target;
                            }
                        }

                        ui.add_space(14.0);
                        ui.separator();
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("Mac 工具清单")
                                .size(12.0)
                                .strong()
                                .color(MUTED),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "{} 项 · {}",
                                self.config.source_inventory.item_count(),
                                self.config.source_inventory.captured_at
                            ))
                            .size(11.0)
                            .color(MUTED),
                        );
                        if ui
                            .add_enabled(
                                !self.run_state.active() && self.inventory_rx.is_none(),
                                egui::Button::new(if self.inventory_rx.is_some() {
                                    "正在更新…"
                                } else {
                                    "从 GitHub 更新清单"
                                })
                                .min_size(egui::vec2(ui.available_width(), 36.0)),
                            )
                            .clicked()
                        {
                            self.refresh_inventory();
                        }
                        egui::CollapsingHeader::new("高级配置与导出")
                            .default_open(false)
                            .show(ui, |ui| {
                                if ui.button("编辑 Profile JSON").clicked() {
                                    self.show_json_editor = true;
                                }
                                if ui.button("导出 Profile").clicked() {
                                    self.export_profile();
                                }
                                if ui.button("导出 Mac 清单").clicked() {
                                    self.export_inventory();
                                }
                            });
                    });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "Profile v{}",
                            self.config.profile.profile_version
                        ))
                        .size(11.0)
                        .color(MUTED),
                    );
                    ui.label(
                        egui::RichText::new("Windows 原生执行模式")
                            .size(11.0)
                            .color(SUCCESS),
                    );
                });
            });
    }

    fn render_console(&mut self, root: &mut egui::Ui) {
        egui::Panel::right("activity")
            .default_size(430.0)
            .min_size(340.0)
            .max_size(560.0)
            .resizable(true)
            .frame(
                egui::Frame::default()
                    .fill(PANEL)
                    .stroke(egui::Stroke::new(1.0_f32, BORDER))
                    .inner_margin(egui::Margin::same(14)),
            )
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("运行记录")
                            .size(16.0)
                            .strong()
                            .color(TEXT),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("导出").clicked() {
                            self.export_logs();
                        }
                        if ui
                            .add_enabled(
                                !self.run_state.active(),
                                egui::Button::new("清空").small(),
                            )
                            .clicked()
                        {
                            self.logs.clear();
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.auto_scroll, "跟随最新日志");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format!("{} 条", self.logs.len()))
                                .size(11.0)
                                .color(MUTED),
                        );
                    });
                });
                ui.add_space(4.0);

                let reserved = 82.0;
                egui::ScrollArea::vertical()
                    .id_salt("log_scroll")
                    .stick_to_bottom(self.auto_scroll)
                    .max_height((ui.available_height() - reserved).max(120.0))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 6.0;
                        for log in &self.logs {
                            log_row(ui, log);
                        }
                    });

                ui.add_space(8.0);
                ui.add(
                    egui::ProgressBar::new(self.progress)
                        .show_percentage()
                        .animate(self.run_state.active())
                        .corner_radius(6)
                        .fill(ACCENT),
                );
                if let RunState::Finished(summary) = &self.run_state {
                    ui.label(
                        egui::RichText::new(format!(
                            "{} · {} 失败 · {} 警告 · {:.1}s",
                            summary.outcome.label(),
                            summary.errors,
                            summary.warnings,
                            summary.elapsed.as_secs_f32()
                        ))
                        .size(11.0)
                        .color(self.run_state.color()),
                    );
                } else {
                    ui.label(
                        egui::RichText::new(self.run_state.label())
                            .size(11.0)
                            .color(self.run_state.color()),
                    );
                }
            });
    }

    fn render_workspace(&mut self, root: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(BG)
                    .inner_margin(egui::Margin::same(20)),
            )
            .show(root, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(self.selected_target.label())
                            .size(26.0)
                            .strong()
                            .color(TEXT),
                    );
                    ui.label(
                        egui::RichText::new(self.selected_target.description())
                            .size(13.0)
                            .color(MUTED),
                    );
                    ui.add_space(16.0);

                    let package_target = matches!(
                        self.selected_target,
                        ExecutionTarget::FullSetup | ExecutionTarget::DevToolsOnly
                    );
                    let mut request = self.config.clone();
                    request.target_mode = self.selected_target;
                    let plan = request.resolved();
                    let packages = plan.as_ref().ok().map(|config| &config.profile.packages);
                    let winget_count = packages.filter(|_| package_target).map_or(0, |packages| {
                        packages.winget_core.len() + packages.winget_dev.len()
                    });
                    let cli_count = packages.filter(|_| package_target).map_or(0, |packages| {
                        packages.scoop_tools.len()
                            + packages.cargo_packages.len()
                            + packages.npm_globals.len()
                            + packages.pip_packages.len()
                            + packages.uv_tools.len()
                    });
                    let extension_count = if self.selected_target == ExecutionTarget::FullSetup
                        && self.config.include_vscode_extensions
                    {
                        self.config.profile.vscode_config.extensions.len()
                    } else {
                        0
                    };
                    let parity = self
                        .config
                        .profile
                        .parity_report(&self.config.source_inventory)
                        .ok();
                    ui.columns(4, |columns| {
                        metric_card(&mut columns[0], "本次 WinGet 候选", winget_count);
                        metric_card(&mut columns[1], "本次 CLI 候选", cli_count);
                        metric_card(&mut columns[2], "本次 IDE 扩展", extension_count);
                        metric_card(
                            &mut columns[3],
                            "Mac 清单覆盖",
                            parity.as_ref().map_or(0, |report| report.covered()),
                        );
                    });
                    ui.add_space(14.0);

                    if let Some(error) = &self.inventory_error {
                        ui.colored_label(DANGER, error);
                    }
                    if package_target {
                        card(ui, |ui| {
                            ui.label(egui::RichText::new("本次部署计划").size(16.0).strong().color(TEXT));
                            match &plan {
                                Ok(plan) => {
                                    ui.label(
                                        egui::RichText::new(if self.config.include_curated_extras {
                                            "已选择完整预设目录；已安装项目会在执行时跳过。"
                                        } else {
                                            "仅安装 Mac 清单的 Windows 对等项及所需运行时；已安装项目会在执行时跳过。"
                                        })
                                        .color(MUTED),
                                    );
                                    egui::CollapsingHeader::new("查看软件清单")
                                        .default_open(false)
                                        .show(ui, |ui| {
                                            let packages = &plan.profile.packages;
                                            package_plan_row(ui, "WinGet 核心", packages.winget_core.iter().map(|app| app.id.as_str()));
                                            package_plan_row(ui, "WinGet 开发", packages.winget_dev.iter().map(|app| app.id.as_str()));
                                            package_plan_row(ui, "Scoop", packages.scoop_tools.iter().map(String::as_str));
                                            package_plan_row(ui, "Cargo", packages.cargo_packages.iter().map(String::as_str));
                                            package_plan_row(ui, "NPM", packages.npm_globals.iter().map(String::as_str));
                                            package_plan_row(ui, "Pip", packages.pip_packages.iter().map(String::as_str));
                                            package_plan_row(ui, "UV", packages.uv_tools.iter().map(String::as_str));
                                        });
                                }
                                Err(error) => {
                                    ui.colored_label(DANGER, format!("计划无法执行：{error}"));
                                }
                            }
                            if let Some(parity) = &parity {
                                if !parity.manual.is_empty() {
                                    egui::CollapsingHeader::new(format!("需手动处理 {} 项", parity.manual.len()))
                                        .show(ui, |ui| {
                                            for item in &parity.manual {
                                                ui.label(item);
                                            }
                                        });
                                }
                            }
                        });
                        ui.add_space(14.0);
                    }

                    card(ui, |ui| {
                        ui.label(
                            egui::RichText::new("执行选项")
                                .size(16.0)
                                .strong()
                                .color(TEXT),
                        );
                        ui.add_space(8.0);
                        match self.selected_target {
                            ExecutionTarget::FullSetup => {
                                option(
                                    ui,
                                    &mut self.config.include_curated_extras,
                                    "安装额外预设软件",
                                    "默认关闭；开启后安装完整目录和 LTSC 常用商店应用",
                                    self.run_state.active(),
                                );
                                ui.columns(2, |columns| {
                                    option(
                                        &mut columns[0],
                                        &mut self.config.include_dev_tools,
                                        "开发工具与运行时",
                                        "WinGet、Scoop、Cargo、NPM、Pip、UV",
                                        self.run_state.active(),
                                    );
                                    option(
                                        &mut columns[1],
                                        &mut self.config.include_docker_wsl,
                                        "Windows 可选功能",
                                        "WSL2、.NET 3.5、Sandbox（按系统能力）",
                                        self.run_state.active(),
                                    );
                                    option(
                                        &mut columns[0],
                                        &mut self.config.include_storage_optimization,
                                        "存储介质优化",
                                        "TRIM + Windows /O 自动选择 SSD/HDD 策略",
                                        self.run_state.active(),
                                    );
                                    option(
                                        &mut columns[1],
                                        &mut self.config.include_vscode_extensions,
                                        "IDE 设置与扩展",
                                        "保留现有 JSON 键并合并受管配置",
                                        self.run_state.active(),
                                    );
                                    option(
                                        &mut columns[0],
                                        &mut self.config.include_git_shell_configs,
                                        "Git 与 Shell Profile",
                                        "以受管区块更新，不覆盖个人内容",
                                        self.run_state.active(),
                                    );
                                    option(
                                        &mut columns[0],
                                        &mut self.config.include_agent_skills,
                                        "Agent Skills / Plugins",
                                        "仅释放对应内置目录与 MCP 配置",
                                        self.run_state.active(),
                                    );
                                    option(
                                        &mut columns[1],
                                        &mut self.config.include_deep_win_tweaks,
                                        "系统性能与隐私",
                                        "Rust 直接写入 Windows 注册表",
                                        self.run_state.active(),
                                    );
                                    option(
                                        &mut columns[0],
                                        &mut self.config.include_ollama_models,
                                        "Ollama 模型",
                                        "长任务可取消，最长 30 分钟",
                                        self.run_state.active(),
                                    );
                                    option(
                                        &mut columns[1],
                                        &mut self.config.include_npmrc_config,
                                        "NPM 镜像配置",
                                        "以受管区块保留已有 .npmrc",
                                        self.run_state.active(),
                                    );
                                });
                            }
                            ExecutionTarget::DevToolsOnly => {
                                option(
                                    ui,
                                    &mut self.config.include_curated_extras,
                                    "安装完整预设软件目录",
                                    "默认仅补齐 Mac 清单中的工具",
                                    self.run_state.active(),
                                );
                                ui.label(
                                    egui::RichText::new(
                                        "按依赖顺序先检查包管理器与镜像，再安装各语言工具。",
                                    )
                                    .color(MUTED),
                                );
                            }
                            ExecutionTarget::NetworkOnly => {
                                ui.label(
                                    egui::RichText::new(
                                        "先保存 TCP 原始配置；每条命令独立超时，取消会终止进程树。",
                                    )
                                    .color(MUTED),
                                );
                            }
                            ExecutionTarget::StorageOnly => {
                                ui.add_enabled_ui(!self.run_state.active(), |ui| {
                                    ui.checkbox(
                                        &mut self.config.profile.system_tweaks.enable_trim,
                                        "启用 NTFS / ReFS TRIM 删除通知",
                                    );
                                    ui.checkbox(
                                        &mut self.config.profile.system_tweaks.optimize_storage,
                                        "使用 Windows /O 按介质类型优化所有固定卷",
                                    );
                                });
                                ui.label(
                                    egui::RichText::new(
                                        "不会把 SSD 强制当 HDD 碎片整理；Windows 会自行选择 retrim 或 defrag。",
                                    )
                                    .color(MUTED),
                                );
                            }
                            ExecutionTarget::WindowsFeaturesOnly => {
                                let features = &mut self.config.profile.windows_features;
                                ui.add_enabled_ui(!self.run_state.active(), |ui| {
                                    ui.checkbox(&mut features.enable_wsl2, "WSL2 + VirtualMachinePlatform");
                                    ui.checkbox(&mut features.enable_netfx3, ".NET Framework 3.5");
                                    ui.checkbox(
                                        &mut features.enable_windows_sandbox,
                                        "Windows Sandbox（不支持时仅警告）",
                                    );
                                    ui.checkbox(
                                        &mut features.enable_hyper_v,
                                        "完整 Hyper-V（默认关闭，可能与其他虚拟化方案冲突）",
                                    );
                                });
                            }
                            ExecutionTarget::VSCodeExtensionsOnly => {
                                ui.checkbox(
                                    &mut self.config.include_npmrc_config,
                                    "同时更新 NPM 受管配置区块",
                                );
                            }
                            ExecutionTarget::AgentSkillsOnly => {
                                ui.label(
                                    egui::RichText::new(
                                        "只释放 Skills、Plugins 和 mcp_config.json，不再把字体或 Profile 误写入目标目录。",
                                    )
                                    .color(MUTED),
                                );
                            }
                            ExecutionTarget::SystemTweaksOnly => {
                                let tweaks = &mut self.config.profile.system_tweaks;
                                ui.add_enabled_ui(!self.run_state.active(), |ui| {
                                    ui.checkbox(
                                        &mut tweaks.create_rollback_journal,
                                        "变更前创建本机回滚账本",
                                    );
                                    ui.checkbox(
                                        &mut tweaks.activate_ultimate_performance,
                                        "启用卓越性能电源计划",
                                    );
                                    ui.checkbox(
                                        &mut tweaks.extreme_ac_power_settings,
                                        "AC 极限档：CPU 100%、不睡眠、关闭 USB/PCIe 节能",
                                    );
                                    ui.checkbox(
                                        &mut tweaks.disable_hibernation,
                                        "关闭休眠与快速启动（高风险，默认关闭）",
                                    );
                                    ui.checkbox(&mut tweaks.disable_telemetry, "限制遥测策略");
                                    ui.checkbox(
                                        &mut tweaks.disable_consumer_features,
                                        "关闭消费内容推荐",
                                    );
                                });
                            }
                            ExecutionTarget::RollbackLatest => {
                                ui.label(
                                    egui::RichText::new(
                                        "将逆序恢复最近账本里的注册表、TCP 与原电源计划；不会卸载已安装的软件。",
                                    )
                                    .color(WARNING),
                                );
                            }
                        }

                        if matches!(
                            self.selected_target,
                            ExecutionTarget::FullSetup | ExecutionTarget::NetworkOnly
                        ) {
                            ui.add_space(12.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("网络模式")
                                        .strong()
                                        .color(TEXT),
                                );
                                ui.add_enabled_ui(!self.run_state.active(), |ui| {
                                    egui::ComboBox::from_id_salt("network_mode")
                                        .selected_text(match self.config.network_mode {
                                            NetworkMode::Basic => "Basic",
                                            NetworkMode::Optimized => "Optimized",
                                            NetworkMode::Extreme => "Extreme",
                                        })
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut self.config.network_mode,
                                                NetworkMode::Basic,
                                                "Basic · 刷新 DNS",
                                            );
                                            ui.selectable_value(
                                                &mut self.config.network_mode,
                                                NetworkMode::Optimized,
                                                "Optimized · DNS + TCP 自动调优",
                                            );
                                            ui.selectable_value(
                                                &mut self.config.network_mode,
                                                NetworkMode::Extreme,
                                                "Extreme · RSS/RSC + Fast Open + CTCP/ECN",
                                            );
                                        });
                                });
                            });
                        }
                    });

                    ui.add_space(14.0);
                    card(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(if self.run_state.active() {
                                        "任务正在执行"
                                    } else {
                                        "准备开始"
                                    })
                                    .size(16.0)
                                    .strong()
                                    .color(TEXT),
                                );
                                ui.label(
                                    egui::RichText::new(if self.admin_status {
                                        "管理员权限已确认；执行结果会实时显示在右侧。"
                                    } else {
                                        "当前不是管理员；系统级步骤可能失败，建议以管理员身份运行。"
                                    })
                                    .size(12.0)
                                    .color(if self.admin_status { MUTED } else { WARNING }),
                                );
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if self.run_state.active() {
                                        let cancel = egui::Button::new(
                                            egui::RichText::new("取消任务")
                                                .strong()
                                                .color(TEXT),
                                        )
                                        .fill(DANGER)
                                        .corner_radius(7)
                                        .min_size(egui::vec2(116.0, 40.0));
                                        if ui
                                            .add_enabled(
                                                !matches!(
                                                    self.run_state,
                                                    RunState::Cancelling
                                                ),
                                                cancel,
                                            )
                                            .clicked()
                                        {
                                            self.cancel_setup();
                                        }
                                    } else {
                                        let needs_admin = (self.selected_target.requires_admin()
                                            && !self.admin_status)
                                            || self.inventory_error.is_some()
                                            || self.inventory_rx.is_some()
                                            || plan.is_err();
                                        let run = egui::Button::new(
                                            egui::RichText::new(if needs_admin {
                                                "需要管理员权限"
                                            } else {
                                                "应用这份计划"
                                            })
                                                .strong()
                                                .color(TEXT),
                                        )
                                        .fill(ACCENT)
                                        .corner_radius(7)
                                        .min_size(egui::vec2(116.0, 40.0));
                                        if ui
                                            .add_enabled(
                                                !needs_admin,
                                                run,
                                            )
                                            .clicked()
                                        {
                                            self.start_setup();
                                        }
                                    }
                                },
                            );
                        });
                    });
                });
            });
    }

    fn render_profile_editor(&mut self, ctx: &egui::Context) {
        if !self.show_json_editor {
            return;
        }
        let mut open = self.show_json_editor;
        let mut apply = false;
        let mut reset = false;
        egui::Window::new("Profile JSON")
            .open(&mut open)
            .default_size([760.0, 620.0])
            .min_size([560.0, 420.0])
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("保存前会校验空值、重复包 ID 与重复列表项。").color(MUTED),
                );
                if let Some(error) = &self.json_editor_error {
                    ui.label(egui::RichText::new(error).color(DANGER));
                }
                ui.add_space(4.0);
                let editor_height = (ui.available_height() - 54.0).max(240.0);
                egui::ScrollArea::vertical()
                    .max_height(editor_height)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.json_editor_text)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .desired_rows(26),
                        );
                    });
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(!self.run_state.active(), egui::Button::new("应用配置"))
                        .clicked()
                    {
                        apply = true;
                    }
                    if ui
                        .add_enabled(
                            !self.run_state.active(),
                            egui::Button::new("恢复内置 Profile"),
                        )
                        .clicked()
                    {
                        reset = true;
                    }
                });
            });
        self.show_json_editor = open;
        if reset {
            self.reset_profile_editor();
        }
        if apply && self.apply_profile_editor() {
            self.push_log(LogLevel::Ok, "Profile JSON 已校验并应用。");
        }
    }
}

impl Drop for SetupApp {
    fn drop(&mut self) {
        if let Some(cancellation) = &self.inventory_cancellation {
            cancellation.cancel();
        }
        if let Some(cancellation) = &self.cancellation {
            cancellation.cancel();
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
        if let Some(worker) = self.inventory_worker.take() {
            let _ = worker.join();
        }
    }
}

impl eframe::App for SetupApp {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll_events();
        let ctx = root.ctx().clone();
        if self.run_state.active() {
            ctx.request_repaint_after(std::time::Duration::from_millis(75));
        }

        self.render_header(root);
        self.render_navigation(root);
        self.render_console(root);
        self.render_workspace(root);
        self.render_profile_editor(&ctx);
    }
}

fn status_chip(ui: &mut egui::Ui, label: &str, color: egui::Color32) {
    egui::Frame::default()
        .fill(color.gamma_multiply(0.18))
        .stroke(egui::Stroke::new(1.0_f32, color.gamma_multiply(0.7)))
        .corner_radius(255)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(label).size(11.0).strong().color(color));
        });
}

fn card<R>(ui: &mut egui::Ui, contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::default()
        .fill(CARD)
        .stroke(egui::Stroke::new(1.0_f32, BORDER))
        .corner_radius(10)
        .inner_margin(egui::Margin::same(16))
        .show(ui, contents)
        .inner
}

fn metric_card(ui: &mut egui::Ui, label: &str, value: usize) {
    card(ui, |ui| {
        ui.label(
            egui::RichText::new(value.to_string())
                .size(24.0)
                .strong()
                .color(TEXT),
        );
        ui.label(egui::RichText::new(label).size(11.0).color(MUTED));
    });
}

fn option(ui: &mut egui::Ui, enabled: &mut bool, title: &str, description: &str, running: bool) {
    egui::Frame::default()
        .fill(BG.gamma_multiply(0.8))
        .corner_radius(7)
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.add_enabled_ui(!running, |ui| {
                ui.checkbox(enabled, egui::RichText::new(title).strong().color(TEXT));
            });
            ui.label(egui::RichText::new(description).size(11.0).color(MUTED));
        });
}

fn log_row(ui: &mut egui::Ui, log: &LogMessage) {
    let (marker, color) = match log.level {
        LogLevel::Info => ("·", MUTED),
        LogLevel::Ok => ("✓", SUCCESS),
        LogLevel::Warn => ("!", WARNING),
        LogLevel::Error => ("×", DANGER),
        LogLevel::Start => ("›", ACCENT),
        LogLevel::End => ("■", SUCCESS),
    };
    egui::Frame::default()
        .fill(BG.gamma_multiply(0.8))
        .corner_radius(5)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.label(
                    egui::RichText::new(&log.time)
                        .monospace()
                        .size(10.0)
                        .color(MUTED),
                );
                ui.label(egui::RichText::new(marker).strong().color(color));
                ui.add(
                    egui::Label::new(egui::RichText::new(&log.message).size(11.0).color(color))
                        .wrap(),
                );
            });
        });
}

fn package_plan_row<'a>(ui: &mut egui::Ui, provider: &str, names: impl Iterator<Item = &'a str>) {
    let names = names.collect::<Vec<_>>();
    if !names.is_empty() {
        ui.label(
            egui::RichText::new(format!("{provider} · {} 项", names.len()))
                .strong()
                .color(TEXT),
        );
        ui.label(names.join("、"));
    }
}

fn export_path(file_name: &str) -> PathBuf {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let desktop = home.join("Desktop");
    if desktop.is_dir() {
        desktop.join(file_name)
    } else {
        home.join(file_name)
    }
}
