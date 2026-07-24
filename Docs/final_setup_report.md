# Windows LTSC 终极配置与 macOS 100% 对齐全量报告 (v1.0.0)

## 概要

本报告记录了将当前 macOS 工作站的完整开发环境、包矩阵、配置文件、AI Agent Skills、VS Code 扩展及网络/优化策略移植到 Windows LTSC 设备的 100% 完整交付成果。

所有部署逻辑已整合进单文件原生 Windows 软件 `ltsc_setup_gui.exe` (15 MB)。

---

## 100% 交付与覆盖汇总

1. **软件与 CLI 工具矩阵 (100% 同等能力)**
   - Winget GUI 软件库: 20+ 款核心 GUI 开发与生产力应用
   - Scoop CLI 工具栈: 40+ 款系统、网络、命令行替代与开发工具
   - Cargo 工具链: `rtk` (Rust Token Killer)、`cargo-edit`、`krokiet`、`fclones` 等
   - NPM 全局 AI 套件: `@anthropic-ai/claude-code`、`opencode-ai`、`pyright` 等
   - Pip / UV Python 库: `kimi-cli`、`ruff`、`numpy`、`torch` 等

2. **AI Agent Skills & Hooks 嵌入释出**
   - 通过 `include_dir` 静态嵌入 55+ 真实 Agent Skills (`bigquery`, `cloud-sql-*`, `ponytail`, `rtk`, `alloydb-*` 等) 及 `mcp_config.json` 到 `.exe` 内存 payload 中，运行时释放至 `%USERPROFILE%\.gemini\config`。

3. **Git 与 PowerShell 7 个人 Profile 自动化**
   - 自动绑定 `user.name = "nowaytouse"`、`500MB PostBuffer` 及全局 GitIgnore。
   - 自动在 PowerShell Profile 中接入 UTF-8 编码、`starship`、`zoxide` 及全套快捷别名 (`g`, `ls`, `ll`, `cat`, `find`, `grep`, `top`)。

4. **IDE 扩展与偏好设置**
   - 自动安装 13 款主流开发插件并部署 `settings.json` 到 Code 和 Cursor 目录。

5. **国内高速镜像**
   - Cargo 稀疏索引镜像 (清华源)、Pip 清华源及 npmmirror 镜像。

6. **Windows LTSC 深度性能与隐私优化**
   - 解锁并激活“卓越性能”电源计划。
   - 禁用 Telemetry 隐私跟踪及开始菜单 Bing 搜索。
   - 资源管理器优化（打开至此电脑、显示拓展名/隐藏文件、开启长路径）。
   - 设置 `SystemResponsiveness = 0` 及禁用网络限速。
