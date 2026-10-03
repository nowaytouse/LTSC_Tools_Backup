# Changelog

## Unreleased - 2026-10-04

- 新增项目根目录 `AGENTS.md`：参考 MFB 的错误显式化、行为锁定、范围、隐私和证据规则，适配 Rust/Windows 部署流程与代码风格。
- 保留 eframe 0.36.2、winreg 0.56 的更新；Windows 集合类型保持与 windows 0.62.2 一致，避免独立升级导致投影类型不兼容。
- 固定 CI Action 的完整提交，并显式指定 Rust stable，避免将 Action 提交号误当成工具链通道。
- Scoop 引导优先使用现有 Git；自定义 `SCOOP` 根目录的 shim 纳入当前进程命令查找。新增定向回归检查，尚待新提交的 Windows CI 和 LTSC 实机验收。
- 刷新本机 160 项工具清单并建立 6 个新名称的映射；Gradle、LLD、pinact 与 UI/UX CLI 可由现有提供程序安装，Crane 发布归档和 ngtcp2 构建库明确保留为手动项。Gradle 的选择计划包含必要 JDK；VS Code 采集不可用的警告保留，没有伪装成完整采集。
- 将旧 `uipro-cli` 映射到本机当前的 `ui-ux-pro-max-cli`，不同时全局安装两个提供同一 `uipro` 命令的包；定向回归检查锁定别名去重和 Gradle 运行时选择。
- 从成功 CI 37089825898 更新根目录 Windows EXE，源提交为 `3fd48ac`。它包含上一节清单功能，但不包含本节后续源码改动；来源、摘要和设备验收边界见 `READINESS.md`。

## Unreleased - 2026-10-03

- 默认 Mac 工具清单在 GitHub Raw 请求失败或响应无效时自动改用 GitHub Contents API；默认来源请求有界等待并仅允许 HTTPS 重定向。
- 支持独占使用自定义 HTTPS 清单来源，以及按相同校验规则导入不超过 1 MiB 的本地 JSON；导入/下载失败或取消时保留最后一份有效缓存。
- 清单更新可在 GUI 中取消或恢复默认地址；新增原始来源不可达、无效内容、映射不兼容、备用源失败和取消的回归测试。此节功能已包含在 2026-10-04 导入的成功 CI 产物中，尚未进行 LTSC 设备验收。

## 2.1.0 - 2026-08-24

- 将 GUI 限定为 Windows 原生构建，关闭 eframe 的 Web/X11/Wayland 默认功能，并内置 UAC 管理员 manifest。
- 新增 macOS 无 GUI 清单采集器、165 项当前快照、逐项 parity 规则和未知项阻断；公开快照默认省略可能暴露个人仓库命名空间的 Homebrew tap 列表。
- 扩充 PowerToys、Sysinternals、VC++ Runtime、Codex CLI、Repomix、Krew、uutils、libvips 与本机 Cargo 工具。
- 新增网络原配置快照，移除 WinHTTP 代理导入；Extreme 改为 RSS/RSC、Fast Open、CTCP 与 ECN。
- 新增 TRIM、按介质 `/O` 存储优化、Windows Features 专项任务和可选 Sandbox/.NET 3.5/Hyper-V。
- 新增可复用的受管卓越性能计划和仅修改 AC 参数的极限电源档。
- 新增持久化回滚账本与“撤销上次调优”；休眠默认关闭选项为 false，无法读到原状态时拒绝变更。
- 新增 Windows/macOS CI、每周全量 provider 在线审计和发布产物上传。
- Windows release 静态链接 VC Runtime，并在 CI 中阻断缺失 UAC 清单或重新引入动态 `VCRUNTIME*.dll` 的产物。

## 2.0.0 - 2026-08-24

- 重做 GUI：任务导航、配置卡片、独立运行记录、明确结果摘要与取消按钮。
- 用结构化 `SetupEvent::Finished` 取代“看到 End 日志才结束”的脆弱状态机。
- 新增可取消、有界输出的命令执行层；取消和超时会终止 Windows 进程树。
- 删除 `src/Scripts/00_QuickSetup.ps1`，移除核心流程中的 PowerShell 命令拼接。
- 网络、WSL、UWP、注册表、电源计划与最终审计改用 Rust/Windows 原生实现。
- WinGet 改用 curl + DISM 原生引导；Scoop 由 Rust 部署官方仓库和 shim。
- 修正执行顺序：先准备 WSL、镜像与运行时，再安装依赖它们的工具和 IDE 扩展。
- Profile 加入 Git 与 Rustup 基础包，版本升至 2.0.0。
- 全量核对官方 WinGet/Scoop 清单；修正 Podman Desktop 与 PeaZip ID，移除无可用 Windows 清单的映射，并把 ChatGPT/UWP 改为 Microsoft Store 产品 ID。
- 配置写入改为保守合并；检测到无法安全合并的 JSONC 时不会覆盖用户文件。
- Agent 资源释放范围收紧为 Skills、Plugins 与 MCP 配置。
- 新增取消、管道排空、受管区块、Profile 校验、包识别、GUID 解析与 JSON 合并测试。

## 1.1.0 - 2026-08-24

- 并发排空子进程 stdout/stderr。
- 增加 PATH 刷新、包存在检查、有限重试与失败计数。
- 根据本机 Homebrew 快照更新 Windows 等价工具矩阵。

## 1.0.0 - 2026-07-24

- 初始 Rust/egui GUI 与内置资源版本。
