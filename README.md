# 🚀 Windows LTSC Ultimate Workstation Setup (Rust Native GUI)

> **macOS 100% 同等能力 & 深度环境一键同步移植工具**  
> 一键解锁卓越性能、全量部署 100+ 开发者软件库、恢复 LTSC 原生应用、静默释出 55+ 真实 AI Agent Skills / Hooks、同步 VS Code/Cursor 插件与配置。

---

## 🌟 核心特性

- **双击直运行原生 GUI (`ltsc_setup_gui.exe`)**：无脚本依托、无命令行依赖。在 Windows 下直接双击打开原生 GUI 界面，勾选配置后一键自动化完成。
- **55+ 真实 AI Agent Skills 内存嵌入 (`include_dir`)**：所有 Agent Skills、Plugins 插件及 `AGENTS.md` 规则均直接静态编译打包在 15MB `.exe` 二进制文件内部，运行即自动释放解压至 `%USERPROFILE%\.gemini\config`。
- **macOS 100+ 软件矩阵对齐**：涵盖 VS Code、Cursor、Chrome、Brave、Docker Desktop、Podman Desktop、Krita、Bitwarden、Gpg4win、PeaZip、Sing-box、Mihomo、Ollama、Whisper-cpp、Ripgrep、Fd、Fzf、Bat、Eza、Starship、Fastfetch 等 100+ 桌面与 CLI 工具。
- **Git 账号与全量偏好**：自动配置 `user.name = "nowaytouse"`、`user.email`、500MB PostBuffer、全局 GitIgnore 及 LFS 流程。
- **PowerShell 7 Profile 自动化**：一键写入 UTF-8 控制台编码、`starship` 主题、`zoxide` 路径跳转及 `g` / `ls` / `ll` / `cat` / `find` / `grep` / `top` 高效快捷别名。
- **VS Code & Cursor 扩展与 Settings 同步**：自动安装 13 款主流开发插件并部署 `settings.json`。
- **国内高速加速镜像**：自动配置 Cargo 稀疏索引镜像、Pip 清华源及 npmmirror 镜像。
- **Windows LTSC 深度性能与隐私优化**：激活卓越性能电源计划、禁用 Telemetry 与 Bing 开始菜单搜索、优化资源管理器与 CPU/网络响应。

---

## 🖥️ 快速使用指南 (Windows)

1. 在 Windows 设备上打开本项目根目录。
2. 双击运行 **[ltsc_setup_gui.exe](file:///Users/nyamiiko/Downloads/GitHub/LTSC_Tools_Backup/ltsc_setup_gui.exe)** (建议右键选择“以管理员身份运行”)。
3. 在 GUI 界面勾选所需模块（默认全部勾选）。
4. 点击 **▶️ 一键开始全套配置**，在右侧日志框实时查看进度。
5. 配置完成后，点击 **💾 导出日志到桌面** 保存记录，建议重启系统使系统级优化生效。

---

## 🛠️ 项目架构

```text
LTSC_Tools_Backup/
├── ltsc_setup_gui.exe        # 预编译 Windows 15MB 原生单文件 GUI 程序
├── Cargo.toml                 # Rust 项目依赖与编译配置
├── assets/                    # 编译期静态嵌入的资源 Payload
│   ├── skills/                # 55+ 真实 AI Agent Skills 目录
│   ├── plugins/               # Lean-ctx & Ponytail 插件包
│   ├── AGENTS.md              # 全局 Agent 规则
│   └── mcp_config.json        # MCP 配置文件
├── src/                       # Rust 核心源代码
│   ├── main.rs                # GUI 程序入口与 960x640 窗口设置
│   ├── app.rs                 # eframe/egui 界面布局与控制逻辑
│   ├── installer.rs           # 多线程后台部署引擎与静态资源解压
│   ├── config.rs              # 100+ 软件矩阵与 SetupConfig 状态结构
│   └── utils.rs               # PowerShell/原生命令执行与日志格式化
├── Docs/                      # 软件清单与环境配置报告
└── CHANGELOG.md               # 版本更新日志
```

---

## 📜 许可与说明

本项目专为 Windows LTSC 及开发工作站深度配置打造，所有静态资源均已集成至二进制可执行文件。
